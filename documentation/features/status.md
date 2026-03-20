# Feature: status

## Purpose

Quick health check of what is currently playing.

## Usage

```bash
mpwall status
```

## Output Example

```
mpwall status
────────────────────────────────────────────────
  Monitor : eDP-1
  Status  : running  [autostart]
  File    : city.mp4
  Process : PID 18432
────────────────────────────────────────────────
```

## Color Coding

| Color | Meaning |
|-------|---------|
| Green | Process running and alive |
| Yellow | Autostart enabled |
| Red | Process stopped or PID is stale |

PID liveness is always verified against `/proc/<pid>` — a dead process is shown as `stopped (stale PID)`.
