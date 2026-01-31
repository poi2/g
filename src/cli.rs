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
    #[command(about = "Repository management")]
    SonicRepository {
        #[command(subcommand)]
        cmd: RepositoryCommands,
    },

    #[command(about = "Worktree management")]
    SonicWorktree {
        #[command(subcommand)]
        cmd: WorktreeCommands,
    },

    #[command(about = "Switch branches with fzf")]
    SonicSwitch {
        #[arg(help = "Branch name")]
        branch: Option<String>,

        #[arg(short, long, help = "Interactive selection with fzf")]
        interactive: bool,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    #[command(external_subcommand)]
    External(Vec<String>),
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

    #[command(about = "Switch to a worktree")]
    Switch {
        #[arg(help = "Branch name")]
        branch: Option<String>,

        #[arg(short, long, help = "Interactive selection with fzf")]
        interactive: bool,

        #[arg(short, long, help = "Create new worktree and switch to it")]
        create: bool,

        #[arg(long, help = "Base branch for new branch (used with --create)")]
        base: Option<String>,
    },
}
