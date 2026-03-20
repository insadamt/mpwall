use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Returns the path to the state file: ~/.local/share/mpwall/state.json
pub fn state_path() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            PathBuf::from(home).join(".local").join("share")
        });
    base.join("mpwall").join("state.json")
}

/// Returns the path to the library file: ~/.local/share/mpwall/library.json
pub fn library_path() -> PathBuf {
    state_path().parent().unwrap().join("library.json")
}

/// Per-monitor wallpaper entry stored in state.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorState {
    /// Absolute path to the active wallpaper video file
    pub wallpaper_path: String,

    /// PID of the mpvpaper process, if running
    pub pid: Option<u32>,

    /// Whether autostart is enabled for this monitor
    pub autostart: bool,
}

/// Top-level state file structure.
/// Key = monitor name (e.g. "DP-1", "eDP-1", "all")
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    /// Map of monitor name -> wallpaper state
    pub monitors: HashMap<String, MonitorState>,
}

impl State {
    /// Load state from disk. Returns empty state if file does not exist.
    pub fn load() -> Result<Self> {
        let path = state_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read state file at {}", path.display()))?;
        let state: Self = serde_json::from_str(&raw).with_context(|| {
            format!(
                "Failed to parse state file at {}.\nTip: delete it to reset mpwall state.",
                path.display()
            )
        })?;
        Ok(state)
    }

    /// Save state to disk. Creates parent directories if needed.
    pub fn save(&self) -> Result<()> {
        let path = state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create state directory at {}", parent.display())
            })?;
        }
        let raw = serde_json::to_string_pretty(self)
            .context("Failed to serialize state to JSON")?;
        fs::write(&path, raw)
            .with_context(|| format!("Failed to write state file at {}", path.display()))?;
        Ok(())
    }

    /// Get state for a specific monitor.
    pub fn get_monitor(&self, monitor: &str) -> Option<&MonitorState> {
        self.monitors.get(monitor)
    }

    /// Set state for a specific monitor.
    pub fn set_monitor(&mut self, monitor: String, entry: MonitorState) {
        self.monitors.insert(monitor, entry);
    }

    /// Remove state for a specific monitor (e.g. after stop).
    pub fn clear_monitor(&mut self, monitor: &str) {
        self.monitors.remove(monitor);
    }

    /// Returns true if any monitor has an active (non-None) PID.
    pub fn any_active(&self) -> bool {
        self.monitors.values().any(|m| m.pid.is_some())
    }
}

/// Simple wallpaper library: a list of absolute paths the user has saved.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Library {
    pub entries: Vec<String>,
}

impl Library {
    pub fn load() -> Result<Self> {
        let path = library_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read library file at {}", path.display()))?;
        let lib: Self = serde_json::from_str(&raw)
            .with_context(|| format!("Failed to parse library file at {}", path.display()))?;
        Ok(lib)
    }

    pub fn save(&self) -> Result<()> {
        let path = library_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let raw = serde_json::to_string_pretty(self)
            .context("Failed to serialize library to JSON")?;
        fs::write(&path, raw)
            .with_context(|| format!("Failed to write library file at {}", path.display()))?;
        Ok(())
    }

    pub fn add(&mut self, path: String) {
        if !self.entries.contains(&path) {
            self.entries.push(path);
        }
    }

    pub fn remove(&mut self, path: &str) {
        self.entries.retain(|e| e != path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_has_no_active_monitors() {
        let state = State::default();
        assert!(!state.any_active());
    }

    #[test]
    fn set_and_get_monitor_state() {
        let mut state = State::default();
        state.set_monitor(
            "eDP-1".to_string(),
            MonitorState {
                wallpaper_path: "/home/user/Videos/wallpapers/city.mp4".to_string(),
                pid: Some(12345),
                autostart: false,
            },
        );
        let entry = state.get_monitor("eDP-1").unwrap();
        assert_eq!(entry.pid, Some(12345));
        assert_eq!(entry.wallpaper_path, "/home/user/Videos/wallpapers/city.mp4");
    }

    #[test]
    fn clear_monitor_removes_entry() {
        let mut state = State::default();
        state.set_monitor(
            "DP-1".to_string(),
            MonitorState {
                wallpaper_path: "/tmp/test.mp4".to_string(),
                pid: Some(999),
                autostart: false,
            },
        );
        state.clear_monitor("DP-1");
        assert!(state.get_monitor("DP-1").is_none());
    }

    #[test]
    fn library_add_no_duplicates() {
        let mut lib = Library::default();
        lib.add("/tmp/a.mp4".to_string());
        lib.add("/tmp/a.mp4".to_string());
        assert_eq!(lib.entries.len(), 1);
    }
}
