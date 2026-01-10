use serde::{Deserialize, Serialize};
use crossterm::event::KeyCode;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub navigation: NavigationBindings,
    pub actions: ActionBindings,
    pub views: ViewBindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationBindings {
    pub up: String,
    pub down: String,
    pub page_up: String,
    pub page_down: String,
    pub go_to_start: String,
    pub go_to_end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionBindings {
    pub quit: String,
    pub help: String,
    pub toggle_theme: String,
    pub checkout: String,
    pub create_branch: String,
    pub view_diff: String,
    pub view_details: String,
    pub cancel: String,
    pub confirm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBindings {
    pub next_view: String,
    pub prev_view: String,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            navigation: NavigationBindings::default(),
            actions: ActionBindings::default(),
            views: ViewBindings::default(),
        }
    }
}

impl Default for NavigationBindings {
    fn default() -> Self {
        NavigationBindings {
            up: "j/k/↑/↓".to_string(),
            down: "j/k/↑/↓".to_string(),
            page_up: "PageUp".to_string(),
            page_down: "PageDown".to_string(),
            go_to_start: "Home".to_string(),
            go_to_end: "End".to_string(),
        }
    }
}

impl Default for ActionBindings {
    fn default() -> Self {
        ActionBindings {
            quit: "q/Esc".to_string(),
            help: "?".to_string(),
            toggle_theme: "t".to_string(),
            checkout: "c".to_string(),
            create_branch: "b".to_string(),
            view_diff: "D".to_string(),
            view_details: "Enter".to_string(),
            cancel: "Esc".to_string(),
            confirm: "Enter".to_string(),
        }
    }
}

impl Default for ViewBindings {
    fn default() -> Self {
        ViewBindings {
            next_view: "]".to_string(),
            prev_view: "[".to_string(),
        }
    }
}

impl KeyBindings {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_config_path();

        if let Some(path) = config_path {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                return Ok(toml::from_str(&content)?);
            }
        }

        Ok(KeyBindings::default())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path()
            .unwrap_or_else(|| {
                let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
                path.push("openisl");
                path.push("keybindings.toml");
                path
            });

        std::fs::create_dir_all(config_path.parent().unwrap())?;
        let toml = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, toml)?;
        Ok(())
    }

    pub fn parse_key(&self, key_code: KeyCode, modifiers: crossterm::event::KeyModifiers) -> Option<&str> {
        let key_str = format_key_code(key_code, modifiers);

        if self.actions.quit.contains(&key_str) {
            Some("quit")
        } else if self.actions.help.contains(&key_str) {
            Some("help")
        } else if self.actions.toggle_theme.contains(&key_str) {
            Some("toggle_theme")
        } else if self.actions.checkout.contains(&key_str) {
            Some("checkout")
        } else if self.actions.create_branch.contains(&key_str) {
            Some("create_branch")
        } else if self.actions.view_diff.contains(&key_str) {
            Some("view_diff")
        } else if self.actions.view_details.contains(&key_str) {
            Some("view_details")
        } else if self.navigation.page_up.contains(&key_str) {
            Some("page_up")
        } else if self.navigation.page_down.contains(&key_str) {
            Some("page_down")
        } else if self.navigation.go_to_start.contains(&key_str) {
            Some("go_to_start")
        } else if self.navigation.go_to_end.contains(&key_str) {
            Some("go_to_end")
        } else if key_code == KeyCode::Char('j') || key_code == KeyCode::Down {
            Some("up")
        } else if key_code == KeyCode::Char('k') || key_code == KeyCode::Up {
            Some("down")
        } else {
            None
        }
    }
}

fn format_key_code(key_code: KeyCode, modifiers: crossterm::event::KeyModifiers) -> String {
    let modifier_str = if modifiers.contains(crossterm::event::KeyModifiers::SHIFT) {
        "Shift+"
    } else if modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
        "Ctrl+"
    } else {
        ""
    };

    match key_code {
        KeyCode::Char(c) => format!("{}{}", modifier_str, c),
        KeyCode::Enter => format!("{}Enter", modifier_str),
        KeyCode::Esc => format!("{}Esc", modifier_str),
        KeyCode::Up => format!("{}↑", modifier_str),
        KeyCode::Down => format!("{}↓", modifier_str),
        KeyCode::Left => format!("{}←", modifier_str),
        KeyCode::Right => format!("{}→", modifier_str),
        KeyCode::Home => format!("{}Home", modifier_str),
        KeyCode::End => format!("{}End", modifier_str),
        KeyCode::PageUp => format!("{}PageUp", modifier_str),
        KeyCode::PageDown => format!("{}PageDown", modifier_str),
        KeyCode::Tab => format!("{}Tab", modifier_str),
        KeyCode::Backspace => format!("{}Backspace", modifier_str),
        KeyCode::Delete => format!("{}Delete", modifier_str),
        KeyCode::Insert => format!("{}Insert", modifier_str),
        KeyCode::F(n) => format!("{}F{}", modifier_str, n),
        _ => format!("{}{:?}", modifier_str, key_code),
    }
}

fn get_config_path() -> Option<std::path::PathBuf> {
    if let Some(dir) = dirs::config_dir() {
        let path = dir.join("openisl").join("keybindings.toml");
        if path.exists() {
            return Some(path);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_keybindings() {
        let bindings = KeyBindings::default();
        assert!(bindings.actions.quit.contains("q"));
        assert!(bindings.actions.help.contains("?"));
        assert!(bindings.navigation.up.contains("j"));
    }

    #[test]
    fn test_keybindings_serde() {
        let bindings = KeyBindings::default();
        let toml = toml::to_string(&bindings).unwrap();
        let decoded: KeyBindings = toml::from_str(&toml).unwrap();
        assert_eq!(bindings.actions.quit, decoded.actions.quit);
    }

    #[test]
    fn test_parse_key_up() {
        let bindings = KeyBindings::default();
        assert_eq!(bindings.parse_key(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE), Some("up"));
        assert_eq!(bindings.parse_key(KeyCode::Down, crossterm::event::KeyModifiers::NONE), Some("up"));
    }

    #[test]
    fn test_parse_key_down() {
        let bindings = KeyBindings::default();
        assert_eq!(bindings.parse_key(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE), Some("down"));
        assert_eq!(bindings.parse_key(KeyCode::Up, crossterm::event::KeyModifiers::NONE), Some("down"));
    }

    #[test]
    fn test_parse_key_quit() {
        let bindings = KeyBindings::default();
        assert_eq!(bindings.parse_key(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE), Some("quit"));
        assert_eq!(bindings.parse_key(KeyCode::Esc, crossterm::event::KeyModifiers::NONE), Some("quit"));
    }
}
