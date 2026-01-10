pub mod log;
pub mod branch;
pub mod status;
pub mod diff;
pub mod smart_log;

pub use log::get_commits;
pub use branch::{get_branches, get_current_branch};
pub use status::{get_status, StatusType, FileStatus};
pub use diff::get_diff;
pub use smart_log::SmartLogFormatter;
