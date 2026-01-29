use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "g")]
#[command(about = "Git Worktree Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "repo")]
    #[command(about = "Repository management")]
    Repository {
        #[command(subcommand)]
        cmd: RepositoryCommands,
    },

    #[command(alias = "wt")]
    #[command(about = "Worktree management")]
    Worktree {
        #[command(subcommand)]
        cmd: WorktreeCommands,
    },
}

#[derive(Subcommand)]
pub enum RepositoryCommands {
    #[command(about = "Clone a repository to $HOME/src/{host}/{org}/{repo}")]
    Clone {
        #[arg(help = "Git repository URL")]
        url: String,
    },
}

#[derive(Subcommand)]
pub enum WorktreeCommands {
    #[command(short_flag = 'c')]
    #[command(about = "Create a new worktree")]
    Create {
        #[arg(help = "Branch name")]
        branch: String,

        #[arg(long, help = "Base branch for new branch")]
        base: Option<String>,
    },

    #[command(short_flag = 'l')]
    #[command(about = "List worktrees")]
    List,

    #[command(short_flag = 'd')]
    #[command(about = "Delete a worktree")]
    Delete {
        #[arg(help = "Branch name")]
        branch: String,
    },

    #[command(short_flag = 'D')]
    #[command(about = "Force delete a worktree")]
    ForceDelete {
        #[arg(help = "Branch name")]
        branch: String,
    },
}
