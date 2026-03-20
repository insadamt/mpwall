use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::tui::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub schema_version: u32,
    pub wallpaper_dir: String,
    pub mpvpaper_flags: String,
    pub loop_video: bool,
    pub volume: u8,
    pub speed: f32,
    #[serde(default)]
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema_version: 1,
            wallpaper_dir: default_wallpaper_dir(),
            mpvpaper_flags: String::new(),
            loop_video: true,
            volume: 0,
            speed: 1.0,
            theme: Theme::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            let cfg = Config::default();
            cfg.save()?;
            return Ok(cfg);
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {}", path.display()))?;
        let cfg: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse config.toml")?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config dir at {}", parent.display()))?;
        }
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config to {}", path.display()))?;
        Ok(())
    }

    pub fn build_mpvpaper_flags(&self) -> Vec<String> {
        let mut opts: Vec<String> = Vec::new();
        if self.loop_video {
            opts.push("--loop".to_string());
        }
        opts.push("--no-audio".to_string());
        opts.push(format!("--volume={}", self.volume));
        opts.push(format!("--speed={}", self.speed));
        opts
    }
}

pub fn config_path() -> PathBuf {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            PathBuf::from(home).join(".config")
        });
    config_home.join("mpwall").join("config.toml")
}

fn default_wallpaper_dir() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    Path::new(&home)
        .join("Videos")
        .join("wallpapers")
        .to_string_lossy()
        .to_string()
}
