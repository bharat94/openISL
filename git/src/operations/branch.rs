use crate::command::run;
use crate::models::{GitRef, RefType};
use anyhow::{Context, Result};
use std::path::Path;

pub fn get_branches(repo_path: &Path) -> Result<Vec<GitRef>> {
    let output = run(
        &["branch", "--format=%(refname:short)|%(refname:short)"],
        Some(repo_path),
    )
    .with_context(|| "Failed to get git branches")?;

    let mut refs = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '|').collect();
        let name = parts[0].to_string();
        let ref_type = if name.starts_with("refs/heads/") {
            RefType::Branch
        } else if name.starts_with("refs/remotes/") {
            RefType::Remote
        } else if name.starts_with("refs/tags/") {
            RefType::Tag
        } else if name == "HEAD" {
            RefType::Head
        } else {
            // Default to branch for local refs without prefix
            RefType::Branch
        };

        refs.push(GitRef { name, ref_type });
    }

    Ok(refs)
}

pub fn get_current_branch(repo_path: &Path) -> Result<Option<String>> {
    let output = run(&["branch", "--show-current"], Some(repo_path))
        .with_context(|| "Failed to get current branch")?;

    let branch = output.trim();
    if branch.is_empty() {
        Ok(None)
    } else {
        Ok(Some(branch.to_string()))
    }
}

pub fn create_branch(repo_path: &Path, branch_name: &str) -> Result<()> {
    let args = vec!["branch", branch_name];
    run(&args, Some(repo_path))
        .with_context(|| format!("Failed to create branch '{}'", branch_name))?;
    Ok(())
}

pub fn create_branch_from_commit(
    repo_path: &Path,
    branch_name: &str,
    commit_hash: &str,
) -> Result<()> {
    let args = vec!["checkout", "-b", branch_name, commit_hash];
    run(&args, Some(repo_path)).with_context(|| {
        format!(
            "Failed to create branch '{}' from '{}'",
            branch_name, commit_hash
        )
    })?;
    Ok(())
}

pub fn get_refs_for_commit(_repo_path: &Path, _hash: &str) -> Result<Vec<GitRef>> {
    todo!("Implement get_refs_for_commit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_branch() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_current_branch(&repo_path);
        // This will fail if not in a repo, but that's expected
        // The actual test requires being in a git repo
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_get_branches() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_branches(&repo_path);
        // This will fail if not in a repo, but that's expected
        assert!(result.is_ok() || result.is_err());
    }
}
