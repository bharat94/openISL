use crate::command::run_success;
use anyhow::{Context, Result};
use std::path::Path;

/// Initialize a new Git repository in the given directory.
pub fn init(repo_path: &Path) -> Result<()> {
    run_success(&["init"], Some(repo_path)).with_context(|| "Failed to initialize repository")?;
    Ok(())
}

/// Clone a remote repository into the given destination directory.
pub fn clone(url: &str, destination: &str) -> Result<()> {
    run_success(&["clone", url, destination], None)
        .with_context(|| format!("Failed to clone '{}'", url))?;
    Ok(())
}
