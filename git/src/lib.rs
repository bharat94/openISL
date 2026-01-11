pub mod command;
pub mod error;
pub mod models;
pub mod operations;

pub use command::{find_repo_root, is_git_repo};
pub use error::GitError;
pub use models::{Commit, GitRef, RefType};
pub use operations::{
    create_tag, delete_tag, get_branches, get_commit_diff, get_commits, get_current_branch,
    get_diff, get_status, remote_add, remote_list, remote_remove, tag_list, FileStatus, Remote,
    SmartLogFormatter, StatusType, Tag,
};
