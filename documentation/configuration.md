# Configuration

mpwall stores its config at `~/.config/mpwall/config.toml`.

This file is **optional**. mpwall works out of the box with no configuration. All values have safe, sensible defaults.

## Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schema_version` | integer | `1` | Internal schema version. Do not edit manually. |
| `wallpaper_dir` | string | `~/Videos/wallpapers` | Directory scanned by `mpwall list` and the TUI Browser panel. |
| `mpvpaper_flags` | string | `"--loop"` | Raw extra flags appended to every `mpvpaper` invocation. |
| `loop_video` | bool | `true` | Loop the video continuously. |
| `volume` | integer (0–100) | `0` | Audio volume. `0` passes `--no-audio` to mpvpaper (muted). |
| `speed` | float | `1.0` | Playback speed multiplier. |
| `theme` | string | `"lamess_ui"` | UI color theme. Options: `lamess_ui`, `cyan`, `monochrome`. |

## Example Config

```toml
schema_version = 1
wallpaper_dir = "/home/user/Videos/wallpapers"
mpvpaper_flags = "--loop"
loop_video = true
volume = 0
speed = 1.0
theme = "lamess_ui"
```

## Editing Config

**Via TUI (recommended):**
1. Open mpwall with no arguments
2. Press `Tab` until the Settings panel is active
3. Navigate fields with `↑`/`↓`, press `e` to edit, `Enter` to confirm
4. Press `s` to save — mpwall validates before writing

**Via editor:**
```bash
$EDITOR ~/.config/mpwall/config.toml
```

## Validation Rules

- `volume` must be an integer between `0` and `100`
- `speed` must be a positive decimal greater than `0.0`
- `wallpaper_dir` must be a valid filesystem path (warning shown if it does not exist)
- If the file is malformed, mpwall prints a clear error with the file path and the fix

## Reset to Defaults

```bash
rm ~/.config/mpwall/config.toml
```

mpwall will recreate it with defaults the next time you save from the TUI.
