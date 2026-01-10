use anyhow::{Context, Result};
use std::path::Path;
use crate::command::run;

pub fn get_diff(repo_path: &Path, commit: Option<&str>, staged: bool) -> Result<String> {
    let mut args = vec!["diff"];

    if staged {
        args.push("--staged");
    }

    if let Some(c) = commit {
        args.push(c);
    }

    run(&args, Some(repo_path))
        .with_context(|| format!("Failed to get diff for commit: {:?}", commit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_diff() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_diff(&repo_path, None, false);
        // This will work if in a git repo
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_diff_staged() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_diff(&repo_path, None, true);
        // This will work if in a git repo
        assert!(result.is_ok());
    }
}
