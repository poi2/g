use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn switch_branch(
    repo_root: &PathBuf,
    branch: Option<&str>,
    interactive: bool,
    args: &[String],
) -> Result<()> {
    if interactive {
        let branches = get_branches(repo_root)?;

        let opts = crate::fzf::FzfOptions {
            prompt: Some("Select branch: ".to_string()),
            preview: Some("git log {} -n 10 --oneline --color=always".to_string()),
            ..Default::default()
        };

        if let Some(selection) = crate::fzf::run_fzf(&branches, Some(opts))? {
            let branch_name = selection.trim();

            let status = Command::new("git")
                .args(["switch", branch_name])
                .current_dir(repo_root)
                .status()
                .context("Failed to execute git switch")?;

            if !status.success() {
                anyhow::bail!("git switch failed");
            }

            println!("Switched to branch '{}'", branch_name);
        }

        return Ok(());
    }

    let mut cmd = Command::new("git");
    cmd.arg("switch");

    if let Some(branch_name) = branch {
        cmd.arg(branch_name);
    }

    cmd.args(args);

    let status = cmd
        .current_dir(repo_root)
        .status()
        .context("Failed to execute git switch")?;

    if !status.success() {
        anyhow::bail!("git switch failed");
    }

    Ok(())
}

pub fn list_branches(repo_root: &PathBuf, options: &[String]) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("branch");
    cmd.args(options);

    let status = cmd
        .current_dir(repo_root)
        .status()
        .context("Failed to execute git branch")?;

    if !status.success() {
        anyhow::bail!("git branch failed");
    }

    Ok(())
}

pub fn new_branch(repo_root: &PathBuf, branch: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["switch", "-c", branch])
        .current_dir(repo_root)
        .status()
        .context("Failed to execute git switch -c")?;

    if !status.success() {
        anyhow::bail!("Failed to create branch: {}", branch);
    }

    println!("Created and switched to branch: {}", branch);
    Ok(())
}

pub fn move_branch(repo_root: &PathBuf, old: Option<&str>, new: &str) -> Result<()> {
    let old_branch = if let Some(branch) = old {
        branch.to_string()
    } else {
        get_current_branch(repo_root)?
    };

    let status = Command::new("git")
        .args(["branch", "-m", &old_branch, new])
        .current_dir(repo_root)
        .status()
        .context("Failed to execute git branch -m")?;

    if !status.success() {
        anyhow::bail!("Failed to rename branch: {} -> {}", old_branch, new);
    }

    println!("Renamed branch: {} -> {}", old_branch, new);
    Ok(())
}

pub fn delete_branches(
    repo_root: &PathBuf,
    branch: Option<&str>,
    force: bool,
    all: bool,
    interactive: bool,
) -> Result<()> {
    let delete_flag = if force { "-D" } else { "-d" };

    if all {
        let current = get_current_branch(repo_root)?;
        let base = get_base_branch(repo_root)?;
        let branches = get_branches(repo_root)?;

        let to_delete: Vec<String> = branches
            .into_iter()
            .filter(|b| b != &current && b != &base)
            .collect();

        if to_delete.is_empty() {
            println!("No branches to delete");
            return Ok(());
        }

        for branch in to_delete {
            let status = Command::new("git")
                .args(["branch", delete_flag, &branch])
                .current_dir(repo_root)
                .status()
                .context("Failed to execute git branch -d")?;

            if status.success() {
                println!("Deleted branch: {}", branch);
            }
        }

        return Ok(());
    }

    if interactive {
        let current = get_current_branch(repo_root)?;
        let base = get_base_branch(repo_root)?;
        let mut branches = get_branches(repo_root)?;

        branches.retain(|b| b != &current && b != &base);

        if !force {
            branches = get_merged_branches(repo_root, &branches)?;
        }

        if branches.is_empty() {
            println!("No branches to delete");
            return Ok(());
        }

        let selection = crate::fzf::select(&branches, "Select branch to delete")?;

        let status = Command::new("git")
            .args(["branch", delete_flag, &selection])
            .current_dir(repo_root)
            .status()
            .context("Failed to execute git branch")?;

        if !status.success() {
            anyhow::bail!("Failed to delete branch: {}", selection);
        }

        println!("Deleted branch: {}", selection);
        return Ok(());
    }

    if let Some(branch_name) = branch {
        let status = Command::new("git")
            .args(["branch", delete_flag, branch_name])
            .current_dir(repo_root)
            .status()
            .context("Failed to execute git branch")?;

        if !status.success() {
            anyhow::bail!("Failed to delete branch: {}", branch_name);
        }

        println!("Deleted branch: {}", branch_name);
        return Ok(());
    }

    anyhow::bail!("Branch name, --all, or --interactive flag required");
}

fn get_current_branch(repo_root: &PathBuf) -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repo_root)
        .output()
        .context("Failed to get current branch")?;

    if !output.status.success() {
        anyhow::bail!("Failed to get current branch");
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn get_base_branch(repo_root: &PathBuf) -> Result<String> {
    let output = Command::new("git")
        .args(["config", "init.defaultBranch"])
        .current_dir(repo_root)
        .output()
        .context("Failed to get default branch")?;

    if output.status.success() {
        let branch = String::from_utf8(output.stdout)?.trim().to_string();
        if !branch.is_empty() {
            return Ok(branch);
        }
    }

    Ok("main".to_string())
}

fn get_merged_branches(repo_root: &PathBuf, branches: &[String]) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["branch", "--merged", "--format=%(refname:short)"])
        .current_dir(repo_root)
        .output()
        .context("Failed to get merged branches")?;

    if !output.status.success() {
        return Ok(branches.to_vec());
    }

    let merged: Vec<String> = String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(branches
        .iter()
        .filter(|b| merged.contains(b))
        .cloned()
        .collect())
}

fn get_branches(repo_root: &PathBuf) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["branch", "--format=%(refname:short)"])
        .current_dir(repo_root)
        .output()
        .context("Failed to execute git branch")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git branch failed: {}", stderr);
    }

    let stdout = String::from_utf8(output.stdout)?;
    let branches: Vec<String> = stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(branches)
}
