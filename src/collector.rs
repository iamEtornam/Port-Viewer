use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

use crate::framework;
use crate::process::{DockerContainer, PortEntry, ProcessInfo, ProcessStatus};

pub async fn collect_all_data(show_all: bool) -> Result<Vec<PortEntry>> {
    // Run all data collection steps concurrently
    let (lsof_result, docker_result) =
        tokio::join!(collect_listening_ports(), collect_docker_containers(),);

    let port_data = lsof_result.context("Failed to collect listening ports")?;
    let docker_containers = docker_result.unwrap_or_default();

    // Extract all PIDs
    let pids: Vec<u32> = port_data.iter().map(|(pid, _, _)| *pid).collect();

    if pids.is_empty() {
        return Ok(Vec::new());
    }

    // Collect process info and CWDs concurrently
    let (ps_result, cwd_result) =
        tokio::join!(collect_process_info(&pids), collect_process_cwds(&pids),);

    let process_info = ps_result.context("Failed to collect process info")?;
    let cwd_map = cwd_result.context("Failed to collect process CWDs")?;

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

async fn collect_listening_ports() -> Result<Vec<(u32, u16, String)>> {
    let output = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-P", "-n"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .context("Failed to run lsof")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    // Parse lsof output
    // Format: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME
    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }

        if let Ok(pid) = parts[1].parse::<u32>() {
            let name = parts[8];
            if let Some((address, port_str)) = name.rsplit_once(':') {
                if let Ok(port) = port_str.parse::<u16>() {
                    results.push((pid, port, address.to_string()));
                }
            }
        }
    }

    Ok(results)
}

async fn collect_process_info(pids: &[u32]) -> Result<HashMap<u32, ProcessInfo>> {
    if pids.is_empty() {
        return Ok(HashMap::new());
    }

    let pid_list = pids
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let output = Command::new("ps")
        .args(["-o", "pid,comm,etime,rss,ppid,stat", "-p", &pid_list])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .context("Failed to run ps")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut map = HashMap::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        if let Ok(pid) = parts[0].parse::<u32>() {
            let name = parts[1].to_string();
            let etime = parts[2].to_string();
            let rss = parts[3].parse::<u64>().unwrap_or(0);
            let ppid = parts[4].parse::<u32>().unwrap_or(0);
            let stat = parts[5];

            let status = if stat.contains('Z') {
                ProcessStatus::Zombie
            } else if ppid == 1 {
                ProcessStatus::Orphaned
            } else {
                ProcessStatus::Healthy
            };

            let uptime_seconds = parse_uptime(&etime);

            let info = ProcessInfo {
                pid,
                name: name.clone(),
                command: name,
                uptime: format_uptime(uptime_seconds),
                uptime_seconds,
                memory_kb: rss,
                ppid,
                status,
                cwd: None,
                project_name: None,
                framework: None,
                git_branch: None,
            };

            map.insert(pid, info);
        }
    }

    // Get full command lines
    for (pid, info) in map.iter_mut() {
        if let Ok(cmdline) = get_process_cmdline(*pid).await {
            info.command = cmdline;
        }
    }

    Ok(map)
}

async fn collect_process_cwds(pids: &[u32]) -> Result<HashMap<u32, String>> {
    if pids.is_empty() {
        return Ok(HashMap::new());
    }

    let pid_list = pids
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let output = Command::new("lsof")
        .args(["-d", "cwd", "-a", "-p", &pid_list, "-Fn"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut map = HashMap::new();
    let mut current_pid: Option<u32> = None;

    // Parse lsof -Fn output
    for line in stdout.lines() {
        if let Some(stripped) = line.strip_prefix('p') {
            current_pid = stripped.parse().ok();
        } else if let Some(path) = line.strip_prefix('n') {
            if let Some(pid) = current_pid {
                map.insert(pid, path.to_string());
            }
        }
    }

    Ok(map)
}

async fn collect_docker_containers() -> Result<Vec<DockerContainer>> {
    // Check if docker is available
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

async fn get_process_cmdline(pid: u32) -> Result<String> {
    // Try /proc first (Linux)
    #[cfg(target_os = "linux")]
    {
        let path = format!("/proc/{}/cmdline", pid);
        if let Ok(content) = tokio::fs::read(&path).await {
            let cmdline = String::from_utf8_lossy(&content)
                .replace('\0', " ")
                .trim()
                .to_string();
            if !cmdline.is_empty() {
                return Ok(cmdline);
            }
        }
    }

    // Fallback to ps
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
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

fn parse_uptime(etime: &str) -> u64 {
    // Parse formats like: "10:30" (MM:SS), "1:10:30" (HH:MM:SS), "2-10:30:15" (DD-HH:MM:SS)
    let parts: Vec<&str> = etime.split(&['-', ':'][..]).collect();

    match parts.len() {
        2 => {
            // MM:SS
            parts[0].parse::<u64>().unwrap_or(0) * 60 + parts[1].parse::<u64>().unwrap_or(0)
        }
        3 => {
            // HH:MM:SS
            parts[0].parse::<u64>().unwrap_or(0) * 3600
                + parts[1].parse::<u64>().unwrap_or(0) * 60
                + parts[2].parse::<u64>().unwrap_or(0)
        }
        4 => {
            // DD-HH:MM:SS
            parts[0].parse::<u64>().unwrap_or(0) * 86400
                + parts[1].parse::<u64>().unwrap_or(0) * 3600
                + parts[2].parse::<u64>().unwrap_or(0) * 60
                + parts[3].parse::<u64>().unwrap_or(0)
        }
        _ => 0,
    }
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
