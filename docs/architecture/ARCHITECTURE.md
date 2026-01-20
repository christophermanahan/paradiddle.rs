# Paradiddle.rs Architecture

This document provides a high-level overview of the Paradiddle.rs architecture.

## Overview

Paradiddle.rs is a terminal-based IDE designed for AI-assisted development workflows. It combines:

- A TUI (terminal user interface) workbench
- Integrated terminal emulation
- Context capture for LLM assistance
- Extensible window and layout system

## Crate Structure

```
paradiddle.rs/
├── cli-ide-base/       # Core primitives (Event system)
├── cli-ide-platform/   # DI container, platform abstractions
├── cli-ide-workbench/  # Window system, App core, TUI rendering
└── cli-ide-demo/       # Demo application
```

### cli-ide-base

Foundation crate providing:

- `Event<T>` - Publish-subscribe event system with `map`, `filter`, `debounce`
- Core traits and utilities shared across crates

### cli-ide-platform

Platform abstraction layer:

- `ServiceContainer` - Dependency injection
- Platform-specific implementations (future)

### cli-ide-workbench

Main application crate:

- `App` - Application state and lifecycle
- `Window` trait and implementations (`EditorWindow`, `TerminalWindow`)
- `FocusManager` - Window focus tracking
- `KeybindingRouter` - Key-to-action dispatch
- Input abstractions (`AppEvent`, `AppKey`)

### cli-ide-demo

Demonstration binary that launches the TUI.

## Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                          App                                 │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ FocusManager │  │KeybindRouter │  │   WindowIds      │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
│         │                 │                   │             │
│         └─────────────────┼───────────────────┘             │
│                           │                                 │
│                           ▼                                 │
│  ┌────────────────────────────────────────────────────────┐│
│  │                    Windows                              ││
│  │  ┌────────────────┐      ┌──────────────────┐         ││
│  │  │  EditorWindow  │      │  TerminalWindow  │         ││
│  │  └────────────────┘      └──────────────────┘         ││
│  └────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Key Abstractions

### Event System

The `Event<T>` type provides reactive event handling:

```rust
let event: Event<i32> = Event::new();
let doubled = event.clone().map(|x| x * 2);
let subscriber = doubled.subscribe();
event.emit(21);
// subscriber receives 42
```

### Window Trait

Windows implement the `Window` trait:

```rust
pub trait Window {
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn render_with_focus(&mut self, frame: &mut Frame, area: Rect, focused: bool);
}
```

### Input Abstraction

Terminal input is abstracted through `AppEvent` and `AppKey`, decoupling the app from specific terminal libraries.

## Terminal & Context Capture

For details on how terminal sessions are managed and context is captured for AI assistance, see:

**[Context Capture Model](./CONTEXT_CAPTURE_MODEL.md)**

This document covers:

- Event-driven context ingestion
- Shell integration and PTY handling
- Tool adapters and semantic events
- Shared session semantics
- Security and trust boundaries

## Future Architecture

See [rust-ide-plans.md](./rust-ide-plans.md) for planned architectural evolution:

- Phase 2: PTY-backed terminals, daemon architecture
- Phase 3: Context sharing, collaboration features
- Phase 4: Full IDE capabilities

## Related Documents

- [Developer Guide](../DEV.md) - Development setup and workflows
- [Keybindings](../input/keybindings.md) - Canonical keybinding reference
- [Governance](../GOVERNANCE.md) - Project governance model
