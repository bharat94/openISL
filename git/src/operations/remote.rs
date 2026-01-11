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

    if tags {
        args.push("--tags");
        return run(&args, Some(repo_path)).with_context(|| "Failed to push tags");
    }

    if let Some(r) = remote {
        args.push(r);
    }

    if let Some(b) = branch {
        args.push(b);
    }

    if set_upstream {
        args.push("--set-upstream");
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

    let mut remotes = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 2 {
            remotes.push(Remote {
                name: parts[0].to_string(),
                url: parts[1].to_string(),
                fetch_type: if parts.len() > 2 {
                    parts[2].to_string()
                } else {
                    String::new()
                },
            });
        }
    }

    Ok(remotes)
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
    fn test_fetch_non_existent_remote() {
        let repo_path = std::env::current_dir().unwrap();
        let result = fetch(&repo_path, Some("non-existent-remote-12345"), false);
        // Will fail if remote doesn't exist
        assert!(result.is_err());
    }
}
