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
}

#[derive(Subcommand)]
pub enum RepositoryCommands {
    #[command(about = "Clone a repository to $HOME/src/{host}/{org}/{repo}")]
    Clone {
        #[arg(help = "Git repository URL")]
        url: String,
    },
}
