//! Keyboard event handlers
//!
//! Contains key event dispatch and per-view key handling logic, plus
//! list navigation helpers shared by keyboard and mouse handlers.
use super::super::*;

impl App {
    pub(crate) fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.view_mode {
            ViewMode::List => self.handle_list_key(key),
            ViewMode::Details => self.handle_details_key(key),
            ViewMode::Diff => self.handle_diff_key(key),
            ViewMode::Help => self.handle_help_key(key),
            ViewMode::InputBranch => self.handle_input_key(key),
            ViewMode::Search => self.handle_search_key(key),
            ViewMode::BranchSearch => self.handle_branch_search_key(key),
            ViewMode::Filter => self.handle_filter_key(key),
            ViewMode::Stats => self.handle_stats_key(key),
            ViewMode::CommandPalette => self.handle_command_palette_key(key),
            ViewMode::Stash => self.handle_stash_key(key),
            ViewMode::HunkStaging => self.handle_hunk_staging_key(key), // Handle hunk staging mode
        }
    }

    pub(crate) fn handle_list_key(&mut self, key: KeyEvent) -> bool {
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
                if self.active_panel == PanelType::Branches {
                    self.is_branch_searching = true;
                    self.branch_search_query.clear();
                    self.view_mode = ViewMode::BranchSearch;
                } else {
                    self.is_searching = true;
                    self.search_query.clear();
                    self.view_mode = ViewMode::Search;
                }
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
            KeyCode::Char('A') => {
                self.amend_commit();
            }
            KeyCode::Char('D') => {
                self.drop_commit();
            }
            KeyCode::Char('S') => {
                self.squash_commits();
            }
            KeyCode::Char('C') => {
                self.cherry_pick_commit();
            }
            KeyCode::Char('R') => {
                self.revert_commit();
            }
            KeyCode::Char('c') => {
                self.branch_input.clear();
                self.view_mode = ViewMode::InputBranch;
                self.status_message = "Enter branch name (or Esc to cancel):".to_string();
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
            KeyCode::Char('m') => self.toggle_mouse_mode(),
            KeyCode::Char(' ') => {
                if self.active_panel == PanelType::Files {
                    self.toggle_file_stage();
                } else {
                    self.move_down();
                }
            }
            KeyCode::Char('U') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.unstage_all_files();
            }
            _ => {}
        }
        false
    }

    pub(crate) fn handle_hunk_staging_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.is_hunk_staging_mode = false;
                self.view_mode = ViewMode::Diff; // Exit hunk staging mode
                self.status_message = "Exited hunk staging mode".to_string();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                // Move down to next line or next hunk
                if let Some(hunk) = self.hunks.get(self.selected_hunk_index) {
                    if self.selected_hunk_line_index < hunk.lines.len().saturating_sub(1) {
                        self.selected_hunk_line_index += 1;
                    } else if self.selected_hunk_index < self.hunks.len().saturating_sub(1) {
                        self.selected_hunk_index += 1;
                        self.selected_hunk_line_index = 0; // Reset line selection for new hunk
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                // Move up to previous line or previous hunk
                if self.selected_hunk_line_index > 0 {
                    self.selected_hunk_line_index -= 1;
                } else if self.selected_hunk_index > 0 {
                    self.selected_hunk_index -= 1;
                    // Move to the last line of the previous hunk
                    if let Some(hunk) = self.hunks.get(self.selected_hunk_index) {
                        self.selected_hunk_line_index = hunk.lines.len().saturating_sub(1);
                    }
                }
            }
            KeyCode::Char(' ') => {
                // Toggle selection of the current line or hunk
                if let Some(hunk) = self.hunks.get_mut(self.selected_hunk_index) {
                    if let Some(line) = hunk.lines.get_mut(self.selected_hunk_line_index) {
                        line.is_selected = !line.is_selected;
                        self.status_message = format!(
                            "Line {} in hunk {} selection toggled",
                            self.selected_hunk_line_index, self.selected_hunk_index
                        );
                    }
                }
            }
            KeyCode::Char('s') => {
                // Stage selected lines/hunks
                self.stage_selected_hunks_or_lines();
            }
            KeyCode::Char('u') => {
                // Unstage selected lines/hunks
                self.unstage_selected_hunks_or_lines();
            }
            _ => {}
        }
        false
    }

    pub(crate) fn handle_search_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.clear_search();
                self.view_mode = ViewMode::List; // Exit search mode
                return false;
            }
            KeyCode::Enter => {
                // In search mode, Enter key doesn't perform search; it exits.
                self.view_mode = ViewMode::List; // Exit search mode
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

    pub(crate) fn handle_branch_search_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.clear_branch_search();
                self.view_mode = ViewMode::List; // Exit branch search mode
                return false;
            }
            KeyCode::Enter => {
                // In search mode, Enter key doesn't perform search; it exits.
                self.view_mode = ViewMode::List; // Exit branch search mode
                return false;
            }
            KeyCode::Backspace => {
                self.branch_search_query.pop();
                self.filter_branches();
            }
            KeyCode::Char(c) => {
                if c.is_ascii_alphanumeric()
                    || c == '-'
                    || c == '_'
                    || c == ' '
                    || c == '.'
                    || c == '/'
                {
                    self.branch_search_query.push(c);
                    self.filter_branches();
                }
            }
            _ => {}
        }
        false
    }

    pub(crate) fn handle_details_key(&mut self, key: KeyEvent) -> bool {
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

    pub(crate) fn handle_input_key(&mut self, key: KeyEvent) -> bool {
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

    pub(crate) fn handle_diff_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.view_mode = ViewMode::List,
            KeyCode::Char('?') => self.view_mode = ViewMode::Help,
            KeyCode::Char('i') => {
                if self.active_panel == PanelType::Files && !self.hunks.is_empty() {
                    self.is_hunk_staging_mode = true;
                    self.view_mode = ViewMode::HunkStaging;
                    self.status_message = "Hunk staging mode (j/k to navigate, Space to select, s to stage, u to unstage)".to_string();
                } else {
                    self.status_message =
                        "Select a file with diffs to enter hunk staging mode".to_string();
                }
            }
            _ => {}
        }
        false
    }

    pub(crate) fn handle_help_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?') => {
                self.view_mode = ViewMode::List;
            }
            _ => {}
        }
        false
    }

    pub(crate) fn move_down(&mut self) {
        if self.selected_index < self.commits.len().saturating_sub(1) {
            self.selected_index += 1;
            if self.selected_index >= self.scroll_offset + 20 {
                self.scroll_offset = self.selected_index - 20 + 1;
            }
        }
    }

    pub(crate) fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index = self.selected_index.saturating_sub(1);
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index.saturating_sub(1);
            }
        }
    }

    pub(crate) fn page_down(&mut self) {
        let max_index = self.commits.len().saturating_sub(1);
        self.selected_index = (self.selected_index + 20).min(max_index);
        self.scroll_offset = (self.scroll_offset + 20).min(max_index.saturating_sub(20));
    }

    pub(crate) fn page_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(20);
        self.scroll_offset = self.scroll_offset.saturating_sub(20);
    }

    pub(crate) fn go_to_start(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub(crate) fn go_to_end(&mut self) {
        if !self.commits.is_empty() {
            self.selected_index = self.commits.len() - 1;
            self.scroll_offset = self.commits.len().saturating_sub(20);
        }
    }

    pub(crate) fn handle_filter_key(&mut self, key: KeyEvent) -> bool {
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

    pub(crate) fn handle_stats_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                self.view_mode = ViewMode::List;
            }
            _ => {}
        }
        false
    }

    pub(crate) fn handle_command_palette_key(&mut self, key: KeyEvent) -> bool {
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

    pub(crate) fn handle_stash_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.view_mode = ViewMode::List;
                self.stash_diff_content.clear();
                return false;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.selected_stash_index < self.stashes.len().saturating_sub(1) {
                    self.selected_stash_index += 1;
                    if self.selected_stash_index >= self.stash_scroll_offset + 10 {
                        self.stash_scroll_offset = self.selected_stash_index - 10 + 1;
                    }
                    if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                        let stash_name = stash.name.clone();
                        self.fetch_stash_diff(&stash_name);
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected_stash_index > 0 {
                    self.selected_stash_index = self.selected_stash_index.saturating_sub(1);
                    if self.selected_stash_index < self.stash_scroll_offset {
                        self.stash_scroll_offset = self.selected_stash_index.saturating_sub(1);
                    }
                    if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                        let stash_name = stash.name.clone();
                        self.fetch_stash_diff(&stash_name);
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                    let stash_name = stash.name.clone();
                    self.fetch_stash_diff(&stash_name);
                }
            }
            KeyCode::Char('a') => {
                if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                    let stash_name = stash.name.clone();
                    self.apply_stash(Some(&stash_name));
                }
            }
            KeyCode::Char('d') => {
                if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                    let stash_name = stash.name.clone();
                    self.drop_stash(Some(&stash_name));
                }
            }
            KeyCode::Char('p') => {
                if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                    let stash_name = stash.name.clone();
                    self.pop_stash(Some(&stash_name));
                }
            }
            _ => {}
        }
        false
    }

    pub(crate) fn execute_command(&mut self, action: &str) {
        match action {
            "toggle_sidebar" => self.toggle_sidebar(),
            "next_panel" => self.next_panel(),
            "prev_panel" => self.prev_panel(),
            "move_up" => match self.active_panel {
                PanelType::Commits => self.move_up(),
                PanelType::Files => self.move_file_selection_up(),
                PanelType::Branches => {
                    if self.selected_branch_index > 0 {
                        self.selected_branch_index -= 1;
                    }
                }
                PanelType::Stash => {
                    let _ = self.handle_stash_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
                }
            },
            "move_down" => match self.active_panel {
                PanelType::Commits => self.move_down(),
                PanelType::Files => self.move_file_selection_down(),
                PanelType::Branches => {
                    if self.selected_branch_index < self.branches.len().saturating_sub(1) {
                        self.selected_branch_index += 1;
                    }
                }
                PanelType::Stash => {
                    let _ = self.handle_stash_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
                }
            },
            "go_to_start" => self.go_to_start(),
            "go_to_end" => self.go_to_end(),
            "view_details" => self.view_mode = ViewMode::Details,
            "search" => {
                self.is_searching = true;
                self.search_query.clear();
                self.view_mode = ViewMode::Search;
            }
            "toggle_stage" => self.toggle_file_stage(),
            "stage_all" => self.stage_all_files(),
            "unstage_all" => self.unstage_all_files(),
            "amend" => self.amend_commit(),
            "drop" => self.drop_commit(),
            "squash" => self.squash_commits(),
            "cherry_pick" => self.cherry_pick_commit(),
            "revert" => self.revert_commit(),
            "toggle_theme" => self.theme.next(),
            "toggle_mouse" => self.toggle_mouse_mode(),
            "help" => self.view_mode = ViewMode::Help,
            "quit" => {} // This would typically return a signal to quit the app
            "command_palette" => self.open_command_palette(),
            "view_stashes" => {
                self.refresh_stashes();
                self.view_mode = ViewMode::Stash;
            }
            "apply_stash" => {
                if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                    let stash_name = stash.name.clone();
                    self.apply_stash(Some(&stash_name));
                }
            }
            "drop_stash" => {
                if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                    let stash_name = stash.name.clone();
                    self.drop_stash(Some(&stash_name));
                }
            }
            "pop_stash" => {
                if let Some(stash) = self.stashes.get(self.selected_stash_index) {
                    let stash_name = stash.name.clone();
                    self.pop_stash(Some(&stash_name));
                }
            }
            _ => {
                self.status_message = format!("Unknown command: {}", action);
            }
        }
    }
}
