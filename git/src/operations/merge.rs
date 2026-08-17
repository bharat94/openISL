use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

/// Merge the given branch (or commit) into the current branch.
pub fn merge(repo_path: &Path, target: &str, no_edit: bool) -> Result<String> {
    let mut args = vec!["merge", target];

    if no_edit {
        args.push("--no-edit");
    }

    run(&args, Some(repo_path)).with_context(|| format!("Failed to merge '{}'", target))
}
