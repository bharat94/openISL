//! Commit list rendering
//!
//! Contains rendering logic for the commit list, sidebar, file and branch
//! panels, and the commit details view.
use super::super::*;
use super::render_footer;

pub(crate) fn render_list_view(app: &App, frame: &mut ratatui::Frame) {
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

pub(crate) fn render_sidebar(app: &App, area: Rect, frame: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3), // Added for Stash tab
            Constraint::Min(10),
        ])
        .split(area);

    render_panel_tab(app, PanelType::Files, " FILES ", chunks[0], frame);
    render_panel_tab(app, PanelType::Branches, " BRANCHES ", chunks[1], frame);
    render_panel_tab(app, PanelType::Stash, " STASH ", chunks[2], frame); // Added

    match app.active_panel {
        PanelType::Files => render_files_panel(app).render(chunks[3], frame.buffer_mut()),
        PanelType::Branches => {
            if app.view_mode == ViewMode::BranchSearch {
                let branch_search_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // For the search input
                        Constraint::Min(10),   // For the filtered branches list
                    ])
                    .split(chunks[3]);
                render_branch_search_input(app, branch_search_chunks[0], frame);
                render_branches_panel(app).render(branch_search_chunks[1], frame.buffer_mut());
            } else {
                render_branches_panel(app).render(chunks[3], frame.buffer_mut())
            }
        }
        PanelType::Commits => {
            render_commits_panel(app, chunks[3]).render(chunks[3], frame.buffer_mut())
        }
        _ => {} // Handles PanelType::Stash, as its content rendering is done in render_stash_view
    }
}

pub(crate) fn render_panel_tab(
    app: &App,
    panel_type: PanelType,
    title: &str,
    area: Rect,
    frame: &mut ratatui::Frame,
) {
    let is_active = app.active_panel == panel_type;
    let (style, border_type, border_color) = if is_active {
        (
            Style::default()
                .fg(app.theme.selected)
                .bg(app.theme.panel_title_active_bg)
                .add_modifier(Modifier::BOLD),
            BorderType::Double,
            app.theme.panel_border_active,
        )
    } else {
        (
            Style::default()
                .fg(app.theme.text)
                .bg(app.theme.panel_title_inactive_bg),
            BorderType::Plain,
            app.theme.panel_border_inactive,
        )
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(border_color))
        .style(style);

    block.render(area, frame.buffer_mut());
}

pub(crate) fn render_files_panel(app: &App) -> impl Widget + '_ {
    let items: Vec<ListItem<'_>> = app
        .files
        .iter()
        .map(|file| {
            let status_char = match file.status {
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
            let status_color = match file.status {
                openisl_git::StatusType::Modified => app.theme.file_status_modified,
                openisl_git::StatusType::Added => app.theme.file_status_added,
                openisl_git::StatusType::Deleted => app.theme.file_status_deleted,
                openisl_git::StatusType::Untracked => app.theme.file_status_untracked,
                openisl_git::StatusType::ModifiedStaged => app.theme.file_status_modified, // Staged modified
                openisl_git::StatusType::AddedStaged => app.theme.file_status_added, // Staged added
                openisl_git::StatusType::DeletedStaged => app.theme.file_status_deleted, // Staged deleted
                openisl_git::StatusType::Renamed => app.theme.accent, // Renamed files
                openisl_git::StatusType::Conflicted => app.theme.error, // Conflicted files
            };

            let content = format!("{} {}", status_char, file.path);
            let is_selected = app.selected_file_index
                == app
                    .files
                    .iter()
                    .position(|f| f.path == file.path)
                    .unwrap_or(0);
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.selected)
                    .bg(app.theme.selected_bg)
            } else {
                Style::default().fg(status_color)
            };
            ListItem::new(Line::from(content)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!("Files ({})", app.files.len()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(app.theme.panel_border_inactive)),
    );

    list
}

pub(crate) fn render_branches_panel(app: &App) -> impl Widget + '_ {
    let items: Vec<ListItem<'_>> = app
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
                Style::default()
                    .fg(app.theme.selected)
                    .bg(app.theme.selected_bg)
            } else {
                Style::default().fg(app.theme.branch_name)
            };
            ListItem::new(Line::from(content)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!("Branches ({})", app.branches.len()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(app.theme.panel_border_inactive)),
    );

    list
}

pub(crate) fn render_commits_panel(app: &App, area: Rect) -> impl Widget + '_ {
    let panel_height = area.height.saturating_sub(2) as usize;
    let visible_count = panel_height.max(1);
    let raw_lines = format_tree_lines(
        app.tree.nodes(),
        app.scroll_offset,
        visible_count,
        &app.theme,
    );

    let items: Vec<ListItem<'_>> = raw_lines // Corrected: assign to items
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let global_index = app.scroll_offset + i;
            let is_selected = global_index == app.selected_index;

            let styled_line = if is_selected {
                line.style(
                    Style::default()
                        .fg(app.theme.selected)
                        .add_modifier(Modifier::BOLD)
                        .bg(app.theme.selected_bg),
                )
            } else {
                line
            };
            ListItem::new(styled_line)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!(
                "Commits ({}/{})",
                app.selected_index + 1,
                app.commits.len(),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(app.theme.panel_border_inactive)),
    );

    list
}

pub(crate) fn render_main_content(app: &App, area: Rect, frame: &mut ratatui::Frame) {
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
    let raw_lines = format_tree_lines(
        app.tree.nodes(),
        app.scroll_offset,
        visible_count,
        &app.theme,
    );

    let lines: Vec<Line<'_>> = raw_lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let global_index = app.scroll_offset + i;
            let is_selected = global_index == app.selected_index;

            if is_selected {
                line.style(
                    Style::default()
                        .fg(app.theme.selected)
                        .add_modifier(Modifier::BOLD)
                        .bg(app.theme.selected_bg),
                )
            } else {
                line
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

pub(crate) fn render_details_view(app: &App, frame: &mut ratatui::Frame) {
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
