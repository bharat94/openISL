pub mod command;
pub mod error;
pub mod models;
pub mod operations;
pub mod vcs;

pub use command::{find_repo_root, is_git_repo};
pub use error::GitError;
pub use models::{Commit, GitRef, RefType};
pub use operations::{
    amend_commit, cherry_pick_commit, create_tag, delete_tag, drop_commit, get_branches,
    get_commit_diff, get_commit_message, get_commits, get_current_branch, get_diff,
    get_staged_files, get_status, get_sync_state, has_staged_changes, has_unstaged_changes,
    remote_add, remote_list, remote_remove, revert_commit, squash_commits, stage_all, stage_file,
    tag_commit, tag_list, unstage_all, unstage_file, FileStatus, Remote, SmartLogFormatter,
    StatusType, Tag,
};

pub use vcs::{Change, Ref, RefType as VcsRefType, SyncState};
