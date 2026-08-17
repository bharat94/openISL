use crate::command::{run, run_with_stdin};
use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum HunkLineType {
    Addition,
    Deletion,
    Context,
}

#[derive(Debug, Clone)]
pub struct HunkLine {
    pub content: String,
    pub line_type: HunkLineType,
    pub is_selected: bool, // For TUI interaction (visual staging)
}

#[derive(Debug, Clone)]
pub struct Hunk {
    pub header: String,
    pub lines: Vec<HunkLine>, // Changed from `content: Vec<String>`
    pub is_staged: bool,
    pub old_start: usize,  // Start line in old file
    pub old_lines: usize,  // Number of lines in old file
    pub new_start: usize,  // Start line in new file
    pub new_lines: usize,  // Number of lines in new file
    pub is_selected: bool, // For TUI interaction (visual staging of entire hunk)
}

pub fn get_file_diff_hunks(repo_path: &Path, file_path: &Path, staged: bool) -> Result<Vec<Hunk>> {
    let mut args = vec!["diff", "--no-color", "--no-ext-diff"];
    if staged {
        args.push("--staged");
    }
    // For unstaged changes, plain `git diff` compares the working tree against
    // the index. `--no-index` (previously used) compares two arbitrary paths
    // and is wrong here.
    args.push("--unified=3"); // Show 3 context lines for better hunk manipulation
    args.push("--");
    args.push(file_path.to_str().context("Invalid file path")?);

    let output = run(&args, Some(repo_path)).context("Failed to get file diff hunks")?;

    let mut hunks = Vec::new();
    let mut current_hunk: Option<Hunk> = None;

    for line in output.lines() {
        if line.starts_with("diff --git")
            || line.starts_with("--- a/")
            || line.starts_with("+++ b/")
        {
            // New file diff or file headers, finalize current hunk if exists
            if let Some(h) = current_hunk.take() {
                hunks.push(h);
            }
            continue;
        }

        if line.starts_with("@@") {
            // Hunk header
            if let Some(h) = current_hunk.take() {
                hunks.push(h);
            }

            let mut old_start = 0;
            let mut old_lines = 0;
            let mut new_start = 0;
            let mut new_lines = 0;

            if let Some(parts) = line.split("@@").nth(1) {
                let range_parts: Vec<&str> = parts.trim().split(' ').collect();
                if range_parts.len() >= 2 {
                    let old_range = range_parts[0];
                    let new_range = range_parts[1];

                    if let Some(start_str) = old_range.split(',').next() {
                        old_start = start_str.trim_start_matches('-').parse().unwrap_or(0);
                    }
                    if let Some(count_str) = old_range.split(',').nth(1) {
                        old_lines = count_str.parse().unwrap_or(0);
                    } else {
                        old_lines = 1;
                    }

                    if let Some(start_str) = new_range.split(',').next() {
                        new_start = start_str.trim_start_matches('+').parse().unwrap_or(0);
                    }
                    if let Some(count_str) = new_range.split(',').nth(1) {
                        new_lines = count_str.parse().unwrap_or(0);
                    } else {
                        new_lines = 1;
                    }
                }
            }

            current_hunk = Some(Hunk {
                header: line.to_string(),
                lines: Vec::new(),
                is_staged: staged,
                old_start,
                old_lines,
                new_start,
                new_lines,
                is_selected: false,
            });
        } else if let Some(hunk) = &mut current_hunk {
            let line_type = if line.starts_with('+') {
                HunkLineType::Addition
            } else if line.starts_with('-') {
                HunkLineType::Deletion
            } else {
                HunkLineType::Context
            };
            hunk.lines.push(HunkLine {
                content: line.to_string(),
                line_type,
                is_selected: false,
            });
        }
    }

    if let Some(h) = current_hunk.take() {
        hunks.push(h);
    }

    Ok(hunks)
}

/// Build a patch containing the full hunk, prefixed with the file headers that
/// `git apply` needs to identify the target file.
fn build_hunk_patch(file_path: &Path, hunk: &Hunk) -> String {
    let path = file_path.to_string_lossy();
    let mut patch = String::new();
    patch.push_str(&format!("diff --git a/{path} b/{path}\n"));
    patch.push_str(&format!("--- a/{path}\n"));
    patch.push_str(&format!("+++ b/{path}\n"));
    patch.push_str(&hunk.header);
    patch.push('\n');
    for line in &hunk.lines {
        patch.push_str(&line.content);
        patch.push('\n');
    }
    patch
}

/// Build a patch containing only the selected lines of a hunk, with the hunk
/// header recomputed so the partial hunk applies at the correct positions.
///
/// Returns `None` when no lines are selected.
fn build_partial_hunk_patch(file_path: &Path, hunk: &Hunk) -> Option<String> {
    let mut old_line = hunk.old_start;
    let mut new_line = hunk.new_start;

    let mut old_start: Option<usize> = None;
    let mut new_start: Option<usize> = None;
    let mut old_count = 0;
    let mut new_count = 0;
    let mut body = String::new();

    for line in &hunk.lines {
        match line.line_type {
            HunkLineType::Deletion => {
                if line.is_selected {
                    old_start.get_or_insert(old_line);
                    new_start.get_or_insert(new_line);
                    old_count += 1;
                    body.push_str(&line.content);
                    body.push('\n');
                }
                old_line += 1;
            }
            HunkLineType::Addition => {
                if line.is_selected {
                    old_start.get_or_insert(old_line);
                    new_start.get_or_insert(new_line);
                    new_count += 1;
                    body.push_str(&line.content);
                    body.push('\n');
                }
                new_line += 1;
            }
            HunkLineType::Context => {
                old_line += 1;
                new_line += 1;
            }
        }
    }

    if body.is_empty() {
        return None;
    }

    let old_start = old_start.unwrap_or(old_line);
    let new_start = new_start.unwrap_or(new_line);

    let path = file_path.to_string_lossy();
    let mut patch = String::new();
    patch.push_str(&format!("diff --git a/{path} b/{path}\n"));
    patch.push_str(&format!("--- a/{path}\n"));
    patch.push_str(&format!("+++ b/{path}\n"));
    patch.push_str(&format!(
        "@@ -{old_start},{old_count} +{new_start},{new_count} @@\n"
    ));
    patch.push_str(&body);

    Some(patch)
}

/// Stage a whole hunk by applying its patch to the index.
pub fn stage_hunk(repo_path: &Path, file_path: &Path, hunk: &Hunk) -> Result<()> {
    let patch = build_hunk_patch(file_path, hunk);
    run_with_stdin(&["apply", "--cached", "-"], Some(repo_path), &patch).context(format!(
        "Failed to stage hunk for file {}",
        file_path.display()
    ))?;
    Ok(())
}

/// Unstage a whole hunk by applying its patch to the index in reverse.
pub fn unstage_hunk(repo_path: &Path, file_path: &Path, hunk: &Hunk) -> Result<()> {
    let patch = build_hunk_patch(file_path, hunk);
    run_with_stdin(
        &["apply", "--cached", "--reverse", "-"],
        Some(repo_path),
        &patch,
    )
    .context(format!(
        "Failed to unstage hunk for file {}",
        file_path.display()
    ))?;
    Ok(())
}

/// Stage only the selected lines of a hunk.
pub fn stage_hunk_lines(repo_path: &Path, file_path: &Path, hunk: &Hunk) -> Result<()> {
    let patch = build_partial_hunk_patch(file_path, hunk)
        .ok_or_else(|| anyhow::anyhow!("No lines selected in hunk"))?;
    run_with_stdin(
        &["apply", "--cached", "--unidiff-zero", "-"],
        Some(repo_path),
        &patch,
    )
    .context(format!(
        "Failed to stage selected lines for file {}",
        file_path.display()
    ))?;
    Ok(())
}

/// Unstage only the selected lines of a hunk.
pub fn unstage_hunk_lines(repo_path: &Path, file_path: &Path, hunk: &Hunk) -> Result<()> {
    let patch = build_partial_hunk_patch(file_path, hunk)
        .ok_or_else(|| anyhow::anyhow!("No lines selected in hunk"))?;
    run_with_stdin(
        &["apply", "--cached", "--reverse", "--unidiff-zero", "-"],
        Some(repo_path),
        &patch,
    )
    .context(format!(
        "Failed to unstage selected lines for file {}",
        file_path.display()
    ))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    fn git(repo: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(args)
            .current_dir(repo)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} failed", args);
    }

    fn git_output(repo: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .unwrap();
        assert!(output.status.success(), "git {:?} failed", args);
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    /// Create a temp repo with a committed file:
    ///   line1, line2, line3, line4, line5
    fn create_test_repo() -> (TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init", "-q"]);
        git(dir.path(), &["config", "user.email", "test@example.com"]);
        git(dir.path(), &["config", "user.name", "Test"]);
        fs::write(
            dir.path().join("test.txt"),
            "line1\nline2\nline3\nline4\nline5\n",
        )
        .unwrap();
        git(dir.path(), &["add", "test.txt"]);
        git(dir.path(), &["commit", "-q", "-m", "initial"]);
        let file = dir.path().join("test.txt");
        (dir, file)
    }

    #[test]
    fn test_get_unstaged_hunks() {
        let (dir, file) = create_test_repo();
        let repo = dir.path();
        fs::write(&file, "line1\nCHANGED\nline3\nline4\nline5\n").unwrap();

        let hunks = get_file_diff_hunks(repo, Path::new("test.txt"), false).unwrap();
        assert_eq!(hunks.len(), 1);
        assert!(!hunks[0].is_staged);
        assert_eq!(hunks[0].old_start, 1);
        assert!(hunks[0]
            .lines
            .iter()
            .any(|l| l.line_type == HunkLineType::Deletion && l.content.contains("line2")));
        assert!(hunks[0]
            .lines
            .iter()
            .any(|l| l.line_type == HunkLineType::Addition && l.content.contains("CHANGED")));
    }

    #[test]
    fn test_get_staged_hunks() {
        let (dir, file) = create_test_repo();
        let repo = dir.path();
        fs::write(&file, "line1\nCHANGED\nline3\nline4\nline5\n").unwrap();
        git(repo, &["add", "test.txt"]);

        let hunks = get_file_diff_hunks(repo, Path::new("test.txt"), true).unwrap();
        assert_eq!(hunks.len(), 1);
        assert!(hunks[0].is_staged);
    }

    #[test]
    fn test_stage_hunk() {
        let (dir, file) = create_test_repo();
        let repo = dir.path();
        fs::write(&file, "line1\nCHANGED\nline3\nline4\nline5\n").unwrap();

        let hunks = get_file_diff_hunks(repo, Path::new("test.txt"), false).unwrap();
        stage_hunk(repo, Path::new("test.txt"), &hunks[0]).unwrap();

        let staged = git_output(repo, &["diff", "--cached", "--", "test.txt"]);
        assert!(staged.contains("+CHANGED"));
        assert!(staged.contains("-line2"));
    }

    #[test]
    fn test_stage_hunk_lines_partial() {
        let (dir, file) = create_test_repo();
        let repo = dir.path();
        // Commit a 7-line file, then edit lines 3 and 5.
        fs::write(&file, "line1\nline2\nline3\nline4\nline5\nline6\nline7\n").unwrap();
        git(repo, &["add", "test.txt"]);
        git(repo, &["commit", "-q", "-m", "second"]);
        fs::write(
            &file,
            "line1\nline2\nCHANGED3\nline4\nCHANGED5\nline6\nline7\n",
        )
        .unwrap();

        let hunks = get_file_diff_hunks(repo, Path::new("test.txt"), false).unwrap();
        assert_eq!(hunks.len(), 1);

        // Select only the second change (line5 -> CHANGED5).
        let mut selected = hunks[0].clone();
        let mut found = false;
        for line in selected.lines.iter_mut() {
            if (line.line_type == HunkLineType::Addition && line.content.contains("CHANGED5"))
                || (line.line_type == HunkLineType::Deletion && line.content.contains("line5"))
            {
                line.is_selected = true;
                found = true;
            }
        }
        assert!(found, "expected to find the line5 change in the hunk");

        stage_hunk_lines(repo, Path::new("test.txt"), &selected).unwrap();

        let staged = git_output(repo, &["diff", "--cached", "--", "test.txt"]);
        assert!(staged.contains("+CHANGED5"), "staged diff was:\n{}", staged);
        assert!(!staged.contains("CHANGED3"), "staged diff was:\n{}", staged);

        // The unselected change must remain unstaged.
        let unstaged = git_output(repo, &["diff", "--", "test.txt"]);
        assert!(
            unstaged.contains("CHANGED3"),
            "unstaged diff was:\n{}",
            unstaged
        );
        assert!(
            !unstaged.contains("+CHANGED5"),
            "unstaged diff was:\n{}",
            unstaged
        );
    }

    #[test]
    fn test_unstage_hunk() {
        let (dir, file) = create_test_repo();
        let repo = dir.path();
        fs::write(&file, "line1\nCHANGED\nline3\nline4\nline5\n").unwrap();
        git(repo, &["add", "test.txt"]);

        let staged_hunks = get_file_diff_hunks(repo, Path::new("test.txt"), true).unwrap();
        unstage_hunk(repo, Path::new("test.txt"), &staged_hunks[0]).unwrap();

        let staged = git_output(repo, &["diff", "--cached", "--", "test.txt"]);
        assert!(!staged.contains("CHANGED"));
    }

    #[test]
    fn test_stage_hunk_with_no_changes_returns_empty() {
        let (dir, _file) = create_test_repo();
        let repo = dir.path();
        let hunks = get_file_diff_hunks(repo, Path::new("test.txt"), false).unwrap();
        assert!(hunks.is_empty());
    }
}
