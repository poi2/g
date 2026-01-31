mod branch;
mod cli;
mod config;
mod fzf;
mod git;
mod path;
mod repo;
mod repository;
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
                RepositoryCommands::Ls => {
                    repository::list_repositories(&config)?;
                }
                RepositoryCommands::Switch {
                    repository: repo,
                    interactive,
                } => {
                    repository::switch_repository(&config, repo.as_deref(), interactive)?;
                }
                RepositoryCommands::Delete {
                    repository: repo,
                    interactive,
                } => {
                    repository::delete_repository(&config, repo.as_deref(), interactive)?;
                }
                RepositoryCommands::New { repository: repo } => {
                    repository::new_repository(&config, &repo)?;
                }
            }
        }
        Commands::SonicWorktree { cmd } => {
            use cli::WorktreeCommands;
            let repo_info = repo::RepoInfo::detect()?;

            match cmd {
                WorktreeCommands::New { branch, base } => {
                    let _ = worktree::create_worktree(&repo_info, &branch, base.as_deref())?;
                }
                WorktreeCommands::Ls => {
                    worktree::list_worktrees(&repo_info)?;
                }
                WorktreeCommands::Mv { old, new } => {
                    worktree::move_worktree(&repo_info, old.as_deref(), &new)?;
                }
                WorktreeCommands::Switch {
                    branch,
                    interactive,
                } => {
                    worktree::switch_worktree(&repo_info, branch.as_deref(), interactive)?;
                }
                WorktreeCommands::Delete {
                    branch,
                    force,
                    all,
                    interactive,
                } => {
                    worktree::delete_worktrees(
                        &repo_info,
                        branch.as_deref(),
                        force,
                        all,
                        interactive,
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
        Commands::SonicBranch { cmd } => {
            use cli::BranchCommands;
            let repo_info = repo::RepoInfo::detect()?;

            match cmd {
                BranchCommands::Ls { options } => {
                    branch::list_branches(&repo_info.repo_root, &options)?;
                }
                BranchCommands::New { branch: branch_name } => {
                    branch::new_branch(&repo_info.repo_root, &branch_name)?;
                }
                BranchCommands::Mv { old, new } => {
                    branch::move_branch(&repo_info.repo_root, old.as_deref(), &new)?;
                }
                BranchCommands::Delete {
                    branch: branch_name,
                    force,
                    all,
                    interactive,
                } => {
                    branch::delete_branches(
                        &repo_info.repo_root,
                        branch_name.as_deref(),
                        force,
                        all,
                        interactive,
                    )?;
                }
            }
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
