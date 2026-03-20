# Feature: stop

## Purpose

Stop the currently playing wallpaper cleanly.

## Usage

```bash
mpwall stop [--monitor <name>]
```

## Behavior

1. Loads `state.json`
2. Resolves target monitors
3. For each monitor with an active PID: sends SIGTERM
4. Updates `state.json` — clears PID and path, preserves `autostart` flag

If the PID is already dead (stale state), mpwall cleans up gracefully without error.
