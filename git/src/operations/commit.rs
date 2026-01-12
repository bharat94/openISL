use crate::command::run_success;
use anyhow::{Context, Result};
use std::path::Path;

pub fn amend_commit(repo_path: &Path, amend_message: Option<&str>) -> Result<()> {
    if let Some(msg) = amend_message {
        run_success(&["commit", "--amend", "-m", msg], Some(repo_path))
            .with_context(|| "Failed to amend commit with message")?;
    } else {
        run_success(&["commit", "--amend", "--no-edit"], Some(repo_path))
            .with_context(|| "Failed to amend commit")?;
    }
    Ok(())
}

pub fn reword_commit(_repo_path: &Path, _commit_hash: &str, _message: &str) -> Result<()> {
    // TODO: Implement proper reword with interactive rebase
    Ok(())
}

pub fn drop_commit(repo_path: &Path, commit_hash: &str) -> Result<()> {
    run_success(
        &[
            "rebase",
            "--onto",
            &format!("^{}", commit_hash),
            commit_hash,
        ],
        Some(repo_path),
    )
    .with_context(|| format!("Failed to drop commit {}", commit_hash))?;
    Ok(())
}

pub fn squash_commits(repo_path: &Path, commit_hash: &str, message: &str) -> Result<()> {
    run_success(&["reset", "--soft", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to reset to {}", commit_hash))?;

    run_success(&["commit", "-m", message], Some(repo_path))
        .with_context(|| "Failed to create squashed commit")?;

    Ok(())
}

pub fn get_commit_message(repo_path: &Path, commit_hash: &str) -> Result<String> {
    let output = crate::command::run(&["log", "-1", "--format=%B", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to get message for commit {}", commit_hash))?;

    Ok(output)
}

pub fn tag_commit(
    repo_path: &Path,
    commit_hash: &str,
    tag_name: &str,
    message: Option<&str>,
) -> Result<()> {
    let mut args = vec!["tag", "-a", tag_name, commit_hash];
    if let Some(msg) = message {
        args.push("-m");
        args.push(msg);
    }
    run_success(&args, Some(repo_path))
        .with_context(|| format!("Failed to tag commit {} as {}", commit_hash, tag_name))?;
    Ok(())
}

pub fn cherry_pick_commit(repo_path: &Path, commit_hash: &str) -> Result<()> {
    run_success(&["cherry-pick", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to cherry-pick commit {}", commit_hash))?;
    Ok(())
}

pub fn revert_commit(repo_path: &Path, commit_hash: &str) -> Result<()> {
    run_success(&["revert", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to revert commit {}", commit_hash))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_get_commit_message() {
        let repo_path = current_dir().unwrap();
        let result = get_commit_message(&repo_path, "HEAD");
        assert!(result.is_ok());
    }
}
