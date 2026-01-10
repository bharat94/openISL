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
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn toggle(&mut self) {
        if self.name == "dark" {
            *self = Theme::light();
        } else {
            *self = Theme::dark();
        }
    }
}
