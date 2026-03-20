# Feature: enable / disable

## enable

Adds the current wallpaper to Hyprland autostart so it persists across reboots.

```bash
mpwall enable
```

Writes to `~/.config/hypr/hyprland.conf`:

```
# mpwall start
exec-once = mpwall set /path/to/video.mp4 --monitor eDP-1
# mpwall end
```

- Only lines inside this block are ever touched
- Running `enable` twice is safe — the old block is replaced, never duplicated
- Sets `autostart: true` in `state.json`

## disable

Stops the wallpaper and removes the autostart entry.

```bash
mpwall disable
```

- Sends SIGTERM to the active process
- Removes the `# mpwall start` / `# mpwall end` block from `hyprland.conf`
- Sets `autostart: false` in `state.json`
- Each step runs independently — a failure in one does not block the other
