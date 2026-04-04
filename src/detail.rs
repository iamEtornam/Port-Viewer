use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

use crate::collector;

pub async fn show_port_detail(port: u16) -> Result<()> {
    let entries = collector::collect_all_data(true).await?;
    let entry = entries
        .iter()
        .find(|e| e.port == port)
        .context(format!("No process found listening on port {}", port))?;

    let proc = &entry.process;

    // Print detail card
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!(
        "║ {} Port {}                                         ║",
        proc.status.symbol(),
        format!(":{}", port).cyan()
    );
    println!("╠═══════════════════════════════════════════════════════════════╣");

    println!("║ {:<15} {:<45} ║", "Process:", proc.name.bright_white());
    println!(
        "║ {:<15} {:<45} ║",
        "PID:",
        proc.pid.to_string().bright_white()
    );

    if let Some(project) = &proc.project_name {
        println!("║ {:<15} {:<45} ║", "Project:", project.bright_white());
    }

    if let Some(cwd) = &proc.cwd {
        let cwd_display = if cwd.len() > 45 {
            format!("...{}", &cwd[cwd.len() - 42..])
        } else {
            cwd.clone()
        };
        println!("║ {:<15} {:<45} ║", "Path:", cwd_display.bright_black());
    }

    if let Some(fw) = &proc.framework {
        println!(
            "║ {:<15} {:<45} ║",
            "Framework:",
            format!("{} {}", fw.emoji(), fw.display_name()).bright_white()
        );
    }

    if let Some(branch) = &proc.git_branch {
        println!(
            "║ {:<15} {:<45} ║",
            "Git Branch:",
            format!("🌿 {}", branch).bright_green()
        );
    }

    println!("║ {:<15} {:<45} ║", "Uptime:", proc.uptime.bright_white());

    println!(
        "║ {:<15} {:<45} ║",
        "Memory:",
        format!("{:.1} MB", proc.memory_mb()).bright_white()
    );

    println!(
        "║ {:<15} {:<45} ║",
        "Parent PID:",
        proc.ppid.to_string().bright_white()
    );

    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║ Command:                                                      ║");

    // Word-wrap command
    let cmd_parts = wrap_text(&proc.command, 59);
    for part in cmd_parts {
        println!("║ {:<61} ║", part.bright_black());
    }

    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Interactive kill prompt
    print!(
        "{} {} [y/N]: ",
        "Kill this process?".yellow(),
        format!("(PID {})", proc.pid).bright_black()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        kill_process(proc.pid).await?;
    } else {
        println!("{}", "Cancelled.".bright_black());
    }

    Ok(())
}

async fn kill_process(pid: u32) -> Result<()> {
    println!("{} PID {}...", "Sending SIGTERM to".yellow(), pid);

    // Send SIGTERM
    Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    // Wait 3 seconds
    sleep(Duration::from_secs(3)).await;

    // Check if still alive
    let check = Command::new("ps")
        .args(["-p", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    if check.success() {
        println!("{} Sending SIGKILL...", "Process still alive.".yellow());
        Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;
    }

    println!("{}", "✓ Process terminated.".green());
    Ok(())
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
        if current_line.len() + word.len() + 1 > width
            && !current_line.is_empty()
        {
            lines.push(current_line.clone());
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
