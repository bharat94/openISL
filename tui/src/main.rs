use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, Key, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Terminal,
};
use std::io::stdout;
use std::path::Path;
use openisl_git::{get_commits, Commit};

struct App {
    commits: Vec<Commit>,
    selected_index: usize,
    scroll_offset: usize,
    show_help: bool,
    current_branch: String,
}

impl App {
    fn new(commits: Vec<Commit>, current_branch: String) -> Self {
        Self {
            commits,
            selected_index: 0,
            scroll_offset: 0,
            show_help: false,
            current_branch,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key {
            Key {
                code: Key::Char('q'),
                ..
            } => return true,

            Key {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }
            | Key {
                code: Key::Char('c'),
                ..
            } => return true,

            Key {
                code: Key::Char('j'),
                ..
            }
            | Key {
                code: Key::Down,
                ..
            } => self.move_down(),

            Key {
                code: Key::Char('k'),
                ..
            }
            | Key {
                code: Key::Up,
                ..
            } => self.move_up(),

            Key {
                code: Key::PageDown,
                ..
            } => self.page_down(),

            Key {
                code: Key::PageUp,
                ..
            } => self.page_up(),

            Key {
                code: Key::Home,
                ..
            } => self.go_to_start(),

            Key {
                code: Key::End,
                ..
            } => self.go_to_end(),

            Key {
                code: Key::Char('?'),
                ..
            } => self.show_help = !self.show_help,

            _ => {}
        }
        false
    }

    fn move_down(&mut self) {
        if self.selected_index < self.commits.len().saturating_sub(1) {
            self.selected_index += 1;
            if self.selected_index >= self.scroll_offset + 20 {
                self.scroll_offset = self.selected_index - 20 + 1;
            }
        }
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index = self.selected_index.saturating_sub(1);
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index.saturating_sub(1);
            }
        }
    }

    fn page_down(&mut self) {
        let max_index = self.commits.len().saturating_sub(1);
        self.selected_index = (self.selected_index + 20).min(max_index);
        self.scroll_offset = (self.scroll_offset + 20).min(max_index.saturating_sub(20));
    }

    fn page_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(20);
        self.scroll_offset = self.scroll_offset.saturating_sub(20);
    }

    fn go_to_start(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    fn go_to_end(&mut self) {
        if !self.commits.is_empty() {
            self.selected_index = self.commits.len() - 1;
            self.scroll_offset = self.commits.len().saturating_sub(20);
        }
    }

    fn visible_commits(&self) -> &[Commit] {
        let end = (self.scroll_offset + 20).min(self.commits.len());
        &self.commits[self.scroll_offset..end]
    }
}

fn render(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let title = Paragraph::new("openisl log")
        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let commit_list: Vec<Text> = app
        .visible_commits()
        .iter()
        .enumerate()
        .map(|(i, commit)| {
            let global_index = app.scroll_offset + i;
            let is_selected = global_index == app.selected_index;

            let branch_info = if !commit.refs.is_empty() {
                let names: Vec<&str> = commit.refs.iter().map(|r| r.name.as_str()).collect();
                format!(" [{}]", names.join(", "))
            } else {
                String::new()
            };

            let line = if is_selected {
                format!(
                    "> {} - {}{}",
                    commit.short_hash.yellow(),
                    commit.summary.white().bold(),
                    branch_info.green()
                )
            } else {
                format!(
                    "  {} - {}{}",
                    commit.short_hash.dim(),
                    commit.summary.white(),
                    branch_info.green()
                )
            };

            Text::from(line)
        })
        .collect();

    let commit_widget = Paragraph::new(commit_list)
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
                .style(Style::default().fg(Color::White)),
        )
        .scroll((app.scroll_offset as u16, 0));
    commit_widget.render(chunks[1], frame.buffer_mut());

    let help_text = if app.show_help {
        Text::from(
            "Navigation: j/k or ↑/↓  Move | PageUp/PageDown  Jump | Home/End  Bounds | ?  Toggle Help | q  Quit",
        )
    } else {
        Text::from("Press ? for help | q to quit")
    };
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    help_widget.render(chunks[2], frame.buffer_mut());
}

fn run_tui(commits: Vec<Commit>, current_branch: String) -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(event::EnableMouseCapture)?;

    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(commits, current_branch);

    loop {
        terminal.draw(|frame| render(&app, frame))?;

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
    stdout.execute(event::DisableMouseCapture)?;

    Ok(())
}

fn main() -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let commits = get_commits(&repo_path, Some(100))?;
    let current_branch = "main".to_string();

    run_tui(commits, current_branch)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_commit(hash: &str, summary: &str) -> Commit {
        Commit {
            hash: hash.to_string(),
            short_hash: hash[..7].to_string(),
            message: summary.to_string(),
            summary: summary.to_string(),
            author: "Test".to_string(),
            email: "test@example.com".to_string(),
            date: Utc::now(),
            parent_hashes: vec![],
            refs: vec![],
        }
    }

    #[test]
    fn test_app_new() {
        let commits = vec![
            create_test_commit("abc123def456789", "First commit"),
            create_test_commit("def456ghi789abc", "Second commit"),
        ];
        let app = App::new(commits, "main".to_string());

        assert_eq!(app.commits.len(), 2);
        assert_eq!(app.selected_index, 0);
        assert_eq!(app.scroll_offset, 0);
        assert!(!app.show_help);
        assert_eq!(app.current_branch, "main");
    }

    #[test]
    fn test_app_move_down() {
        let commits = vec![
            create_test_commit("abc123def456789", "First commit"),
            create_test_commit("def456ghi789abc", "Second commit"),
            create_test_commit("ghi789jkl012345", "Third commit"),
        ];
        let mut app = App::new(commits, "main".to_string());

        assert_eq!(app.selected_index, 0);
        app.move_down();
        assert_eq!(app.selected_index, 1);
        app.move_down();
        assert_eq!(app.selected_index, 2);
        app.move_down();
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn test_app_move_up() {
        let commits = vec![
            create_test_commit("abc123def456789", "First commit"),
            create_test_commit("def456ghi789abc", "Second commit"),
        ];
        let mut app = App::new(commits, "main".to_string());
        app.selected_index = 1;

        app.move_up();
        assert_eq!(app.selected_index, 0);
        app.move_up();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_app_go_to_start() {
        let commits = (0..25).map(|i| create_test_commit(&format!("{:x}", i), &format!("Commit {}", i))).collect();
        let mut app = App::new(commits, "main".to_string());
        app.selected_index = 20;
        app.scroll_offset = 15;

        app.go_to_start();
        assert_eq!(app.selected_index, 0);
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_app_go_to_end() {
        let commits = (0..25).map(|i| create_test_commit(&format!("{:x}", i), &format!("Commit {}", i))).collect();
        let mut app = App::new(commits, "main".to_string());

        app.go_to_end();
        assert_eq!(app.selected_index, 24);
    }

    #[test]
    fn test_app_toggle_help() {
        let commits = vec![create_test_commit("abc123def456789", "First commit")];
        let mut app = App::new(commits, "main".to_string());

        assert!(!app.show_help);
        app.show_help = !app.show_help;
        assert!(app.show_help);
    }

    #[test]
    fn test_app_handle_key_quit() {
        let commits = vec![create_test_commit("abc123def456789", "First commit")];
        let mut app = App::new(commits, "main".to_string());

        let quit = app.handle_key(Key::new('q'));
        assert!(quit);
    }

    #[test]
    fn test_app_visible_commits() {
        let commits = (0..50).map(|i| create_test_commit(&format!("{:x}", i), &format!("Commit {}", i))).collect();
        let mut app = App::new(commits, "main".to_string());

        app.scroll_offset = 0;
        assert_eq!(app.visible_commits().len(), 20);

        app.scroll_offset = 30;
        assert_eq!(app.visible_commits().len(), 20);

        app.scroll_offset = 40;
        assert_eq!(app.visible_commits().len(), 10);
    }
}
