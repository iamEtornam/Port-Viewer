use anyhow::Result;
use clap::{Parser, Subcommand};

mod clean;
mod collector;
mod detail;
mod framework;
mod process;
mod ps_view;
mod renderer;
mod watch;

#[derive(Parser)]
#[command(
    name = "ports",
    author,
    version,
    about = "A beautiful, blazing-fast CLI tool to inspect and manage processes listening on your machine's ports",
    long_about = None
)]
struct Cli {
    /// Show all ports including system services
    #[arg(long, global = true)]
    all: bool,

    /// Port number to inspect in detail
    #[arg(value_name = "PORT")]
    port: Option<u16>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show all running dev processes
    Ps {
        /// Show all processes, not just dev processes
        #[arg(long)]
        all: bool,
    },
    /// Real-time monitoring (poll every 1s)
    Watch,
    /// Find and interactively kill orphaned processes
    Clean,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Ps { all }) => {
            ps_view::show_processes(all).await?;
        }
        Some(Commands::Watch) => {
            watch::watch_ports(cli.all).await?;
        }
        Some(Commands::Clean) => {
            clean::clean_orphans().await?;
        }
        None => {
            if let Some(port) = cli.port {
                detail::show_port_detail(port).await?;
            } else {
                // Default view: show ports
                let entries = collector::collect_all_data(cli.all).await?;
                renderer::render_ports_table(&entries, cli.all)?;
            }
        }
    }

    Ok(())
}
