// SPDX-License-Identifier: Apache-2.0
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name    = "infolyt",
    version,
    about   = "Complete filesystem intelligence — index, analyse, organise.",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format
    #[arg(long, global = true, default_value = "table",
          value_parser = ["table", "json", "csv"])]
    output: String,

    /// Suppress progress output
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Simulate actions without executing
    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory or drive and build the file index
    Scan {
        /// Paths to scan
        paths: Vec<std::path::PathBuf>,
        /// Watch for filesystem changes continuously
        #[arg(long)]
        watch: bool,
        /// Exclude patterns (glob)
        #[arg(long)]
        exclude: Vec<String>,
    },
    /// Find and manage duplicate files
    Duplicates {
        #[command(subcommand)]
        cmd: DuplicatesCmd,
    },
    /// Generate file organisation recommendations
    Recommend {
        /// Automatically apply safe recommendations
        #[arg(long)]
        apply_safe: bool,
    },
    /// Manage quarantined files
    Quarantine {
        #[command(subcommand)]
        cmd: QuarantineCmd,
    },
    /// Show daemon and index status
    Status,
}

#[derive(Subcommand)]
enum DuplicatesCmd {
    /// List all duplicate groups
    List {
        #[arg(long, default_value = "0")]
        min_wasted_mb: u64,
    },
}

#[derive(Subcommand)]
enum QuarantineCmd {
    /// List quarantined files
    List,
    /// Restore a file from quarantine
    Restore { id: u64 },
    /// Permanently delete quarantined files
    Purge {
        #[arg(long)]
        older_than_days: Option<u64>,
        #[arg(long)]
        all: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt::init();

    match cli.command {
        Commands::Scan { paths, watch, exclude } => {
            println!("Scanning {:?}...", paths);
            // TODO: connect to daemon and start scan
        }
        Commands::Status => {
            println!("Infolyt daemon: running");
        }
        _ => {
            println!("Coming soon.");
        }
    }

    Ok(())
}