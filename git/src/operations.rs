pub mod branch;
pub mod checkout;
pub mod commit;
pub mod diff;
pub mod editor;
pub mod hunk;
pub mod log;
pub mod remote;
pub mod smart_log;
pub mod stage;
pub mod stash;
pub mod status;
pub mod sync;
pub mod tag;

pub use branch::{
    create_branch, create_branch_from_commit, get_branches, get_current_branch,
    get_refs_for_commit,
};
pub use checkout::{checkout, checkout_commit};
pub use commit::{
    amend_commit, cherry_pick_commit, drop_commit, get_commit_message, revert_commit,
    reword_commit, squash_commits, tag_commit,
};
pub use diff::{get_commit_diff, get_diff};
pub use editor::open_in_editor;
pub use hunk::{get_file_diff_hunks, stage_hunk, unstage_hunk, Hunk, HunkLine, HunkLineType};
pub use log::get_commits;
pub use remote::{fetch, pull, push, remote_add, remote_list, remote_remove, Remote};
pub use smart_log::SmartLogFormatter;
pub use stage::{
    get_file_hunks, get_staged_files, get_unstaged_files, has_staged_changes, has_unstaged_changes,
    stage_all, stage_file, stage_hunk_by_lines, unstage_all, unstage_file, DiffHunk,
};
pub use stash::{
    get_stash_list, stash_apply, stash_drop, stash_pop, stash_push, stash_show, StashEntry,
};
pub use status::{get_status, FileStatus, StatusType};
pub use sync::get_sync_state;
pub use tag::{create_tag, delete_tag, show_tag, tag_list, Tag};
