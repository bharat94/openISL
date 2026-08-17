use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

/// Rebase the current branch onto the given upstream.
pub fn rebase(repo_path: &Path, upstream: Option<&str>, interactive: bool) -> Result<String> {
    let mut args = vec!["rebase"];

    if interactive {
        args.push("--interactive");
    }

    if let Some(upstream) = upstream {
        args.push(upstream);
    }

    run(&args, Some(repo_path)).with_context(|| "Failed to rebase")
}
