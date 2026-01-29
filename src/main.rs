mod cli;
mod git;
mod path;
mod repo;
mod worktree;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

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
        Commands::Worktree { cmd } => {
            use cli::WorktreeCommands;
            let repo_info = repo::RepoInfo::detect()?;

            match cmd {
                WorktreeCommands::Create { branch, base } => {
                    worktree::create_worktree(&repo_info, &branch, base.as_deref())?;
                }
            }
        }
    }

    Ok(())
}
