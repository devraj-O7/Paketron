mod cli;
mod core;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { name } => {
            commands::install::install(name).await?;
        }
        Commands::Uninstall { name } => {
            commands::uninstall::uninstall(name).await?;
        }
        Commands::Search { query } => {
            commands::search::search(query).await?;
        }
        Commands::List => {
            println!("Listing packages (not implemented)");
        }
    }
    Ok(())
}
