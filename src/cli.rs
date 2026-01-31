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

    #[command(about = "Branch management")]
    SonicBranch {
        #[command(subcommand)]
        cmd: BranchCommands,
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

    #[command(about = "List all repositories")]
    Ls,

    #[command(about = "Switch to a repository")]
    Switch {
        #[arg(help = "Repository name (e.g., github.com/user/repo)")]
        repository: Option<String>,

        #[arg(short, long, help = "Interactive selection with fzf")]
        interactive: bool,
    },

    #[command(about = "Delete a repository")]
    Delete {
        #[arg(help = "Repository name (e.g., github.com/user/repo)")]
        repository: Option<String>,

        #[arg(short, long, help = "Interactive selection with fzf")]
        interactive: bool,
    },

    #[command(about = "Create a new repository")]
    New {
        #[arg(help = "Repository path (e.g., github.com/user/repo)")]
        repository: String,
    },
}

#[derive(Subcommand)]
pub enum WorktreeCommands {
    #[command(about = "Create a new worktree")]
    New {
        #[arg(help = "Branch name")]
        branch: String,

        #[arg(long, help = "Base branch for new branch")]
        base: Option<String>,
    },

    #[command(about = "List worktrees")]
    Ls,

    #[command(about = "Rename a worktree")]
    Mv {
        #[arg(help = "Old branch name (omit to rename current)")]
        old: Option<String>,

        #[arg(help = "New branch name")]
        new: String,
    },

    #[command(about = "Switch to a worktree")]
    Switch {
        #[arg(help = "Branch name")]
        branch: Option<String>,

        #[arg(short, long, help = "Interactive selection with fzf")]
        interactive: bool,
    },

    #[command(about = "Delete worktrees")]
    Delete {
        #[arg(help = "Branch name")]
        branch: Option<String>,

        #[arg(short, long, help = "Force delete")]
        force: bool,

        #[arg(short, long, help = "Delete all except current")]
        all: bool,

        #[arg(short, long, help = "Interactive selection with fzf")]
        interactive: bool,
    },
}

#[derive(Subcommand)]
pub enum BranchCommands {
    #[command(about = "List branches")]
    Ls {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        options: Vec<String>,
    },

    #[command(about = "Create a new branch")]
    New {
        #[arg(help = "Branch name")]
        branch: String,
    },

    #[command(about = "Rename a branch")]
    Mv {
        #[arg(help = "Old branch name (omit to rename current branch)")]
        old: Option<String>,

        #[arg(help = "New branch name")]
        new: String,
    },

    #[command(about = "Delete branches")]
    Delete {
        #[arg(help = "Branch name")]
        branch: Option<String>,

        #[arg(short, long, help = "Force delete")]
        force: bool,

        #[arg(short, long, help = "Delete all branches except base/current")]
        all: bool,

        #[arg(short, long, help = "Interactive selection with fzf")]
        interactive: bool,
    },
}
