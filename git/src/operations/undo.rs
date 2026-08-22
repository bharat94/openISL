use crate::command::run_success;
use anyhow::{Context, Result};
use std::path::Path;

/// Undo the last operation by moving HEAD (and the index/working tree) back to
/// the previous state recorded in the reflog.
///
/// This is a destructive operation: uncommitted working-tree changes are
/// discarded, matching `git reset --hard HEAD@{1}`.
pub fn undo_last(repo_path: &Path) -> Result<()> {
    run_success(&["reset", "--hard", "HEAD@{1}"], Some(repo_path))
        .with_context(|| "Failed to undo last operation")?;
    Ok(())
}
