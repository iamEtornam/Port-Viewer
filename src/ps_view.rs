use anyhow::Result;
use colored::Colorize;
use std::collections::HashMap;
use tabled::{settings::Style, Table, Tabled};
#[cfg(unix)]
use tokio::time::{sleep, Duration};

use crate::collector;
use crate::platform;
use crate::process::ProcessInfo;

#[derive(Tabled)]
struct ProcessRow {
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "PROCESS")]
    process: String,
    #[tabled(rename = "CPU%")]
    cpu: String,
    #[tabled(rename = "MEM")]
    mem: String,
    #[tabled(rename = "PROJECT")]
    project: String,
    #[tabled(rename = "FRAMEWORK")]
    framework: String,
    #[tabled(rename = "UPTIME")]
    uptime: String,
    #[tabled(rename = "WHAT")]
    what: String,
}

pub async fn show_processes(show_all: bool) -> Result<()> {
    let entries = collector::collect_all_data(true).await?;
    let mut all_processes: Vec<&ProcessInfo> = entries.iter().map(|e| &e.process).collect();

    // Get unique processes (may have multiple ports)
    let mut seen_pids = std::collections::HashSet::new();
    all_processes.retain(|p| {
        if seen_pids.contains(&p.pid) {
            false
        } else {
            seen_pids.insert(p.pid);
            true
        }
    });

    // Filter if needed
    if !show_all {
        all_processes.retain(|p| p.is_dev_process() && !p.is_system_process());
    }

    if all_processes.is_empty() {
        println!("{}", "No processes found.".yellow());
        return Ok(());
    }

    // Measure CPU
    let cpu_map = measure_cpu_usage(&all_processes).await?;

    // Group Docker processes
    let mut docker_count = 0;
    let mut non_docker: Vec<&ProcessInfo> = Vec::new();

    for proc in &all_processes {
        if proc.is_docker_process() {
            docker_count += 1;
        } else {
            non_docker.push(proc);
        }
    }

    let mut rows: Vec<ProcessRow> = non_docker
        .iter()
        .map(|proc| {
            let cpu = cpu_map.get(&proc.pid).copied().unwrap_or(0.0);
            let project = proc.project_name.clone().unwrap_or_else(|| "-".to_string());
            let framework = match &proc.framework {
                Some(fw) => format!("{} {}", fw.emoji(), fw.display_name()),
                None => "-".to_string(),
            };
            let what = summarize_command(&proc.command);

            ProcessRow {
                pid: proc.pid.to_string(),
                process: proc.name.clone(),
                cpu: format!("{:.1}", cpu),
                mem: format!("{:.0}M", proc.memory_mb()),
                project,
                framework,
                uptime: proc.uptime.clone(),
                what,
            }
        })
        .collect();

    // Add Docker summary row
    if docker_count > 0 {
        rows.push(ProcessRow {
            pid: "-".to_string(),
            process: "docker".to_string(),
            cpu: "-".to_string(),
            mem: "-".to_string(),
            project: "-".to_string(),
            framework: format!("🐳 Docker · {} processes", docker_count),
            uptime: "-".to_string(),
            what: "Container runtime".to_string(),
        });
    }

    let mut table = Table::new(rows);
    table.with(Style::rounded());

    println!("{}", table);

    let total = all_processes.len();
    println!(
        "\n{} {} running",
        total.to_string().bright_cyan(),
        if total == 1 { "process" } else { "processes" }
    );

    Ok(())
}

async fn measure_cpu_usage(processes: &[&ProcessInfo]) -> Result<HashMap<u32, f64>> {
    let pids: Vec<u32> = processes.iter().map(|p| p.pid).collect();
    if pids.is_empty() {
        return Ok(HashMap::new());
    }

    let pid_list = pids
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

    #[cfg(unix)]
    {
        let sample1 = platform::get_cpu_sample(&pid_list).await?;
        sleep(Duration::from_millis(200)).await;
        let sample2 = platform::get_cpu_sample(&pid_list).await?;

        let mut cpu_map = HashMap::new();
        for (pid, cpu1) in sample1 {
            if let Some(cpu2) = sample2.get(&pid) {
                let diff = cpu2 - cpu1;
                let cpu_percent = (diff / 0.2).max(0.0);
                cpu_map.insert(pid, cpu_percent);
            }
        }

        Ok(cpu_map)
    }

    #[cfg(windows)]
    {
        platform::get_cpu_sample(&pid_list).await
    }
}

fn summarize_command(cmd: &str) -> String {
    // Strip binary path and show meaningful args
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return "-".to_string();
    }

    // Get just the binary name
    let binary = parts[0].split('/').next_back().unwrap_or(parts[0]);

    // Get meaningful args (skip flags)
    let args: Vec<&str> = parts
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with('-'))
        .take(3)
        .copied()
        .collect();

    if args.is_empty() {
        binary.to_string()
    } else {
        format!("{} {}", binary, args.join(" "))
    }
}
