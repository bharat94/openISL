use crate::command;
use crate::vcs::SyncState;
use anyhow::{Context, Result};
use std::path::Path;

#[cfg(test)]
use tempfile;

/// Fetch sync state (ahead/behind counts and conflicts)
pub fn get_sync_state(repo_path: &Path) -> Result<SyncState> {
    let mut sync_state = SyncState::default();

    // Detect remote tracking branch
    let current_branch = get_current_branch(repo_path)?;
    let (remote_name, remote_branch) = if let Some(branch) = current_branch {
        get_tracking_remote(repo_path, &branch)?
    } else {
        return Ok(sync_state);
    };

    sync_state.remote_name = remote_name.clone();

    // Get ahead/behind counts
    if let (Some(ref remote), Some(ref branch)) = (&remote_name, &remote_branch) {
        let remote_ref = format!("{}/{}", remote, branch);
        get_ahead_behind(repo_path, &remote_ref, &mut sync_state)?;
    }

    // Check for conflicts
    let status = crate::get_status(repo_path)?;
    sync_state.has_conflicts = status
        .iter()
        .any(|f| matches!(f.status, crate::StatusType::Conflicted));

    Ok(sync_state)
}

fn get_tracking_remote(repo_path: &Path, branch: &str) -> Result<(Option<String>, Option<String>)> {
    let output = command::run_raw(
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            &format!("{}@{{u}}", branch),
        ],
        Some(repo_path),
    )
    .context("Failed to get tracking remote")?;

    if output.status.success() {
        let tracking_lossy = String::from_utf8_lossy(&output.stdout);
        let tracking = tracking_lossy.trim();
        if tracking.is_empty() || tracking.contains("@") {
            return Ok((None, None));
        }

        if let Some(pos) = tracking.find('/') {
            let remote = tracking[..pos].to_string();
            let branch_name = tracking[pos + 1..].to_string();
            Ok((Some(remote), Some(branch_name)))
        } else {
            Ok((None, None))
        }
    } else {
        Ok((None, None))
    }
}

fn get_ahead_behind(repo_path: &Path, remote_ref: &str, sync_state: &mut SyncState) -> Result<()> {
    // Get ahead count (local commits not pushed)
    let ahead_output = command::run_raw(
        &["rev-list", "--count", &format!("HEAD...{}", remote_ref)],
        Some(repo_path),
    )
    .context("Failed to get ahead count")?;

    if ahead_output.status.success() {
        let ahead_lossy = String::from_utf8_lossy(&ahead_output.stdout);
        let ahead = ahead_lossy.trim().parse().ok();
        sync_state.local_unpushed = ahead;
    }

    // Get behind count (remote commits not pulled)
    let behind_output = command::run_raw(
        &["rev-list", "--count", &format!("{}...HEAD", remote_ref)],
        Some(repo_path),
    )
    .context("Failed to get behind count")?;

    if behind_output.status.success() {
        let behind_lossy = String::from_utf8_lossy(&behind_output.stdout);
        let behind = behind_lossy.trim().parse().ok();
        sync_state.remote_unpulled = behind;
    }

    Ok(())
}

/// Get current branch (wrapper for get_current_branch)
pub fn get_current_branch(repo_path: &Path) -> Result<Option<String>> {
    crate::get_current_branch(repo_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sync_state_no_remote() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path();

        command::run_raw(&["init"], Some(repo_path)).unwrap();
        command::run_raw(&["config", "user.email", "test@test.com"], Some(repo_path)).unwrap();
        command::run_raw(&["config", "user.name", "Test User"], Some(repo_path)).unwrap();

        std::fs::write(repo_path.join("test.txt"), "test").unwrap();
        command::run_raw(&["add", "test.txt"], Some(repo_path)).unwrap();
        command::run_raw(&["commit", "-m", "Initial commit"], Some(repo_path)).unwrap();

        let sync_state = get_sync_state(repo_path).unwrap();

        assert_eq!(sync_state.local_unpushed, None);
        assert_eq!(sync_state.remote_unpulled, None);
        assert!(!sync_state.has_conflicts);
    }

    #[test]
    fn test_get_sync_state_with_conflicts() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path();

        command::run_raw(&["init"], Some(repo_path)).unwrap();
        command::run_raw(&["config", "user.email", "test@test.com"], Some(repo_path)).unwrap();
        command::run_raw(&["config", "user.name", "Test User"], Some(repo_path)).unwrap();

        std::fs::write(repo_path.join("test.txt"), "test").unwrap();
        command::run_raw(&["add", "test.txt"], Some(repo_path)).unwrap();
        command::run_raw(&["commit", "-m", "First commit"], Some(repo_path)).unwrap();

        command::run_raw(&["checkout", "-b", "conflict-branch"], Some(repo_path)).unwrap();
        std::fs::write(repo_path.join("test.txt"), "conflict").unwrap();
        command::run_raw(&["add", "test.txt"], Some(repo_path)).unwrap();
        command::run_raw(&["commit", "-m", "Conflict commit"], Some(repo_path)).unwrap();

        command::run_raw(&["checkout", "main"], Some(repo_path)).unwrap();

        std::fs::write(repo_path.join("test.txt"), "main-change").unwrap();
        command::run_raw(&["add", "test.txt"], Some(repo_path)).unwrap();
        command::run_raw(&["commit", "-m", "Main commit"], Some(repo_path)).unwrap();

        command::run_raw(&["merge", "conflict-branch"], Some(repo_path)).unwrap();

        let sync_state = get_sync_state(repo_path).unwrap();

        assert!(sync_state.has_conflicts);
    }
}
