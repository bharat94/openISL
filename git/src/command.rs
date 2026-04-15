use crate::error::GitError;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

pub fn run(args: &[&str], cwd: Option<&Path>) -> Result<String> {
    let output = run_raw(args, cwd)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_success(args: &[&str], cwd: Option<&Path>) -> Result<()> {
    let output = run_raw(args, cwd)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()).into());
    }

    Ok(())
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

/// Run a git command with environment variables set
pub fn run_with_env(
    args: &[&str],
    cwd: Option<&Path>,
    env: &HashMap<&str, &str>,
) -> Result<String> {
    let mut cmd = Command::new("git");

    for arg in args {
        cmd.arg(arg);
    }

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    for (key, value) in env {
        cmd.env(key, value);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Run a git command with stdin input
pub fn run_with_stdin(args: &[&str], cwd: Option<&Path>, stdin_data: &str) -> Result<String> {
    let mut cmd = Command::new("git");

    for arg in args {
        cmd.arg(arg);
    }

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(stdin_data.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
