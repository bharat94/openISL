//! TUI Application Module
//!
//! This module contains the main application logic for the terminal user interface.
//! It is organized into submodules:
//!
//! - `state`: State types (ViewMode, PanelType, FilterMode, etc.)
//! - `handlers`: Event handlers (keyboard, mouse, commit operations)
//! - `render`: UI rendering functions

pub mod handlers;
pub mod render;
pub mod state;

pub use state::{CommandAction, FilterMode, PanelType, RepoStats, StatusBarMode, ViewMode};

pub(crate) use crate::diff::{DiffParser, DiffStats};
pub(crate) use crate::keybindings::KeyBindings;
pub(crate) use crate::theme::Theme;
pub(crate) use crate::tree::{format_tree_lines, CommitTree};
pub(crate) use anyhow::Result;
pub(crate) use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    MouseButton, MouseEvent, MouseEventKind,
};
pub(crate) use crossterm::execute;
pub(crate) use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
pub(crate) use openisl_git::operations::hunk::{
    get_file_diff_hunks, stage_hunk, stage_hunk_lines, unstage_hunk, unstage_hunk_lines,
    HunkLineType,
};
pub(crate) use openisl_git::operations::{
    get_stash_list, stash_apply, stash_drop, stash_pop, stash_show, StashEntry,
};
pub(crate) use openisl_git::{get_commit_diff, Commit, FileStatus, GitRef};
pub(crate) use ratatui::widgets::Clear;
pub(crate) use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Line, Modifier, Span, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget},
    Terminal,
};
pub(crate) use std::io::stdout;
pub(crate) use std::path::Path;

use crate::app::render::{
    render_branch_search_input, render_command_palette, render_details_view, render_diff_view,
    render_filter_view, render_help_overlay, render_hunk_staging_view, render_input_view,
    render_list_view, render_search_view, render_stash_view, render_stats_view,
};

pub struct App {
    pub commits: Vec<Commit>,
    pub filtered_commits: Vec<Commit>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub show_help: bool,
    pub current_branch: String,
    pub theme: Theme,
    pub view_mode: ViewMode,
    pub diff_content: String,
    pub diff_stats: DiffStats,
    pub status_message: String,
    pub branch_input: String,
    pub repo_path: Option<std::path::PathBuf>,
    pub keybindings: KeyBindings,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub is_searching: bool,
    pub branch_search_query: String,
    pub is_branch_searching: bool,
    pub is_loading: bool,
    pub tree: CommitTree,
    pub filter_mode: FilterMode,
    pub filter_input: String,
    pub is_filtering: bool,
    pub show_stats: bool,
    pub stats: RepoStats,
    pub sidebar_visible: bool,
    pub active_panel: PanelType,
    pub files: Vec<FileStatus>,
    pub selected_file_index: usize,
    pub file_scroll_offset: usize,
    pub branches: Vec<GitRef>,
    pub all_branches: Vec<GitRef>,
    pub selected_branch_index: usize,
    pub branch_scroll_offset: usize,
    pub stashes: Vec<StashEntry>,
    pub selected_stash_index: usize,
    pub stash_scroll_offset: usize,
    pub stash_diff_content: String,
    pub command_palette_input: String,
    pub command_palette_results: Vec<CommandAction>,
    pub hunks: Vec<openisl_git::operations::hunk::Hunk>, // Added for hunk staging
    pub selected_hunk_index: usize,                      // Added for hunk staging
    pub selected_hunk_line_index: usize,                 // Added for line-by-line hunk staging
    pub is_hunk_staging_mode: bool,                      // Added for hunk staging
    pub current_file_diff_output: String,                // Store raw diff for hunk operations
    pub mouse_scroll_offset: usize,
    pub last_click_position: Option<(u16, u16)>,
    pub last_click_time: Option<std::time::Instant>,
    pub mouse_enabled: bool,
    pub repo_ahead: Option<usize>,
    pub repo_behind: Option<usize>,
    pub has_conflicts: bool,
}

impl App {
    pub fn new(
        commits: Vec<Commit>,
        current_branch: String,
        repo_path: Option<std::path::PathBuf>,
    ) -> Self {
        let all_branches = if let Some(ref path) = repo_path {
            openisl_git::get_branches(path).unwrap_or_default()
        } else {
            Vec::new()
        };

        let mut app = Self {
            commits: commits.clone(),
            filtered_commits: commits.clone(),
            selected_index: 0,
            scroll_offset: 0,
            show_help: false,
            current_branch,
            theme: Theme::dark(),
            view_mode: ViewMode::List,
            diff_content: String::new(),
            diff_stats: DiffStats::default(),
            status_message: String::new(),
            branch_input: String::new(),
            repo_path,
            keybindings: KeyBindings::load().unwrap_or_default(),
            search_query: String::new(),
            search_results: Vec::new(),
            is_searching: false,
            branch_search_query: String::new(),
            is_branch_searching: false,
            is_loading: false,
            tree: CommitTree::new(commits.clone()),
            filter_mode: FilterMode::Author,
            filter_input: String::new(),
            is_filtering: false,
            show_stats: false,
            stats: RepoStats::default(),
            sidebar_visible: true,
            active_panel: PanelType::Commits,
            files: Vec::new(),
            selected_file_index: 0,
            file_scroll_offset: 0,
            branches: all_branches.clone(),
            all_branches,
            selected_branch_index: 0,
            branch_scroll_offset: 0,
            stashes: Vec::new(),
            selected_stash_index: 0,
            stash_scroll_offset: 0,
            stash_diff_content: String::new(),
            command_palette_input: String::new(),
            command_palette_results: Vec::new(),
            hunks: Vec::new(),                       // Initialized
            selected_hunk_index: 0,                  // Initialized
            selected_hunk_line_index: 0,             // Initialized
            is_hunk_staging_mode: false,             // Initialized
            current_file_diff_output: String::new(), // Initialized
            mouse_scroll_offset: 0,
            last_click_position: None,
            last_click_time: None,
            mouse_enabled: false,
            repo_ahead: None,
            repo_behind: None,
            has_conflicts: false,
        };
        app.calculate_stats();
        app.populate_command_palette();
        app.refresh_sync_state();
        app
    }

    fn populate_command_palette(&mut self) {
        self.command_palette_results = Self::get_all_commands();
    }

    pub fn set_commits(&mut self, commits: Vec<Commit>) {
        self.commits = commits.clone();
        self.filtered_commits = commits.clone();
        self.tree = CommitTree::new(commits);
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.calculate_stats();
    }

    pub fn parse_diff(&mut self) {
        if self.diff_content.is_empty() {
            self.diff_stats = DiffStats::default();
            return;
        }
        let lines = DiffParser::parse(&self.diff_content);
        self.diff_stats = DiffParser::count_stats(&lines);
    }

    pub fn calculate_stats(&mut self) {
        let now = chrono::Utc::now();
        let one_day = chrono::Duration::days(1);
        let one_week = chrono::Duration::days(7);
        let one_month = chrono::Duration::days(30);

        let mut author_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for commit in &self.commits {
            *author_counts.entry(commit.author.clone()).or_insert(0) += 1;

            if now.signed_duration_since(commit.date) <= one_day {
                self.stats.commits_today += 1;
            }
            if now.signed_duration_since(commit.date) <= one_week {
                self.stats.commits_this_week += 1;
            }
            if now.signed_duration_since(commit.date) <= one_month {
                self.stats.commits_this_month += 1;
            }
        }

        let mut commits_by_author: Vec<_> = author_counts.into_iter().collect();
        commits_by_author.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

        self.stats.total_commits = self.commits.len();
        self.stats.total_authors = commits_by_author.len();
        self.stats.commits_by_author = commits_by_author;
    }

    pub fn apply_filter(&mut self) {
        if self.filter_input.is_empty() {
            self.filtered_commits = self.commits.clone();
            self.is_filtering = false;
            return;
        }

        self.is_filtering = true;
        let query = self.filter_input.to_lowercase();

        self.filtered_commits = self
            .commits
            .iter()
            .filter(|commit| match self.filter_mode {
                FilterMode::Author => commit.author.to_lowercase().contains(&query),
                FilterMode::Message => {
                    commit.summary.to_lowercase().contains(&query)
                        || commit.message.to_lowercase().contains(&query)
                }
                FilterMode::Date => commit.date.format("%Y-%m-%d").to_string().contains(&query),
            })
            .cloned()
            .collect();

        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn clear_filter(&mut self) {
        self.filter_input.clear();
        self.filtered_commits = self.commits.clone();
        self.is_filtering = false;
    }

    pub fn visible_commits(&self) -> &[Commit] {
        let commits = if self.is_filtering {
            &self.filtered_commits
        } else {
            &self.commits
        };
        let end = (self.scroll_offset + 20).min(commits.len());
        &commits[self.scroll_offset..end]
    }

    pub fn selected_commit(&self) -> Option<&Commit> {
        let commits = if self.is_filtering {
            &self.filtered_commits
        } else {
            &self.commits
        };
        commits.get(self.selected_index)
    }

    pub fn search(&mut self) {
        if self.search_query.is_empty() {
            self.search_results.clear();
            return;
        }

        self.search_results = self
            .commits
            .iter()
            .enumerate()
            .filter(|(_, commit)| {
                let query = self.search_query.to_lowercase();
                commit.summary.to_lowercase().contains(&query)
                    || commit.message.to_lowercase().contains(&query)
                    || commit.author.to_lowercase().contains(&query)
                    || commit.short_hash.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect();

        if !self.search_results.is_empty() {
            self.selected_index = self.search_results[0];
            self.scroll_offset = 0;
        }
    }

    pub fn next_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        if let Some(current_pos) = self
            .search_results
            .iter()
            .position(|&i| i == self.selected_index)
        {
            if current_pos + 1 < self.search_results.len() {
                self.selected_index = self.search_results[current_pos + 1];
                if self.selected_index >= self.scroll_offset + 20 {
                    self.scroll_offset = self.selected_index - 20 + 1;
                }
            }
        }
    }

    pub fn prev_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        if let Some(current_pos) = self
            .search_results
            .iter()
            .position(|&i| i == self.selected_index)
        {
            if current_pos > 0 {
                self.selected_index = self.search_results[current_pos - 1];
                if self.selected_index < self.scroll_offset {
                    self.scroll_offset = self.selected_index.saturating_sub(1);
                }
            }
        }
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_results.clear();
        self.is_searching = false;
    }

    pub fn filter_branches(&mut self) {
        if self.branch_search_query.is_empty() {
            self.branches = self.all_branches.clone(); // Assuming all_branches holds all branches
        } else {
            let query = self.branch_search_query.to_lowercase();
            self.branches = self
                .all_branches
                .iter()
                .filter(|branch| branch.name.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
        self.selected_branch_index = 0;
        self.branch_scroll_offset = 0;
    }

    pub fn clear_branch_search(&mut self) {
        self.branch_search_query.clear();
        self.filter_branches();
        self.is_branch_searching = false;
    }

    pub fn toggle_search(&mut self) {
        if self.is_searching && self.search_query.is_empty() {
            self.is_searching = false;
        } else {
            self.is_searching = !self.is_searching;
            if !self.is_searching {
                self.clear_search();
            }
        }
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
    }

    pub fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            PanelType::Files => PanelType::Branches,
            PanelType::Branches => PanelType::Commits,
            PanelType::Commits => PanelType::Stash,
            PanelType::Stash => PanelType::Files,
        };
        self.status_message = format!("Switched to {} panel", self.panel_name());
    }

    pub fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            PanelType::Files => PanelType::Stash,
            PanelType::Branches => PanelType::Files,
            PanelType::Commits => PanelType::Branches,
            PanelType::Stash => PanelType::Commits,
        };
        self.status_message = format!("Switched to {} panel", self.panel_name());
    }

    fn panel_name(&self) -> String {
        match self.active_panel {
            PanelType::Files => "Files",
            PanelType::Branches => "Branches",
            PanelType::Commits => "Commits",
            PanelType::Stash => "Stash",
        }
        .to_string()
    }

    pub fn open_command_palette(&mut self) {
        self.view_mode = ViewMode::CommandPalette;
        self.command_palette_input.clear();
        self.filter_command_palette();
        self.status_message = "Type to search commands".to_string();
    }

    pub fn filter_command_palette(&mut self) {
        let all_commands = Self::get_all_commands();
        let active_panel_str = self.panel_name().to_lowercase(); // Get the current active panel as a string

        self.command_palette_results = all_commands
            .into_iter()
            .filter(|action| {
                let matches_query = self.command_palette_input.is_empty()
                    || action
                        .name
                        .to_lowercase()
                        .contains(&self.command_palette_input.to_lowercase())
                    || action
                        .description
                        .to_lowercase()
                        .contains(&self.command_palette_input.to_lowercase())
                    || action
                        .action
                        .contains(&self.command_palette_input.to_lowercase());

                let matches_context = action.context.is_empty() // Command is always available
                    || action.context.contains(&active_panel_str); // Command is available in the current panel

                matches_query && matches_context
            })
            .collect();
    }

    fn get_all_commands() -> Vec<CommandAction> {
        vec![
            CommandAction {
                name: "Toggle Sidebar".to_string(),
                description: "Show/hide the sidebar panel".to_string(),
                action: "toggle_sidebar".to_string(),
                keys: vec!["Ctrl+B".to_string()],
                context: vec![], // Always available
            },
            CommandAction {
                name: "Next Panel".to_string(),
                description: "Move focus to next panel".to_string(),
                action: "next_panel".to_string(),
                keys: vec!["Tab".to_string(), "→".to_string(), "l".to_string()],
                context: vec![], // Always available
            },
            CommandAction {
                name: "Previous Panel".to_string(),
                description: "Move focus to previous panel".to_string(),
                action: "prev_panel".to_string(),
                keys: vec!["Shift+Tab".to_string(), "←".to_string(), "h".to_string()],
                context: vec![], // Always available
            },
            CommandAction {
                name: "Navigate Up".to_string(),
                description: "Move selection up".to_string(),
                action: "move_up".to_string(),
                keys: vec!["k".to_string(), "↑".to_string()],
                context: vec![
                    "files".to_string(),
                    "branches".to_string(),
                    "commits".to_string(),
                    "stash".to_string(),
                ],
            },
            CommandAction {
                name: "Navigate Down".to_string(),
                description: "Move selection down".to_string(),
                action: "move_down".to_string(),
                keys: vec!["j".to_string(), "↓".to_string()],
                context: vec![
                    "files".to_string(),
                    "branches".to_string(),
                    "commits".to_string(),
                    "stash".to_string(),
                ],
            },
            CommandAction {
                name: "Stage/Unstage File".to_string(),
                description: "Stage or unstage the selected file".to_string(),
                action: "toggle_stage".to_string(),
                keys: vec!["Space".to_string()],
                context: vec!["files".to_string()],
            },
            CommandAction {
                name: "Stage All".to_string(),
                description: "Stage all files".to_string(),
                action: "stage_all".to_string(),
                keys: vec!["Ctrl+S".to_string()],
                context: vec!["files".to_string()],
            },
            CommandAction {
                name: "Unstage All".to_string(),
                description: "Unstage all files".to_string(),
                action: "unstage_all".to_string(),
                keys: vec!["Ctrl+U".to_string()],
                context: vec!["files".to_string()],
            },
            CommandAction {
                name: "Amend Commit".to_string(),
                description: "Amend the last commit".to_string(),
                action: "amend".to_string(),
                keys: vec!["A".to_string()],
                context: vec!["commits".to_string()],
            },
            CommandAction {
                name: "Drop Commit".to_string(),
                description: "Remove the selected commit".to_string(),
                action: "drop".to_string(),
                keys: vec!["D".to_string()],
                context: vec!["commits".to_string()],
            },
            CommandAction {
                name: "Squash Commits".to_string(),
                description: "Squash selected commit into previous".to_string(),
                action: "squash".to_string(),
                keys: vec!["S".to_string()],
                context: vec!["commits".to_string()],
            },
            CommandAction {
                name: "Cherry-Pick".to_string(),
                description: "Cherry-pick the selected commit".to_string(),
                action: "cherry_pick".to_string(),
                keys: vec!["C".to_string()],
                context: vec!["commits".to_string()],
            },
            CommandAction {
                name: "Revert Commit".to_string(),
                description: "Revert the selected commit".to_string(),
                action: "revert".to_string(),
                keys: vec!["R".to_string()],
                context: vec!["commits".to_string()],
            },
            CommandAction {
                name: "Go to Start".to_string(),
                description: "Jump to first item".to_string(),
                action: "go_to_start".to_string(),
                keys: vec!["gg".to_string(), "Home".to_string()],
                context: vec![],
            },
            CommandAction {
                name: "Go to End".to_string(),
                description: "Jump to last item".to_string(),
                action: "go_to_end".to_string(),
                keys: vec!["G".to_string(), "End".to_string()],
                context: vec![],
            },
            CommandAction {
                name: "View Details".to_string(),
                description: "Show commit/file details".to_string(),
                action: "view_details".to_string(),
                keys: vec!["Enter".to_string()],
                context: vec!["commits".to_string(), "files".to_string()],
            },
            CommandAction {
                name: "Search".to_string(),
                description: "Search commits or files".to_string(),
                action: "search".to_string(),
                keys: vec!["/".to_string()],
                context: vec![
                    "commits".to_string(),
                    "files".to_string(),
                    "branches".to_string(),
                ],
            },
            CommandAction {
                name: "Toggle Theme".to_string(),
                description: "Switch between dark/light theme".to_string(),
                action: "toggle_theme".to_string(),
                keys: vec!["t".to_string()],
                context: vec![], // Always available
            },
            CommandAction {
                name: "Toggle Mouse Mode".to_string(),
                description: "Enable/disable mouse support".to_string(),
                action: "toggle_mouse".to_string(),
                keys: vec!["m".to_string()],
                context: vec![], // Always available
            },
            CommandAction {
                name: "Show Help".to_string(),
                description: "Display keyboard shortcuts".to_string(),
                action: "help".to_string(),
                keys: vec!["?".to_string()],
                context: vec![], // Always available
            },
            CommandAction {
                name: "Quit".to_string(),
                description: "Exit openisl".to_string(),
                action: "quit".to_string(),
                keys: vec!["q".to_string(), "Esc".to_string()],
                context: vec![], // Always available
            },
            CommandAction {
                name: "View Stashes".to_string(),
                description: "Open dedicated stash management view".to_string(),
                action: "view_stashes".to_string(),
                keys: vec!["L".to_string()],
                context: vec!["commits".to_string()], // Stashes are typically related to the commit history
            },
            CommandAction {
                name: "Apply Stash".to_string(),
                description: "Apply selected stash".to_string(),
                action: "apply_stash".to_string(),
                keys: vec!["A".to_string()],
                context: vec!["stash".to_string()],
            },
            CommandAction {
                name: "Drop Stash".to_string(),
                description: "Drop selected stash".to_string(),
                action: "drop_stash".to_string(),
                keys: vec!["D".to_string()],
                context: vec!["stash".to_string()],
            },
            CommandAction {
                name: "Pop Stash".to_string(),
                description: "Pop selected stash (apply and drop)".to_string(),
                action: "pop_stash".to_string(),
                keys: vec!["P".to_string()],
                context: vec!["stash".to_string()],
            },
            CommandAction {
                name: "Open in Editor".to_string(),
                description: "Open selected file in external editor".to_string(),
                action: "open_in_editor".to_string(),
                keys: vec!["E".to_string()],
                context: vec!["files".to_string()],
            },
            CommandAction {
                name: "Command Palette".to_string(),
                description: "Open command search".to_string(),
                action: "command_palette".to_string(),
                keys: vec!["Ctrl+P".to_string()],
                context: vec![], // Always available
            },
        ]
    }

    pub fn format_commit_details(&self, commit: &Commit) -> String {
        format!(
            "Commit: {}\nShort:   {}\nAuthor:  {} <{}>\nDate:    {}\n\n{}\n\nParents: {}",
            commit.hash,
            commit.short_hash,
            commit.author,
            commit.email,
            commit.date,
            commit.message,
            if commit.parent_hashes.is_empty() {
                "None (initial commit)".to_string()
            } else {
                commit.parent_hashes.join(", ")
            }
        )
    }

    pub fn move_file_selection_down(&mut self) {
        if self.selected_file_index < self.files.len().saturating_sub(1) {
            self.selected_file_index += 1;
            if self.selected_file_index >= self.file_scroll_offset + 10 {
                self.file_scroll_offset = self.selected_file_index - 10 + 1;
            }
        }
    }

    pub fn move_file_selection_up(&mut self) {
        if self.selected_file_index > 0 {
            self.selected_file_index = self.selected_file_index.saturating_sub(1);
            if self.selected_file_index < self.file_scroll_offset {
                self.file_scroll_offset = self.selected_file_index.saturating_sub(1);
            }
        }
    }
}

pub fn run_tui(
    commits: Vec<Commit>,
    current_branch: String,
    repo_path: Option<std::path::PathBuf>,
) -> Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;

    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(commits.clone(), current_branch, repo_path);
    app.set_commits(commits);

    loop {
        terminal.draw(|frame| match app.view_mode {
            ViewMode::List => render_list_view(&app, frame),
            ViewMode::Details => render_details_view(&app, frame),
            ViewMode::Diff => render_diff_view(&app, frame),
            ViewMode::Help => render_help_overlay(&app, frame),
            ViewMode::InputBranch => render_input_view(&app, frame),
            ViewMode::Search => render_search_view(&app, frame),
            ViewMode::BranchSearch => render_branch_search_input(&app, frame.size(), frame),
            ViewMode::Filter => render_filter_view(&app, frame),
            ViewMode::Stats => render_stats_view(&app, frame),
            ViewMode::CommandPalette => render_command_palette(&app, frame),
            ViewMode::Stash => render_stash_view(&app, frame),
            ViewMode::HunkStaging => render_hunk_staging_view(&app, frame), // Render hunk staging view
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    let should_quit = app.handle_key(key);
                    if should_quit {
                        break;
                    }
                }
                Event::Mouse(mouse_event) => {
                    app.handle_mouse(mouse_event);
                }
                Event::Resize(_, _) => {}
                Event::FocusGained | Event::FocusLost | Event::Paste(_) => {}
            }
        }
    }

    terminal.clear()?;
    disable_raw_mode()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openisl_git::operations::hunk::{Hunk, HunkLine, HunkLineType};

    fn create_test_commits() -> Vec<Commit> {
        vec![
            Commit {
                hash: "abc123def456789".to_string(),
                short_hash: "abc123d".to_string(),
                message: "First commit\n\nThis is the body".to_string(),
                summary: "First commit".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: chrono::Utc::now(),
                parent_hashes: vec![],
                refs: vec![],
            },
            Commit {
                hash: "def456ghi789abc".to_string(),
                short_hash: "def456g".to_string(),
                message: "Second commit".to_string(),
                summary: "Second commit".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: chrono::Utc::now(),
                parent_hashes: vec!["abc123def456789".to_string()],
                refs: vec![],
            },
            Commit {
                hash: "ghi789jkl012345".to_string(),
                short_hash: "ghi789j".to_string(),
                message: "Third commit".to_string(),
                summary: "Third commit".to_string(),
                author: "other@example.com".to_string(),
                email: "other@example.com".to_string(),
                date: chrono::Utc::now(),
                parent_hashes: vec!["def456ghi789abc".to_string()],
                refs: vec![],
            },
        ]
    }

    fn mock_hunks() -> Vec<Hunk> {
        vec![
            Hunk {
                header: "@@ -1,3 +1,4 @@".to_string(),
                old_start: 1,
                old_lines: 3,
                new_start: 1,
                new_lines: 4,
                lines: vec![
                    HunkLine {
                        content: "line1".to_string(),
                        line_type: HunkLineType::Context, // Full path
                        is_selected: false,
                    },
                    HunkLine {
                        content: "line2_removed".to_string(),
                        line_type: HunkLineType::Deletion, // Corrected variant name
                        is_selected: false,
                    },
                    HunkLine {
                        content: "line2_added".to_string(),
                        line_type: HunkLineType::Addition, // Corrected variant name
                        is_selected: false,
                    },
                    HunkLine {
                        content: "line3".to_string(),
                        line_type: HunkLineType::Context, // Full path
                        is_selected: false,
                    },
                ],
                is_selected: false,
                is_staged: false, // Added missing field
            },
            Hunk {
                header: "@@ -5,2 +5,2 @@".to_string(),
                old_start: 5,
                old_lines: 2,
                new_start: 5,
                new_lines: 2,
                lines: vec![
                    HunkLine {
                        content: "line5_removed".to_string(),
                        line_type: HunkLineType::Deletion, // Corrected variant name
                        is_selected: false,
                    },
                    HunkLine {
                        content: "line5_added".to_string(),
                        line_type: HunkLineType::Addition, // Corrected variant name
                        is_selected: false,
                    },
                ],
                is_selected: false,
                is_staged: false, // Added missing field
            },
        ]
    }

    #[test]
    fn test_hunk_staging_mode_entry_exit() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);
        app.files = vec![FileStatus {
            path: "test.rs".to_string(),
            status: openisl_git::StatusType::Modified,
        }];
        app.active_panel = PanelType::Files;
        app.hunks = mock_hunks();

        // Simulate entering diff view
        app.view_mode = ViewMode::Diff;
        assert_eq!(app.view_mode, ViewMode::Diff);

        // Simulate pressing 'i' to enter hunk staging mode
        app.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::HunkStaging);
        assert!(app.is_hunk_staging_mode);
        assert!(app.status_message.contains("Hunk staging mode"));

        // Simulate pressing 'Esc' to exit hunk staging mode
        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::Diff);
        assert!(!app.is_hunk_staging_mode);
        assert!(app.status_message.contains("Exited hunk staging mode"));
    }

    #[test]
    fn test_hunk_navigation() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);
        app.hunks = mock_hunks();
        app.view_mode = ViewMode::HunkStaging;

        assert_eq!(app.selected_hunk_index, 0);
        assert_eq!(app.selected_hunk_line_index, 0);

        // Move down within the first hunk
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(app.selected_hunk_index, 0);
        assert_eq!(app.selected_hunk_line_index, 1);

        app.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
        assert_eq!(app.selected_hunk_index, 0);
        assert_eq!(app.selected_hunk_line_index, 2);

        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(app.selected_hunk_index, 0);
        assert_eq!(app.selected_hunk_line_index, 3);

        // Move down to the next hunk
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(app.selected_hunk_index, 1);
        assert_eq!(app.selected_hunk_line_index, 0);

        // Move up within the second hunk
        app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(app.selected_hunk_index, 0);
        assert_eq!(app.selected_hunk_line_index, 3); // Should move to last line of previous hunk

        // Move up to the start
        app.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)); // Should be at 0,0 now
        assert_eq!(app.selected_hunk_index, 0);
        assert_eq!(app.selected_hunk_line_index, 0);

        // Boundary test: try moving up from 0,0
        app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(app.selected_hunk_index, 0);
        assert_eq!(app.selected_hunk_line_index, 0);
    }

    #[test]
    fn test_hunk_line_selection_toggle() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);
        app.hunks = mock_hunks();
        app.view_mode = ViewMode::HunkStaging;

        // Select the first line of the first hunk
        assert!(!app.hunks[0].lines[0].is_selected);
        app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
        assert!(app.hunks[0].lines[0].is_selected);
        assert!(app.status_message.contains("selection toggled"));

        // Toggle it back off
        app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
        assert!(!app.hunks[0].lines[0].is_selected);
    }

    #[test]
    fn test_hunk_staging_actions() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);
        app.files = vec![FileStatus {
            path: "test.rs".to_string(),
            status: openisl_git::StatusType::Modified,
        }];
        app.active_panel = PanelType::Files;
        app.hunks = mock_hunks();
        app.view_mode = ViewMode::HunkStaging;

        // Test with no repo path - should show error
        app.repo_path = None;
        app.hunks[0].lines[1].is_selected = true; // Select 'line2_removed'
        app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
        assert!(app.status_message.contains("No repository path available"));

        // Test unstaging without repo
        app.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
        assert!(app.status_message.contains("No repository path available"));

        // Test with no file selected (set repo path first to test this case)
        app.repo_path = Some(std::path::PathBuf::from("/mock/repo"));
        app.files.clear();
        app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
        assert!(app
            .status_message
            .contains("No file selected to stage hunks"));
    }

    #[test]
    fn test_app_navigation_down() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.selected_index, 0);
        app.move_down();
        assert_eq!(app.selected_index, 1);
        app.move_down();
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn test_app_navigation_up() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.selected_index = 2;
        app.move_up();
        assert_eq!(app.selected_index, 1);
        app.move_up();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_app_navigation_boundaries() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.move_up();
        assert_eq!(app.selected_index, 0);

        app.selected_index = 2;
        app.move_down();
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn test_app_navigation_page_down() {
        let commits = create_test_commits();
        let mut app = App::new(commits.clone(), "main".to_string(), None);

        app.selected_index = 0;
        app.page_down();
        assert!(app.selected_index >= 1);
    }

    #[test]
    fn test_app_navigation_page_up() {
        let commits = create_test_commits();
        let mut app = App::new(commits.clone(), "main".to_string(), None);

        app.selected_index = 2;
        app.page_up();
        assert!(app.selected_index <= 2);
    }

    #[test]
    fn test_app_go_to_start() {
        let commits = create_test_commits();
        let mut app = App::new(commits.clone(), "main".to_string(), None);

        app.selected_index = 2;
        app.go_to_start();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_app_go_to_end() {
        let commits = create_test_commits();
        let mut app = App::new(commits.clone(), "main".to_string(), None);

        app.go_to_end();
        assert_eq!(app.selected_index, commits.len() - 1);
    }

    #[test]
    fn test_view_mode_transitions() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.view_mode, ViewMode::List);

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::Details);

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::List);

        app.handle_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::Help);

        app.handle_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::List);
    }

    #[test]
    fn test_branch_input_mode() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.view_mode, ViewMode::List);
        assert!(app.branch_input.is_empty());

        app.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::InputBranch);
        assert!(app.status_message.contains("branch name"));

        app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
        assert_eq!(app.branch_input, "f");

        app.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
        assert_eq!(app.branch_input, "fe");

        app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(app.branch_input, "f");

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::List);
        assert!(app.status_message.contains("Created branch"));
    }

    #[test]
    fn test_branch_input_special_chars() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('_'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));

        assert_eq!(app.branch_input, "f-_/");
    }

    #[test]
    fn test_branch_input_rejects_invalid_chars() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));

        assert!(app.branch_input.is_empty());
    }

    #[test]
    fn test_branch_input_cancel() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));

        assert_eq!(app.branch_input, "f");

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

        assert_eq!(app.view_mode, ViewMode::List);
        assert!(app.branch_input.is_empty());
    }

    #[test]
    fn test_checkout_key() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert!(app.status_message.is_empty());

        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT));
        assert_eq!(app.view_mode, ViewMode::Diff);
    }

    #[test]
    fn test_checkout_from_details_view() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::Details);

        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT));
        assert_eq!(app.view_mode, ViewMode::Diff);
    }

    #[test]
    fn test_theme_toggle() {
        let mut theme = Theme::dark();
        assert_eq!(theme.name(), "dark");

        theme.next();
        assert_eq!(theme.name(), "light");

        theme.next();
        assert_eq!(theme.name(), "monokai");

        theme.next();
        assert_eq!(theme.name(), "nord");

        theme.next();
        assert_eq!(theme.name(), "dark");
    }

    #[test]
    fn test_theme_dark_colors() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "dark");
        assert_eq!(theme.title, Color::Rgb(0, 191, 255));
        assert_eq!(theme.text, Color::Rgb(200, 200, 200));
        assert_eq!(theme.border, Color::Rgb(255, 215, 0));
        assert_eq!(theme.selected, Color::Rgb(255, 255, 255));
        assert_eq!(theme.selected_bg, Color::Rgb(70, 70, 100));
    }

    #[test]
    fn test_theme_light_colors() {
        let theme = Theme::light();
        assert_eq!(theme.name, "light");
        assert_eq!(theme.title, Color::Blue);
        assert_eq!(theme.text, Color::DarkGray);
        assert_eq!(theme.border, Color::Black);
        assert_eq!(theme.selected, Color::Black);
        assert_eq!(theme.selected_bg, Color::Gray);
    }

    #[test]
    fn test_visible_commits() {
        let commits = create_test_commits();
        let app = App::new(commits, "main".to_string(), None);

        let visible = app.visible_commits();
        assert_eq!(visible.len(), 3);
    }

    #[test]
    fn test_visible_commits_with_scroll() {
        let commits = create_test_commits();
        let mut app = App::new(commits.clone(), "main".to_string(), None);

        app.scroll_offset = 1;
        let visible = app.visible_commits();
        assert_eq!(visible.len(), 2);
        assert_eq!(visible[0].short_hash, "def456g");
    }

    #[test]
    fn test_selected_commit() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.selected_commit().unwrap().short_hash, "abc123d");

        app.move_down();
        assert_eq!(app.selected_commit().unwrap().short_hash, "def456g");

        app.move_down();
        assert_eq!(app.selected_commit().unwrap().short_hash, "ghi789j");
    }

    #[test]
    fn test_selected_commit_bounds() {
        let commits = create_test_commits();
        let app = App::new(commits, "main".to_string(), None);

        assert!(app.selected_commit().is_some());
    }

    #[test]
    fn test_format_commit_details() {
        let commits = create_test_commits();
        let app = App::new(commits, "main".to_string(), None);
        let commit = app.selected_commit().unwrap();

        let details = app.format_commit_details(commit);
        assert!(details.contains("abc123def456789"));
        assert!(details.contains("test@example.com"));
        assert!(details.contains("First commit"));
        assert!(details.contains("None (initial commit)"));
    }

    #[test]
    fn test_format_commit_details_with_parents() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.move_down();
        let commit = app.selected_commit().unwrap();
        let details = app.format_commit_details(commit);
        assert!(details.contains("abc123def456789"));
    }

    #[test]
    fn test_quit_from_list() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let should_quit = app.handle_key(quit_event);
        assert!(should_quit);
    }

    #[test]
    fn test_quit_from_details() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.view_mode = ViewMode::Details;
        let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let should_quit = app.handle_key(quit_event);
        assert!(!should_quit);
        assert_eq!(app.view_mode, ViewMode::List);
    }

    #[test]
    fn test_help_mode_exit() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.view_mode = ViewMode::Help;
        let quit_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let should_quit = app.handle_key(quit_event);
        assert!(!should_quit);
        assert_eq!(app.view_mode, ViewMode::List);
    }

    #[test]
    fn test_fetch_diff_no_repo() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.fetch_diff();
        assert_eq!(app.diff_content, "No repository path available");
    }

    #[test]
    fn test_diff_view_sets_content() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT));
        assert_eq!(app.view_mode, ViewMode::Diff);
    }

    #[test]
    fn test_diff_view_exit() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.view_mode = ViewMode::Diff;
        app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::List);
    }

    #[test]
    fn test_search_toggle() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert!(!app.is_searching);
        assert!(app.search_query.is_empty());

        app.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));
        assert!(app.is_searching);
    }

    #[test]
    fn test_search_functionality() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "First".to_string();
        app.search();

        assert_eq!(app.search_results.len(), 1);
        assert_eq!(app.search_results[0], 0);
    }

    #[test]
    fn test_search_case_insensitive() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "FIRST".to_string();
        app.search();

        assert_eq!(app.search_results.len(), 1);
        assert_eq!(app.search_results[0], 0);
    }

    #[test]
    fn test_search_by_author() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "other@example.com".to_string();
        app.search();

        assert_eq!(app.search_results.len(), 1);
        assert_eq!(app.search_results[0], 2);
    }

    #[test]
    fn test_search_by_hash() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "abc123d".to_string();
        app.search();

        assert_eq!(app.search_results.len(), 1);
        assert_eq!(app.search_results[0], 0);
    }

    #[test]
    fn test_search_no_results() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "nonexistent".to_string();
        app.search();

        assert!(app.search_results.is_empty());
    }

    #[test]
    fn test_search_empty_query() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "commit".to_string();
        app.search();
        assert!(!app.search_results.is_empty());

        app.search_query = "".to_string();
        app.search();
        assert!(app.search_results.is_empty());
    }

    #[test]
    fn test_search_navigation() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "commit".to_string();
        app.search();

        assert_eq!(app.search_results.len(), 3);

        app.next_search_result();
        assert_eq!(app.selected_index, app.search_results[1]);

        app.prev_search_result();
        assert_eq!(app.selected_index, app.search_results[0]);
    }

    #[test]
    fn test_search_navigation_bounds() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "commit".to_string();
        app.search();

        app.selected_index = app.search_results[0];
        app.prev_search_result();
        assert_eq!(app.selected_index, app.search_results[0]);

        app.selected_index = app.search_results[app.search_results.len() - 1];
        app.next_search_result();
        assert_eq!(
            app.selected_index,
            app.search_results[app.search_results.len() - 1]
        );
    }

    #[test]
    fn test_search_clear() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "test".to_string();
        app.search();
        assert!(!app.search_results.is_empty());

        app.clear_search();
        assert!(app.search_query.is_empty());
        assert!(app.search_results.is_empty());
        assert!(!app.is_searching);
    }

    #[test]
    fn test_search_escape() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));
        assert!(app.is_searching);

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(!app.is_searching);
    }

    #[test]
    fn test_ctrl_navigation() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.search_query = "commit".to_string();
        app.search();

        let initial_index = app.selected_index;
        app.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::CONTROL));
        assert_ne!(app.selected_index, initial_index);
    }

    #[test]
    fn test_ctrl_p_navigation() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.view_mode, ViewMode::List);
        app.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        assert_eq!(app.view_mode, ViewMode::CommandPalette);
    }

    #[test]
    fn test_set_commits_updates_tree() {
        let commits = create_test_commits();
        let mut app = App::new(vec![], "main".to_string(), None);

        app.set_commits(commits.clone());
        assert_eq!(app.commits.len(), 3);
        assert_eq!(app.tree.nodes().len(), 3);
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_set_commits_resets_selection() {
        let commits = create_test_commits();
        let mut app = App::new(commits.clone(), "main".to_string(), None);

        app.selected_index = 2;
        app.scroll_offset = 1;

        app.set_commits(commits);
        assert_eq!(app.selected_index, 0);
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_key_event_returns_false_for_regular_keys() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        let result = app.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert!(!result);
    }

    #[test]
    fn test_status_message_update() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.status_message = "Test message".to_string();
        assert!(app.status_message.contains("Test"));
    }

    #[test]
    fn test_branch_input_empty_submit() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::List);
    }

    #[test]
    fn test_shift_d_from_details() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::Details);

        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT));
        assert_eq!(app.view_mode, ViewMode::Diff);
    }

    #[test]
    fn test_view_mode_enum_values() {
        assert_eq!(ViewMode::List as u8, 0);
        assert_eq!(ViewMode::Details as u8, 1);
        assert_eq!(ViewMode::Diff as u8, 2);
        assert_eq!(ViewMode::Help as u8, 3);
        assert_eq!(ViewMode::InputBranch as u8, 4);
        assert_eq!(ViewMode::Search as u8, 5);
    }

    #[test]
    fn test_commit_display_impl() {
        let commit = &create_test_commits()[0];
        let display = format!("{}", commit);
        assert!(display.contains("abc123d"));
        assert!(display.contains("First commit"));
    }

    #[test]
    fn test_app_new_with_repo_path() {
        let commits = create_test_commits();
        let repo_path = Some(std::path::PathBuf::from("/test/repo"));
        let app = App::new(commits, "main".to_string(), repo_path);

        assert_eq!(app.repo_path, Some(std::path::PathBuf::from("/test/repo")));
    }

    #[test]
    fn test_filter_by_author() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.filter_mode = FilterMode::Author;
        app.filter_input = "test@example.com".to_string();
        app.apply_filter();

        assert!(app.is_filtering);
        assert_eq!(app.filtered_commits.len(), 2);
    }

    #[test]
    fn test_filter_by_message() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.filter_mode = FilterMode::Message;
        app.filter_input = "First".to_string();
        app.apply_filter();

        assert!(app.is_filtering);
        assert_eq!(app.filtered_commits.len(), 1);
        assert!(app.filtered_commits[0].summary.contains("First"));
    }

    #[test]
    fn test_filter_by_date() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        app.filter_mode = FilterMode::Date;
        app.filter_input = today;
        app.apply_filter();

        assert!(app.is_filtering);
        assert_eq!(app.filtered_commits.len(), 3);
    }

    #[test]
    fn test_filter_case_insensitive() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.filter_mode = FilterMode::Message;
        app.filter_input = "FIRST".to_string();
        app.apply_filter();

        assert_eq!(app.filtered_commits.len(), 1);
    }

    #[test]
    fn test_clear_filter() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.filter_input = "test".to_string();
        app.apply_filter();
        assert!(app.is_filtering);

        app.clear_filter();
        assert!(!app.is_filtering);
        assert!(app.filter_input.is_empty());
        assert_eq!(app.filtered_commits.len(), app.commits.len());
    }

    #[test]
    fn test_empty_filter_shows_all() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.filter_input = "nonexistent".to_string();
        app.apply_filter();
        assert_eq!(app.filtered_commits.len(), 0);

        app.filter_input = "".to_string();
        app.apply_filter();
        assert!(!app.is_filtering);
        assert_eq!(app.filtered_commits.len(), 3);
    }

    #[test]
    fn test_stats_calculated_on_init() {
        let commits = create_test_commits();
        let app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.stats.total_commits, 3);
        assert_eq!(app.stats.total_authors, 2);
        assert!(app.stats.commits_today > 0);
    }

    #[test]
    fn test_stats_commits_by_author() {
        let commits = create_test_commits();
        let app = App::new(commits, "main".to_string(), None);

        let author_counts: Vec<(String, usize)> = app.stats.commits_by_author.clone();
        assert_eq!(author_counts.len(), 2);

        for (author, count) in author_counts {
            if author.contains("test@example.com") {
                assert_eq!(count, 2);
            } else if author.contains("other@example.com") {
                assert_eq!(count, 1);
            }
        }
    }

    #[test]
    fn test_view_mode_filter_and_stats() {
        assert_eq!(ViewMode::Filter as u8, 7);
        assert_eq!(ViewMode::Stats as u8, 8);
    }

    #[test]
    fn test_sidebar_toggle() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert!(app.sidebar_visible);
        app.toggle_sidebar();
        assert!(!app.sidebar_visible);
        app.toggle_sidebar();
        assert!(app.sidebar_visible);
    }

    #[test]

    fn test_panel_navigation() {
        let commits = create_test_commits();

        let mut app = App::new(commits, "main".to_string(), None);

        // Initial state

        assert_eq!(app.active_panel, PanelType::Commits);

        // Next panel sequence

        app.next_panel(); // Commits -> Stash

        assert_eq!(app.active_panel, PanelType::Stash);

        app.next_panel(); // Stash -> Files

        assert_eq!(app.active_panel, PanelType::Files);

        app.next_panel(); // Files -> Branches

        assert_eq!(app.active_panel, PanelType::Branches);

        app.next_panel(); // Branches -> Commits

        assert_eq!(app.active_panel, PanelType::Commits);

        // Previous panel sequence

        app.prev_panel(); // Commits -> Branches

        assert_eq!(app.active_panel, PanelType::Branches);

        app.prev_panel(); // Branches -> Files

        assert_eq!(app.active_panel, PanelType::Files);

        app.prev_panel(); // Files -> Stash

        assert_eq!(app.active_panel, PanelType::Stash);

        app.prev_panel(); // Stash -> Commits

        assert_eq!(app.active_panel, PanelType::Commits);
    }

    #[test]
    fn test_vim_keybindings_gg_go_to_start() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.selected_index = 2;
        assert_eq!(app.selected_index, 2);

        app.handle_key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_vim_keybindings_g_go_to_end() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.selected_index, 0);
        app.handle_key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn test_theme_cycle() {
        let mut theme = Theme::dark();
        assert_eq!(theme.name(), "dark");

        theme.next();
        assert_eq!(theme.name(), "light");

        theme.next();
        assert_eq!(theme.name(), "monokai");

        theme.next();
        assert_eq!(theme.name(), "nord");

        theme.next();
        assert_eq!(theme.name(), "dark");
    }

    #[test]
    fn test_theme_set() {
        let mut theme = Theme::dark();
        theme.set("monokai");
        assert_eq!(theme.name(), "monokai");
        theme.set("nord");
        assert_eq!(theme.name(), "nord");
        theme.set("invalid");
        assert_eq!(theme.name(), "dark");
    }

    #[test]
    fn test_command_palette_opens() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.view_mode, ViewMode::List);
        app.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        assert_eq!(app.view_mode, ViewMode::CommandPalette);
    }

    #[test]
    fn test_command_palette_filter() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.command_palette_input = "theme".to_string();
        app.filter_command_palette();

        assert!(!app.command_palette_results.is_empty());
        assert!(app
            .command_palette_results
            .iter()
            .any(|r| r.name.contains("Theme")));
    }

    #[test]
    fn test_panel_type_values() {
        assert_eq!(PanelType::Files as u8, 0);
        assert_eq!(PanelType::Branches as u8, 1);
        assert_eq!(PanelType::Commits as u8, 2);
    }

    #[test]
    fn test_command_palette_execute() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert!(app.sidebar_visible);
        app.execute_command("toggle_sidebar");
        assert!(!app.sidebar_visible);
        app.execute_command("toggle_theme");
        assert_eq!(app.theme.name(), "light");
    }

    #[test]
    fn test_mouse_scroll_down() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.selected_index, 0);
        app.mouse_scroll_down();
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_mouse_scroll_up() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.selected_index = 2;
        app.mouse_scroll_up();
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_mouse_scroll_bounds() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.selected_index = 0;
        app.mouse_scroll_up();
        assert_eq!(app.selected_index, 0);

        app.selected_index = 2;
        app.mouse_scroll_down();
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn test_mouse_click_sets_position() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);
        app.mouse_enabled = true;

        let mouse_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };

        app.handle_mouse(mouse_event);
        assert_eq!(app.last_click_position, Some((10, 5)));
        assert!(app.last_click_time.is_some());
    }

    #[test]
    fn test_mouse_double_click_detection() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);
        app.mouse_enabled = true;

        let mouse_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };

        app.handle_mouse(mouse_event);
        assert_eq!(app.view_mode, ViewMode::List);

        app.handle_mouse(mouse_event);
        assert_eq!(app.view_mode, ViewMode::Details);
    }

    #[test]
    fn test_toggle_file_stage_with_wrong_panel() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.active_panel = PanelType::Commits;
        app.toggle_file_stage();
        assert_eq!(app.status_message, "");
    }

    #[test]
    fn test_stage_all_files() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.repo_path = None;
        app.stage_all_files();
        assert_eq!(app.status_message, "No repository path available");
    }

    #[test]
    fn test_unstage_all_files() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.repo_path = None;
        app.unstage_all_files();
        assert_eq!(app.status_message, "No repository path available");
    }

    #[test]
    fn test_file_selection_navigation() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.files = vec![
            FileStatus {
                path: "file1.rs".to_string(),
                status: openisl_git::StatusType::Modified,
            },
            FileStatus {
                path: "file2.rs".to_string(),
                status: openisl_git::StatusType::Added,
            },
            FileStatus {
                path: "file3.rs".to_string(),
                status: openisl_git::StatusType::Untracked,
            },
        ];

        assert_eq!(app.selected_file_index, 0);
        app.move_file_selection_down();
        assert_eq!(app.selected_file_index, 1);
        app.move_file_selection_down();
        assert_eq!(app.selected_file_index, 2);
        app.move_file_selection_down();
        assert_eq!(app.selected_file_index, 2);
        app.move_file_selection_up();
        assert_eq!(app.selected_file_index, 1);
        app.move_file_selection_up();
        assert_eq!(app.selected_file_index, 0);
        app.move_file_selection_up();
        assert_eq!(app.selected_file_index, 0);
    }

    #[test]
    fn test_staging_command_in_palette() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        // Switch to Files panel so staging commands are available
        app.active_panel = PanelType::Files;

        app.command_palette_input = "stage".to_string();
        app.filter_command_palette();

        assert!(!app.command_palette_results.is_empty());
        assert!(app
            .command_palette_results
            .iter()
            .any(|r| r.name.contains("Stage")));
    }

    #[test]
    fn test_execute_stage_command() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.repo_path = None;
        app.execute_command("stage_all");
        assert_eq!(app.status_message, "No repository path available");
    }

    #[test]
    fn test_refresh_files() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.repo_path = None;
        app.refresh_files();
        assert!(app.files.is_empty());
    }

    #[test]
    fn test_mouse_toggle() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert!(!app.mouse_enabled);
        app.toggle_mouse_mode();
        assert!(app.mouse_enabled);
        assert!(app.status_message.contains("ON"));
        app.toggle_mouse_mode();
        assert!(!app.mouse_enabled);
        assert!(app.status_message.contains("OFF"));
    }

    #[test]
    fn test_mouse_disabled_ignores_events() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert!(!app.mouse_enabled);
        let mouse_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        app.handle_mouse(mouse_event);
        assert_eq!(app.last_click_position, None);
    }
}
