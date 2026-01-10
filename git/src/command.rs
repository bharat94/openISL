use anyhow::Result;
use std::process::{Command, Output};
use crate::error::GitError;

pub fn run(args: &[&str], cwd: Option<&std::path::Path>) -> Result<String> {
    let output = run_raw(args, cwd)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::CommandFailed(stderr.to_string()).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_raw(args: &[&str], cwd: Option<&std::path::Path>) -> Result<Output> {
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
