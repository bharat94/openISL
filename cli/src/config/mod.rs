use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub general: GeneralConfig,
    pub tui: TuiConfig,
    pub git: GitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub max_commits: usize,
    pub date_format: String,
    pub verbose: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub theme: String,
    pub page_size: usize,
    pub show_help_on_start: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitConfig {
    pub auto_fetch: bool,
    pub fetch_remotes: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            max_commits: 100,
            date_format: "%Y-%m-%d %H:%M:%S UTC".to_string(),
            verbose: false,
        }
    }
}

impl Default for TuiConfig {
    fn default() -> Self {
        TuiConfig {
            theme: "dark".to_string(),
            page_size: 20,
            show_help_on_start: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path();

        let builder = config::Config::builder()
            .add_source(config::File::with_name("openisl").required(false))
            .add_source(config::Environment::with_prefix("OPENISL").separator("_"));

        let builder = if let Some(path) = config_path {
            builder.add_source(config::File::from(path))
        } else {
            builder
        };

        builder
            .build()
            .context("Failed to build config")?
            .try_deserialize()
            .context("Failed to deserialize config")
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()
            .unwrap_or_else(|| {
                let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
                path.push("openisl");
                path.push("config.toml");
                path
            });

        std::fs::create_dir_all(config_path.parent().unwrap())
            .context("Failed to create config directory")?;

        let toml = toml::to_string_pretty(self).context("Failed to serialize config")?;
        std::fs::write(&config_path, toml).context("Failed to write config")?;

        Ok(())
    }
}

fn get_config_path() -> Option<PathBuf> {
    if let Some(dir) = dirs::config_dir() {
        let path = dir.join("openisl").join("config.toml");
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
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.max_commits, 100);
        assert_eq!(config.tui.theme, "dark");
        assert!(!config.git.auto_fetch);
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let config = Config::default();
        let toml = toml::to_string(&config).unwrap();
        let decoded: Config = toml::from_str(&toml).unwrap();
        assert_eq!(config.general.max_commits, decoded.general.max_commits);
    }
}
