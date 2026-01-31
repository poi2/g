use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

#[allow(dead_code)]
pub struct FzfOptions {
    pub height: Option<String>,
    pub reverse: bool,
    pub border: bool,
    pub prompt: Option<String>,
    pub preview: Option<String>,
}

impl Default for FzfOptions {
    fn default() -> Self {
        Self {
            height: Some("40%".to_string()),
            reverse: true,
            border: true,
            prompt: None,
            preview: None,
        }
    }
}

impl FzfOptions {
    fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        if let Some(ref height) = self.height {
            args.push(format!("--height={}", height));
        }

        if self.reverse {
            args.push("--reverse".to_string());
        }

        if self.border {
            args.push("--border".to_string());
        }

        if let Some(ref prompt) = self.prompt {
            args.push(format!("--prompt={}", prompt));
        }

        if let Some(ref preview) = self.preview {
            args.push(format!("--preview={}", preview));
        }

        args
    }
}

pub fn run_fzf(items: &[String], options: Option<FzfOptions>) -> Result<Option<String>> {
    if !is_fzf_available() {
        anyhow::bail!(
            "fzf not found. Please install fzf:\n  \
            brew install fzf  (macOS)\n  \
            apt install fzf   (Ubuntu/Debian)\n  \
            dnf install fzf   (Fedora)"
        );
    }

    let opts = options.unwrap_or_default();
    let args = opts.to_args();

    let mut child = Command::new("fzf")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn fzf")?;

    if let Some(mut stdin) = child.stdin.take() {
        let input = items.join("\n");
        stdin
            .write_all(input.as_bytes())
            .context("Failed to write to fzf stdin")?;
    }

    let output = child.wait_with_output().context("Failed to wait for fzf")?;

    if !output.status.success() {
        return Ok(None);
    }

    let selection =
        String::from_utf8(output.stdout).context("Failed to parse fzf output as UTF-8")?;

    let trimmed = selection.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    Ok(Some(trimmed.to_string()))
}

pub fn select(items: &[String], prompt: &str) -> Result<String> {
    let options = FzfOptions {
        prompt: Some(format!("{}: ", prompt)),
        ..Default::default()
    };

    match run_fzf(items, Some(options))? {
        Some(selection) => Ok(selection),
        None => anyhow::bail!("No selection made"),
    }
}

fn is_fzf_available() -> bool {
    Command::new("fzf")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fzf_options_default() {
        let opts = FzfOptions::default();
        let args = opts.to_args();
        assert!(args.contains(&"--reverse".to_string()));
        assert!(args.contains(&"--border".to_string()));
    }

    #[test]
    fn test_fzf_options_custom() {
        let opts = FzfOptions {
            height: Some("50%".to_string()),
            reverse: false,
            border: true,
            prompt: Some("Select: ".to_string()),
            preview: None,
        };
        let args = opts.to_args();
        assert!(args.contains(&"--height=50%".to_string()));
        assert!(!args.contains(&"--reverse".to_string()));
        assert!(args.contains(&"--prompt=Select: ".to_string()));
    }

    #[test]
    fn test_is_fzf_available() {
        let available = is_fzf_available();
        println!("fzf available: {}", available);
    }
}
