use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::path;

pub fn clone_repository(url: &str) -> Result<()> {
    let repo_path = path::parse_repo_path(url)
        .context("Failed to parse repository URL")?;

    let home = env::var("HOME")
        .context("HOME environment variable not set")?;

    let target_dir = PathBuf::from(&home)
        .join("src")
        .join(&repo_path);

    if target_dir.exists() {
        anyhow::bail!(
            "Directory already exists: {}\n\
            If you want to re-clone, please remove it first:\n  \
            rm -rf {}",
            target_dir.display(),
            target_dir.display()
        );
    }

    if let Some(parent) = target_dir.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }

    println!("Cloning {} to {}...", url, target_dir.display());

    let status = Command::new("git")
        .args(&["clone", url, target_dir.to_str().unwrap()])
        .status()
        .context("Failed to execute git clone")?;

    if !status.success() {
        anyhow::bail!("git clone failed");
    }

    println!("✓ Cloned to: {}", target_dir.display());

    let default_branch = get_default_branch(&target_dir)?;
    println!("✓ Main branch: {}", default_branch);

    Ok(())
}

fn get_default_branch(repo_path: &PathBuf) -> Result<String> {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(repo_path)
        .output()
        .context("Failed to get default branch")?;

    if !output.status.success() {
        return Ok("unknown".to_string());
    }

    let branch = String::from_utf8(output.stdout)
        .context("Failed to parse branch name")?
        .trim()
        .to_string();

    Ok(branch)
}
