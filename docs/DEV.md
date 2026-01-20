# Developer Guide

This guide covers development setup and common workflows for Paradiddle.rs.

## Prerequisites

### Rust Toolchain

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```bash
rustc --version
cargo --version
```

### Required Components

```bash
rustup component add rustfmt clippy
```

### Optional Tools

```bash
# Just command runner (recommended)
cargo install just

# Code coverage
cargo install cargo-llvm-cov

# Security auditing
cargo install cargo-audit cargo-deny

# Faster test runner
cargo install cargo-nextest
```

## Common Commands

### Using Just (Recommended)

If you have `just` installed:

```bash
just fmt          # Format code
just fmt-check    # Check formatting
just clippy       # Run linter
just test         # Run tests
just build        # Debug build
just build-release # Release build
just demo         # Run demo (requires TTY)
just ci           # Run full CI locally
```

### Using Cargo Directly

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all --all-features

# Build
cargo build
cargo build --release

# Run demo
cargo run -p cli-ide-demo
```

## Running CI Locally

Before pushing, run the full CI suite:

```bash
just ci
# Or manually:
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --all-features
cargo build --release
```

## Debugging

### Verbose Test Output

```bash
cargo test -- --nocapture
cargo test test_name -- --nocapture
```

### Debug Logging

Add to your test or code:

```rust
eprintln!("Debug: {:?}", value);
```

Or use the `log` crate (when available):

```rust
log::debug!("Debug: {:?}", value);
```

### Running Specific Tests

```bash
# Single test
cargo test test_event_broadcast

# Tests in a crate
cargo test -p cli-ide-base

# Tests matching pattern
cargo test event::tests
```

## Running the Demo

The demo runs an interactive TUI application. It requires a real terminal (TTY).

```bash
# Run the demo
cargo run -p cli-ide-demo

# Or with just
just demo
```

**Controls:**
- `q` or `Esc` - Quit the application
- `Tab` - Toggle focus between Editor and Terminal panes

The demo uses a proper event loop that handles resize events and keyboard input.

## Terminal Reset

If the demo crashes and leaves your terminal in a bad state:

```bash
# Reset terminal
reset

# Or
stty sane

# Or (if really stuck)
tput reset
```

The demo includes a terminal guard (RAII pattern) that should automatically restore
the terminal on exit, even on panic. If something goes wrong, use the commands above.

## Project Structure

```
paradiddle.rs/
├── cli-ide-base/       # Core primitives (Event system)
├── cli-ide-platform/   # DI container, platform abstractions
├── cli-ide-workbench/  # Window system, TUI rendering
├── cli-ide-demo/       # Demo application
├── docs/
│   ├── adr/            # Architecture Decision Records
│   ├── milestone1/     # Phase 1 documentation
│   └── review/         # Review process docs
└── .github/
    ├── workflows/      # CI/CD pipelines
    └── ISSUE_TEMPLATE/ # Issue templates
```

## Code Style

- Follow Rust 2021 edition idioms
- Use `rustfmt` defaults
- Keep functions small and focused
- Document public APIs
- Write tests for new functionality

## Testing Philosophy

- Unit tests live next to the code (`mod tests`)
- Integration tests go in `tests/` directory
- Tests should be deterministic (no flakes)
- Use short timeouts for async/timing tests

### App Core Tests

The App core can be tested without a real terminal by driving it with `AppEvent`s.
Tests verify state transitions (quit, resize, focus toggle) and rendering output.

```bash
# Run App integration tests
cargo test -p cli-ide-workbench --test app_tests

# Run with verbose output
cargo test -p cli-ide-workbench --test app_tests -- --nocapture
```

App tests verify:
- `AppEvent::Key(Q)` and `Esc` set `running = false`
- `AppEvent::Resize(w, h)` updates stored dimensions
- `AppEvent::Key(Tab)` toggles focus between panes
- Rendering produces expected window titles and content
- Focus indicators appear on the focused window
- WindowIds are unique and track focus correctly
- Custom keybindings can be registered and dispatched

### Focus Management

The app uses `FocusManager` to track which window has focus. Each window has a
unique `WindowId` assigned at creation. Focus state is reflected visually:

- Focused windows show `[*]` in their title
- Focused windows use thick borders (`BorderType::Thick`)
- Unfocused windows use plain borders

```bash
# Run focus-related tests
cargo test -p cli-ide-workbench focus
```

### Keybinding System

The `KeybindingRouter` dispatches key events to actions. Default bindings:

| Key | Action |
|-----|--------|
| `q` | Quit |
| `Esc` | Quit |
| `Tab` | Toggle Focus |

Custom bindings can be registered:

```rust
use cli_ide_workbench::keybinding::{Action, KeybindingRouter};
use cli_ide_workbench::input::AppKey;

let mut router = KeybindingRouter::new();
router.register_global(AppKey::Char('h'), Action::FocusPrev);
```

```bash
# Run keybinding tests
cargo test -p cli-ide-workbench keybinding
```

### UI Snapshot Tests

The workbench crate includes snapshot tests that render windows to an offscreen buffer
using ratatui's `TestBackend`. These tests require no TTY and are fully CI-compatible.

```bash
# Run all snapshot tests
cargo test -p cli-ide-workbench snapshot

# Run with verbose output to see rendered buffers on failure
cargo test -p cli-ide-workbench snapshot -- --nocapture
```

Snapshot tests verify:
- Window titles and borders render correctly
- Content appears within the bordered area
- Split layouts show both panes without overlap
- Windows render correctly at various sizes

### Performance Benchmarks

Criterion benchmarks measure rendering and event system performance. Benchmarks are
**not** run automatically in CI on every PR to avoid noise.

```bash
# Run all benchmarks
cargo bench

# Run benchmarks for a specific crate
cargo bench -p cli-ide-base      # Event system benchmarks
cargo bench -p cli-ide-workbench # Render benchmarks

# Run a specific benchmark
cargo bench -p cli-ide-base -- event_emit
```

**Available benchmarks:**

| Crate | Benchmark | Description |
|-------|-----------|-------------|
| cli-ide-base | `event_emit` | Emit to N subscribers (1, 4, 16, 64) |
| cli-ide-base | `event_subscribe` | Subscribe operation overhead |
| cli-ide-base | `event_new` | Event creation |
| cli-ide-base | `event_emit_recv_roundtrip` | Full emit→receive latency |
| cli-ide-base | `event_map_transform` | Map transformation setup |
| cli-ide-workbench | `render_editor` | EditorWindow at various sizes |
| cli-ide-workbench | `render_terminal` | TerminalWindow at various sizes |
| cli-ide-workbench | `render_split_layout` | Split layout with both windows |
| cli-ide-workbench | `terminal_creation` | TestBackend terminal creation |
| cli-ide-workbench | `app_new` | App creation |
| cli-ide-workbench | `app_handle_event_*` | Event handling (quit, resize, tab) |
| cli-ide-workbench | `app_render` | Full app render at various sizes |

Benchmark results are stored in `target/criterion/` and include HTML reports.

## Common Issues

### "Device not configured" when running demo

The demo requires an interactive terminal. Run in a real terminal, not a non-TTY environment.

### Clippy warnings about unstable features

Ensure you're using stable Rust:

```bash
rustup default stable
```

### Lock file conflicts

```bash
cargo update
```

## Getting Help

- Check existing issues on GitHub
- Review `docs/` for documentation
- Ask in PR comments for code-specific questions
