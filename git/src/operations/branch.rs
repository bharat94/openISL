use anyhow::Result;
use crate::models::GitRef;

pub fn get_branches(_repo_path: &std::path::Path) -> Result<Vec<GitRef>> {
    todo!("Implement get_branches")
}

pub fn get_current_branch(_repo_path: &std::path::Path) -> Result<Option<String>> {
    todo!("Implement get_current_branch")
}
