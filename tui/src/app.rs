use crate::diff::{DiffParser, DiffStats};
use crate::keybindings::KeyBindings;
use crate::theme::Theme;
use crate::tree::{format_tree_lines, CommitTree};
use anyhow::Result;
use crossterm::{
    event::{
        self, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use openisl_git::{get_commit_diff, Commit, FileStatus, GitRef};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Line, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget},
    Terminal,
};
use std::io::stdout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelType {
    Files,
    Branches,
    Commits,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    List,
    Details,
    Diff,
    Help,
    InputBranch,
    Search,
    Filter,
    Stats,
    CommandPalette,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FilterMode {
    Author,
    Message,
    Date,
}

#[derive(Clone, Debug)]
pub struct CommandAction {
    pub name: String,
    pub description: String,
    pub action: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Default)]
pub struct RepoStats {
    pub total_commits: usize,
    pub total_authors: usize,
    pub commits_by_author: Vec<(String, usize)>,
    pub commits_today: usize,
    pub commits_this_week: usize,
    pub commits_this_month: usize,
}

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
    pub selected_branch_index: usize,
    pub branch_scroll_offset: usize,
    pub command_palette_input: String,
    pub command_palette_results: Vec<CommandAction>,
    pub mouse_scroll_offset: usize,
    pub last_click_position: Option<(u16, u16)>,
    pub last_click_time: Option<std::time::Instant>,
}

impl App {
    pub fn new(
        commits: Vec<Commit>,
        current_branch: String,
        repo_path: Option<std::path::PathBuf>,
    ) -> Self {
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
            branches: Vec::new(),
            selected_branch_index: 0,
            branch_scroll_offset: 0,
            command_palette_input: String::new(),
            command_palette_results: Vec::new(),
            mouse_scroll_offset: 0,
            last_click_position: None,
            last_click_time: None,
        };
        app.calculate_stats();
        app.populate_command_palette();
        app
    }

    fn populate_command_palette(&mut self) {
        self.command_palette_results = vec![
            CommandAction {
                name: "Toggle Sidebar".to_string(),
                description: "Show/hide the sidebar panel".to_string(),
                action: "toggle_sidebar".to_string(),
                keys: vec!["Ctrl+B".to_string()],
            },
            CommandAction {
                name: "Next Panel".to_string(),
                description: "Move focus to next panel".to_string(),
                action: "next_panel".to_string(),
                keys: vec!["Tab".to_string(), "→".to_string(), "l".to_string()],
            },
            CommandAction {
                name: "Previous Panel".to_string(),
                description: "Move focus to previous panel".to_string(),
                action: "prev_panel".to_string(),
                keys: vec!["Shift+Tab".to_string(), "←".to_string(), "h".to_string()],
            },
            CommandAction {
                name: "Navigate Up".to_string(),
                description: "Move selection up".to_string(),
                action: "move_up".to_string(),
                keys: vec!["k".to_string(), "↑".to_string()],
            },
            CommandAction {
                name: "Navigate Down".to_string(),
                description: "Move selection down".to_string(),
                action: "move_down".to_string(),
                keys: vec!["j".to_string(), "↓".to_string()],
            },
            CommandAction {
                name: "Go to Start".to_string(),
                description: "Jump to first item".to_string(),
                action: "go_to_start".to_string(),
                keys: vec!["gg".to_string(), "Home".to_string()],
            },
            CommandAction {
                name: "Go to End".to_string(),
                description: "Jump to last item".to_string(),
                action: "go_to_end".to_string(),
                keys: vec!["G".to_string(), "End".to_string()],
            },
            CommandAction {
                name: "View Details".to_string(),
                description: "Show commit/file details".to_string(),
                action: "view_details".to_string(),
                keys: vec!["Enter".to_string()],
            },
            CommandAction {
                name: "Search".to_string(),
                description: "Search commits or files".to_string(),
                action: "search".to_string(),
                keys: vec!["/".to_string()],
            },
            CommandAction {
                name: "Toggle Theme".to_string(),
                description: "Switch between dark/light theme".to_string(),
                action: "toggle_theme".to_string(),
                keys: vec!["t".to_string()],
            },
            CommandAction {
                name: "Show Help".to_string(),
                description: "Display keyboard shortcuts".to_string(),
                action: "help".to_string(),
                keys: vec!["?".to_string()],
            },
            CommandAction {
                name: "Quit".to_string(),
                description: "Exit openisl".to_string(),
                action: "quit".to_string(),
                keys: vec!["q".to_string(), "Esc".to_string()],
            },
            CommandAction {
                name: "Command Palette".to_string(),
                description: "Open command search".to_string(),
                action: "command_palette".to_string(),
                keys: vec!["Ctrl+P".to_string()],
            },
        ];
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
            PanelType::Commits => PanelType::Files,
        };
        self.status_message = format!("Switched to {} panel", self.panel_name());
    }

    pub fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            PanelType::Files => PanelType::Commits,
            PanelType::Branches => PanelType::Files,
            PanelType::Commits => PanelType::Branches,
        };
        self.status_message = format!("Switched to {} panel", self.panel_name());
    }

    fn panel_name(&self) -> String {
        match self.active_panel {
            PanelType::Files => "Files",
            PanelType::Branches => "Branches",
            PanelType::Commits => "Commits",
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
        if self.command_palette_input.is_empty() {
            self.command_palette_results = Self::get_all_commands();
        } else {
            let query = self.command_palette_input.to_lowercase();
            self.command_palette_results = Self::get_all_commands()
                .into_iter()
                .filter(|action| {
                    action.name.to_lowercase().contains(&query)
                        || action.description.to_lowercase().contains(&query)
                        || action.action.contains(&query)
                })
                .collect();
        }
    }

    fn get_all_commands() -> Vec<CommandAction> {
        vec![
            CommandAction {
                name: "Toggle Sidebar".to_string(),
                description: "Show/hide the sidebar panel".to_string(),
                action: "toggle_sidebar".to_string(),
                keys: vec!["Ctrl+B".to_string()],
            },
            CommandAction {
                name: "Next Panel".to_string(),
                description: "Move focus to next panel".to_string(),
                action: "next_panel".to_string(),
                keys: vec!["Tab".to_string(), "→".to_string(), "l".to_string()],
            },
            CommandAction {
                name: "Previous Panel".to_string(),
                description: "Move focus to previous panel".to_string(),
                action: "prev_panel".to_string(),
                keys: vec!["Shift+Tab".to_string(), "←".to_string(), "h".to_string()],
            },
            CommandAction {
                name: "Navigate Up".to_string(),
                description: "Move selection up".to_string(),
                action: "move_up".to_string(),
                keys: vec!["k".to_string(), "↑".to_string()],
            },
            CommandAction {
                name: "Navigate Down".to_string(),
                description: "Move selection down".to_string(),
                action: "move_down".to_string(),
                keys: vec!["j".to_string(), "↓".to_string()],
            },
            CommandAction {
                name: "Stage/Unstage File".to_string(),
                description: "Stage or unstage the selected file".to_string(),
                action: "toggle_stage".to_string(),
                keys: vec!["Space".to_string()],
            },
            CommandAction {
                name: "Stage All".to_string(),
                description: "Stage all files".to_string(),
                action: "stage_all".to_string(),
                keys: vec!["Ctrl+S".to_string()],
            },
            CommandAction {
                name: "Unstage All".to_string(),
                description: "Unstage all files".to_string(),
                action: "unstage_all".to_string(),
                keys: vec!["Ctrl+U".to_string()],
            },
            CommandAction {
                name: "Go to Start".to_string(),
                description: "Jump to first item".to_string(),
                action: "go_to_start".to_string(),
                keys: vec!["gg".to_string(), "Home".to_string()],
            },
            CommandAction {
                name: "Go to End".to_string(),
                description: "Jump to last item".to_string(),
                action: "go_to_end".to_string(),
                keys: vec!["G".to_string(), "End".to_string()],
            },
            CommandAction {
                name: "View Details".to_string(),
                description: "Show commit/file details".to_string(),
                action: "view_details".to_string(),
                keys: vec!["Enter".to_string()],
            },
            CommandAction {
                name: "Search".to_string(),
                description: "Search commits or files".to_string(),
                action: "search".to_string(),
                keys: vec!["/".to_string()],
            },
            CommandAction {
                name: "Toggle Theme".to_string(),
                description: "Switch between dark/light theme".to_string(),
                action: "toggle_theme".to_string(),
                keys: vec!["t".to_string()],
            },
            CommandAction {
                name: "Show Help".to_string(),
                description: "Display keyboard shortcuts".to_string(),
                action: "help".to_string(),
                keys: vec!["?".to_string()],
            },
            CommandAction {
                name: "Quit".to_string(),
                description: "Exit openisl".to_string(),
                action: "quit".to_string(),
                keys: vec!["q".to_string(), "Esc".to_string()],
            },
            CommandAction {
                name: "Command Palette".to_string(),
                description: "Open command search".to_string(),
                action: "command_palette".to_string(),
                keys: vec!["Ctrl+P".to_string()],
            },
        ]
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.view_mode {
            ViewMode::List => self.handle_list_key(key),
            ViewMode::Details => self.handle_details_key(key),
            ViewMode::Diff => self.handle_diff_key(key),
            ViewMode::Help => self.handle_help_key(key),
            ViewMode::InputBranch => self.handle_input_key(key),
            ViewMode::Search => self.handle_search_key(key),
            ViewMode::Filter => self.handle_filter_key(key),
            ViewMode::Stats => self.handle_stats_key(key),
            ViewMode::CommandPalette => self.handle_command_palette_key(key),
        }
    }

    fn handle_list_key(&mut self, key: KeyEvent) -> bool {
        if self.is_searching {
            return self.handle_search_key(key);
        }

        match key.code {
            KeyCode::Char('q') => return true,
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::PageDown => self.page_down(),
            KeyCode::PageUp => self.page_up(),
            KeyCode::Home => self.go_to_start(),
            KeyCode::End => self.go_to_end(),
            KeyCode::Enter => self.view_mode = ViewMode::Details,
            KeyCode::Tab => self.next_panel(),
            KeyCode::BackTab => self.prev_panel(),
            KeyCode::Char('h') | KeyCode::Left => {
                if self.sidebar_visible {
                    self.prev_panel();
                }
            }
            KeyCode::Char('l') | KeyCode::Right => {
                if self.sidebar_visible {
                    self.next_panel();
                }
            }
            KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_sidebar();
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.open_command_palette();
            }
            KeyCode::Char('/') => {
                self.is_searching = true;
                self.search_query.clear();
            }
            KeyCode::Char('f') => {
                self.filter_input.clear();
                self.filter_mode = FilterMode::Author;
                self.view_mode = ViewMode::Filter;
                self.status_message =
                    "Filter by author (a), message (m), or date (d) - Esc to cancel".to_string();
            }
            KeyCode::Char('s') => {
                self.view_mode = ViewMode::Stats;
            }
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.next_search_result()
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.prev_search_result()
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                self.fetch_diff();
                self.view_mode = ViewMode::Diff;
            }
            KeyCode::Char('c') => {
                if let Some(commit) = self.selected_commit() {
                    self.status_message = format!("Would checkout {}...", &commit.short_hash);
                }
            }
            KeyCode::Char('b') => {
                self.branch_input.clear();
                self.view_mode = ViewMode::InputBranch;
                self.status_message = "Enter branch name (or Esc to cancel):".to_string();
            }
            KeyCode::Char('?') => self.view_mode = ViewMode::Help,
            KeyCode::Char('r') => {
                self.apply_filter();
                self.status_message = format!("Filter: {} commits", self.filtered_commits.len());
            }
            KeyCode::Char('t') => self.theme.next(),
            KeyCode::Char(' ') => {
                if self.active_panel == PanelType::Files {
                    self.toggle_file_stage();
                } else {
                    self.move_down();
                }
            }
            KeyCode::Char('S') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.stage_all_files();
            }
            KeyCode::Char('U') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.unstage_all_files();
            }
            _ => {}
        }
        false
    }

    fn handle_filter_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.clear_filter();
                self.view_mode = ViewMode::List;
                self.status_message.clear();
                return false;
            }
            KeyCode::Enter => {
                self.apply_filter();
                self.status_message = format!("Filter: {} commits", self.filtered_commits.len());
                self.view_mode = ViewMode::List;
                return false;
            }
            KeyCode::Backspace => {
                self.filter_input.pop();
            }
            KeyCode::Char('a') => {
                self.filter_mode = FilterMode::Author;
                self.status_message = "Filtering by author...".to_string();
            }
            KeyCode::Char('m') => {
                self.filter_mode = FilterMode::Message;
                self.status_message = "Filtering by message...".to_string();
            }
            KeyCode::Char('d') => {
                self.filter_mode = FilterMode::Date;
                self.status_message = "Filtering by date (YYYY-MM-DD)...".to_string();
            }
            KeyCode::Char(c) => {
                if c.is_ascii_alphanumeric()
                    || c == '-'
                    || c == '_'
                    || c == ' '
                    || c == '.'
                    || c == '@'
                {
                    self.filter_input.push(c);
                }
            }
            _ => {}
        }
        false
    }

    fn handle_stats_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                self.view_mode = ViewMode::List;
            }
            _ => {}
        }
        false
    }

    fn handle_command_palette_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.view_mode = ViewMode::List;
                self.status_message.clear();
                return false;
            }
            KeyCode::Enter => {
                if !self.command_palette_results.is_empty() {
                    let action = self.command_palette_results[0].action.clone();
                    self.execute_command(&action);
                }
                self.view_mode = ViewMode::List;
                self.status_message.clear();
                return false;
            }
            KeyCode::Backspace => {
                self.command_palette_input.pop();
                self.filter_command_palette();
            }
            KeyCode::Char(c) => {
                self.command_palette_input.push(c);
                self.filter_command_palette();
            }
            KeyCode::Up => {
                if !self.command_palette_results.is_empty() {
                    self.command_palette_results.rotate_right(1);
                }
            }
            KeyCode::Down => {
                if !self.command_palette_results.is_empty() {
                    self.command_palette_results.rotate_left(1);
                }
            }
            _ => {}
        }
        false
    }

    fn execute_command(&mut self, action: &str) {
        match action {
            "toggle_sidebar" => self.toggle_sidebar(),
            "next_panel" => self.next_panel(),
            "prev_panel" => self.prev_panel(),
            "move_up" => self.move_up(),
            "move_down" => self.move_down(),
            "go_to_start" => self.go_to_start(),
            "go_to_end" => self.go_to_end(),
            "view_details" => self.view_mode = ViewMode::Details,
            "search" => {
                self.is_searching = true;
                self.search_query.clear();
            }
            "toggle_stage" => self.toggle_file_stage(),
            "stage_all" => self.stage_all_files(),
            "unstage_all" => self.unstage_all_files(),
            "toggle_theme" => self.theme.next(),
            "help" => self.view_mode = ViewMode::Help,
            "quit" => {}
            _ => {}
        }
    }

    pub fn handle_mouse(&mut self, event: MouseEvent) -> bool {
        match event.kind {
            MouseEventKind::Down(btn) if btn == MouseButton::Left => {
                self.handle_left_click(event);
            }
            MouseEventKind::Down(btn) if btn == MouseButton::Right => {
                self.handle_right_click();
            }
            MouseEventKind::ScrollDown => {
                self.mouse_scroll_down();
            }
            MouseEventKind::ScrollUp => {
                self.mouse_scroll_up();
            }
            _ => {}
        }
        false
    }

    fn handle_left_click(&mut self, event: MouseEvent) {
        let now = std::time::Instant::now();
        let is_double_click = match (self.last_click_position, self.last_click_time) {
            (Some((x, y)), Some(time)) => {
                let time_elapsed = now.duration_since(time);
                x == event.column && y == event.row && time_elapsed.as_millis() < 300
            }
            _ => false,
        };

        self.last_click_position = Some((event.column, event.row));
        self.last_click_time = Some(now);

        if self.sidebar_visible {
            self.handle_sidebar_click(event);
        } else {
            self.handle_main_area_click(event);
        }

        if is_double_click {
            self.handle_double_click();
        }
    }

    fn handle_right_click(&mut self) {
        self.status_message = "Right click - use left click to select".to_string();
    }

    fn handle_sidebar_click(&mut self, event: MouseEvent) {
        let sidebar_width = 30;

        if event.column < sidebar_width {
            let sidebar_y = event.row;

            if sidebar_y < 3 {
                self.active_panel = PanelType::Files;
            } else if sidebar_y < 6 {
                self.active_panel = PanelType::Branches;
            } else {
                self.active_panel = PanelType::Commits;
            }
        }
    }

    fn handle_main_area_click(&mut self, event: MouseEvent) {
        let items_per_page = 15;
        let header_height = 2;
        let offset = self.scroll_offset;

        if event.row >= header_height && event.row < header_height + items_per_page {
            let clicked_index = offset + (event.row - header_height) as usize;
            if clicked_index < self.commits.len() {
                self.selected_index = clicked_index;
            }
        }
    }

    fn handle_double_click(&mut self) {
        match self.view_mode {
            ViewMode::List => {
                self.view_mode = ViewMode::Details;
                self.status_message = "Double-click: viewing details".to_string();
            }
            _ => {}
        }
    }

    fn mouse_scroll_down(&mut self) {
        let max_index = self.commits.len().saturating_sub(1);
        let items_per_page = 15;

        if self.selected_index < max_index {
            self.selected_index += 1;
            if self.selected_index >= self.scroll_offset + items_per_page {
                self.scroll_offset = self.selected_index.saturating_sub(items_per_page - 1);
            }
        }
    }

    fn mouse_scroll_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index = self.selected_index.saturating_sub(1);
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index.saturating_sub(1);
            }
        }
    }

    fn handle_search_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.clear_search();
                return false;
            }
            KeyCode::Enter => {
                if self.search_results.is_empty() {
                    self.clear_search();
                }
                return false;
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                self.search();
            }
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.next_search_result()
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.prev_search_result()
            }
            KeyCode::Char(c) => {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ' || c == '.' {
                    self.search_query.push(c);
                    self.search();
                }
            }
            _ => {}
        }
        false
    }

    fn handle_details_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.view_mode = ViewMode::List;
                return false;
            }
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('c') => {
                if let Some(commit) = self.selected_commit() {
                    self.status_message = format!("Would checkout {}!", &commit.short_hash);
                }
            }
            KeyCode::Char('b') => {
                self.branch_input.clear();
                self.view_mode = ViewMode::InputBranch;
                self.status_message = "Enter branch name (or Esc to cancel):".to_string();
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                self.fetch_diff();
                self.view_mode = ViewMode::Diff;
            }
            _ => {}
        }
        false
    }

    fn handle_input_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.view_mode = ViewMode::List;
                self.branch_input.clear();
                self.status_message.clear();
                return false;
            }
            KeyCode::Enter => {
                if !self.branch_input.is_empty() {
                    if let Some(commit) = self.selected_commit() {
                        self.status_message = format!(
                            "Created branch '{}' from {}",
                            self.branch_input, commit.short_hash
                        );
                    }
                }
                self.branch_input.clear();
                self.view_mode = ViewMode::List;
                return false;
            }
            KeyCode::Backspace => {
                self.branch_input.pop();
            }
            KeyCode::Char(c) => {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '/' {
                    self.branch_input.push(c);
                }
            }
            _ => {}
        }
        false
    }

    fn handle_diff_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.view_mode = ViewMode::List,
            KeyCode::Char('?') => self.view_mode = ViewMode::Help,
            _ => {}
        }
        false
    }

    fn handle_help_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?') => {
                self.view_mode = ViewMode::List;
            }
            _ => {}
        }
        false
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.commits.len().saturating_sub(1) {
            self.selected_index += 1;
            if self.selected_index >= self.scroll_offset + 20 {
                self.scroll_offset = self.selected_index - 20 + 1;
            }
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index = self.selected_index.saturating_sub(1);
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index.saturating_sub(1);
            }
        }
    }

    pub fn page_down(&mut self) {
        let max_index = self.commits.len().saturating_sub(1);
        self.selected_index = (self.selected_index + 20).min(max_index);
        self.scroll_offset = (self.scroll_offset + 20).min(max_index.saturating_sub(20));
    }

    pub fn page_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(20);
        self.scroll_offset = self.scroll_offset.saturating_sub(20);
    }

    pub fn go_to_start(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn go_to_end(&mut self) {
        if !self.commits.is_empty() {
            self.selected_index = self.commits.len() - 1;
            self.scroll_offset = self.commits.len().saturating_sub(20);
        }
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

    pub fn fetch_diff(&mut self) {
        if let Some(commit) = self.selected_commit() {
            if let Some(ref repo_path) = self.repo_path {
                match get_commit_diff(repo_path, &commit.hash) {
                    Ok(diff) => {
                        self.diff_content = diff;
                        self.parse_diff();
                    }
                    Err(e) => {
                        self.diff_content = format!("Error fetching diff: {}", e);
                        self.parse_diff();
                    }
                }
            } else {
                self.diff_content = "No repository path available".to_string();
                self.parse_diff();
            }
        }
    }

    pub fn refresh_files(&mut self) {
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::get_status(repo_path) {
                Ok(files) => {
                    self.files = files;
                }
                Err(e) => {
                    self.status_message = format!("Error loading files: {}", e);
                }
            }
        }
    }

    pub fn stage_selected_file(&mut self) {
        if self.active_panel != PanelType::Files {
            return;
        }

        if self.files.is_empty() {
            self.status_message = "No files to stage".to_string();
            return;
        }

        if let Some(file) = self.files.get(self.selected_file_index) {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::stage_file(repo_path, &file.path) {
                    Ok(_) => {
                        self.status_message = format!("Staged: {}", file.path);
                        self.refresh_files();
                    }
                    Err(e) => {
                        self.status_message = format!("Error staging file: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        }
    }

    pub fn unstage_selected_file(&mut self) {
        if self.active_panel != PanelType::Files {
            return;
        }

        if self.files.is_empty() {
            self.status_message = "No files to unstage".to_string();
            return;
        }

        if let Some(file) = self.files.get(self.selected_file_index) {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::unstage_file(repo_path, &file.path) {
                    Ok(_) => {
                        self.status_message = format!("Unstaged: {}", file.path);
                        self.refresh_files();
                    }
                    Err(e) => {
                        self.status_message = format!("Error unstaging file: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        }
    }

    pub fn toggle_file_stage(&mut self) {
        if self.active_panel != PanelType::Files {
            return;
        }

        if self.files.is_empty() {
            self.status_message = "No files".to_string();
            return;
        }

        if let Some(file) = self.files.get(self.selected_file_index) {
            let is_staged = matches!(
                file.status,
                openisl_git::StatusType::ModifiedStaged
                    | openisl_git::StatusType::AddedStaged
                    | openisl_git::StatusType::DeletedStaged
            );

            if is_staged {
                self.unstage_selected_file();
            } else {
                self.stage_selected_file();
            }
        }
    }

    pub fn stage_all_files(&mut self) {
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::stage_all(repo_path) {
                Ok(_) => {
                    self.status_message = "Staged all files".to_string();
                    self.refresh_files();
                }
                Err(e) => {
                    self.status_message = format!("Error staging all files: {}", e);
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
        }
    }

    pub fn unstage_all_files(&mut self) {
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::unstage_all(repo_path) {
                Ok(_) => {
                    self.status_message = "Unstaged all files".to_string();
                    self.refresh_files();
                }
                Err(e) => {
                    self.status_message = format!("Error unstaging all files: {}", e);
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
        }
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
    execute!(stdout, EnableMouseCapture)?;

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
            ViewMode::Filter => render_filter_view(&app, frame),
            ViewMode::Stats => render_stats_view(&app, frame),
            ViewMode::CommandPalette => render_command_palette(&app, frame),
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

fn render_list_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if app.sidebar_visible { 30 } else { 0 }),
            Constraint::Min(10),
        ])
        .split(frame.size());

    if app.sidebar_visible {
        render_sidebar(app, chunks[0], frame);
    }

    render_main_content(app, chunks[1], frame);

    render_footer(app, frame.size(), frame);
}

fn render_sidebar(app: &App, area: Rect, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(10),
        ])
        .split(area);

    render_panel_tab(app, PanelType::Files, " FILES ", chunks[0], frame);
    render_panel_tab(app, PanelType::Branches, " BRANCHES ", chunks[1], frame);

    match app.active_panel {
        PanelType::Files => render_files_panel(app).render(chunks[2], frame.buffer_mut()),
        PanelType::Branches => render_branches_panel(app).render(chunks[2], frame.buffer_mut()),
        PanelType::Commits => {
            render_commits_panel(app, chunks[2]).render(chunks[2], frame.buffer_mut())
        }
    }
}

fn render_panel_tab(
    app: &App,
    panel_type: PanelType,
    title: &str,
    area: Rect,
    frame: &mut ratatui::Frame,
) {
    let is_active = app.active_panel == panel_type;
    let style = if is_active {
        Style::default()
            .fg(app.theme.selected)
            .bg(app.theme.selected_bg)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.text).bg(app.theme.background)
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .style(style);

    block.render(area, frame.buffer_mut());
}

fn render_files_panel(app: &App) -> impl Widget {
    let items: Vec<ListItem<'static>> = app
        .files
        .iter()
        .map(|file| {
            let status = match file.status {
                openisl_git::StatusType::Modified => "M",
                openisl_git::StatusType::Added => "A",
                openisl_git::StatusType::Deleted => "D",
                openisl_git::StatusType::Untracked => "?",
                openisl_git::StatusType::ModifiedStaged => "M*",
                openisl_git::StatusType::AddedStaged => "A*",
                openisl_git::StatusType::DeletedStaged => "D*",
                openisl_git::StatusType::Renamed => "R",
                openisl_git::StatusType::Conflicted => "C",
            };
            let content = format!("{} {}", status, file.path);
            let is_selected = app.selected_file_index
                == app
                    .files
                    .iter()
                    .position(|f| f.path == file.path)
                    .unwrap_or(0);
            let style = if is_selected {
                Style::default().fg(Color::White).bg(app.theme.selected_bg)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!("Files ({})", app.files.len()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(app.theme.border)),
    );

    list
}

fn render_branches_panel(app: &App) -> impl Widget {
    let items: Vec<ListItem<'static>> = app
        .branches
        .iter()
        .map(|branch| {
            let is_current = branch.name == app.current_branch;
            let prefix = if is_current { "●" } else { "○" };
            let content = format!("{} {}", prefix, branch.name);
            let is_selected = app.selected_branch_index
                == app
                    .branches
                    .iter()
                    .position(|b| b.name == branch.name)
                    .unwrap_or(0);
            let style = if is_selected {
                Style::default().fg(Color::White).bg(app.theme.selected_bg)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!("Branches ({})", app.branches.len()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(app.theme.border)),
    );

    list
}

fn render_commits_panel(app: &App, area: Rect) -> impl Widget {
    let panel_height = area.height.saturating_sub(2) as usize;
    let visible_count = panel_height.max(1);
    let raw_lines = format_tree_lines(app.tree.nodes(), app.scroll_offset, visible_count);

    let lines: Vec<Line<'static>> = raw_lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let global_index = app.scroll_offset + i;
            let is_selected = global_index == app.selected_index;
            let line_clone = line.clone();

            if is_selected {
                Line::from(line_clone).style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                        .bg(app.theme.selected_bg),
                )
            } else {
                Line::from(line_clone).style(Style::default().fg(Color::White))
            }
        })
        .collect();

    let list = List::new(lines).block(
        Block::default()
            .title(format!(
                "Commits ({}/{})",
                app.selected_index + 1,
                app.commits.len()
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(app.theme.border)),
    );

    list
}

fn render_main_content(app: &App, area: Rect, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(area);

    let title = Paragraph::new(format!(
        "openisl - {} - {}",
        app.repo_path
            .as_ref()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "Unknown".to_string()),
        app.current_branch
    ))
    .style(
        Style::default()
            .fg(app.theme.title)
            .add_modifier(Modifier::BOLD),
    )
    .alignment(Alignment::Left);
    title.render(chunks[0], frame.buffer_mut());

    let content_height = chunks[1].height.saturating_sub(2) as usize;
    let visible_count = content_height.max(1);
    let raw_lines = format_tree_lines(app.tree.nodes(), app.scroll_offset, visible_count);

    let lines: Vec<Line<'static>> = raw_lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let global_index = app.scroll_offset + i;
            let is_selected = global_index == app.selected_index;
            let line_clone = line.clone();

            if is_selected {
                Line::from(line_clone).style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                        .bg(app.theme.selected_bg),
                )
            } else {
                Line::from(line_clone).style(Style::default().fg(Color::White))
            }
        })
        .collect();

    let commit_widget = Paragraph::new(lines).block(
        Block::default()
            .title(format!(
                "Commits ({}/{}) - {}",
                app.selected_index + 1,
                app.commits.len(),
                app.current_branch
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(app.theme.border)),
    );
    commit_widget.render(chunks[1], frame.buffer_mut());

    let status_text = if !app.status_message.is_empty() {
        format!(">> {}", app.status_message)
    } else {
        String::new()
    };
    let status_widget = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left);
    status_widget.render(chunks[2], frame.buffer_mut());
}

fn render_command_palette(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Command Palette")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let input_line = format!("> {}", app.command_palette_input);
    let input_widget = Paragraph::new(input_line)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .title("Search commands")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        );
    input_widget.render(chunks[1], frame.buffer_mut());

    let results: Vec<ListItem> = app
        .command_palette_results
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, action)| {
            let keys = action.keys.join(", ");
            let content = format!("{} - {} ({})", action.name, action.description, keys);
            let style = if i == 0 {
                Style::default()
                    .fg(app.theme.selected)
                    .bg(app.theme.selected_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let results_list = List::new(results).block(
        Block::default()
            .title(format!("Results ({})", app.command_palette_results.len()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(app.theme.border)),
    );
    results_list.render(chunks[1], frame.buffer_mut());

    let help_text = format!(
        "Enter: Execute | ↑↓/jk: Navigate | Esc: Cancel | Theme: {}",
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[2], frame.buffer_mut());
}

fn render_footer(app: &App, area: Rect, frame: &mut ratatui::Frame) {
    let help_text = format!(
        "{}: Panels | {}: Details | {}: Search | {}: Palette | {}: Help | {}: Theme | {}: Quit",
        "←→/Tab",
        app.keybindings.actions.view_details,
        "/",
        "Ctrl+P",
        app.keybindings.actions.help,
        app.keybindings.actions.toggle_theme,
        app.keybindings.actions.quit,
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(area, frame.buffer_mut());
}

fn render_details_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Commit Details")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    if let Some(commit) = app.selected_commit() {
        let details = app.format_commit_details(commit);
        let details_widget = Paragraph::new(details)
            .style(Style::default().fg(app.theme.text))
            .block(
                Block::default()
                    .title(format!("{} - {}", commit.short_hash, commit.summary))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .style(Style::default().fg(app.theme.border)),
            );
        details_widget.render(chunks[1], frame.buffer_mut());
    }

    let status_text = if !app.status_message.is_empty() {
        format!(">> {}", app.status_message)
    } else {
        String::new()
    };
    let status_widget = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left);
    status_widget.render(chunks[2], frame.buffer_mut());

    let help_text = format!(
        "{}: Checkout | {}: New Branch | {}: Diff | {}: Navigate | {}/{}: Back | Theme: {}",
        app.keybindings.actions.checkout,
        app.keybindings.actions.create_branch,
        app.keybindings.actions.view_diff,
        app.keybindings.navigation.up,
        app.keybindings.actions.quit,
        app.keybindings.actions.cancel,
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[3], frame.buffer_mut());
}

fn render_diff_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Commit Diff")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let stats_text = if !app.diff_content.is_empty() {
        app.diff_stats.format_summary()
    } else {
        String::from("No diff available")
    };

    let stats_widget = Paragraph::new(stats_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Left);
    stats_widget.render(chunks[1], frame.buffer_mut());

    let dark_theme = app.theme.name == "dark";

    let diff_widget = if app.diff_content.is_empty() {
        Paragraph::new(vec![Line::from(
            "No diff available. Use 'openisl diff' command for staged/working changes.",
        )])
        .style(Style::default().fg(app.theme.text))
    } else {
        let parsed_lines = DiffParser::parse(&app.diff_content);
        let styled_lines = DiffParser::to_styled_lines(&parsed_lines, dark_theme);
        Paragraph::new(styled_lines).style(Style::default().fg(app.theme.text))
    };

    diff_widget
        .block(
            Block::default()
                .title("Diff View")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        )
        .render(chunks[2], frame.buffer_mut());

    let help_text = format!(
        "{}/{}: Back | {}: Help | Theme: {}",
        app.keybindings.actions.quit,
        app.keybindings.actions.cancel,
        app.keybindings.actions.help,
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[3], frame.buffer_mut());
}

fn render_input_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Create Branch")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let input_prompt = Paragraph::new(format!(
        "Creating branch from commit: {}\n\nBranch name: {}\n\nPress Enter to create, Esc to cancel",
        app.selected_commit()
            .map(|c| c.short_hash.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        app.branch_input
    ))
    .style(Style::default().fg(app.theme.text))
    .alignment(Alignment::Left);
    input_prompt.render(chunks[1], frame.buffer_mut());

    let cursor = if app.branch_input.is_empty() {
        "_"
    } else {
        "|"
    };
    let input_display = Paragraph::new(format!("{} {}", app.branch_input, cursor)).style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );
    input_display.render(chunks[2], frame.buffer_mut());

    let help_text = format!(
        "{}: Cancel | {}: Create | Theme: {}",
        app.keybindings.actions.cancel,
        app.keybindings.actions.confirm,
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[3], frame.buffer_mut());
}

fn render_search_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Search Commits")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let search_info = if app.search_results.is_empty() && !app.search_query.is_empty() {
        format!("No matches found for '{}'", app.search_query)
    } else if !app.search_results.is_empty() {
        format!(
            "{} matches for '{}'",
            app.search_results.len(),
            app.search_query
        )
    } else {
        "Type to search commits (author, message, hash)".to_string()
    };

    let search_widget = Paragraph::new(format!("Search: {}\n\n{}", app.search_query, search_info))
        .style(Style::default().fg(app.theme.text))
        .alignment(Alignment::Left);
    search_widget.render(chunks[1], frame.buffer_mut());

    let commit_lines: Vec<String> = app
        .visible_commits()
        .iter()
        .enumerate()
        .map(|(i, commit)| {
            let global_index = app.scroll_offset + i;
            let is_selected = global_index == app.selected_index;
            let is_match = app.search_results.contains(&global_index);
            let prefix = if is_selected { ">" } else { " " };
            let match_indicator = if is_match { "*" } else { " " };
            format!(
                "{} {} {} - {}",
                prefix, match_indicator, commit.short_hash, commit.summary
            )
        })
        .collect();

    let commit_widget = Paragraph::new(commit_lines.join("\n"))
        .style(Style::default().fg(app.theme.text))
        .block(
            Block::default()
                .title(format!(
                    "Results ({}/{}) - {}",
                    app.search_results.len().max(1),
                    app.commits.len(),
                    app.current_branch
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        );
    commit_widget.render(chunks[2], frame.buffer_mut());

    let help_text = format!(
        "Ctrl+N/P: Next/Prev match | Enter: View | /: Search | Esc: Cancel | Theme: {}",
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[3], frame.buffer_mut());
}

fn render_help_overlay(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Keyboard Shortcuts")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let help_content = format!(
        r#"Navigation:
  {}         Move up
  {}         Move down
  {}         Jump page up
  {}         Jump page down
  {}         Go to first
  {}         Go to last

Actions:
  {}         View commit details
  {}         Checkout selected commit
  {}         Create branch from commit
  {}         View diff
  {}         Toggle dark/light theme
  /           Search commits
  Ctrl+N/P    Next/prev search result

Other:
  {}         Show this help
  {}         Quit or go back

Customize: Edit ~/.config/openisl/keybindings.toml"#,
        app.keybindings.navigation.up,
        app.keybindings.navigation.down,
        app.keybindings.navigation.page_up,
        app.keybindings.navigation.page_down,
        app.keybindings.navigation.go_to_start,
        app.keybindings.navigation.go_to_end,
        app.keybindings.actions.view_details,
        app.keybindings.actions.checkout,
        app.keybindings.actions.create_branch,
        app.keybindings.actions.view_diff,
        app.keybindings.actions.toggle_theme,
        app.keybindings.actions.help,
        app.keybindings.actions.quit,
    );

    let help_widget = Paragraph::new(help_content)
        .style(Style::default().fg(app.theme.text))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        );
    help_widget.render(chunks[1], frame.buffer_mut());

    let help_text = format!(
        "Press {} to close | Theme: {}",
        app.keybindings.actions.help,
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[2], frame.buffer_mut());
}

fn render_filter_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Filter Commits")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let filter_info = match app.filter_mode {
        FilterMode::Author => "Filter by author (press a/m/d to change filter type)",
        FilterMode::Message => "Filter by message (press a/m/d to change filter type)",
        FilterMode::Date => "Filter by date YYYY-MM-DD (press a/m/d to change filter type)",
    };

    let filter_prompt = Paragraph::new(format!(
        "{}\n\nCurrent filter: {}\n\nFilter: {}\n\nPress Enter to apply, Esc to cancel",
        filter_info,
        if app.filter_input.is_empty() {
            "(none)"
        } else {
            &app.filter_input
        },
        app.filter_input
    ))
    .style(Style::default().fg(app.theme.text))
    .alignment(Alignment::Left);
    filter_prompt.render(chunks[1], frame.buffer_mut());

    let cursor = if app.filter_input.is_empty() {
        "_"
    } else {
        "|"
    };
    let input_display = Paragraph::new(format!("{} {}", app.filter_input, cursor)).style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );
    input_display.render(chunks[2], frame.buffer_mut());

    let help_text = format!(
        "Enter: Apply | Esc: Cancel | a/m/d: Filter type | Theme: {}",
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[3], frame.buffer_mut());
}

fn render_stats_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Repository Statistics")
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let stats_content = format!(
        r#"Repository: {}
Current Branch: {}

Commits:
  Total: {}
  Today: {}
  This Week: {}
  This Month: {}

Authors:
  Total: {}

Top Contributors:
"#,
        app.repo_path
            .as_ref()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "Unknown".to_string()),
        app.current_branch,
        app.stats.total_commits,
        app.stats.commits_today,
        app.stats.commits_this_week,
        app.stats.commits_this_month,
        app.stats.total_authors,
    );

    let mut top_contributors = String::new();
    for (i, (author, count)) in app.stats.commits_by_author.iter().take(5).enumerate() {
        top_contributors.push_str(&format!("  {}. {} ({})\n", i + 1, author, count));
    }

    let full_content = format!("{}{}", stats_content, top_contributors);

    let stats_widget = Paragraph::new(full_content)
        .style(Style::default().fg(app.theme.text))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Statistics")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        );
    stats_widget.render(chunks[1], frame.buffer_mut());

    let help_text = format!(
        "Press Enter, Esc, or q to close | Theme: {}",
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[2], frame.buffer_mut());
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

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

        app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
        assert!(app.status_message.contains("Would checkout"));
    }

    #[test]
    fn test_checkout_from_details_view() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.view_mode, ViewMode::Details);

        app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
        assert!(app.status_message.contains("Would checkout"));
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
        assert_eq!(ViewMode::Filter as u8, 6);
        assert_eq!(ViewMode::Stats as u8, 7);
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

        assert_eq!(app.active_panel, PanelType::Commits);
        app.next_panel();
        assert_eq!(app.active_panel, PanelType::Files);
        app.next_panel();
        assert_eq!(app.active_panel, PanelType::Branches);
        app.next_panel();
        assert_eq!(app.active_panel, PanelType::Commits);

        app.prev_panel();
        assert_eq!(app.active_panel, PanelType::Branches);
        app.prev_panel();
        assert_eq!(app.active_panel, PanelType::Files);
        app.prev_panel();
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

        app.view_mode = ViewMode::CommandPalette;
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
}
