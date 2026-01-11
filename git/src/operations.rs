pub mod branch;
pub mod checkout;
pub mod diff;
pub mod log;
pub mod remote;
pub mod smart_log;
pub mod stash;
pub mod status;
pub mod tag;

pub use branch::{create_branch, create_branch_from_commit, get_branches, get_current_branch};
pub use checkout::{checkout, checkout_commit};
pub use diff::{get_commit_diff, get_diff};
pub use log::get_commits;
pub use remote::{fetch, pull, push, remote_add, remote_list, remote_remove, Remote};
pub use smart_log::SmartLogFormatter;
pub use stash::{
    get_stash_list, stash_apply, stash_drop, stash_pop, stash_push, stash_show, StashEntry,
};
pub use status::{get_status, FileStatus, StatusType};
pub use tag::{create_tag, delete_tag, show_tag, tag_list, Tag};
