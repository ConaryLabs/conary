// src/main.rs

use anyhow::Result;
use clap::{Parser, Subcommand};
use conary::packages::rpm::RpmPackage;
use conary::packages::PackageFormat;
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
    /// Import an RPM package into the database
    Import {
        /// Path to the RPM package file
        package_path: String,
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
        Some(Commands::Import {
            package_path,
            db_path,
        }) => {
            info!("Importing RPM package: {}", package_path);

            // Parse the RPM package
            let rpm = RpmPackage::parse(&package_path)?;

            info!(
                "Parsed package: {} version {} ({} files, {} dependencies)",
                rpm.name(),
                rpm.version(),
                rpm.files().len(),
                rpm.dependencies().len()
            );

            // Convert to Trove and insert into database
            let conn = conary::db::open(&db_path)?;
            let mut trove = rpm.to_trove();
            trove.insert(&conn)?;

            println!(
                "Imported package: {} version {}",
                rpm.name(),
                rpm.version()
            );
            println!("  Architecture: {}", rpm.architecture().unwrap_or("none"));
            println!("  Files: {}", rpm.files().len());
            println!("  Dependencies: {}", rpm.dependencies().len());

            // Show provenance info if available
            if let Some(source_rpm) = rpm.source_rpm() {
                println!("  Source RPM: {}", source_rpm);
            }
            if let Some(vendor) = rpm.vendor() {
                println!("  Vendor: {}", vendor);
            }

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
