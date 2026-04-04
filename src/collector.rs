use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

use crate::framework;
use crate::platform;
use crate::process::{DockerContainer, PortEntry, ProcessInfo};

pub async fn collect_all_data(show_all: bool) -> Result<Vec<PortEntry>> {
    // Run all data collection steps concurrently
    let (port_result, docker_result) = tokio::join!(
        platform::collect_listening_ports(),
        collect_docker_containers(),
    );

    let port_data = port_result.context("Failed to collect listening ports")?;
    let docker_containers = docker_result.unwrap_or_default();

    // Extract all PIDs
    let pids: Vec<u32> = port_data.iter().map(|(pid, _, _)| *pid).collect();

    if pids.is_empty() {
        return Ok(Vec::new());
    }

    // Collect process info and CWDs concurrently
    let (ps_result, cwd_result) = tokio::join!(
        platform::collect_process_info_batch(&pids),
        platform::collect_process_cwds(&pids),
    );

    let process_info_data = ps_result.context("Failed to collect process info")?;
    let cwd_map = cwd_result.context("Failed to collect process CWDs")?;

    // Build ProcessInfo from platform data
    let mut process_info_tasks = Vec::new();
    for (pid, data) in process_info_data {
        let status = platform::parse_process_status(&data.stat, data.ppid);
        let uptime_seconds = platform::parse_uptime(&data.etime);

        let task = async move {
            let cmdline = platform::get_process_cmdline(pid)
                .await
                .unwrap_or_else(|_| data.name.clone());

            (
                pid,
                ProcessInfo {
                    pid: data.pid,
                    name: data.name,
                    command: cmdline,
                    uptime: format_uptime(uptime_seconds),
                    uptime_seconds,
                    memory_kb: data.rss,
                    ppid: data.ppid,
                    status,
                    cwd: None,
                    project_name: None,
                    framework: None,
                    git_branch: None,
                },
            )
        };

        process_info_tasks.push(task);
    }

    let process_info_results = futures::future::join_all(process_info_tasks).await;
    let mut process_info = HashMap::new();
    for (pid, info) in process_info_results {
        process_info.insert(pid, info);
    }

    // Build port entries
    let mut entries = Vec::new();
    for (pid, port, address) in port_data {
        if let Some(mut proc_info) = process_info.get(&pid).cloned() {
            // Add CWD and enrich with framework info
            if let Some(cwd) = cwd_map.get(&pid) {
                proc_info.cwd = Some(cwd.clone());
                proc_info.project_name = extract_project_name(cwd);

                // Detect framework
                proc_info.framework = framework::detect_framework(cwd, &proc_info.command).await;

                // Get git branch if in a git repo
                if let Ok(branch) = get_git_branch(cwd).await {
                    proc_info.git_branch = Some(branch);
                }
            }

            // Check if this port is served by a Docker container
            if let Some(container) = find_docker_container(&docker_containers, port) {
                if let Some(service) = container.detect_service() {
                    proc_info.framework = Some(crate::framework::Framework::Unknown(format!(
                        "Docker · {}",
                        service
                    )));
                }
            }

            let entry = PortEntry {
                port,
                pid,
                address,
                process: proc_info,
            };

            // Filter based on show_all flag
            if show_all || should_show_process(&entry.process) {
                entries.push(entry);
            }
        }
    }

    // Sort by port number
    entries.sort_by_key(|e| e.port);

    Ok(entries)
}

async fn collect_docker_containers() -> Result<Vec<DockerContainer>> {
    if which::which("docker").is_err() {
        return Ok(Vec::new());
    }

    let output = Command::new("docker")
        .args(["ps", "--format", "json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut containers = Vec::new();

    for line in stdout.lines() {
        if let Ok(container) = parse_docker_json(line) {
            containers.push(container);
        }
    }

    Ok(containers)
}

fn parse_docker_json(json_line: &str) -> Result<DockerContainer> {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct DockerPs {
        #[serde(rename = "ID")]
        id: String,
        names: String,
        image: String,
        ports: String,
    }

    let ps: DockerPs = serde_json::from_str(json_line)?;
    let ports = parse_docker_ports(&ps.ports);

    Ok(DockerContainer {
        id: ps.id,
        name: ps.names,
        image: ps.image,
        ports,
    })
}

fn parse_docker_ports(ports_str: &str) -> Vec<crate::process::PortMapping> {
    let re = Regex::new(r"0\.0\.0\.0:(\d+)->(\d+)/tcp").unwrap();
    let mut mappings = Vec::new();

    for cap in re.captures_iter(ports_str) {
        if let (Some(host), Some(container)) = (cap.get(1), cap.get(2)) {
            if let (Ok(host_port), Ok(container_port)) =
                (host.as_str().parse(), container.as_str().parse())
            {
                mappings.push(crate::process::PortMapping {
                    host_port,
                    container_port,
                });
            }
        }
    }

    mappings
}

fn find_docker_container(containers: &[DockerContainer], port: u16) -> Option<&DockerContainer> {
    containers
        .iter()
        .find(|c| c.ports.iter().any(|p| p.host_port == port))
}

async fn get_git_branch(cwd: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["-C", cwd, "branch", "--show-current"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(anyhow::anyhow!("Not a git repo"))
    }
}

fn extract_project_name(cwd: &str) -> Option<String> {
    std::path::Path::new(cwd)
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

fn should_show_process(proc: &ProcessInfo) -> bool {
    !proc.is_system_process() && proc.is_dev_process()
}
