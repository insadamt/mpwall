# mpwall

> A professional hybrid CLI/TUI live video wallpaper manager for Hyprland/Wayland.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)
[![AUR](https://img.shields.io/badge/AUR-mpwall-1793d1?logo=arch-linux)](https://aur.archlinux.org/packages/mpwall)

## What is mpwall?

mpwall is a single Rust binary that replaces all manual `mpvpaper` scripting on Hyprland.
It provides:

- A clean CLI for instant wallpaper management
- A full keyboard-driven TUI for browsing, library management, and configuration
- Multi-monitor support with per-monitor state tracking
- Hyprland autostart integration with safe, delimited config edits
- AUR-native distribution — `paru -S mpwall` and you’re done

## Quick Install

```bash
# Via AUR helper
paru -S mpwall

# Or manually
git clone https://aur.archlinux.org/mpwall.git
cd mpwall
makepkg -si
```

## Quick Start

```bash
# Set a wallpaper immediately
mpwall set ~/Videos/wallpapers/city.mp4

# Check status
mpwall status

# Enable autostart on login
mpwall enable

# Open the interactive TUI
mpwall
```

## Documentation Index

| File | Description |
|------|-------------|
| [getting-started.md](./getting-started.md) | Prerequisites and first run |
| [architecture.md](./architecture.md) | System design and module overview |
| [configuration.md](./configuration.md) | All config options with defaults |
| [development-workflow.md](./development-workflow.md) | Build, test, release process |
| [contributing.md](./contributing.md) | How to contribute |
| [features/](./features/) | Per-feature documentation |
