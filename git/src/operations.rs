pub mod branch;
pub mod checkout;
pub mod commit;
pub mod diff;
pub mod log;
pub mod remote;
pub mod smart_log;
pub mod stage;
pub mod stash;
pub mod status;
pub mod sync;
pub mod tag;
pub mod editor;

pub use branch::{create_branch, create_branch_from_commit, get_branches, get_current_branch};
pub use checkout::{checkout, checkout_commit};
pub use commit::{
    amend_commit, cherry_pick_commit, drop_commit, get_commit_message, revert_commit,
    squash_commits, tag_commit,
};
pub use diff::{get_commit_diff, get_diff};
pub use log::get_commits;
pub use remote::{fetch, pull, push, remote_add, remote_list, remote_remove, Remote};
pub use smart_log::SmartLogFormatter;
pub use stage::{
    get_staged_files, get_unstaged_files, has_staged_changes, has_unstaged_changes, stage_all,
    stage_file, stage_hunk, unstage_all, unstage_file,
};
pub use stash::{
    get_stash_list, stash_apply, stash_drop, stash_pop, stash_push, stash_show, StashEntry,
};
pub use status::{get_status, FileStatus, StatusType};
pub use sync::get_sync_state;
pub use tag::{create_tag, delete_tag, show_tag, tag_list, Tag};
pub use editor::open_in_editor;
