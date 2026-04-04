use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

use crate::process::ProcessStatus;

pub async fn collect_listening_ports() -> Result<Vec<(u32, u16, String)>> {
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

pub async fn collect_process_info_batch(pids: &[u32]) -> Result<HashMap<u32, ProcessInfoData>> {
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
            let data = ProcessInfoData {
                pid,
                name: parts[1].to_string(),
                etime: parts[2].to_string(),
                rss: parts[3].parse::<u64>().unwrap_or(0),
                ppid: parts[4].parse::<u32>().unwrap_or(0),
                stat: parts[5].to_string(),
            };

            map.insert(pid, data);
        }
    }

    Ok(map)
}

pub async fn collect_process_cwds(pids: &[u32]) -> Result<HashMap<u32, String>> {
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

pub async fn get_process_cmdline(pid: u32) -> Result<String> {
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

pub fn parse_process_status(stat: &str, ppid: u32) -> ProcessStatus {
    if stat.contains('Z') {
        ProcessStatus::Zombie
    } else if ppid == 1 {
        ProcessStatus::Orphaned
    } else {
        ProcessStatus::Healthy
    }
}

pub fn parse_uptime(etime: &str) -> u64 {
    let parts: Vec<&str> = etime.split(&['-', ':'][..]).collect();

    match parts.len() {
        2 => parts[0].parse::<u64>().unwrap_or(0) * 60 + parts[1].parse::<u64>().unwrap_or(0),
        3 => {
            parts[0].parse::<u64>().unwrap_or(0) * 3600
                + parts[1].parse::<u64>().unwrap_or(0) * 60
                + parts[2].parse::<u64>().unwrap_or(0)
        }
        4 => {
            parts[0].parse::<u64>().unwrap_or(0) * 86400
                + parts[1].parse::<u64>().unwrap_or(0) * 3600
                + parts[2].parse::<u64>().unwrap_or(0) * 60
                + parts[3].parse::<u64>().unwrap_or(0)
        }
        _ => 0,
    }
}

pub async fn kill_process_signal(pid: u32, signal: &str) -> Result<()> {
    Command::new("kill")
        .arg(signal)
        .arg(pid.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;
    Ok(())
}

pub async fn check_process_alive(pid: u32) -> Result<bool> {
    let status = Command::new("ps")
        .args(["-p", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    Ok(status.success())
}

pub async fn get_cpu_sample(pid_list: &str) -> Result<HashMap<u32, f64>> {
    let output = Command::new("ps")
        .args(["-o", "pid,%cpu", "-p", pid_list])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut map = HashMap::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(pid), Ok(cpu)) = (parts[0].parse::<u32>(), parts[1].parse::<f64>()) {
                map.insert(pid, cpu);
            }
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
