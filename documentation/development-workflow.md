# Development Workflow

## Requirements

- Rust stable (`rustup install stable`)
- Arch Linux with Hyprland running (for full manual testing)
- `mpvpaper` installed for process tests
- `cargo` in PATH

## Setup

```bash
git clone https://github.com/insadamt/mpwall.git
cd mpwall
cargo check   # verify it compiles
cargo test    # run all unit + integration tests
```

## Build

```bash
# Debug build (fast compile, no optimizations)
cargo build

# Release build (optimized, stripped binary — use for distribution)
cargo build --release

# Binary location
./target/release/mpwall --help
```

## Testing

```bash
# Run all tests
cargo test

# Run only unit tests (embedded in src/)
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run a specific test by name
cargo test config_defaults_without_file
```

Tests do not require Hyprland or mpvpaper to be running. Monitor and process tests use mock data.

## Linting and Formatting

```bash
cargo fmt          # auto-format all code
cargo clippy       # lint warnings
cargo clippy -- -D warnings   # treat warnings as errors (CI standard)
```

## Running Locally

```bash
# Run CLI commands directly
cargo run -- set ~/Videos/wallpapers/city.mp4
cargo run -- status
cargo run -- list

# Launch TUI
cargo run
```

## Release Process

1. Bump `version` in `Cargo.toml`
2. Update `pkgver` in `PKGBUILD` to match
3. Commit: `git commit -m "chore(release): bump version to X.Y.Z"`
4. Tag: `git tag vX.Y.Z`
5. Push: `git push origin main --tags`
6. Update `sha256sums` in `PKGBUILD` with the new tarball checksum:
   ```bash
   curl -sL https://github.com/insadamt/mpwall/archive/refs/tags/vX.Y.Z.tar.gz | sha256sum
   ```
7. Submit updated `PKGBUILD` to AUR

## Project Structure

```text
mpwall/
├── Cargo.toml              # dependencies and build profile
├── PKGBUILD                # AUR source package
├── PLANNING.md             # task tracker
├── tests/
│   └── integration_tests.rs  # integration test suite
└── src/
    ├── main.rs               # entry point
    ├── lib.rs                # library root (for tests)
    ├── cli/
    │   ├── mod.rs            # clap definitions
    │   └── commands.rs       # command handlers
    ├── core/
    │   ├── config.rs         # TOML config
    │   ├── state.rs          # JSON state + library
    │   ├── monitor.rs        # hyprctl integration
    │   └── process.rs        # mpvpaper spawn/kill
    └── tui/
        ├── mod.rs            # event loop
        ├── app.rs            # app state
        ├── ui.rs             # layout + rendering
        └── panels/
            ├── browser.rs
            ├── status.rs
            ├── library.rs
            └── settings.rs
```
