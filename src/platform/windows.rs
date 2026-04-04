use anyhow::{Context, Result};
use std::collections::HashMap;
use sysinfo::{Pid, ProcessStatus as SysProcessStatus, ProcessesToUpdate, System};

use crate::process::ProcessStatus;

pub async fn collect_listening_ports() -> Result<Vec<(u32, u16, String)>> {
    // On Windows, we'll use sysinfo + netstat parsing
    // This is a placeholder implementation

    // Use netstat to find listening ports
    let output = tokio::process::Command::new("netstat")
        .args(["-ano", "-p", "TCP"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .await
        .context("Failed to run netstat")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    // Parse netstat output
    // Format: TCP  0.0.0.0:PORT  0.0.0.0:0  LISTENING  PID
    for line in stdout.lines().skip(4) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 && parts[0] == "TCP" && parts[3] == "LISTENING" {
            if let Some((address, port_str)) = parts[1].rsplit_once(':') {
                if let (Ok(port), Ok(pid)) = (port_str.parse::<u16>(), parts[4].parse::<u32>()) {
                    results.push((pid, port, address.to_string()));
                }
            }
        }
    }

    Ok(results)
}

pub async fn collect_process_info_batch(pids: &[u32]) -> Result<HashMap<u32, ProcessInfoData>> {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut map = HashMap::new();

    for &pid in pids {
        let sys_pid = Pid::from_u32(pid);
        if let Some(process) = sys.process(sys_pid) {
            let data = ProcessInfoData {
                pid,
                name: process.name().to_string_lossy().to_string(),
                etime: format_duration(process.run_time()),
                rss: process.memory() / 1024, // Convert to KB
                ppid: process.parent().map(|p| p.as_u32()).unwrap_or(0),
                stat: format_process_status(process.status()),
            };

            map.insert(pid, data);
        }
    }

    Ok(map)
}

pub async fn collect_process_cwds(pids: &[u32]) -> Result<HashMap<u32, String>> {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut map = HashMap::new();

    for &pid in pids {
        let sys_pid = Pid::from_u32(pid);
        if let Some(process) = sys.process(sys_pid) {
            if let Some(cwd) = process.cwd() {
                map.insert(pid, cwd.to_string_lossy().to_string());
            }
        }
    }

    Ok(map)
}

pub async fn get_process_cmdline(pid: u32) -> Result<String> {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let sys_pid = Pid::from_u32(pid);
    if let Some(process) = sys.process(sys_pid) {
        Ok(process.cmd().join(" "))
    } else {
        Err(anyhow::anyhow!("Process not found"))
    }
}

pub fn parse_process_status(stat: &str, ppid: u32) -> ProcessStatus {
    if stat.contains("Zombie") {
        ProcessStatus::Zombie
    } else if ppid == 0 || ppid == 4 {
        // System process or orphaned
        ProcessStatus::Orphaned
    } else {
        ProcessStatus::Healthy
    }
}

pub fn parse_uptime(_etime: &str) -> u64 {
    // Already parsed by sysinfo
    0
}

pub async fn kill_process_signal(pid: u32, signal: &str) -> Result<()> {
    let force = signal == "-KILL" || signal == "-9";

    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let sys_pid = Pid::from_u32(pid);
    if let Some(process) = sys.process(sys_pid) {
        if force {
            process.kill();
        } else {
            // On Windows, there's no SIGTERM equivalent
            // We'll use taskkill which is graceful by default
            tokio::process::Command::new("taskkill")
                .args(["/PID", &pid.to_string()])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await?;
        }
    }

    Ok(())
}

pub async fn check_process_alive(pid: u32) -> Result<bool> {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let sys_pid = Pid::from_u32(pid);
    Ok(sys.process(sys_pid).is_some())
}

pub async fn get_cpu_sample(pid_list: &str) -> Result<HashMap<u32, f64>> {
    let pids: Vec<u32> = pid_list.split(',').filter_map(|s| s.parse().ok()).collect();

    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut map = HashMap::new();

    for pid in pids {
        let sys_pid = Pid::from_u32(pid);
        if let Some(process) = sys.process(sys_pid) {
            map.insert(pid, process.cpu_usage() as f64);
        }
    }

    Ok(map)
}

#[derive(Debug, Clone)]
pub struct ProcessInfoData {
    pub pid: u32,
    pub name: String,
    pub etime: String,
    pub rss: u64,
    pub ppid: u32,
    pub stat: String,
}

fn format_process_status(status: SysProcessStatus) -> String {
    match status {
        SysProcessStatus::Run => "R".to_string(),
        SysProcessStatus::Sleep => "S".to_string(),
        SysProcessStatus::Stop => "T".to_string(),
        SysProcessStatus::Zombie => "Z".to_string(),
        _ => "?".to_string(),
    }
}

fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}-{}:{:02}:{:02}", days, hours, minutes, secs)
    } else if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}
