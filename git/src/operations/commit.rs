use crate::command::{run, run_success, run_with_env};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Create a commit from the currently staged changes.
pub fn commit(repo_path: &Path, message: &str) -> Result<()> {
    run_success(&["commit", "-m", message], Some(repo_path))
        .with_context(|| "Failed to create commit")?;
    Ok(())
}

pub fn amend_commit(repo_path: &Path, amend_message: Option<&str>) -> Result<()> {
    if let Some(msg) = amend_message {
        run_success(&["commit", "--amend", "-m", msg], Some(repo_path))
            .with_context(|| "Failed to amend commit with message")?;
    } else {
        run_success(&["commit", "--amend", "--no-edit"], Some(repo_path))
            .with_context(|| "Failed to amend commit")?;
    }
    Ok(())
}

/// Reword a commit's message. For HEAD, uses amend. For other commits, uses interactive rebase.
pub fn reword_commit(repo_path: &Path, commit_hash: &str, message: &str) -> Result<()> {
    // Check if the commit is HEAD
    let head =
        run(&["rev-parse", "HEAD"], Some(repo_path)).with_context(|| "Failed to get HEAD")?;
    let head = head.trim();

    let target = run(&["rev-parse", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to resolve commit {}", commit_hash))?;
    let target = target.trim();

    if head == target {
        // Simple case: amend HEAD
        run_success(&["commit", "--amend", "-m", message], Some(repo_path))
            .with_context(|| "Failed to amend HEAD commit")?;
    } else {
        // Complex case: need to use interactive rebase
        // Find the parent of the target commit
        let parent = run(
            &["rev-parse", &format!("{}^", commit_hash)],
            Some(repo_path),
        )
        .with_context(|| format!("Failed to get parent of {}", commit_hash))?;
        let parent = parent.trim();

        let short_hash = &target[..7.min(target.len())];

        // Create a temporary directory for our editor scripts
        let temp_dir = std::env::temp_dir().join(format!("openisl-reword-{}", std::process::id()));
        fs::create_dir_all(&temp_dir)?;

        // Create the sequence editor script that changes 'pick <hash>' to 'reword <hash>'
        let seq_editor_path = temp_dir.join("seq-editor.sh");
        let seq_editor_script = format!(
            "#!/bin/sh\nsed -i.bak 's/^pick {}/reword {}/g' \"$1\"\n",
            short_hash, short_hash
        );
        fs::write(&seq_editor_path, &seq_editor_script)?;

        // Make it executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&seq_editor_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&seq_editor_path, perms)?;
        }

        // Create the message editor script that writes our message
        let msg_editor_path = temp_dir.join("msg-editor.sh");
        // Escape single quotes in the message for shell
        let escaped_message = message.replace('\'', "'\\''");
        let msg_editor_script = format!("#!/bin/sh\nprintf '%s' '{}' > \"$1\"\n", escaped_message);
        fs::write(&msg_editor_path, &msg_editor_script)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&msg_editor_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&msg_editor_path, perms)?;
        }

        // Run the rebase with our custom editors
        let mut env = HashMap::new();
        let seq_editor_str = seq_editor_path.to_string_lossy();
        let msg_editor_str = msg_editor_path.to_string_lossy();
        env.insert("GIT_SEQUENCE_EDITOR", seq_editor_str.as_ref());
        env.insert("GIT_EDITOR", msg_editor_str.as_ref());

        let result = run_with_env(&["rebase", "-i", parent], Some(repo_path), &env);

        // Clean up temp files
        let _ = fs::remove_dir_all(&temp_dir);

        result.with_context(|| format!("Failed to reword commit {}", commit_hash))?;
    }

    Ok(())
}

pub fn drop_commit(repo_path: &Path, commit_hash: &str) -> Result<()> {
    run_success(
        &[
            "rebase",
            "--onto",
            &format!("^{}", commit_hash),
            commit_hash,
        ],
        Some(repo_path),
    )
    .with_context(|| format!("Failed to drop commit {}", commit_hash))?;
    Ok(())
}

pub fn squash_commits(repo_path: &Path, commit_hash: &str, message: &str) -> Result<()> {
    run_success(&["reset", "--soft", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to reset to {}", commit_hash))?;

    run_success(&["commit", "-m", message], Some(repo_path))
        .with_context(|| "Failed to create squashed commit")?;

    Ok(())
}

pub fn get_commit_message(repo_path: &Path, commit_hash: &str) -> Result<String> {
    let output = crate::command::run(&["log", "-1", "--format=%B", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to get message for commit {}", commit_hash))?;

    Ok(output)
}

pub fn tag_commit(
    repo_path: &Path,
    commit_hash: &str,
    tag_name: &str,
    message: Option<&str>,
) -> Result<()> {
    let mut args = vec!["tag", "-a", tag_name, commit_hash];
    if let Some(msg) = message {
        args.push("-m");
        args.push(msg);
    }
    run_success(&args, Some(repo_path))
        .with_context(|| format!("Failed to tag commit {} as {}", commit_hash, tag_name))?;
    Ok(())
}

pub fn cherry_pick_commit(repo_path: &Path, commit_hash: &str) -> Result<()> {
    run_success(&["cherry-pick", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to cherry-pick commit {}", commit_hash))?;
    Ok(())
}

pub fn revert_commit(repo_path: &Path, commit_hash: &str) -> Result<()> {
    run_success(&["revert", commit_hash], Some(repo_path))
        .with_context(|| format!("Failed to revert commit {}", commit_hash))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_get_commit_message() {
        let repo_path = current_dir().unwrap();
        let result = get_commit_message(&repo_path, "HEAD");
        assert!(result.is_ok());
    }
}
