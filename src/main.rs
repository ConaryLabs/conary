// src/main.rs

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[command(name = "conary")]
#[command(author, version, about = "Modern package manager with atomic operations and rollback", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the Conary database
    Init {
        /// Database path (default: /var/lib/conary/conary.db)
        #[arg(short, long, default_value = "/var/lib/conary/conary.db")]
        db_path: String,
    },
}

fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { db_path }) => {
            info!("Initializing Conary database at: {}", db_path);
            conary::db::init(&db_path)?;
            println!("Database initialized successfully at: {}", db_path);
            Ok(())
        }
        None => {
            // No command provided, show help
            println!("Conary Package Manager v{}", env!("CARGO_PKG_VERSION"));
            println!("Run 'conary --help' for usage information");
            Ok(())
        }
    }
}
