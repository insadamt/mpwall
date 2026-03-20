# Contributing

Contributions are welcome. Please read this before opening a pull request.

## Philosophy

mpwall follows the Hyprland ecosystem ethos: fast, minimal, keyboard-driven, no bloat.
Every addition must earn its place. When in doubt, keep it simple.

## Before You Start

- Check open issues before starting work on a new feature
- For significant changes, open an issue first to discuss the approach
- The MVP scope is fixed — v2 features are tracked in the spec but not yet in scope

## Code Standards

- Run `cargo fmt` before every commit — no exceptions
- Run `cargo clippy -- -D warnings` and fix all warnings
- Every public function must have a doc comment (`///`)
- Every new feature must include at least one test
- Error messages must be actionable — always tell the user what to do

## Commit Style

Follow conventional commits:

```
feat(scope): short description
fix(scope): short description
refactor(scope): short description
docs(scope): short description
chore(scope): short description
```

Examples:
```
feat(cli): add random wallpaper subcommand
fix(process): handle SIGKILL fallback on stubborn processes
docs(config): document speed field validation
```

## Pull Request Checklist

- [ ] `cargo fmt` applied
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] New behavior documented in `documentation/`
- [ ] `PLANNING.md` updated if tasks were added or completed
- [ ] Commit messages follow conventional commits

## What Not to Do

- Do not add GUI dependencies
- Do not call `sudo` or any package manager
- Do not write to files outside `~/.config/mpwall/`, `~/.local/share/mpwall/`, or the Hyprland config block
- Do not break the zero-config first-run experience
