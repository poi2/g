mod cli;
mod git;
mod path;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Repository { cmd } => {
            use cli::RepositoryCommands;
            match cmd {
                RepositoryCommands::Clone { url } => {
                    git::clone_repository(&url)?;
                }
            }
        }
    }

    Ok(())
}
