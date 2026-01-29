use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

use crate::repo::RepoInfo;

pub fn create_worktree(repo_info: &RepoInfo, branch: &str, base: Option<&str>) -> Result<()> {
    let worktree_path = repo_info.worktree_base.join(branch);

    if worktree_path.exists() {
        anyhow::bail!(
            "Worktree directory already exists: {}\n\
            If you want to recreate it, please remove it first:\n  \
            rm -rf {}",
            worktree_path.display(),
            worktree_path.display()
        );
    }

    if let Some(parent) = worktree_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create worktree parent directory: {}",
                parent.display()
            )
        })?;
    }

    let branch_exists = check_branch_exists(&repo_info.main_repo_dir, branch)?;

    let mut cmd = Command::new("git");
    cmd.arg("worktree").arg("add");

    if !branch_exists {
        cmd.arg("-b").arg(branch);
        if let Some(base_branch) = base {
            cmd.arg(base_branch);
        }
    }

    cmd.arg(&worktree_path);
    if branch_exists {
        cmd.arg(branch);
    }

    println!(
        "Creating worktree for branch '{}' at {}...",
        branch,
        worktree_path.display()
    );

    let status = cmd
        .current_dir(&repo_info.main_repo_dir)
        .status()
        .context("Failed to execute git worktree add")?;

    if !status.success() {
        anyhow::bail!("git worktree add failed");
    }

    println!("{}", worktree_path.display());

    Ok(())
}

fn check_branch_exists(repo_root: &std::path::PathBuf, branch: &str) -> Result<bool> {
    let local = Command::new("git")
        .args(["show-ref", "--verify", &format!("refs/heads/{}", branch)])
        .current_dir(repo_root)
        .status()?
        .success();

    if local {
        return Ok(true);
    }

    let remote = Command::new("git")
        .args([
            "show-ref",
            "--verify",
            &format!("refs/remotes/origin/{}", branch),
        ])
        .current_dir(repo_root)
        .status()?
        .success();

    Ok(remote)
}
