use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

/// How aggressively `reset` moves the index and working tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetMode {
    /// Move HEAD only; keep index and working tree (`--soft`).
    Soft,
    /// Move HEAD and index; keep working tree (default).
    Mixed,
    /// Move HEAD, index, and working tree; discard changes (`--hard`).
    Hard,
}

/// Reset the current branch to a target revision.
pub fn reset(repo_path: &Path, mode: ResetMode, target: Option<&str>) -> Result<String> {
    let mut args = vec!["reset"];

    match mode {
        ResetMode::Soft => args.push("--soft"),
        ResetMode::Mixed => {}
        ResetMode::Hard => args.push("--hard"),
    }

    if let Some(target) = target {
        args.push(target);
    }

    run(&args, Some(repo_path)).with_context(|| "Failed to reset")
}
