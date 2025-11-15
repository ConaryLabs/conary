// build.rs

use clap::{Arg, Command};
use clap_mangen::Man;
use std::env;
use std::fs;
use std::path::PathBuf;

fn build_cli() -> Command {
    Command::new("conary")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Conary Contributors")
        .about("Modern package manager with atomic operations and rollback")
        .subcommand_required(false)
        .subcommand(
            Command::new("init")
                .about("Initialize the Conary database")
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .value_name("PATH")
                        .default_value("/var/lib/conary/conary.db")
                        .help("Database path"),
                ),
        )
        .subcommand(
            Command::new("install")
                .about("Install a package (auto-detects RPM, DEB, Arch formats)")
                .arg(Arg::new("package_path").required(true).help("Path to the package file"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                )
                .arg(
                    Arg::new("root")
                        .short('r')
                        .long("root")
                        .default_value("/")
                        .help("Install root directory"),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove an installed package")
                .arg(Arg::new("package_name").required(true).help("Package name to remove"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                ),
        )
        .subcommand(
            Command::new("query")
                .about("Query installed packages")
                .arg(Arg::new("pattern").help("Package name pattern (optional)"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                ),
        )
        .subcommand(
            Command::new("history")
                .about("Show changeset history")
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                ),
        )
        .subcommand(
            Command::new("rollback")
                .about("Rollback a changeset")
                .arg(Arg::new("changeset_id").required(true).help("Changeset ID to rollback"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                )
                .arg(
                    Arg::new("root")
                        .short('r')
                        .long("root")
                        .default_value("/")
                        .help("Install root directory"),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify installed files match their stored hashes")
                .arg(Arg::new("package").help("Package name to verify (optional)"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                )
                .arg(
                    Arg::new("root")
                        .short('r')
                        .long("root")
                        .default_value("/")
                        .help("Install root directory"),
                ),
        )
        .subcommand(
            Command::new("depends")
                .about("Show dependencies of a package")
                .arg(Arg::new("package_name").required(true).help("Package name"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                ),
        )
        .subcommand(
            Command::new("rdepends")
                .about("Show reverse dependencies (what depends on this package)")
                .arg(Arg::new("package_name").required(true).help("Package name"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                ),
        )
        .subcommand(
            Command::new("whatbreaks")
                .about("Show what packages would break if this package is removed")
                .arg(Arg::new("package_name").required(true).help("Package name"))
                .arg(
                    Arg::new("db_path")
                        .short('d')
                        .long("db-path")
                        .default_value("/var/lib/conary/conary.db"),
                ),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completion scripts")
                .arg(
                    Arg::new("shell")
                        .required(true)
                        .value_parser(["bash", "zsh", "fish", "powershell"])
                        .help("Shell type"),
                ),
        )
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Create man directory
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let man_dir = out_dir.join("man");
    fs::create_dir_all(&man_dir).expect("Failed to create man directory");

    // Generate main man page
    let cmd = build_cli();
    let man = Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)
        .expect("Failed to render man page");

    let man_path = man_dir.join("conary.1");
    fs::write(&man_path, buffer).expect("Failed to write man page");

    println!("cargo:warning=Man page generated at {}", man_path.display());
}
