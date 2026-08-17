//! Diff rendering
//!
//! Contains rendering logic for the diff, input, and search views.
use super::super::*;

pub(crate) fn render_diff_view(app: &App, frame: &mut ratatui::Frame) {
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

pub(crate) fn render_input_view(app: &App, frame: &mut ratatui::Frame) {
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

pub(crate) fn render_search_view(app: &App, frame: &mut ratatui::Frame) {
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
