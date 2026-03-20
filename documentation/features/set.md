# Feature: set

## Purpose

Set a video file as the live wallpaper immediately.

## Usage

```bash
mpwall set <file> [--monitor <name>]
```

## Examples

```bash
# Set on all active monitors
mpwall set ~/Videos/wallpapers/city.mp4

# Set on a specific monitor
mpwall set ~/Videos/wallpapers/forest.mp4 --monitor DP-1
```

## Behavior

1. Resolves the file path to absolute
2. Validates the file exists
3. Checks `mpvpaper` is installed
4. Resolves target monitors via `hyprctl`
5. For each monitor: kills any existing `mpvpaper` process
6. Spawns `mpvpaper <monitor> <file> [flags from config]`
7. Stores the new PID in `state.json`

## Error Cases

| Situation | Message |
|-----------|--------|
| File not found | `File not found: /path — check the path and try again.` |
| mpvpaper not installed | `'mpvpaper' not found in PATH. Tip: paru -S mpvpaper` |
| Invalid monitor name | Lists available monitors with tip |
| Hyprland not running | `hyprctl not found. Is Hyprland running?` |
