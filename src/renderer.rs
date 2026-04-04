use anyhow::Result;
use colored::Colorize;
use tabled::{
    settings::{object::Rows, style::Style, Modify, Width},
    Table, Tabled,
};

use crate::process::PortEntry;

#[derive(Tabled)]
struct PortRow {
    #[tabled(rename = "PORT")]
    port: String,
    #[tabled(rename = "PROCESS")]
    process: String,
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "PROJECT")]
    project: String,
    #[tabled(rename = "FRAMEWORK")]
    framework: String,
    #[tabled(rename = "UPTIME")]
    uptime: String,
    #[tabled(rename = "STATUS")]
    status: String,
}

pub fn render_ports_table(entries: &[PortEntry], show_all: bool) -> Result<()> {
    if entries.is_empty() {
        println!("{}", "No listening ports found.".yellow());
        return Ok(());
    }

    let rows: Vec<PortRow> = entries
        .iter()
        .map(|entry| {
            let port_str = format!(":{}", entry.port).cyan().to_string();
            let process_name = entry.process.name.clone();
            let pid_str = entry.pid.to_string();
            let project = entry
                .process
                .project_name
                .clone()
                .unwrap_or_else(|| "-".to_string());
            let framework = match &entry.process.framework {
                Some(fw) => format!("{} {}", fw.emoji(), fw.display_name()),
                None => "-".to_string(),
            };
            let uptime = entry.process.uptime.clone();
            let status = format!("{}", entry.process.status);

            PortRow {
                port: port_str,
                process: process_name,
                pid: pid_str,
                project,
                framework,
                uptime,
                status,
            }
        })
        .collect();

    let mut table = Table::new(rows);
    table
        .with(Style::rounded())
        .with(Modify::new(Rows::first()).with(Width::wrap(100)));

    println!("{}", table);

    // Footer
    let count = entries.len();
    let filter_hint = if show_all {
        "".to_string()
    } else {
        format!(" · {} to show everything", "--all".bright_black())
    };

    println!(
        "\n{} {} active{}",
        count.to_string().bright_cyan(),
        if count == 1 { "port" } else { "ports" },
        filter_hint
    );

    println!("Run {} for details", "ports <number>".bright_black());

    Ok(())
}
