use crate::command::run_success;
use anyhow::{Context, Result};
use std::path::Path;

/// Apply a patch file to the working tree (or the index with `staged`).
pub fn apply_patch(repo_path: &Path, patch: &str, staged: bool) -> Result<()> {
    let mut args = vec!["apply"];
    if staged {
        args.push("--cached");
    }
    args.push(patch);
    run_success(&args, Some(repo_path))
        .with_context(|| format!("Failed to apply patch '{}'", patch))?;
    Ok(())
}
