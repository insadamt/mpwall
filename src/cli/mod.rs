use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "mpwall",
    version = "1.0.0",
    about = "Live video wallpaper manager for Hyprland/Wayland",
    long_about = "mpwall — a hybrid CLI/TUI video wallpaper manager built on top of mpvpaper.\nRun without arguments to launch the interactive TUI."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set a video file as the current wallpaper
    Set {
        /// Path to the video file
        #[arg(value_name = "FILE")]
        file: String,

        /// Target monitor name (e.g. eDP-1, DP-1). Defaults to all monitors.
        #[arg(short, long, value_name = "MONITOR")]
        monitor: Option<String>,
    },

    /// Stop the currently playing wallpaper
    Stop {
        /// Target monitor name. Defaults to all monitors.
        #[arg(short, long, value_name = "MONITOR")]
        monitor: Option<String>,
    },

    /// Add current wallpaper to Hyprland autostart
    Enable,

    /// Remove wallpaper from autostart and stop it
    Disable,

    /// Show current wallpaper status
    Status,

    /// List video files in the wallpaper directory
    List,
}
