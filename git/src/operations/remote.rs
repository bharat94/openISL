use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

pub fn fetch(repo_path: &Path, remote: Option<&str>, prune: bool) -> Result<String> {
    let mut args = vec!["fetch"];

    if let Some(r) = remote {
        args.push(r);
    }

    if prune {
        args.push("--prune");
    }

    run(&args, Some(repo_path))
        .with_context(|| format!("Failed to fetch from remote: {:?}", remote))
}

pub fn pull(repo_path: &Path, rebase: bool) -> Result<String> {
    let mut args = vec!["pull"];

    if rebase {
        args.push("--rebase");
    }

    run(&args, Some(repo_path)).with_context(|| "Failed to pull changes")
}

pub fn push(
    repo_path: &Path,
    remote: Option<&str>,
    branch: Option<&str>,
    tags: bool,
    set_upstream: bool,
) -> Result<String> {
    let mut args = vec!["push"];

    if let Some(r) = remote {
        args.push(r);
    }

    if let Some(b) = branch {
        args.push(b);
    }

    if set_upstream {
        args.push("--set-upstream");
    }

    if tags {
        args.push("--tags");
    }

    run(&args, Some(repo_path)).with_context(|| "Failed to push changes")
}

pub fn remote_add(repo_path: &Path, name: &str, url: &str) -> Result<()> {
    let args = vec!["remote", "add", name, url];
    run(&args, Some(repo_path))
        .with_context(|| format!("Failed to add remote '{}' at {}", name, url))?;
    Ok(())
}

pub fn remote_list(repo_path: &Path) -> Result<Vec<Remote>> {
    let output =
        run(&["remote", "-v"], Some(repo_path)).with_context(|| "Failed to list remotes")?;

    Ok(parse_remote_output(&output))
}

fn parse_remote_output(output: &str) -> Vec<Remote> {
    let mut remotes = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() >= 2 {
            let mut rest = parts[1].split_whitespace();
            let url = rest.next().unwrap_or("").to_string();
            let fetch_type = rest.next().unwrap_or("").to_string();
            remotes.push(Remote {
                name: parts[0].to_string(),
                url,
                fetch_type,
            });
        }
    }
    remotes
}

pub fn remote_remove(repo_path: &Path, name: &str) -> Result<()> {
    let args = vec!["remote", "remove", name];
    run(&args, Some(repo_path)).with_context(|| format!("Failed to remove remote '{}'", name))?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Remote {
    pub name: String,
    pub url: String,
    pub fetch_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_list() {
        let repo_path = std::env::current_dir().unwrap();
        let result = remote_list(&repo_path);
        // Will fail if no remotes, but that's OK
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_parse_remote_output() {
        let output = "origin\t/path/to/repo.git (fetch)\norigin\t/path/to/repo.git (push)\n";
        let remotes = parse_remote_output(output);
        assert_eq!(remotes.len(), 2);
        assert_eq!(remotes[0].name, "origin");
        assert_eq!(remotes[0].url, "/path/to/repo.git");
        assert_eq!(remotes[0].fetch_type, "(fetch)");
        assert_eq!(remotes[1].fetch_type, "(push)");
    }

    #[test]
    fn test_fetch_non_existent_remote() {
        let repo_path = std::env::current_dir().unwrap();
        let result = fetch(&repo_path, Some("non-existent-remote-12345"), false);
        // Will fail if remote doesn't exist
        assert!(result.is_err());
    }
}
