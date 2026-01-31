use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::repo::RepoInfo;

pub struct Worktree {
    pub path: PathBuf,
    #[allow(dead_code)]
    pub head_sha: String,
    pub branch: Option<String>,
    #[allow(dead_code)]
    pub is_bare: bool,
    #[allow(dead_code)]
    pub is_locked: bool,
}

impl Worktree {
    pub fn list(repo_root: &PathBuf) -> Result<Vec<Worktree>> {
        let output = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(repo_root)
            .output()
            .context("Failed to execute git worktree list")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git worktree list failed: {}", stderr);
        }

        let stdout = String::from_utf8(output.stdout)?;
        Self::parse_porcelain(&stdout)
    }

    fn parse_porcelain(output: &str) -> Result<Vec<Worktree>> {
        let mut worktrees = Vec::new();
        let mut current: Option<WorktreeBuilder> = None;

        for line in output.lines() {
            if line.is_empty() {
                if let Some(builder) = current.take() {
                    worktrees.push(builder.build()?);
                }
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            let key = parts[0];
            let value = if parts.len() > 1 { parts[1] } else { "" };

            match key {
                "worktree" => {
                    current = Some(WorktreeBuilder::new(PathBuf::from(value)));
                }
                "HEAD" => {
                    if let Some(ref mut builder) = current {
                        builder.head_sha = value.to_string();
                    }
                }
                "branch" => {
                    if let Some(ref mut builder) = current {
                        let branch_name = value.strip_prefix("refs/heads/").unwrap_or(value);
                        builder.branch = Some(branch_name.to_string());
                    }
                }
                "bare" => {
                    if let Some(ref mut builder) = current {
                        builder.is_bare = true;
                    }
                }
                "locked" => {
                    if let Some(ref mut builder) = current {
                        builder.is_locked = true;
                    }
                }
                _ => {}
            }
        }

        if let Some(builder) = current {
            worktrees.push(builder.build()?);
        }

        Ok(worktrees)
    }
}

struct WorktreeBuilder {
    path: PathBuf,
    head_sha: String,
    branch: Option<String>,
    is_bare: bool,
    is_locked: bool,
}

impl WorktreeBuilder {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            head_sha: String::new(),
            branch: None,
            is_bare: false,
            is_locked: false,
        }
    }

    fn build(self) -> Result<Worktree> {
        Ok(Worktree {
            path: self.path,
            head_sha: self.head_sha,
            branch: self.branch,
            is_bare: self.is_bare,
            is_locked: self.is_locked,
        })
    }
}

pub fn list_worktrees(repo_info: &RepoInfo) -> Result<()> {
    let worktrees = Worktree::list(&repo_info.main_repo_dir)?;

    for wt in worktrees {
        let branch_name = wt.branch.unwrap_or_else(|| "(detached)".to_string());
        println!("{:<20} {}", branch_name, wt.path.display());
    }

    Ok(())
}

pub fn create_worktree(repo_info: &RepoInfo, branch: &str, base: Option<&str>) -> Result<PathBuf> {
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
    }

    cmd.arg(&worktree_path);
    
    if branch_exists {
        cmd.arg(branch);
    } else if let Some(base_branch) = base {
        cmd.arg(base_branch);
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

    Ok(worktree_path)
}

pub fn switch_worktree(
    repo_info: &RepoInfo,
    branch: Option<&str>,
    interactive: bool,
) -> Result<()> {
    if interactive {
        let worktrees = Worktree::list(&repo_info.main_repo_dir)?;
        let items: Vec<String> = worktrees
            .iter()
            .map(|wt| {
                format!(
                    "{:<20} {}",
                    wt.branch.as_deref().unwrap_or("(detached)"),
                    wt.path.display()
                )
            })
            .collect();

        let opts = crate::fzf::FzfOptions {
            prompt: Some("Select worktree: ".to_string()),
            preview: Some("git -C {2} log -n 10 --oneline --color=always".to_string()),
            ..Default::default()
        };

        if let Some(selection) = crate::fzf::run_fzf(&items, Some(opts))? {
            let path = selection
                .split_whitespace()
                .nth(1)
                .ok_or_else(|| anyhow::anyhow!("Failed to parse selection"))?;
            println!("{}", path);
        }

        return Ok(());
    }

    if let Some(branch_name) = branch {
        let worktrees = Worktree::list(&repo_info.main_repo_dir)?;
        let worktree = worktrees
            .iter()
            .find(|wt| wt.branch.as_deref() == Some(branch_name))
            .ok_or_else(|| anyhow::anyhow!("Worktree not found: {}", branch_name))?;

        println!("{}", worktree.path.display());
        return Ok(());
    }

    anyhow::bail!("Specify branch name or use --interactive")
}

pub fn move_worktree(repo_info: &RepoInfo, old: Option<&str>, new: &str) -> Result<()> {
    let old_branch = if let Some(branch) = old {
        branch.to_string()
    } else {
        get_current_branch(&repo_info.repo_root)?
    };

    let worktrees = Worktree::list(&repo_info.main_repo_dir)?;
    let target = worktrees
        .iter()
        .find(|wt| wt.branch.as_deref() == Some(&old_branch))
        .ok_or_else(|| anyhow::anyhow!("Worktree not found for branch: {}", old_branch))?;

    let old_path = &target.path;
    let new_path = old_path
        .parent()
        .unwrap_or(&repo_info.worktree_base)
        .join(new);

    let status = Command::new("git")
        .args(["branch", "-m", &old_branch, new])
        .current_dir(&repo_info.main_repo_dir)
        .status()
        .context("Failed to rename branch")?;

    if !status.success() {
        anyhow::bail!("Failed to rename branch: {} -> {}", old_branch, new);
    }

    let status = Command::new("git")
        .args([
            "worktree",
            "move",
            old_path.to_str().unwrap(),
            new_path.to_str().unwrap(),
        ])
        .current_dir(&repo_info.main_repo_dir)
        .status()
        .context("Failed to move worktree")?;

    if !status.success() {
        Command::new("git")
            .args(["branch", "-m", new, &old_branch])
            .current_dir(&repo_info.main_repo_dir)
            .status()
            .ok();
        anyhow::bail!("Failed to move worktree directory");
    }

    println!("Renamed worktree: {} -> {}", old_branch, new);
    println!("  {} -> {}", old_path.display(), new_path.display());
    Ok(())
}

pub fn delete_worktrees(
    repo_info: &RepoInfo,
    branch: Option<&str>,
    force: bool,
    all: bool,
    interactive: bool,
) -> Result<()> {
    if all {
        let current = get_current_branch(&repo_info.repo_root)?;
        let worktrees = Worktree::list(&repo_info.main_repo_dir)?;

        let to_delete: Vec<&Worktree> = worktrees
            .iter()
            .filter(|wt| {
                !wt.is_bare
                    && wt.branch.as_deref() != Some(&current)
                    && wt.path != repo_info.repo_root
            })
            .collect();

        if to_delete.is_empty() {
            println!("No worktrees to delete");
            return Ok(());
        }

        for wt in to_delete {
            let branch_name = wt.branch.as_deref().unwrap_or("unknown");
            let status = Command::new("git")
                .args([
                    "worktree",
                    "remove",
                    if force { "--force" } else { "" },
                    wt.path.to_str().unwrap(),
                ])
                .args(vec![wt.path.to_str().unwrap()])
                .current_dir(&repo_info.main_repo_dir)
                .status()
                .context("Failed to remove worktree")?;

            if status.success() {
                println!("Deleted worktree: {}", branch_name);
            }
        }

        return Ok(());
    }

    if interactive {
        let current = get_current_branch(&repo_info.repo_root)?;
        let worktrees = Worktree::list(&repo_info.main_repo_dir)?;

        let candidates: Vec<&Worktree> = worktrees
            .iter()
            .filter(|wt| {
                !wt.is_bare
                    && wt.branch.as_deref() != Some(&current)
                    && wt.path != repo_info.repo_root
            })
            .collect();

        if candidates.is_empty() {
            println!("No worktrees to delete");
            return Ok(());
        }

        let items: Vec<String> = candidates
            .iter()
            .map(|wt| {
                format!(
                    "{:<20} {}",
                    wt.branch.as_deref().unwrap_or("(detached)"),
                    wt.path.display()
                )
            })
            .collect();

        let selection = crate::fzf::select(&items, "Select worktree to delete")?;
        let branch_name = selection.split_whitespace().next().unwrap();

        let target = candidates
            .iter()
            .find(|wt| wt.branch.as_deref() == Some(branch_name))
            .ok_or_else(|| anyhow::anyhow!("Worktree not found"))?;

        let mut cmd = Command::new("git");
        cmd.args(["worktree", "remove"]);
        if force {
            cmd.arg("--force");
        }
        cmd.arg(&target.path);

        let status = cmd
            .current_dir(&repo_info.main_repo_dir)
            .status()
            .context("Failed to remove worktree")?;

        if !status.success() {
            anyhow::bail!("Failed to delete worktree");
        }

        println!("Deleted worktree: {}", branch_name);
        return Ok(());
    }

    if let Some(branch_name) = branch {
        delete_worktree(repo_info, branch_name, force)?;
        return Ok(());
    }

    anyhow::bail!("Branch name, --all, or --interactive flag required");
}

pub fn delete_worktree(repo_info: &RepoInfo, branch: &str, force: bool) -> Result<()> {
    let worktrees = Worktree::list(&repo_info.main_repo_dir)?;

    let target = worktrees
        .iter()
        .find(|wt| wt.branch.as_deref() == Some(branch))
        .ok_or_else(|| anyhow::anyhow!("Worktree not found for branch: {}", branch))?;

    if target.is_bare {
        anyhow::bail!("Cannot delete bare repository");
    }

    if !confirm_delete(target)? {
        println!("Cancelled");
        return Ok(());
    }

    let mut cmd = Command::new("git");
    cmd.args(["worktree", "remove"]);

    if force {
        cmd.arg("--force");
    }

    cmd.arg(&target.path);

    let status = cmd
        .current_dir(&repo_info.main_repo_dir)
        .status()
        .context("Failed to execute git worktree remove")?;

    if !status.success() {
        anyhow::bail!("git worktree remove failed");
    }

    let prune_output = Command::new("git")
        .args(["worktree", "prune"])
        .current_dir(&repo_info.main_repo_dir)
        .output()
        .context("Failed to execute git worktree prune")?;

    if !prune_output.status.success() {
        let stderr = String::from_utf8_lossy(&prune_output.stderr);
        anyhow::bail!("git worktree prune failed: {}", stderr);
    }

    println!("Removed worktree: {}", branch);
    println!("  Path: {}", target.path.display());

    Ok(())
}

fn confirm_delete(worktree: &Worktree) -> Result<bool> {
    print!(
        "Delete worktree '{}'? [y/N]: ",
        worktree.branch.as_deref().unwrap_or("unknown")
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
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

fn check_branch_exists(repo_root: &std::path::PathBuf, branch: &str) -> Result<bool> {
    use std::process::Stdio;
    
    let local = Command::new("git")
        .args(["show-ref", "--verify", &format!("refs/heads/{}", branch)])
        .current_dir(repo_root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
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
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success();

    Ok(remote)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_porcelain() {
        let input = r#"worktree /path/to/main
HEAD a1b2c3d4
branch refs/heads/main

worktree /path/to/feature-auth
HEAD f6e5d4c3
branch refs/heads/feature-auth
"#;

        let worktrees = Worktree::parse_porcelain(input).unwrap();
        assert_eq!(worktrees.len(), 2);
        assert_eq!(worktrees[0].path, PathBuf::from("/path/to/main"));
        assert_eq!(worktrees[0].branch, Some("main".to_string()));
        assert_eq!(worktrees[0].head_sha, "a1b2c3d4");
        assert!(!worktrees[0].is_bare);
        assert!(!worktrees[0].is_locked);
        assert_eq!(worktrees[1].path, PathBuf::from("/path/to/feature-auth"));
        assert_eq!(worktrees[1].branch, Some("feature-auth".to_string()));
        assert_eq!(worktrees[1].head_sha, "f6e5d4c3");
        assert!(!worktrees[1].is_bare);
        assert!(!worktrees[1].is_locked);
    }

    #[test]
    fn test_parse_porcelain_detached_head() {
        let input = r#"worktree /path/to/detached
HEAD a1b2c3d4

"#;

        let worktrees = Worktree::parse_porcelain(input).unwrap();
        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/path/to/detached"));
        assert_eq!(worktrees[0].branch, None);
        assert_eq!(worktrees[0].head_sha, "a1b2c3d4");
        assert!(!worktrees[0].is_bare);
        assert!(!worktrees[0].is_locked);
    }

    #[test]
    fn test_parse_porcelain_bare() {
        let input = r#"worktree /path/to/bare
HEAD a1b2c3d4
branch refs/heads/main
bare

"#;

        let worktrees = Worktree::parse_porcelain(input).unwrap();
        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/path/to/bare"));
        assert!(worktrees[0].is_bare);
    }

    #[test]
    fn test_parse_porcelain_locked() {
        let input = r#"worktree /path/to/locked
HEAD a1b2c3d4
branch refs/heads/main
locked

"#;

        let worktrees = Worktree::parse_porcelain(input).unwrap();
        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/path/to/locked"));
        assert!(worktrees[0].is_locked);
    }
}
