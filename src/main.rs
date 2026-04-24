mod commands;
mod process_info;

use clap::{Parser, Subcommand};
use commands::{ps, kill_process, top_mode, tree_mode, process_info_cmd};

/// Process Manager - A CLI tool for managing system processes
#[derive(Parser)]
#[command(name = "pm", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all running processes
    Ps {
        /// Filter by process name (case-insensitive substring match)
        #[arg(short, long)]
        filter: Option<String>,

        /// Sort by: pid, name, cpu, memory, status (default: pid)
        #[arg(short, long, default_value = "pid")]
        sort: String,

        /// Show only top N processes
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// Kill a process by PID
    Kill {
        /// Process ID to kill
        pid: u32,

        /// Force kill (SIGKILL instead of graceful termination)
        #[arg(short, long)]
        force: bool,
    },
    /// Real-time process monitor
    Top {
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "2")]
        interval: u64,

        /// Sort by: pid, name, cpu, memory (default: cpu)
        #[arg(short, long, default_value = "cpu")]
        sort: String,
    },
    /// Show process tree
    Tree {
        /// Filter by process name
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Show detailed information about a specific process
    Info {
        /// Process ID
        pid: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ps { filter, sort, limit } => {
            ps::list_processes(&filter, &sort, limit);
        }
        Commands::Kill { pid, force } => {
            kill_process::kill(pid, force);
        }
        Commands::Top { interval, sort } => {
            top_mode::run(interval, &sort);
        }
        Commands::Tree { filter } => {
            tree_mode::show_tree(&filter);
        }
        Commands::Info { pid } => {
            process_info_cmd::show_info(pid);
        }
    }
}
