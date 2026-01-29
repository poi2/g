use anyhow::{Context, Result};
use regex::Regex;

pub fn parse_repo_path(url: &str) -> Result<String> {
    let https_re = Regex::new(r"https?://([^/]+)/(.+?)(?:\.git)?$")
        .context("Failed to compile HTTPS regex")?;
    let ssh_re = Regex::new(r"git@([^:]+):(.+?)(?:\.git)?$")
        .context("Failed to compile SSH regex")?;

    if let Some(caps) = https_re.captures(url) {
        let host = &caps[1];
        let path = &caps[2];
        return Ok(format!("{}/{}", host, path));
    }

    if let Some(caps) = ssh_re.captures(url) {
        let host = &caps[1];
        let path = &caps[2];
        return Ok(format!("{}/{}", host, path));
    }

    anyhow::bail!("Invalid Git URL: {}", url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_https_url() {
        let url = "https://github.com/poi2/my-project.git";
        let result = parse_repo_path(url).unwrap();
        assert_eq!(result, "github.com/poi2/my-project");
    }

    #[test]
    fn test_parse_https_url_without_git_suffix() {
        let url = "https://github.com/poi2/my-project";
        let result = parse_repo_path(url).unwrap();
        assert_eq!(result, "github.com/poi2/my-project");
    }

    #[test]
    fn test_parse_ssh_url() {
        let url = "git@github.com:poi2/my-project.git";
        let result = parse_repo_path(url).unwrap();
        assert_eq!(result, "github.com/poi2/my-project");
    }

    #[test]
    fn test_parse_ssh_url_without_git_suffix() {
        let url = "git@github.com:poi2/my-project";
        let result = parse_repo_path(url).unwrap();
        assert_eq!(result, "github.com/poi2/my-project");
    }

    #[test]
    fn test_parse_gitlab_url() {
        let url = "https://gitlab.com/team/project.git";
        let result = parse_repo_path(url).unwrap();
        assert_eq!(result, "gitlab.com/team/project");
    }

    #[test]
    fn test_parse_invalid_url() {
        let url = "invalid-url";
        let result = parse_repo_path(url);
        assert!(result.is_err());
    }
}
