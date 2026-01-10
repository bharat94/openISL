pub mod log;
pub mod branch;
pub mod status;
pub mod diff;

pub use log::get_commits;
pub use branch::{get_branches, get_current_branch};
pub use status::get_status;
pub use diff::get_diff;
