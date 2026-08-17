//! Status bar rendering
//!
//! Contains rendering logic for the footer status bar including keyboard
//! shortcuts, sync status, loading indicators, and the spinner.
use super::super::*;

fn get_spinner_char() -> char {
    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let index =
        (std::time::Instant::now().elapsed().as_millis() / 100) as usize % spinner_chars.len();
    spinner_chars[index]
}

pub(crate) fn render_footer(app: &App, area: Rect, frame: &mut ratatui::Frame) {
    let loading_text = if app.is_loading {
        format!(" {} Loading...", get_spinner_char())
    } else {
        String::new()
    };

    let sync_text = if app.is_loading {
        String::new() // Don't show sync info if loading
    } else {
        match (&app.repo_ahead, &app.repo_behind, &app.has_conflicts) {
            (Some(ahead), Some(behind), false) => format!("↑{} ↓{}", ahead, behind),
            (Some(ahead), None, false) => format!("↑{}", ahead),
            (None, Some(behind), false) => format!("↓{}", behind),
            (_, _, true) => "!".to_string(),
            _ => String::new(),
        }
    };

    let sync_prefix = if !sync_text.is_empty() { "Sync: " } else { "" };
    let sync_display = format!("{}{}", sync_prefix, sync_text);

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

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if !loading_text.is_empty() {
                20
            } else if sync_text.is_empty() {
                0
            } else {
                20
            }),
            Constraint::Min(0),
        ])
        .split(area);

    if !loading_text.is_empty() {
        let loading_widget = Paragraph::new(loading_text)
            .style(
                Style::default()
                    .fg(app.theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);
        loading_widget.render(chunks[0], frame.buffer_mut());
    } else if !sync_text.is_empty() {
        let sync_widget = Paragraph::new(sync_display)
            .style(
                Style::default()
                    .fg(if app.has_conflicts {
                        Color::Red
                    } else {
                        app.theme.help
                    })
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);
        sync_widget.render(chunks[0], frame.buffer_mut());
    }

    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[1], frame.buffer_mut());
}
