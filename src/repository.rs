use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::config::Config;
use crate::fzf;

pub fn list_repositories(config: &Config) -> Result<()> {
    let src_root = get_src_root(config)?;

    if !src_root.exists() {
        println!("No repositories found in {}", src_root.display());
        return Ok(());
    }

    let repos = find_repositories(&src_root)?;

    if repos.is_empty() {
        println!("No repositories found in {}", src_root.display());
        return Ok(());
    }

    for repo in repos {
        let relative = repo
            .strip_prefix(&src_root)
            .unwrap_or(&repo)
            .display()
            .to_string();
        println!("{}", relative);
    }

    Ok(())
}

pub fn switch_repository(
    config: &Config,
    repository: Option<&str>,
    interactive: bool,
) -> Result<()> {
    let src_root = get_src_root(config)?;

    let target = if interactive {
        let repos = find_repositories(&src_root)?;

        if repos.is_empty() {
            anyhow::bail!("No repositories found in {}", src_root.display());
        }

        let items: Vec<String> = repos
            .iter()
            .map(|p| p.strip_prefix(&src_root).unwrap_or(p).display().to_string())
            .collect();

        fzf::select(&items, "Select repository")?
    } else if let Some(repo) = repository {
        repo.to_string()
    } else {
        anyhow::bail!("Repository name or --interactive flag required");
    };

    let target_path = src_root.join(&target);

    if !target_path.exists() {
        anyhow::bail!("Repository not found: {}", target_path.display());
    }

    println!("{}", target_path.display());
    Ok(())
}

pub fn delete_repository(
    config: &Config,
    repository: Option<&str>,
    interactive: bool,
) -> Result<()> {
    let src_root = get_src_root(config)?;

    let target = if interactive {
        let repos = find_repositories(&src_root)?;

        if repos.is_empty() {
            anyhow::bail!("No repositories found in {}", src_root.display());
        }

        let items: Vec<String> = repos
            .iter()
            .map(|p| p.strip_prefix(&src_root).unwrap_or(p).display().to_string())
            .collect();

        fzf::select(&items, "Select repository to delete")?
    } else if let Some(repo) = repository {
        repo.to_string()
    } else {
        anyhow::bail!("Repository name or --interactive flag required");
    };

    let target_path = src_root.join(&target);

    if !target_path.exists() {
        anyhow::bail!("Repository not found: {}", target_path.display());
    }

    if !is_git_repository(&target_path)? {
        anyhow::bail!("Not a git repository: {}", target_path.display());
    }

    fs::remove_dir_all(&target_path)
        .with_context(|| format!("Failed to delete repository: {}", target_path.display()))?;

    println!("Deleted repository: {}", target);
    Ok(())
}

pub fn new_repository(config: &Config, repository: &str) -> Result<()> {
    let src_root = get_src_root(config)?;
    let target_path = src_root.join(repository);

    if target_path.exists() {
        anyhow::bail!("Repository already exists: {}", target_path.display());
    }

    fs::create_dir_all(&target_path)
        .with_context(|| format!("Failed to create directory: {}", target_path.display()))?;

    let status = Command::new("git")
        .args(["init"])
        .current_dir(&target_path)
        .status()
        .context("Failed to execute git init")?;

    if !status.success() {
        fs::remove_dir_all(&target_path).ok();
        anyhow::bail!("Failed to initialize git repository");
    }

    println!("Created repository: {}", repository);
    println!("{}", target_path.display());
    Ok(())
}

fn get_src_root(config: &Config) -> Result<std::path::PathBuf> {
    if let Some(ref root) = config.root {
        return Ok(PathBuf::from(root));
    }

    let home = env::var("HOME").context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join("src"))
}

fn find_repositories(root: &std::path::Path) -> Result<Vec<PathBuf>> {
    let mut repos = Vec::new();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_git_repos(&path, &mut repos)?;
            }
        }
    }

    repos.sort();
    Ok(repos)
}

fn collect_git_repos(dir: &std::path::Path, repos: &mut Vec<PathBuf>) -> Result<()> {
    if dir.join(".git").exists() {
        repos.push(dir.to_path_buf());
        return Ok(());
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                collect_git_repos(&path, repos)?;
            }
        }
    }

    Ok(())
}

fn is_git_repository(path: &std::path::Path) -> Result<bool> {
    Ok(path.join(".git").exists())
}
