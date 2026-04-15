use crate::command::{run, run_with_stdin};
use anyhow::{anyhow, Context, Result};
use std::path::Path;

/// Represents a hunk in a diff with its boundaries (simpler version for stage.rs)
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// The header line of the hunk (e.g., "@@ -1,5 +1,6 @@")
    pub header: String,
    /// The content lines of the hunk (including +, -, and context lines)
    pub lines: Vec<String>,
    /// Starting line number in the original file
    pub old_start: usize,
    /// Number of lines in the original file
    pub old_count: usize,
    /// Starting line number in the new file
    pub new_start: usize,
    /// Number of lines in the new file
    pub new_count: usize,
}

impl DiffHunk {
    /// Parse a hunk header line to extract line numbers
    fn parse_header(header: &str) -> Option<(usize, usize, usize, usize)> {
        // Format: @@ -old_start,old_count +new_start,new_count @@
        // or: @@ -old_start +new_start @@
        let header = header.trim();
        if !header.starts_with("@@") {
            return None;
        }

        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let old_part = parts[1].trim_start_matches('-');
        let new_part = parts[2].trim_start_matches('+');

        let parse_range = |s: &str| -> (usize, usize) {
            if let Some(idx) = s.find(',') {
                let start = s[..idx].parse().unwrap_or(1);
                let count = s[idx + 1..].parse().unwrap_or(1);
                (start, count)
            } else {
                (s.parse().unwrap_or(1), 1)
            }
        };

        let (old_start, old_count) = parse_range(old_part);
        let (new_start, new_count) = parse_range(new_part);

        Some((old_start, old_count, new_start, new_count))
    }
}

/// Parse hunks from a file's diff output
fn parse_hunks_from_diff(diff: &str) -> Vec<DiffHunk> {
    let mut hunks = Vec::new();
    let mut current_hunk: Option<DiffHunk> = None;

    for line in diff.lines() {
        if line.starts_with("@@") {
            // Save previous hunk if any
            if let Some(hunk) = current_hunk.take() {
                hunks.push(hunk);
            }

            // Parse the new hunk header
            if let Some((old_start, old_count, new_start, new_count)) =
                DiffHunk::parse_header(line)
            {
                current_hunk = Some(DiffHunk {
                    header: line.to_string(),
                    lines: Vec::new(),
                    old_start,
                    old_count,
                    new_start,
                    new_count,
                });
            }
        } else if let Some(ref mut hunk) = current_hunk {
            // Add line to current hunk if it's a diff line
            if line.starts_with('+')
                || line.starts_with('-')
                || line.starts_with(' ')
                || line.starts_with('\\')
            {
                hunk.lines.push(line.to_string());
            }
        }
    }

    // Don't forget the last hunk
    if let Some(hunk) = current_hunk {
        hunks.push(hunk);
    }

    hunks
}

/// Extract the diff header for a file (the lines before the first hunk)
fn extract_diff_header(diff: &str) -> String {
    let mut header_lines = Vec::new();

    for line in diff.lines() {
        if line.starts_with("@@") {
            break;
        }
        header_lines.push(line);
    }

    header_lines.join("\n")
}

pub fn stage_file(repo_path: &Path, file: &str) -> Result<()> {
    run(&["add", file], Some(repo_path))
        .with_context(|| format!("Failed to stage file: {}", file))?;
    Ok(())
}

pub fn stage_all(repo_path: &Path) -> Result<()> {
    run(&["add", "-A"], Some(repo_path)).with_context(|| "Failed to stage all files")?;
    Ok(())
}

pub fn unstage_file(repo_path: &Path, file: &str) -> Result<()> {
    run(&["reset", "--", file], Some(repo_path))
        .with_context(|| format!("Failed to unstage file: {}", file))?;
    Ok(())
}

pub fn unstage_all(repo_path: &Path) -> Result<()> {
    run(&["reset", "HEAD"], Some(repo_path)).with_context(|| "Failed to unstage all files")?;
    Ok(())
}

/// Stage a specific hunk by its line range in the original file.
///
/// # Arguments
/// * `repo_path` - Path to the repository
/// * `file` - The file path relative to the repository root
/// * `start_line` - Starting line number of the hunk in the original file
/// * `end_line` - Ending line number of the hunk in the original file
///
/// # Returns
/// * `Ok(())` if the hunk was staged successfully
/// * `Err` if the hunk could not be staged
pub fn stage_hunk_by_lines(
    repo_path: &Path,
    file: &str,
    start_line: usize,
    end_line: usize,
) -> Result<()> {
    // Get the unstaged diff for this file
    let diff = run(&["diff", "--", file], Some(repo_path))
        .with_context(|| format!("Failed to get diff for file: {}", file))?;

    if diff.trim().is_empty() {
        return Err(anyhow!("No unstaged changes in file: {}", file));
    }

    // Extract the diff header
    let header = extract_diff_header(&diff);

    // Parse all hunks
    let hunks = parse_hunks_from_diff(&diff);

    // Find the hunk that covers the requested line range
    let matching_hunk = hunks.iter().find(|h| {
        let hunk_end = h.old_start + h.old_count.saturating_sub(1);
        start_line >= h.old_start && end_line <= hunk_end
    });

    let hunk = matching_hunk.ok_or_else(|| {
        anyhow!(
            "No hunk found covering lines {}-{} in file: {}",
            start_line,
            end_line,
            file
        )
    })?;

    // Build a patch with just the selected hunk
    let mut patch = String::new();
    patch.push_str(&header);
    patch.push('\n');
    patch.push_str(&hunk.header);
    patch.push('\n');
    for line in &hunk.lines {
        patch.push_str(line);
        patch.push('\n');
    }

    // Apply the patch to the staging area
    run_with_stdin(&["apply", "--cached", "-"], Some(repo_path), &patch)
        .with_context(|| format!("Failed to stage hunk for file: {}", file))?;

    Ok(())
}

/// Get all hunks for a file's unstaged changes
pub fn get_file_hunks(repo_path: &Path, file: &str) -> Result<Vec<DiffHunk>> {
    let diff = run(&["diff", "--", file], Some(repo_path))
        .with_context(|| format!("Failed to get diff for file: {}", file))?;

    Ok(parse_hunks_from_diff(&diff))
}

pub fn get_staged_files(repo_path: &Path) -> Result<Vec<String>> {
    let output = run(&["diff", "--cached", "--name-only"], Some(repo_path))
        .with_context(|| "Failed to get staged files")?;

    Ok(output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect())
}

pub fn get_unstaged_files(repo_path: &Path) -> Result<Vec<String>> {
    let output = run(&["diff", "--name-only"], Some(repo_path))
        .with_context(|| "Failed to get unstaged files")?;

    Ok(output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect())
}

pub fn has_staged_changes(repo_path: &Path) -> Result<bool> {
    let output = run(&["status", "--porcelain"], Some(repo_path))
        .with_context(|| "Failed to check for staged changes")?;

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("M ")
            || line.starts_with("A ")
            || line.starts_with("D ")
            || line.starts_with("R ")
        {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn has_unstaged_changes(repo_path: &Path) -> Result<bool> {
    let output = run(&["status", "--porcelain"], Some(repo_path))
        .with_context(|| "Failed to check for unstaged changes")?;

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with(" M") || line.starts_with(" A") || line.starts_with(" D") {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_staged_changes() {
        let repo_path = std::env::current_dir().unwrap();
        let result = has_staged_changes(&repo_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_staged_files() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_staged_files(&repo_path);
        assert!(result.is_ok());
    }
}
