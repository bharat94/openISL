pub mod command;
pub mod error;
pub mod models;
pub mod operations;

pub use command::{find_repo_root, is_git_repo};
pub use error::GitError;
pub use models::{Commit, GitRef, RefType};
pub use operations::{
    create_tag, delete_tag, get_branches, get_commit_diff, get_commits, get_current_branch,
    get_diff, get_staged_files, get_status, has_staged_changes, has_unstaged_changes, remote_add,
    remote_list, remote_remove, stage_all, stage_file, tag_list, unstage_all, unstage_file,
    FileStatus, Remote, SmartLogFormatter, StatusType, Tag,
};
