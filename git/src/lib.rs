pub mod models;
pub mod operations;
pub mod command;
pub mod error;

pub use command::{find_repo_root, is_git_repo};
pub use error::GitError;
pub use models::{Commit, GitRef, RefType};
pub use operations::{get_commits, get_branches, get_current_branch, get_status, get_diff, get_commit_diff, StatusType, FileStatus, SmartLogFormatter, remote_list, tag_list, remote_add, remote_remove, create_tag, delete_tag, Remote, Tag};
