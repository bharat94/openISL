use crate::command::{run, run_success};
use anyhow::{Context, Result};
use std::path::Path;

/// List files that currently have merge conflicts.
pub fn get_conflicted_files(repo_path: &Path) -> Result<Vec<String>> {
    let output = run(&["diff", "--name-only", "--diff-filter=U"], Some(repo_path))
        .with_context(|| "Failed to list conflicted files")?;

    Ok(output
        .lines()
        .map(str::to_string)
        .filter(|l| !l.trim().is_empty())
        .collect())
}

/// Mark one or more conflicted files as resolved by staging them.
pub fn mark_resolved(repo_path: &Path, paths: &[&str]) -> Result<()> {
    let mut args = vec!["add"];
    args.extend_from_slice(paths);
    run_success(&args, Some(repo_path)).with_context(|| "Failed to mark files as resolved")?;
    Ok(())
}
