# mpwall

> A professional hybrid CLI/TUI live video wallpaper manager for Hyprland/Wayland.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![AUR](https://img.shields.io/badge/AUR-mpwall-1793d1?logo=arch-linux)](https://aur.archlinux.org/packages/mpwall)

mpwall replaces all manual `mpvpaper` scripting with a single, reliable binary.
Set video wallpapers from the CLI in one command, or use the full TUI to browse your library, manage playlists, and configure everything interactively.

## Features

- `mpwall set video.mp4` — instant wallpaper, zero setup
- Full keyboard-driven TUI — browser, status, library, settings
- Multi-monitor support — independent wallpaper per screen
- Hyprland autostart integration — safe, delimited config edits
- Single Rust binary — no runtime, no shell dependency
- AUR-native — `paru -S mpwall`

## Install

```bash
paru -S mpwall
```

Or from source:

```bash
git clone https://github.com/insadamt/mpwall.git
cd mpwall
cargo build --release
sudo install -Dm755 target/release/mpwall /usr/bin/mpwall
```

**Dependencies:** `mpvpaper`, `hyprland`, Nerd Fonts (for TUI icons)

## Usage

```bash
mpwall set ~/Videos/wallpapers/city.mp4    # set wallpaper
mpwall set city.mp4 --monitor DP-1        # target specific monitor
mpwall stop                                # stop wallpaper
mpwall enable                              # add to Hyprland autostart
mpwall disable                             # remove from autostart
mpwall status                              # check what’s running
mpwall list                                # list available wallpapers
mpwall                                     # open TUI
```

## TUI Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch panel |
| `↑` `↓` / `j` `k` | Navigate |
| `Enter` | Set wallpaper |
| `/` | Filter (Browser) |
| `a` | Add to Library |
| `d` | Remove from Library |
| `e` | Edit field (Settings) |
| `s` | Save settings |
| `?` | Help overlay |
| `q` | Quit |

## Documentation

Full documentation is in [`documentation/`](documentation/README.md).

## License

MIT — see [LICENSE](LICENSE)
