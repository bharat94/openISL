use anyhow::{Context, Result};
use std::path::Path;
use crate::command::run;

pub fn get_diff(repo_path: &Path, commit: Option<&str>, staged: bool) -> Result<String> {
    let mut args = vec!["diff"];

    if staged {
        args.push("--staged");
    }

    if let Some(c) = commit {
        args.push(c);
    }

    run(&args, Some(repo_path))
        .with_context(|| format!("Failed to get diff for commit: {:?}", commit))
}

pub fn get_commit_diff(repo_path: &Path, commit_hash: &str) -> Result<String> {
    let parent_hash = get_parent_hash(repo_path, commit_hash)
        .with_context(|| format!("Failed to get parent of commit: {}", commit_hash))?;

    if parent_hash.is_empty() {
        get_commit_content(commit_hash)
    } else {
        let args = vec!["diff", &parent_hash, commit_hash];
        run(&args, Some(repo_path))
            .with_context(|| format!("Failed to get diff between {} and {}", parent_hash, commit_hash))
    }
}

fn get_parent_hash(repo_path: &Path, commit_hash: &str) -> Result<String> {
    let args = vec!["rev-list", "--parents", "-n", "1", commit_hash];
    let output = run(&args, Some(repo_path))
        .with_context(|| format!("Failed to get parent hash for: {}", commit_hash))?;

    let parts: Vec<&str> = output.split_whitespace().collect();
    if parts.len() > 1 {
        Ok(parts[1].to_string())
    } else {
        Ok(String::new())
    }
}

fn get_commit_content(commit_hash: &str) -> Result<String> {
    let args = vec!["show", "--no-patch", commit_hash];
    let output = run(&args, None)
        .with_context(|| format!("Failed to get commit content for: {}", commit_hash))?;

    Ok(format!("Initial commit: {}\n\nNo parent diff available (use 'git show' for details)", output.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_diff() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_diff(&repo_path, None, false);
        // This will work if in a git repo
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_diff_staged() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_diff(&repo_path, None, true);
        // This will work if in a git repo
        assert!(result.is_ok());
    }
}
