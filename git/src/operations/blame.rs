use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

/// Annotate each line of a file with the commit that last touched it (`git blame`).
pub fn get_blame(repo_path: &Path, path: &str) -> Result<String> {
    run(&["blame", "--", path], Some(repo_path))
        .with_context(|| format!("Failed to blame '{}'", path))
}
