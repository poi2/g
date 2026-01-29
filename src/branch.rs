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
