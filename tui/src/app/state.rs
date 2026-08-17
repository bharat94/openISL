//! State types for the TUI application
//!
//! This module contains all the state-related types including:
//! - View modes and panel types
//! - Filter modes
//! - Command palette actions
//! - Repository statistics

/// Represents the different panels in the sidebar
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelType {
    Files,
    Branches,
    Commits,
    Stash,
}

/// Represents the current view mode of the application
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    List,
    Details,
    Diff,
    Help,
    InputBranch,
    Search,
    BranchSearch,
    Filter,
    Stats,
    CommandPalette,
    Stash,
    HunkStaging,
}

/// Represents the filter mode for commit filtering
#[derive(Clone, Debug, PartialEq)]
pub enum FilterMode {
    Author,
    Message,
    Date,
}

/// Represents a command action in the command palette
#[derive(Clone, Debug)]
pub struct CommandAction {
    pub name: String,
    pub description: String,
    pub action: String,
    pub keys: Vec<String>,
    pub context: Vec<String>,
}

/// Repository statistics
#[derive(Debug, Default)]
pub struct RepoStats {
    pub total_commits: usize,
    pub total_authors: usize,
    pub commits_by_author: Vec<(String, usize)>,
    pub commits_today: usize,
    pub commits_this_week: usize,
    pub commits_this_month: usize,
}

/// Status bar display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusBarMode {
    Normal,
    Searching,
    Filtering,
    CommandPalette,
}
