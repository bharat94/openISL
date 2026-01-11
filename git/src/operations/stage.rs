use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

pub fn stage_file(repo_path: &Path, file: &str) -> Result<()> {
    run(&["add", file], Some(repo_path))
        .with_context(|| format!("Failed to stage file: {}", file))?;
    Ok(())
}

pub fn stage_all(repo_path: &Path) -> Result<()> {
    run(&["add", "-A"], Some(repo_path)).with_context(|| "Failed to stage all files")?;
    Ok(())
}

pub fn unstage_file(repo_path: &Path, file: &str) -> Result<()> {
    run(&["reset", "--", file], Some(repo_path))
        .with_context(|| format!("Failed to unstage file: {}", file))?;
    Ok(())
}

pub fn unstage_all(repo_path: &Path) -> Result<()> {
    run(&["reset", "HEAD"], Some(repo_path)).with_context(|| "Failed to unstage all files")?;
    Ok(())
}

pub fn stage_hunk(repo_path: &Path, file: &str, hunk_start: usize, hunk_end: usize) -> Result<()> {
    run(&["apply", "--cached", "-"], Some(repo_path))
        .with_context(|| format!("Failed to stage hunk for file: {}", file))?;
    Ok(())
}

pub fn get_staged_files(repo_path: &Path) -> Result<Vec<String>> {
    let output = run(&["diff", "--cached", "--name-only"], Some(repo_path))
        .with_context(|| "Failed to get staged files")?;

    Ok(output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect())
}

pub fn get_unstaged_files(repo_path: &Path) -> Result<Vec<String>> {
    let output = run(&["diff", "--name-only"], Some(repo_path))
        .with_context(|| "Failed to get unstaged files")?;

    Ok(output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect())
}

pub fn has_staged_changes(repo_path: &Path) -> Result<bool> {
    let output = run(&["status", "--porcelain"], Some(repo_path))
        .with_context(|| "Failed to check for staged changes")?;

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("M ")
            || line.starts_with("A ")
            || line.starts_with("D ")
            || line.starts_with("R ")
        {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn has_unstaged_changes(repo_path: &Path) -> Result<bool> {
    let output = run(&["status", "--porcelain"], Some(repo_path))
        .with_context(|| "Failed to check for unstaged changes")?;

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with(" M") || line.starts_with(" A") || line.starts_with(" D") {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_staged_changes() {
        let repo_path = std::env::current_dir().unwrap();
        let result = has_staged_changes(&repo_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_staged_files() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_staged_files(&repo_path);
        assert!(result.is_ok());
    }
}
