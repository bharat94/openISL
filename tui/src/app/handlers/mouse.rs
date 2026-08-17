//! Mouse event handlers
//!
//! Contains mouse event dispatch and click/scroll handling logic.
use super::super::*;

impl App {
    pub(crate) fn handle_mouse(&mut self, event: MouseEvent) -> bool {
        if !self.mouse_enabled {
            return false;
        }
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.handle_left_click(event);
            }
            MouseEventKind::Down(MouseButton::Right) => {
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
        if self.view_mode == ViewMode::List {
            self.view_mode = ViewMode::Details;
            self.status_message = "Double-click: viewing details".to_string();
        }
    }

    pub(crate) fn mouse_scroll_down(&mut self) {
        let max_index = self.commits.len().saturating_sub(1);
        let items_per_page = 15;

        if self.selected_index < max_index {
            self.selected_index += 1;
            if self.selected_index >= self.scroll_offset + items_per_page {
                self.scroll_offset = self.selected_index.saturating_sub(items_per_page - 1);
            }
        }
    }

    pub(crate) fn mouse_scroll_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index = self.selected_index.saturating_sub(1);
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index.saturating_sub(1);
            }
        }
    }

    pub(crate) fn toggle_mouse_mode(&mut self) {
        self.mouse_enabled = !self.mouse_enabled;
        if self.mouse_enabled {
            let _ = execute!(stdout(), EnableMouseCapture);
            self.status_message = "Mouse mode: ON (click and scroll enabled)".to_string();
        } else {
            let _ = execute!(stdout(), DisableMouseCapture);
            self.status_message = "Mouse mode: OFF (use keyboard navigation)".to_string();
        }
    }
}
