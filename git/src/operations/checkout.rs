use anyhow::{Context, Result};
use std::path::Path;
use crate::command::run;

pub fn checkout(repo_path: &Path, target: &str) -> Result<()> {
    let args = vec!["checkout", target];
    run(&args, Some(repo_path))
        .with_context(|| format!("Failed to checkout '{}'", target))?;
    Ok(())
}

pub fn checkout_commit(repo_path: &Path, commit_hash: &str) -> Result<()> {
    let args = vec!["checkout", commit_hash];
    run(&args, Some(repo_path))
        .with_context(|| format!("Failed to checkout commit '{}'", commit_hash))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkout_non_existent() {
        let repo_path = std::env::current_dir().unwrap();
        let result = checkout(&repo_path, "non-existent-branch-12345");
        assert!(result.is_err());
    }
}
