use anyhow::{Context, Result};
use std::path::Path;
use tempfile; // Added for temporary file creation
use crate::command::run;

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
    pub old_start: usize, // Start line in old file
    pub old_lines: usize, // Number of lines in old file
    pub new_start: usize, // Start line in new file
    pub new_lines: usize, // Number of lines in new file
    pub is_selected: bool, // For TUI interaction (visual staging of entire hunk)
}

pub fn get_file_diff_hunks(repo_path: &Path, file_path: &Path, staged: bool) -> Result<Vec<Hunk>> {
    let mut args = vec!["diff", "--no-color", "--no-ext-diff"];
    if staged {
        args.push("--staged");
    } else {
        args.push("--no-index"); // For unstaged changes, compare with HEAD
    }
    args.push("--unified=3"); // Show 3 context lines for better hunk manipulation
    args.push("--");
    args.push(file_path.to_str().context("Invalid file path")?);

    let output = run(&args, Some(repo_path)).context("Failed to get file diff hunks")?;

    let mut hunks = Vec::new();
    let mut current_hunk: Option<Hunk> = None;

    for line in output.lines() {
        if line.starts_with("diff --git") || line.starts_with("--- a/") || line.starts_with("+++ b/") {
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

pub fn stage_hunk(repo_path: &Path, file_path: &Path, hunk_index: usize, _diff_output: &str) -> Result<()> {
    // Stage a specific hunk by creating a temporary patch file and applying it.
    // This assumes diff_output contains the full diff for the file.
    // We need to extract the specific hunk and apply it with `git apply --cached`.

    let hunks = get_file_diff_hunks(repo_path, file_path, false)?; // Get unstaged hunks
    if let Some(hunk_to_stage) = hunks.get(hunk_index) {
        let patch_content = format!("{}\n{}", hunk_to_stage.header, hunk_to_stage.lines.iter().map(|l| l.content.clone()).collect::<Vec<String>>().join("\n"));
        let temp_dir = tempfile::tempdir()?;
        let patch_file_path = temp_dir.path().join("hunk.patch");
        std::fs::write(&patch_file_path, patch_content)?;

        run(
            &[
                "apply",
                "--cached", // Stage the changes
                "--unidiff-zero",
                patch_file_path.to_str().context("Invalid patch file path")?,
            ],
            Some(repo_path),
        )
        .context(format!("Failed to stage hunk {} for file {}", hunk_index, file_path.display()))?;

        Ok(())
    } else {
        Err(anyhow::anyhow!("Hunk not found at index {}", hunk_index))
    }
}

pub fn unstage_hunk(repo_path: &Path, file_path: &Path, hunk_index: usize, _diff_output: &str) -> Result<()> {
    // Unstage a specific hunk by creating a temporary patch file and applying it in reverse.
    // This assumes diff_output contains the full diff for the file.

    let hunks = get_file_diff_hunks(repo_path, file_path, true)?; // Get staged hunks
    if let Some(hunk_to_unstage) = hunks.get(hunk_index) {
        let patch_content = format!("{}\n{}", hunk_to_unstage.header, hunk_to_unstage.lines.iter().map(|l| l.content.clone()).collect::<Vec<String>>().join("\n"));
        let temp_dir = tempfile::tempdir()?;
        let patch_file_path = temp_dir.path().join("hunk.patch");
        std::fs::write(&patch_file_path, patch_content)?;

        run(
            &[
                "apply",
                "--cached", // Unstage the changes (apply in reverse to index)
                "--reverse",
                "--unidiff-zero",
                patch_file_path.to_str().context("Invalid patch file path")?,
            ],
            Some(repo_path),
        )
        .context(format!("Failed to unstage hunk {} for file {}", hunk_index, file_path.display()))?;

        Ok(())
    } else {
        Err(anyhow::anyhow!("Hunk not found at index {}", hunk_index))
    }
}
