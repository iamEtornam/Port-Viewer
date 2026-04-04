use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};
use std::process::Stdio;
use tabled::{settings::Style, Table, Tabled};
use tokio::process::Command;
use tokio::time::{sleep, Duration};

use crate::collector;
use crate::process::ProcessStatus;

#[derive(Tabled)]
struct OrphanRow {
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "PROCESS")]
    process: String,
    #[tabled(rename = "PROJECT")]
    project: String,
    #[tabled(rename = "UPTIME")]
    uptime: String,
    #[tabled(rename = "STATUS")]
    status: String,
}

pub async fn clean_orphans() -> Result<()> {
    let entries = collector::collect_all_data(true).await?;

    // Find orphaned dev processes
    let orphans: Vec<_> = entries
        .iter()
        .filter(|e| {
            (e.process.status == ProcessStatus::Orphaned
                || e.process.status == ProcessStatus::Zombie)
                && e.process.is_dev_process()
        })
        .collect();

    if orphans.is_empty() {
        println!("{}", "✓ No orphaned processes found.".green());
        return Ok(());
    }

    // Display orphans
    println!(
        "{} {} orphaned process{}:",
        "Found".yellow(),
        orphans.len().to_string().bright_yellow(),
        if orphans.len() == 1 { "" } else { "es" }
    );
    println!();

    let rows: Vec<OrphanRow> = orphans
        .iter()
        .map(|e| OrphanRow {
            pid: e.pid.to_string(),
            process: e.process.name.clone(),
            project: e
                .process
                .project_name
                .clone()
                .unwrap_or_else(|| "-".to_string()),
            uptime: e.process.uptime.clone(),
            status: format!("{}", e.process.status),
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{}", table);
    println!();

    // Interactive cleanup
    for entry in orphans {
        let proc = &entry.process;
        print!(
            "{} PID {} {} [y/N/a(ll)/q(uit)]: ",
            "Kill".yellow(),
            proc.pid.to_string().bright_white(),
            proc.name.bright_black()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "y" => {
                kill_process_graceful(proc.pid).await?;
            }
            "a" => {
                // Kill all remaining
                println!("{}", "Killing all orphans...".yellow());
                for e in &entries {
                    if (e.process.status == ProcessStatus::Orphaned
                        || e.process.status == ProcessStatus::Zombie)
                        && e.process.is_dev_process()
                    {
                        kill_process_graceful(e.pid).await?;
                    }
                }
                break;
            }
            "q" => {
                println!("{}", "Cancelled.".bright_black());
                break;
            }
            _ => {
                println!("{}", "Skipped.".bright_black());
            }
        }
    }

    println!("\n{}", "✓ Cleanup complete.".green());
    Ok(())
}

async fn kill_process_graceful(pid: u32) -> Result<()> {
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
        // Send SIGKILL
        Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;
    }

    println!("  {} PID {}", "✓".green(), pid);
    Ok(())
}
