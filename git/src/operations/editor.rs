use crate::command::run;
use anyhow::{Context, Result};
use std::env;
use std::path::Path;

pub fn open_in_editor(repo_path: &Path, file_path: &Path) -> Result<()> {
    let editor = env::var("GIT_EDITOR")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string()); // Default to vi if no editor is set

    let file_path_str = file_path.to_str().context("Invalid file path")?;

    run(&[&editor, file_path_str], Some(repo_path))
        .context(format!("Failed to open file in editor: {}", editor))?;

    Ok(())
}
