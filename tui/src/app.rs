use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Terminal,
};
use std::io::stdout;
use openisl_git::{Commit, get_commit_diff};
use crate::theme::Theme;
use crate::keybindings::KeyBindings;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    List,
    Details,
    Diff,
    Help,
    InputBranch,
}

pub struct App {
    pub commits: Vec<Commit>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub show_help: bool,
    pub current_branch: String,
    pub theme: Theme,
    pub view_mode: ViewMode,
    pub diff_content: String,
    pub status_message: String,
    pub branch_input: String,
    pub repo_path: Option<std::path::PathBuf>,
    pub keybindings: KeyBindings,
}

impl App {
    pub fn new(commits: Vec<Commit>, current_branch: String, repo_path: Option<std::path::PathBuf>) -> Self {
        Self {
            commits,
            selected_index: 0,
            scroll_offset: 0,
            show_help: false,
            current_branch,
            theme: Theme::dark(),
            view_mode: ViewMode::List,
            diff_content: String::new(),
            status_message: String::new(),
            branch_input: String::new(),
            repo_path,
            keybindings: KeyBindings::load().unwrap_or_default(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.view_mode {
            ViewMode::List => self.handle_list_key(key),
            ViewMode::Details => self.handle_details_key(key),
            ViewMode::Diff => self.handle_diff_key(key),
            ViewMode::Help => self.handle_help_key(key),
            ViewMode::InputBranch => self.handle_input_key(key),
        }
    }

    fn handle_list_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return true,
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::PageDown => self.page_down(),
            KeyCode::PageUp => self.page_up(),
            KeyCode::Home => self.go_to_start(),
            KeyCode::End => self.go_to_end(),
            KeyCode::Enter => self.view_mode = ViewMode::Details,
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
            KeyCode::Char('t') => self.theme.toggle(),
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
                self.status_message.clear();
                return false;
            }
            KeyCode::Enter => {
                if !self.branch_input.is_empty() {
                    if let Some(commit) = self.selected_commit() {
                        self.status_message = format!(
                            "Created branch '{}' from {}",
                            self.branch_input,
                            commit.short_hash
                        );
                    }
                    self.branch_input.clear();
                    self.view_mode = ViewMode::List;
                }
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

    pub fn visible_commits(&self) -> &[Commit] {
        let end = (self.scroll_offset + 20).min(self.commits.len());
        &self.commits[self.scroll_offset..end]
    }

    pub fn selected_commit(&self) -> Option<&Commit> {
        self.commits.get(self.selected_index)
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
                    Ok(diff) => self.diff_content = diff,
                    Err(e) => self.diff_content = format!("Error fetching diff: {}", e),
                }
            } else {
                self.diff_content = "No repository path available".to_string();
            }
        }
    }
}

pub fn run_tui(commits: Vec<Commit>, current_branch: String, repo_path: Option<std::path::PathBuf>) -> Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;

    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(commits, current_branch, repo_path);

    loop {
        terminal.draw(|frame| {
            match app.view_mode {
                ViewMode::List => render_list_view(&app, frame),
                ViewMode::Details => render_details_view(&app, frame),
                ViewMode::Diff => render_diff_view(&app, frame),
                ViewMode::Help => render_help_overlay(&app, frame),
                ViewMode::InputBranch => render_input_view(&app, frame),
            }
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                let should_quit = app.handle_key(key);
                if should_quit {
                    break;
                }
            }
        }
    }

    terminal.clear()?;
    disable_raw_mode()?;

    Ok(())
}

fn render_list_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("openisl log")
        .style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let commit_lines: Vec<String> = app
        .visible_commits()
        .iter()
        .enumerate()
        .map(|(i, commit)| {
            let global_index = app.scroll_offset + i;
            let is_selected = global_index == app.selected_index;
            let prefix = if is_selected { ">" } else { " " };
            format!("{} {} - {}", prefix, commit.short_hash, commit.summary)
        })
        .collect();

    let commit_widget = Paragraph::new(commit_lines.join("\n"))
        .style(Style::default().fg(app.theme.text))
        .block(
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

    let help_text = format!(
        "{}: Details | {}: Checkout | {}: New Branch | {}: Diff | {}: Help | {}: Theme | {}: Quit | Theme: {}",
        app.keybindings.actions.view_details,
        app.keybindings.actions.checkout,
        app.keybindings.actions.create_branch,
        app.keybindings.actions.view_diff,
        app.keybindings.actions.help,
        app.keybindings.actions.toggle_theme,
        app.keybindings.actions.quit,
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[3], frame.buffer_mut());
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
        .style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
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
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("Commit Diff")
        .style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let diff_text = if app.diff_content.is_empty() {
        "No diff available. Use 'openisl diff' command for staged/working changes."
    } else {
        &app.diff_content
    };

    let diff_widget = Paragraph::new(diff_text)
        .style(Style::default().fg(app.theme.text))
        .block(
            Block::default()
                .title("Diff View")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        );
    diff_widget.render(chunks[1], frame.buffer_mut());

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
    help_widget.render(chunks[2], frame.buffer_mut());
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
        .style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
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
    let input_display = Paragraph::new(format!("{} {}", app.branch_input, cursor))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
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
        .style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
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
        ]
    }

    #[test]
    fn test_app_navigation_down() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.selected_index, 0);
        app.move_down();
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_app_navigation_up() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        app.selected_index = 1;
        app.move_up();
        assert_eq!(app.selected_index, 0);
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
    fn test_checkout_key() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert!(app.status_message.is_empty());

        app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
        assert!(app.status_message.contains("Would checkout"));
    }

    #[test]
    fn test_theme_toggle() {
        let mut theme = Theme::dark();
        assert_eq!(theme.name(), "dark");

        theme.toggle();
        assert_eq!(theme.name(), "light");

        theme.toggle();
        assert_eq!(theme.name(), "dark");
    }

    #[test]
    fn test_theme_dark_colors() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "dark");
        assert_eq!(theme.title, Color::Cyan);
        assert_eq!(theme.text, Color::Gray);
    }

    #[test]
    fn test_theme_light_colors() {
        let theme = Theme::light();
        assert_eq!(theme.name, "light");
        assert_eq!(theme.title, Color::Blue);
        assert_eq!(theme.text, Color::DarkGray);
    }

    #[test]
    fn test_visible_commits() {
        let commits = create_test_commits();
        let app = App::new(commits, "main".to_string(), None);

        let visible = app.visible_commits();
        assert_eq!(visible.len(), 2);
    }

    #[test]
    fn test_selected_commit() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        assert_eq!(app.selected_commit().unwrap().short_hash, "abc123d");

        app.move_down();
        assert_eq!(app.selected_commit().unwrap().short_hash, "def456g");
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
    fn test_quit_from_list() {
        let commits = create_test_commits();
        let mut app = App::new(commits, "main".to_string(), None);

        let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let should_quit = app.handle_key(quit_event);
        assert!(should_quit);
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
}
