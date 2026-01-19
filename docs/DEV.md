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
