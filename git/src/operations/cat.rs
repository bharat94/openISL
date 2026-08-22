use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

/// Print the contents of a file at a given revision.
pub fn get_file_at_revision(repo_path: &Path, revision: &str, path: &str) -> Result<String> {
    let spec = format!("{}:{}", revision, path);
    run(&["show", &spec], Some(repo_path))
        .with_context(|| format!("Failed to show '{}' at '{}'", path, revision))
}
