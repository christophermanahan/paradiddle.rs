# Paradiddle.rs – Rust Rewrite of the Paradiddle IDE

This repository contains the initial scaffolding for **Paradiddle.rs**, a Rust-based rewrite of the [Paradiddle](https://github.com/christophermanahan/paradiddle) editor. It follows the multi-workspace, AI-assisted development workflow described in the accompanying documentation.

## Phase 1 Status

✅ **Build:** Compiles cleanly with `cargo build`
✅ **Tests:** 11 unit tests passing (`cargo test`)
✅ **Linting:** No warnings with `cargo clippy -D warnings`
✅ **Formatting:** Formatted with `cargo fmt`

## Quick Start

```bash
# Build the workspace
cargo build

# Run all tests
cargo test

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all

# Run the demo (requires interactive terminal)
cargo run -p cli-ide-demo
```

> **Note:** The demo application requires an interactive terminal (TTY) to display the TUI. It will show an Editor window and Terminal window side-by-side.

## Project Goals

1. **Rust from Scratch:** Rebuild the Paradiddle IDE (originally a Neovim configuration built on NvChad) as a native Rust application, optimized for performance and user experience.
2. **Multi-Workspace AI Workflow:** Provide a structure that allows multiple AI code agents to work simultaneously, each in its own workspace, with human oversight and review gates.
3. **Phase 1 Implementation:** Implement the foundational features defined in the [rust-ide-plans.md](https://github.com/christophermanahan/paradiddle/blob/main/docs/architecture/rust-ide-plans.md) document: an event system, dependency injection (service container), base window abstractions, and a simple text/terminal window demo using the `ratatui` crate.

## Repository Structure

```
paradiddle.rs/
├── Cargo.toml           # Top-level workspace definition
├── CLAUDE.md            # Instructions for AI agents
├── docs/
│   └── milestone1/
│       ├── journal.md   # Running log for workspace agents
│       └── roadmap.md   # Phase 1 task list with priorities
├── cli-ide-base/        # Common primitives (events, etc.)
├── cli-ide-platform/    # Dependency injection and platform abstractions
├── cli-ide-workbench/   # Window system and rendering
└── cli-ide-demo/        # Minimal demo application
```

## Crate Overview

| Crate | Description |
|-------|-------------|
| `cli-ide-base` | Event system with `map`, `filter`, `debounce` transformations |
| `cli-ide-platform` | Dependency injection container (`ServiceContainer`) |
| `cli-ide-workbench` | Window trait and implementations (`EditorWindow`, `TerminalWindow`) |
| `cli-ide-demo` | Demo application showing side-by-side windows |

## Contributions

This is a starting point for Phase 1 implementation. Future work will flesh out the event system, service container, window manager, and integrated AI tooling. Pull requests are welcome!
