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
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            name: "dark",
            background: Color::Reset,
            text: Color::Gray,
            title: Color::Cyan,
            border: Color::White,
            help: Color::DarkGray,
            selected: Color::White,
            selected_bg: Color::DarkGray,
            accent: Color::Magenta,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            addition: Color::Green,
            deletion: Color::Red,
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
