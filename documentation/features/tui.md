# Feature: TUI

## Purpose

An interactive terminal interface for browsing, managing, and configuring wallpapers without typing paths.

## Launch

```bash
mpwall     # no arguments
```

## Layout

```
┌─────────────────────────────────────────────────┐
│  mpwall  │ Browser │ Status │ Library │ Settings │
├─────────────────────────────────────────────────┤
│                                                 │
│               [active panel content]            │
│                                                 │
├─────────────────────────────────────────────────┤
│  Tab: switch  |  q: quit  |  ?: help            │
└─────────────────────────────────────────────────┘
```

## Panels

### Browser
Scrollable list of all video files in the wallpaper directory.
- `↑`/`↓` or `j`/`k` — navigate
- `Enter` — set as wallpaper immediately
- `/` — enter filter mode (type to search, `Esc` to clear)
- `a` — add to Library
- Active wallpaper marked with `▶`

### Status
Real-time wallpaper status per monitor, refreshed every 2 seconds.
- `e` — enable autostart
- `d` — disable autostart and stop wallpaper

### Library
Saved wallpapers you’ve bookmarked.
- `↑`/`↓` or `j`/`k` — navigate
- `Enter` — set as wallpaper
- `d` — remove from library
- `!` indicator — file no longer exists on disk

### Settings
Edit config values without touching the TOML file.
- `↑`/`↓` or `j`/`k` — navigate fields
- `e` or `Enter` — start editing a field
- `Esc` — cancel edit
- `s` — validate and save to `config.toml`

## Global Keybindings

| Key | Action |
|-----|--------|
| `Tab` | Next panel |
| `Shift+Tab` | Previous panel |
| `q` | Quit |
| `?` | Toggle help overlay |
| `Ctrl+C` | Force quit |
