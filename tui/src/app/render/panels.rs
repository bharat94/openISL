//! Panel rendering
//!
//! Contains rendering logic for the command palette, stash view, hunk
//! staging view, and help/filter/stats overlays.
use super::super::*;
use super::render_footer;

pub(crate) fn render_command_palette(app: &App, frame: &mut ratatui::Frame) {
    // Determine the size of the command palette overlay
    let size = frame.size();
    let width = size.width.saturating_sub(4);
    let height = size.height.saturating_sub(4);
    let area = Rect::new(2, 2, width, height);

    let block = Block::default()
        .title("Command Palette")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(app.theme.command_palette_border))
        .style(Style::default().bg(app.theme.command_palette_bg));
    frame.render_widget(Clear, area); // This clears out the background
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .margin(1)
        .split(area);

    // Render input
    let input_text = format!("/{}", app.command_palette_input);
    let input_widget = Paragraph::new(input_text)
        .style(Style::default().fg(app.theme.command_palette_input_fg))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(app.theme.command_palette_input_border)),
        );
    frame.render_widget(input_widget, chunks[0]);

    // Render results
    let items: Vec<ListItem<'_>> = app
        .command_palette_results
        .iter()
        .map(|cmd| {
            let keys_str = if cmd.keys.is_empty() {
                String::new()
            } else {
                format!(" ({})", cmd.keys.join(", "))
            };
            let content = format!("{} - {}{}", cmd.name, cmd.description, keys_str);
            ListItem::new(Line::from(content))
                .style(Style::default().fg(app.theme.command_palette_item_fg))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default())
        .highlight_style(
            Style::default()
                .bg(app.theme.command_palette_selected_bg)
                .fg(app.theme.command_palette_selected_fg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    frame.render_widget(list, chunks[1]);
}

pub(crate) fn render_branch_search_input(app: &App, area: Rect, frame: &mut ratatui::Frame) {
    let input_line = format!("/ {}", app.branch_search_query);
    let input_widget = Paragraph::new(input_line)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .title("Branch Search")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        );
    input_widget.render(area, frame.buffer_mut());
}

pub(crate) fn render_stash_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());

    // Render stash list
    let items: Vec<ListItem<'_>> = app
        .stashes
        .iter()
        .enumerate()
        .map(|(i, stash)| {
            let content = format!("[{}] {} ({})", i, stash.message, stash.name);
            let is_selected = i == app.selected_stash_index;
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.selected)
                    .bg(app.theme.selected_bg)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(Line::from(content)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Stashes")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(app.theme.border)),
        )
        .highlight_style(
            Style::default()
                .bg(app.theme.selected_bg)
                .fg(app.theme.selected)
                .add_modifier(Modifier::BOLD),
        );

    // Placeholder for actual diff rendering
    let diff_text = Paragraph::new(app.stash_diff_content.clone())
        .block(
            Block::default()
                .title("Stash Diff")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(app.theme.border)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(list, chunks[0]);
    frame.render_widget(diff_text, chunks[1]);

    render_footer(app, frame.size(), frame);
}

pub(crate) fn render_hunk_staging_view(app: &App, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let file_name = if let Some(file) = app.files.get(app.selected_file_index) {
        file.path.clone()
    } else {
        "No file selected".to_string()
    };

    let title = Paragraph::new(format!("Hunk Staging: {}", file_name))
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    let mut lines: Vec<Line> = Vec::new();
    for (hunk_idx, hunk) in app.hunks.iter().enumerate() {
        let hunk_header = format!("Hunk {}/{}", hunk_idx + 1, app.hunks.len());
        lines.push(
            Line::from(hunk_header).style(
                Style::default()
                    .fg(app.theme.hunk_header)
                    .add_modifier(Modifier::BOLD),
            ),
        );

        for (line_idx, line) in hunk.lines.iter().enumerate() {
            let mut style = Style::default();
            let prefix = match line.line_type {
                HunkLineType::Addition => {
                    // Corrected variant name
                    style = style.fg(app.theme.diff_added);
                    "+"
                }
                HunkLineType::Deletion => {
                    // Corrected variant name
                    style = style.fg(app.theme.diff_removed);
                    "-"
                }
                HunkLineType::Context => {
                    style = style.fg(app.theme.diff_context);
                    " "
                }
            };

            // Highlight selected line for navigation
            if hunk_idx == app.selected_hunk_index && line_idx == app.selected_hunk_line_index {
                style = style.bg(app.theme.selected_bg).add_modifier(Modifier::BOLD);
            }

            // Indicate selected lines for staging/unstaging
            let selection_indicator = if line.is_selected { "*" } else { " " };

            lines.push(Line::from(vec![
                Span::styled(format!("{} {}", selection_indicator, prefix), style),
                Span::styled(line.content.clone(), style),
            ]));
        }
    }

    let hunk_list = Paragraph::new(lines)
        .block(
            Block::default()
                .title("Hunks")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(app.theme.border)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(hunk_list, chunks[1]);

    let help_text = format!(
        "j/k: Navigate lines | Space: Toggle selection | s: Stage selected | u: Unstage selected | Esc: Exit | Theme: {}",
        app.theme.name()
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(app.theme.help))
        .alignment(Alignment::Center);
    help_widget.render(chunks[2], frame.buffer_mut());
}

pub(crate) fn render_help_overlay(app: &App, frame: &mut ratatui::Frame) {
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

pub(crate) fn render_filter_view(app: &App, frame: &mut ratatui::Frame) {
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

pub(crate) fn render_stats_view(app: &App, frame: &mut ratatui::Frame) {
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
