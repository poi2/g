use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::path;

#[allow(dead_code)]
pub struct RepoInfo {
    pub repo_root: PathBuf,
    pub remote_url: String,
    pub repo_path: String,
    pub main_repo_dir: PathBuf,
    pub worktree_base: PathBuf,
}

#[allow(dead_code)]
impl RepoInfo {
    pub fn detect() -> Result<Self> {
        let repo_root = Self::find_git_root()?;
        let remote_url = Self::get_remote_url(&repo_root)?;
        let repo_path = path::parse_repo_path(&remote_url)?;

        let home = env::var("HOME").context("HOME environment variable not set")?;
        let main_repo_dir = PathBuf::from(&home).join("src").join(&repo_path);

        let worktree_base = Self::get_worktree_base()?.join(&repo_path);

        Ok(Self {
            repo_root,
            remote_url,
            repo_path,
            main_repo_dir,
            worktree_base,
        })
    }

    fn find_git_root() -> Result<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .context("Failed to execute git rev-parse")?;

        if !output.status.success() {
            anyhow::bail!(
                "Not in a git repository.\n\
                Run this command from within a git repository."
            );
        }

        let path = String::from_utf8(output.stdout)
            .context("Failed to parse git root path")?
            .trim()
            .to_string();

        Ok(PathBuf::from(path))
    }

    fn get_remote_url(repo_root: &PathBuf) -> Result<String> {
        let output = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .current_dir(repo_root)
            .output()
            .context("Failed to get remote URL")?;

        if !output.status.success() {
            anyhow::bail!(
                "No remote 'origin' found.\n\
                Please add a remote:\n  \
                git remote add origin <url>"
            );
        }

        Ok(String::from_utf8(output.stdout)?
            .trim()
            .to_string())
    }

    pub fn get_worktree_base() -> Result<PathBuf> {
        if let Ok(base) = env::var("G_WORKTREE_BASE") {
            return Ok(PathBuf::from(base));
        }

        let home = env::var("HOME").context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join("src/.worktrees"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_worktree_base_default() {
        env::remove_var("G_WORKTREE_BASE");
        let base = RepoInfo::get_worktree_base().unwrap();
        assert!(base.to_str().unwrap().ends_with("src/.worktrees"));
    }

    #[test]
    fn test_get_worktree_base_custom() {
        env::set_var("G_WORKTREE_BASE", "/custom/path");
        let base = RepoInfo::get_worktree_base().unwrap();
        assert_eq!(base, PathBuf::from("/custom/path"));
        env::remove_var("G_WORKTREE_BASE");
    }
}
