use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "paketron")]
#[command(about = "A modern package manager for Windows", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a package
    Install {
        /// Name of the package to install
        name: String,
    },
    /// Uninstall a package
    Uninstall {
        /// Name of the package to uninstall
        name: String,
    },
    /// Search for packages
    Search {
        /// Query string
        query: String,
    },
    /// List installed packages
    List,
}
