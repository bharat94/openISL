use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use crate::error::GitError;

pub fn run(args: &[&str], cwd: Option<&Path>) -> Result<String> {
    let output = run_raw(args, cwd)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_raw(args: &[&str], cwd: Option<&Path>) -> Result<Output> {
    let mut cmd = Command::new("git");

    for arg in args {
        cmd.arg(arg);
    }

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    let output = cmd.output()?;

    Ok(output)
}

pub fn find_repo_root(path: &Path) -> Result<PathBuf> {
    let mut current = std::fs::canonicalize(path)?;

    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Ok(current);
        }

        current = match current.parent() {
            Some(parent) => parent.to_path_buf(),
            None => {
                return Err(GitError::RepositoryNotFound.into());
            }
        };
    }
}

pub fn is_git_repo(path: &Path) -> bool {
    let mut current = path;
    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return true;
        }

        current = match current.parent() {
            Some(parent) => parent,
            None => return false,
        };
    }
}
