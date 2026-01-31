use anyhow::{Context, Result};
use git2::Config as GitConfig;
use std::collections::HashMap;

pub struct Config {
    #[allow(dead_code)]
    pub root: Option<String>,
    pub aliases: HashMap<String, String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let git_config = GitConfig::open_default().context("Failed to open git config")?;

        let root = git_config
            .get_string("sonic-git.root")
            .ok()
            .filter(|s| !s.is_empty());

        let mut aliases = HashMap::new();

        if let Ok(mut entries) = git_config.entries(Some("sonic-git.alias.*")) {
            while let Some(entry) = entries.next() {
                if let Ok(entry) = entry {
                    if let (Some(name), Some(value)) = (entry.name(), entry.value()) {
                        if let Some(alias_name) = name.strip_prefix("sonic-git.alias.") {
                            aliases.insert(alias_name.to_string(), value.to_string());
                        }
                    }
                }
            }
        }

        Ok(Config { root, aliases })
    }

    pub fn resolve_alias(&self, cmd: &str) -> Option<Vec<String>> {
        self.aliases.get(cmd).map(|alias_value| {
            alias_value
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load() {
        let config = Config::load();
        assert!(config.is_ok());
    }

    #[test]
    fn test_resolve_alias() {
        let mut aliases = HashMap::new();
        aliases.insert("s".to_string(), "sonic-switch -i".to_string());

        let config = Config {
            root: None,
            aliases,
        };

        let resolved = config.resolve_alias("s");
        assert_eq!(
            resolved,
            Some(vec!["sonic-switch".to_string(), "-i".to_string()])
        );
    }
}
