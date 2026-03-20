# Feature: Multi-Monitor Support

## Purpose

Manage wallpapers independently on each connected monitor.

## How It Works

mpwall uses `hyprctl monitors -j` to enumerate all connected monitors at runtime.
Each monitor has its own entry in `state.json`, with its own:
- wallpaper path
- mpvpaper PID
- autostart flag

## CLI Usage

```bash
# Set on all monitors (default)
mpwall set city.mp4

# Set on a specific monitor
mpwall set city.mp4 --monitor DP-1
mpwall set forest.mp4 --monitor eDP-1

# Stop all monitors
mpwall stop

# Stop a specific monitor
mpwall stop --monitor DP-1
```

## Single Monitor Usage

No `--monitor` flag is needed. mpwall targets all active monitors by default. Single-monitor users never need to think about this feature.

## Monitor Names

Monitor names come from Hyprland. To see yours:

```bash
hyprctl monitors -j | grep '"name"'
# or
mpwall status
```

Common names: `eDP-1` (laptop screen), `DP-1`, `DP-2`, `HDMI-A-1`.

## Error Handling

If you pass an invalid monitor name, mpwall shows the available monitors:

```
error: Monitor 'DP-3' not found.
Available monitors: eDP-1, DP-1
Tip: use `mpwall status` to see active monitors.
```
