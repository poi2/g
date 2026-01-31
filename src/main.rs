mod branch;
mod cli;
mod config;
mod fzf;
mod git;
mod path;
mod repo;
mod worktree;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let config = config::Config::load().unwrap_or_else(|_| config::Config {
        root: None,
        aliases: std::collections::HashMap::new(),
    });

    let mut args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        if let Some(resolved) = config.resolve_alias(&args[1]) {
            args.splice(1..2, resolved);
        }
    }

    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::SonicRepository { cmd } => {
            use cli::RepositoryCommands;
            match cmd {
                RepositoryCommands::Clone { url } => {
                    git::clone_repository(&url)?;
                }
            }
        }
        Commands::SonicWorktree { cmd } => {
            use cli::WorktreeCommands;
            let repo_info = repo::RepoInfo::detect()?;

            match cmd {
                WorktreeCommands::Create { branch, base } => {
                    let _ = worktree::create_worktree(&repo_info, &branch, base.as_deref())?;
                }
                WorktreeCommands::List => {
                    worktree::list_worktrees(&repo_info)?;
                }
                WorktreeCommands::Delete { branch } => {
                    worktree::delete_worktree(&repo_info, &branch, false)?;
                }
                WorktreeCommands::ForceDelete { branch } => {
                    worktree::delete_worktree(&repo_info, &branch, true)?;
                }
                WorktreeCommands::Switch {
                    branch,
                    interactive,
                    create,
                    base,
                } => {
                    worktree::switch_worktree(
                        &repo_info,
                        branch.as_deref(),
                        interactive,
                        create,
                        base.as_deref(),
                    )?;
                }
            }
        }
        Commands::SonicSwitch {
            branch,
            interactive,
            args,
        } => {
            let repo_info = repo::RepoInfo::detect()?;
            branch::switch_branch(&repo_info.repo_root, branch.as_deref(), interactive, &args)?;
        }
        Commands::External(args) => {
            use std::process::Command;
            let status = Command::new("git")
                .args(&args)
                .status()
                .map_err(|e| anyhow::anyhow!("Failed to execute git: {}", e))?;

            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    }

    Ok(())
}
