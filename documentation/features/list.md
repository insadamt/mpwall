# Feature: list

## Purpose

List all video files in the configured wallpaper directory.

## Usage

```bash
mpwall list
```

## Output Example

```
Wallpapers in /home/user/Videos/wallpapers
────────────────────────────────────────────────
    1.  city.mp4  (42.3 MB)
    2.  forest.mkv  (118.7 MB)
    3.  rain.webm  (9.1 MB)
────────────────────────────────────────────────
  3 file(s) found
```

## Supported Extensions

`mp4`, `mkv`, `webm`, `mov`, `avi`

## Notes

- The directory is read from `~/.config/mpwall/config.toml` — defaults to `~/Videos/wallpapers`
- Files are sorted alphabetically
- Use the TUI Browser panel for a more interactive experience
