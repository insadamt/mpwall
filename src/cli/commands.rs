use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::{
    config::Config,
    monitor::resolve_monitors,
    process::{is_pid_alive, kill_pid, spawn_mpvpaper},
    state::{MonitorState, State},
};

// ---------------------------------------------------------------------------
// Autostart delimiters — NEVER modify lines outside these markers
// ---------------------------------------------------------------------------
const AUTOSTART_START: &str = "# mpwall start";
const AUTOSTART_END: &str = "# mpwall end";

// ---------------------------------------------------------------------------
// set
// ---------------------------------------------------------------------------

/// Set a video file as wallpaper on the given monitor(s).
pub fn cmd_set(file: &str, monitor: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let mut state = State::load()?;

    // Resolve absolute path
    let video_path = resolve_video_path(file)?;

    // Resolve target monitors
    let monitors = resolve_monitors(monitor)?;

    for mon in &monitors {
        // Kill any existing mpvpaper process for this monitor
        if let Some(entry) = state.get_monitor(mon) {
            if let Some(pid) = entry.pid {
                kill_pid(pid)?;
            }
        }

        // Build flags from config
        let flags = config.build_mpvpaper_flags();

        // Spawn mpvpaper
        let pid = spawn_mpvpaper(mon, video_path.to_str().unwrap(), &flags)
            .with_context(|| format!("Failed to set wallpaper on monitor '{}'", mon))?;

        // Persist state
        let autostart = state
            .get_monitor(mon)
            .map(|e| e.autostart)
            .unwrap_or(false);

        state.set_monitor(
            mon.clone(),
            MonitorState {
                wallpaper_path: video_path.to_string_lossy().to_string(),
                pid: Some(pid),
                autostart,
            },
        );

        println!("\x1b[32m✓\x1b[0m Wallpaper set on {} (PID {})", mon, pid);
    }

    state.save()?;
    Ok(())
}

// ---------------------------------------------------------------------------
// stop
// ---------------------------------------------------------------------------

/// Stop the wallpaper on the given monitor(s).
pub fn cmd_stop(monitor: Option<&str>) -> Result<()> {
    let mut state = State::load()?;
    let monitors = resolve_monitors(monitor)?;
    let mut stopped_any = false;

    for mon in &monitors {
        if let Some(entry) = state.get_monitor(mon).cloned() {
            if let Some(pid) = entry.pid {
                kill_pid(pid)?;
                println!("\x1b[32m✓\x1b[0m Stopped wallpaper on {} (PID {})", mon, pid);
            } else {
                println!("\x1b[33m~\x1b[0m No active wallpaper on {}", mon);
            }
            // Preserve autostart flag, clear pid and path
            state.set_monitor(
                mon.clone(),
                MonitorState {
                    wallpaper_path: String::new(),
                    pid: None,
                    autostart: entry.autostart,
                },
            );
            stopped_any = true;
        } else {
            println!("\x1b[33m~\x1b[0m No state found for monitor {}", mon);
        }
    }

    if stopped_any {
        state.save()?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// enable
// ---------------------------------------------------------------------------

/// Write mpwall autostart entry to hyprland.conf.
pub fn cmd_enable() -> Result<()> {
    let state = State::load()?;

    // Gather all active wallpapers
    let active: Vec<(&String, &MonitorState)> = state
        .monitors
        .iter()
        .filter(|(_, e)| !e.wallpaper_path.is_empty())
        .collect();

    if active.is_empty() {
        bail!(
            "No active wallpaper found.\nTip: run `mpwall set <file>` first, then `mpwall enable`."
        );
    }

    let hypr_conf = hyprland_conf_path()?;

    // Read current config
    let content = fs::read_to_string(&hypr_conf)
        .with_context(|| format!("Failed to read {}", hypr_conf.display()))?;

    // If block already exists, remove it first to avoid duplicates
    let content = remove_mpwall_block(&content);

    // Build new autostart block
    let mut block = format!("\n{}\n", AUTOSTART_START);
    for (mon, entry) in &active {
        block.push_str(&format!(
            "exec-once = mpwall set {} --monitor {}\n",
            entry.wallpaper_path, mon
        ));
    }
    block.push_str(&format!("{}\n", AUTOSTART_END));

    let new_content = format!("{}{}", content, block);
    fs::write(&hypr_conf, new_content)
        .with_context(|| format!("Failed to write {}", hypr_conf.display()))?;

    // Update autostart flag in state
    let mut state = State::load()?;
    for (mon, entry) in active {
        state.set_monitor(
            mon.clone(),
            MonitorState {
                autostart: true,
                ..entry.clone()
            },
        );
    }
    state.save()?;

    println!("\x1b[32m✓\x1b[0m Autostart enabled in {}", hypr_conf.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// disable
// ---------------------------------------------------------------------------

/// Remove mpwall autostart block from hyprland.conf and stop the wallpaper.
pub fn cmd_disable() -> Result<()> {
    // Step 1: stop wallpaper (independent — don't let failure block step 2)
    if let Err(e) = cmd_stop(None) {
        eprintln!("\x1b[33mWarning:\x1b[0m Could not stop wallpaper: {}", e);
    }

    // Step 2: remove autostart block
    let hypr_conf = hyprland_conf_path()?;
    let content = fs::read_to_string(&hypr_conf)
        .with_context(|| format!("Failed to read {}", hypr_conf.display()))?;

    let new_content = remove_mpwall_block(&content);

    if new_content == content {
        println!("\x1b[33m~\x1b[0m No autostart entry found — nothing to remove.");
    } else {
        fs::write(&hypr_conf, new_content)
            .with_context(|| format!("Failed to write {}", hypr_conf.display()))?;
        println!("\x1b[32m✓\x1b[0m Autostart entry removed from {}", hypr_conf.display());
    }

    // Step 3: update autostart flags in state
    let mut state = State::load()?;
    for entry in state.monitors.values_mut() {
        entry.autostart = false;
    }
    state.save()?;

    Ok(())
}

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

/// Print current wallpaper status for all monitors.
pub fn cmd_status() -> Result<()> {
    let state = State::load()?;

    if state.monitors.is_empty() {
        println!("\x1b[33mNo wallpaper state found.\x1b[0m Run `mpwall set <file>` to get started.");
        return Ok(());
    }

    println!("\x1b[1mmpwall status\x1b[0m");
    println!("{}", "─".repeat(48));

    for (mon, entry) in &state.monitors {
        let (status_color, status_text, pid_text) = match entry.pid {
            Some(pid) if is_pid_alive(pid) => (
                "\x1b[32m",
                "running",
                format!("PID {}", pid),
            ),
            Some(pid) => (
                "\x1b[31m",
                "stopped (stale PID)",
                format!("PID {} (dead)", pid),
            ),
            None => ("\x1b[31m", "stopped", "no PID".to_string()),
        };

        let autostart_indicator = if entry.autostart {
            "\x1b[33m[autostart]\x1b[0m"
        } else {
            ""
        };

        let wallpaper = if entry.wallpaper_path.is_empty() {
            "(none)".to_string()
        } else {
            // Show filename only for brevity
            Path::new(&entry.wallpaper_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| entry.wallpaper_path.clone())
        };

        println!(
            "  Monitor : {}",
            mon
        );
        println!(
            "  Status  : {}{}\x1b[0m {}",
            status_color, status_text, autostart_indicator
        );
        println!("  File    : {}", wallpaper);
        println!("  Process : {}", pid_text);
        println!("{}", "─".repeat(48));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// list
// ---------------------------------------------------------------------------

/// List video files in the configured wallpaper directory.
pub fn cmd_list() -> Result<()> {
    let config = Config::load()?;
    let dir = Path::new(&config.wallpaper_dir);

    if !dir.exists() {
        bail!(
            "Wallpaper directory not found: {}\nTip: create it or set a different path with `mpwall` TUI > Settings.",
            dir.display()
        );
    }

    let video_extensions = ["mp4", "mkv", "webm", "mov", "avi"];

    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?
        .filter_map(|entry| entry.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| video_extensions.contains(&e.to_lowercase().as_str()))
                    .unwrap_or(false)
        })
        .collect();

    if files.is_empty() {
        println!("\x1b[33mNo video files found in {}\x1b[0m", dir.display());
        println!("Tip: add .mp4, .mkv, .webm, .mov, or .avi files to that directory.");
        return Ok(());
    }

    files.sort();

    println!("\x1b[1mWallpapers in {}\x1b[0m", dir.display());
    println!("{}", "─".repeat(48));
    for (i, file) in files.iter().enumerate() {
        let name = file.file_name().unwrap().to_string_lossy();
        let size = fs::metadata(file)
            .map(|m| format_size(m.len()))
            .unwrap_or_else(|_| "?".to_string());
        println!("  {:>3}.  {}  \x1b[90m({})\x1b[0m", i + 1, name, size);
    }
    println!("{}", "─".repeat(48));
    println!("  {} file(s) found", files.len());

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Resolve a video file path to an absolute PathBuf.
fn resolve_video_path(file: &str) -> Result<PathBuf> {
    let path = PathBuf::from(file);
    let abs = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };
    if !abs.exists() {
        bail!(
            "File not found: {}\nTip: check the path and try again.",
            abs.display()
        );
    }
    Ok(abs)
}

/// Return the path to hyprland.conf.
fn hyprland_conf_path() -> Result<PathBuf> {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            PathBuf::from(home).join(".config")
        });
    let path = config_home.join("hypr").join("hyprland.conf");
    if !path.exists() {
        bail!(
            "hyprland.conf not found at {}\nTip: ensure Hyprland is installed and has been started at least once.",
            path.display()
        );
    }
    Ok(path)
}

/// Remove the mpwall-managed block from a hyprland.conf string.
/// Everything between AUTOSTART_START and AUTOSTART_END (inclusive) is removed.
fn remove_mpwall_block(content: &str) -> String {
    let mut result = String::new();
    let mut inside_block = false;

    for line in content.lines() {
        if line.trim() == AUTOSTART_START {
            inside_block = true;
            continue;
        }
        if line.trim() == AUTOSTART_END {
            inside_block = false;
            continue;
        }
        if !inside_block {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

/// Format a byte count into a human-readable string.
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_mpwall_block_cleans_correctly() {
        let input = "line1\n# mpwall start\nexec-once = mpwall set foo.mp4 --monitor eDP-1\n# mpwall end\nline2\n";
        let result = remove_mpwall_block(input);
        assert!(result.contains("line1"));
        assert!(result.contains("line2"));
        assert!(!result.contains("exec-once"));
        assert!(!result.contains(AUTOSTART_START));
        assert!(!result.contains(AUTOSTART_END));
    }

    #[test]
    fn remove_mpwall_block_no_block_unchanged() {
        let input = "line1\nline2\n";
        let result = remove_mpwall_block(input);
        assert_eq!(result, input);
    }

    #[test]
    fn format_size_mb() {
        assert_eq!(format_size(2 * 1024 * 1024), "2.0 MB");
    }

    #[test]
    fn format_size_bytes() {
        assert_eq!(format_size(512), "512 B");
    }
}
