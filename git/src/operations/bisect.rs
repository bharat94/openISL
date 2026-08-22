use crate::command::{run, run_success};
use anyhow::{Context, Result};
use std::path::Path;

/// Start a bisect session between a known-good and a known-bad revision.
pub fn bisect_start(repo_path: &Path, bad: &str, good: &str) -> Result<String> {
    run_success(&["bisect", "start"], Some(repo_path)).with_context(|| "Failed to start bisect")?;
    run_success(&["bisect", "bad", bad], Some(repo_path))
        .with_context(|| format!("Failed to mark '{}' as bad", bad))?;
    run_success(&["bisect", "good", good], Some(repo_path))
        .with_context(|| format!("Failed to mark '{}' as good", good))?;
    run(&["bisect", "log"], Some(repo_path)).with_context(|| "Failed to read bisect state")
}

/// Mark the current revision as good and get the next candidate to test.
pub fn bisect_good(repo_path: &Path, revision: Option<&str>) -> Result<String> {
    let mut args = vec!["bisect", "good"];
    if let Some(rev) = revision {
        args.push(rev);
    }
    run(&args, Some(repo_path)).with_context(|| "Failed to mark revision as good")
}

/// Mark the current revision as bad and get the next candidate to test.
pub fn bisect_bad(repo_path: &Path, revision: Option<&str>) -> Result<String> {
    let mut args = vec!["bisect", "bad"];
    if let Some(rev) = revision {
        args.push(rev);
    }
    run(&args, Some(repo_path)).with_context(|| "Failed to mark revision as bad")
}

/// Skip the current revision and test the next candidate instead.
pub fn bisect_skip(repo_path: &Path) -> Result<String> {
    run(&["bisect", "skip"], Some(repo_path)).with_context(|| "Failed to skip revision")
}

/// End the bisect session and return to the pre-bisect HEAD.
pub fn bisect_reset(repo_path: &Path) -> Result<()> {
    run_success(&["bisect", "reset"], Some(repo_path)).with_context(|| "Failed to reset bisect")
}
