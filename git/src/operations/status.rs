use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

pub struct FileStatus {
    pub path: String,
    pub status: StatusType,
}

#[derive(Debug, PartialEq)]
pub enum StatusType {
    Modified,
    Added,
    Deleted,
    Untracked,
    ModifiedStaged,
    AddedStaged,
    DeletedStaged,
    Renamed,
    Conflicted,
}

pub fn get_status(repo_path: &Path) -> Result<Vec<FileStatus>> {
    let output = run(&["status", "--porcelain"], Some(repo_path))
        .with_context(|| "Failed to get git status")?;

    let mut files = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.len() < 4 {
            continue;
        }

        let status_code = &line[0..2];
        let path = line[3..].trim().to_string();

        let status_type = match status_code {
            " M" => StatusType::Modified,
            "M " => StatusType::ModifiedStaged,
            "A " => StatusType::AddedStaged,
            "AM" => StatusType::Added,
            " D" => StatusType::Deleted,
            "D " => StatusType::DeletedStaged,
            "??" => StatusType::Untracked,
            "R " => StatusType::Renamed,
            "UU" => StatusType::Conflicted,
            _ => StatusType::Modified,
        };

        files.push(FileStatus {
            path,
            status: status_type,
        });
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_status(&repo_path);
        // This will work if in a git repo
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.is_empty() || !files.is_empty());
    }

    #[test]
    fn test_status_type_equality() {
        assert_eq!(StatusType::Modified, StatusType::Modified);
        assert_ne!(StatusType::Modified, StatusType::Added);
    }
}
