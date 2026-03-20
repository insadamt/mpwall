# Getting Started

## Prerequisites

Before installing mpwall, ensure the following are present on your system:

| Dependency | Purpose | Install |
|------------|---------|--------|
| Hyprland | Wayland compositor | `paru -S hyprland` |
| mpvpaper | Video wallpaper engine | `paru -S mpvpaper` |
| Nerd Fonts | TUI icons | `paru -S ttf-nerd-fonts-symbols` |
| cargo | Required for AUR source build | `sudo pacman -S rust` |

mpwall will **not** install these for you. If any are missing, it will tell you exactly what to do.

## Installation

### Via AUR (recommended)

```bash
paru -S mpwall
```

### Manual build from source

```bash
git clone https://github.com/insadamt/mpwall.git
cd mpwall
cargo build --release
sudo install -Dm755 target/release/mpwall /usr/bin/mpwall
```

## First Run

### 1. Set your first wallpaper

```bash
mpwall set ~/Videos/wallpapers/city.mp4
```

This immediately:
- Kills any existing wallpaper process
- Spawns `mpvpaper` with the video on all active monitors
- Saves the PID and file path to `~/.local/share/mpwall/state.json`

### 2. Check it’s running

```bash
mpwall status
```

You will see monitor name, file name, PID, and autostart status.

### 3. Enable autostart

```bash
mpwall enable
```

This writes a managed block to `~/.config/hypr/hyprland.conf`:

```
# mpwall start
exec-once = mpwall set /path/to/city.mp4 --monitor eDP-1
# mpwall end
```

mpwall will never touch any lines outside this block.

### 4. Open the TUI

```bash
mpwall
```

No arguments — launches the interactive TUI. Navigate with `Tab`, quit with `q`, help with `?`.

## Default Paths

| File | Path |
|------|------|
| Config | `~/.config/mpwall/config.toml` |
| State | `~/.local/share/mpwall/state.json` |
| Library | `~/.local/share/mpwall/library.json` |
| Wallpaper dir | `~/Videos/wallpapers` |

All paths respect `XDG_CONFIG_HOME` and `XDG_DATA_HOME`.
