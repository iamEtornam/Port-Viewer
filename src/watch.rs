use anyhow::Result;
use chrono::Local;
use colored::Colorize;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use std::collections::HashSet;
use std::io::{stdout, Write};
use tokio::time::{sleep, Duration};

use crate::collector;
use crate::renderer;

pub async fn watch_ports(show_all: bool) -> Result<()> {
    println!(
        "{}",
        "Starting port monitor (Ctrl+C to exit)...".bright_cyan()
    );
    println!();

    let mut previous_ports: HashSet<u16> = HashSet::new();
    let mut terminal_setup = false;

    loop {
        let entries = collector::collect_all_data(show_all).await?;
        let current_ports: HashSet<u16> = entries.iter().map(|e| e.port).collect();

        // Detect changes
        let new_ports: Vec<u16> = current_ports.difference(&previous_ports).copied().collect();
        let stopped_ports: Vec<u16> = previous_ports.difference(&current_ports).copied().collect();

        // Print changes
        let timestamp = Local::now().format("%H:%M:%S").to_string();

        for port in new_ports {
            if let Some(entry) = entries.iter().find(|e| e.port == port) {
                let framework = entry
                    .process
                    .framework
                    .as_ref()
                    .map(|f| f.display_name())
                    .unwrap_or("Unknown");
                let project = entry
                    .process
                    .project_name
                    .as_deref()
                    .unwrap_or(&entry.process.name);

                println!(
                    "[{}] {} {} started — {} / {} / {}",
                    timestamp.bright_black(),
                    entry.process.status.symbol(),
                    format!(":{}", port).cyan(),
                    entry.process.name.bright_white(),
                    framework.bright_white(),
                    project.bright_white()
                );
            }
        }

        for port in stopped_ports {
            println!(
                "[{}] {} {} stopped",
                timestamp.bright_black(),
                "✕".red(),
                format!(":{}", port).cyan()
            );
        }

        // Setup terminal for table redrawing
        if !terminal_setup {
            terminal_setup = true;
        } else {
            // Clear screen and move cursor to top
            execute!(
                stdout(),
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
        }

        // Render current state
        renderer::render_ports_table(&entries, show_all)?;
        stdout().flush()?;

        previous_ports = current_ports;

        // Wait 1 second
        sleep(Duration::from_secs(1)).await;
    }
}
