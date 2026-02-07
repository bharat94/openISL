use ratatui::style::Color;

#[derive(Clone, Copy, Debug, Default)]
#[allow(dead_code)]
pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub text: Color,
    pub title: Color,
    pub border: Color,
    pub help: Color,
    pub selected: Color,
    pub selected_bg: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub addition: Color,
    pub deletion: Color,
    pub panel_title_active_bg: Color,
    pub panel_title_inactive_bg: Color,
    pub panel_border_active: Color,
    pub panel_border_inactive: Color,
    pub commit_hash: Color,
    pub commit_date: Color,
    pub commit_author: Color,
    pub branch_name: Color,
    pub file_status_added: Color,
    pub file_status_modified: Color,
    pub file_status_deleted: Color,
    pub file_status_untracked: Color,
    pub search_match_fg: Color,
    pub search_match_bg: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            name: "dark",
            background: Color::Reset,
            text: Color::Rgb(200, 200, 200),
            title: Color::Rgb(0, 191, 255),
            border: Color::Rgb(255, 215, 0),
            help: Color::Rgb(100, 100, 100),
            selected: Color::Rgb(255, 255, 255),
            selected_bg: Color::Rgb(70, 70, 100),
            accent: Color::Rgb(255, 0, 128),
            success: Color::Rgb(0, 255, 127),
            warning: Color::Rgb(255, 215, 0),
            error: Color::Rgb(255, 69, 0),
            addition: Color::Rgb(0, 255, 127),
            deletion: Color::Rgb(255, 69, 0),
            panel_title_active_bg: Color::Rgb(70, 70, 100),
            panel_title_inactive_bg: Color::Reset,
            panel_border_active: Color::Rgb(0, 191, 255),
            panel_border_inactive: Color::Rgb(255, 215, 0),
            commit_hash: Color::Rgb(170, 170, 170),
            commit_date: Color::Rgb(150, 150, 150),
            commit_author: Color::Rgb(120, 120, 255),
            branch_name: Color::Rgb(0, 255, 127),
            file_status_added: Color::Rgb(0, 255, 127),
            file_status_modified: Color::Rgb(255, 215, 0),
            file_status_deleted: Color::Rgb(255, 69, 0),
            file_status_untracked: Color::Rgb(255, 165, 0),
            search_match_fg: Color::Black,
            search_match_bg: Color::Yellow,
        }
    }

    pub fn light() -> Self {
        Theme {
            name: "light",
            background: Color::Reset,
            text: Color::DarkGray,
            title: Color::Blue,
            border: Color::Black,
            help: Color::Gray,
            selected: Color::Black,
            selected_bg: Color::Gray,
            accent: Color::Magenta,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            addition: Color::Green,
            deletion: Color::Red,
            panel_title_active_bg: Color::Gray,
            panel_title_inactive_bg: Color::Reset,
            panel_border_active: Color::Blue,
            panel_border_inactive: Color::Black,
            commit_hash: Color::DarkGray,
            commit_date: Color::Gray,
            commit_author: Color::Blue,
            branch_name: Color::Green,
            file_status_added: Color::Green,
            file_status_modified: Color::Yellow,
            file_status_deleted: Color::Red,
            file_status_untracked: Color::LightYellow,
            search_match_fg: Color::Black,
            search_match_bg: Color::LightYellow,
        }
    }

    pub fn monokai() -> Self {
        Theme {
            name: "monokai",
            background: Color::Reset,
            text: Color::Rgb(248, 248, 248),
            title: Color::Rgb(255, 209, 102),
            border: Color::Rgb(248, 248, 248),
            help: Color::Rgb(128, 128, 128),
            selected: Color::Rgb(248, 248, 248),
            selected_bg: Color::Rgb(78, 74, 103),
            accent: Color::Rgb(189, 147, 249),
            success: Color::Rgb(166, 227, 161),
            warning: Color::Rgb(249, 226, 175),
            error: Color::Rgb(243, 139, 168),
            addition: Color::Rgb(166, 227, 161),
            deletion: Color::Rgb(243, 139, 168),
            panel_title_active_bg: Color::Rgb(78, 74, 103),
            panel_title_inactive_bg: Color::Reset,
            panel_border_active: Color::Rgb(255, 209, 102),
            panel_border_inactive: Color::Rgb(248, 248, 248),
            commit_hash: Color::Rgb(170, 170, 170),
            commit_date: Color::Rgb(150, 150, 150),
            commit_author: Color::Rgb(189, 147, 249),
            branch_name: Color::Rgb(166, 227, 161),
            file_status_added: Color::Rgb(166, 227, 161),
            file_status_modified: Color::Rgb(249, 226, 175),
            file_status_deleted: Color::Rgb(243, 139, 168),
            file_status_untracked: Color::Rgb(255, 165, 0),
            search_match_fg: Color::Black,
            search_match_bg: Color::Rgb(249, 226, 175),
        }
    }

    pub fn nord() -> Self {
        Theme {
            name: "nord",
            background: Color::Reset,
            text: Color::Rgb(216, 222, 233),
            title: Color::Rgb(136, 192, 208),
            border: Color::Rgb(236, 239, 244),
            help: Color::Rgb(129, 161, 193),
            selected: Color::Rgb(236, 239, 244),
            selected_bg: Color::Rgb(76, 86, 106),
            accent: Color::Rgb(129, 161, 193),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            error: Color::Rgb(191, 97, 106),
            addition: Color::Rgb(163, 190, 140),
            deletion: Color::Rgb(191, 97, 106),
            panel_title_active_bg: Color::Rgb(76, 86, 106),
            panel_title_inactive_bg: Color::Reset,
            panel_border_active: Color::Rgb(136, 192, 208),
            panel_border_inactive: Color::Rgb(236, 239, 244),
            commit_hash: Color::Rgb(170, 170, 170),
            commit_date: Color::Rgb(150, 150, 150),
            commit_author: Color::Rgb(129, 161, 193),
            branch_name: Color::Rgb(163, 190, 140),
            file_status_added: Color::Rgb(163, 190, 140),
            file_status_modified: Color::Rgb(235, 203, 139),
            file_status_deleted: Color::Rgb(191, 97, 106),
            file_status_untracked: Color::Rgb(235, 203, 139),
            search_match_fg: Color::Black,
            search_match_bg: Color::Rgb(235, 203, 139),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn next(&mut self) {
        match self.name {
            "dark" => *self = Theme::light(),
            "light" => *self = Theme::monokai(),
            "monokai" => *self = Theme::nord(),
            "nord" => *self = Theme::dark(),
            _ => *self = Theme::dark(),
        }
    }

    pub fn set(&mut self, name: &str) {
        *self = match name {
            "dark" => Theme::dark(),
            "light" => Theme::light(),
            "monokai" => Theme::monokai(),
            "nord" => Theme::nord(),
            _ => Theme::dark(),
        };
    }
}
