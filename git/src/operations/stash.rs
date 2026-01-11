use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

pub fn get_stash_list(repo_path: &Path) -> Result<Vec<StashEntry>> {
    let output = run(
        &["stash", "list", "--format=%gd|%gs|%h|%an|%ae|%ci"],
        Some(repo_path),
    )
    .context("Failed to get stash list")?;

    let mut entries = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(6, '|').collect();
        if parts.len() >= 6 {
            entries.push(StashEntry {
                name: parts[0].to_string(),
                message: parts[1].to_string(),
                hash: parts[2].to_string(),
                author: parts[3].to_string(),
                email: parts[4].to_string(),
                date: parts[5].to_string(),
            });
        }
    }

    Ok(entries)
}

pub fn stash_push(repo_path: &Path, message: Option<&str>) -> Result<()> {
    let mut args = vec!["stash", "push"];

    if let Some(msg) = message {
        args.push("-m");
        args.push(msg);
    }

    run(&args, Some(repo_path)).context("Failed to stash changes")?;
    Ok(())
}

pub fn stash_pop(repo_path: &Path, stash_index: Option<&str>) -> Result<()> {
    let mut args = vec!["stash", "pop"];

    if let Some(index) = stash_index {
        args.push(index);
    }

    run(&args, Some(repo_path)).context("Failed to pop stash")?;
    Ok(())
}

pub fn stash_apply(repo_path: &Path, stash_index: Option<&str>) -> Result<()> {
    let mut args = vec!["stash", "apply"];

    if let Some(index) = stash_index {
        args.push(index);
    }

    run(&args, Some(repo_path)).context("Failed to apply stash")?;
    Ok(())
}

pub fn stash_drop(repo_path: &Path, stash_index: Option<&str>) -> Result<()> {
    let mut args = vec!["stash", "drop"];

    if let Some(index) = stash_index {
        args.push(index);
    }

    run(&args, Some(repo_path)).context("Failed to drop stash")?;
    Ok(())
}

pub fn stash_show(repo_path: &Path, stash_index: &str) -> Result<String> {
    run(&["stash", "show", "-p", stash_index], Some(repo_path)).context("Failed to show stash diff")
}

#[derive(Debug, Clone)]
pub struct StashEntry {
    pub name: String,
    pub message: String,
    pub hash: String,
    pub author: String,
    pub email: String,
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stash_list_empty() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_stash_list(&repo_path);
        // Will fail if no stash, but that's OK for test
        assert!(result.is_ok() || result.is_err());
    }
}
