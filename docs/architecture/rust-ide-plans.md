# Rust IDE Implementation Plans

This document outlines the phased implementation plan for Paradiddle.rs.

## Phase Overview

| Phase | Focus | Status |
|-------|-------|--------|
| Phase 1 | Foundation (Event system, DI, TUI basics) | Complete |
| Phase 2 | Terminal & Context (PTY, daemon, adapters) | Planning |
| Phase 3 | Collaboration (Sharing, remote sessions) | Future |
| Phase 4 | Full IDE (LSP, editing, debugging) | Future |

## Phase 1: Foundation (Complete)

### Delivered

- Cargo workspace with 4 crates
- `Event<T>` publish-subscribe system
- `ServiceContainer` dependency injection
- `Window` trait and implementations
- `App` core with event loop
- `FocusManager` and `KeybindingRouter`
- Non-TTY testing infrastructure
- CI/CD pipeline and governance

### PRs

| PR | Description | Status |
|----|-------------|--------|
| #1 | Bootstrap workspace | Merged |
| #2 | Governance & CI/CD | Merged |
| #3 | UI snapshot tests | Merged |
| #4 | App core & event loop | Merged |
| #5 | Focus & keybinding router | Merged |
| #5.1 | Context capture model docs | Current |

## Phase 2: Terminal & Context

### Goals

- PTY-backed terminal emulation
- Shell integration for command boundaries
- Event-driven context capture
- Daemon architecture for single-writer log
- Initial tool adapters (git, basic)

### Architecture Reference

The context capture architecture is defined in:

**[Context Capture Model](./CONTEXT_CAPTURE_MODEL.md)**

Key concepts:
- Event taxonomy (primitive, state probes, semantic)
- Capability levels (0-4)
- Shell hooks over PTY parsing
- Single-writer daemon model

### Planned PRs

| PR | Description | Dependencies |
|----|-------------|--------------|
| #7 | Layout primitives (splits, resize) | #5.1 |
| #8 | PTY-backed terminals | #7 |
| #9 | Daemon + IPC architecture | #8 |
| #10 | Shell integration hooks | #9 |
| #11 | Git adapter (Level 3) | #10 |

### Key Decisions Pending

- PTY library selection (portable_pty vs custom)
- IPC mechanism (Unix sockets vs named pipes)
- Event log format (append-only file vs embedded DB)
- Shell integration installation (auto vs manual)

## Phase 3: Collaboration

### Goals

- Session sharing (read-only, collaborative)
- Remote context ingestion
- Actor attribution
- Replay functionality

### Architecture Reference

Shared context semantics are defined in [Context Capture Model](./CONTEXT_CAPTURE_MODEL.md), Section 8.

### Planned PRs

| PR | Description | Dependencies |
|----|-------------|--------------|
| #12 | Session sharing protocol | #11 |
| #13 | Remote client support | #12 |
| #14 | Replay viewer | #13 |

## Phase 4: Full IDE

### Goals

- LSP client integration
- Text editing with rope data structure
- Syntax highlighting
- Debugging (DAP)
- Project navigation

### Planned PRs

| PR | Description | Dependencies |
|----|-------------|--------------|
| #15+ | LSP client | #14 |
| #16+ | Rope-based editing | #15 |
| #17+ | Tree-sitter highlighting | #16 |
| #18+ | DAP debugging | #17 |

## PR Numbering Note

PR #5.1 was inserted after PR #5 to document the context capture model before Phase 2 implementation. This shifted all subsequent PR numbers by +1:

| Original | New | Description |
|----------|-----|-------------|
| PR #6 | PR #7 | Layout Primitives |
| PR #7 | PR #8 | Configurable Keybindings (now PTY Terminals) |
| PR #8+ | PR #9+ | Subsequent PRs |

Historical PRs (#1–#5) retain their original numbers.

## Risk Areas

### Technical Risks

| Risk | Mitigation |
|------|------------|
| PTY portability (Windows) | Use portable_pty, test early |
| Shell integration adoption | Graceful degradation, clear docs |
| Event log growth | Bounded size, rotation policy |
| Security (secrets in logs) | Redaction at ingestion, opt-in |

### Scope Risks

| Risk | Mitigation |
|------|------------|
| Feature creep | Strict PR scoping, deferred items |
| Premature optimization | Profile before optimizing |
| Over-abstraction | YAGNI principle, concrete first |

## Success Metrics

### Phase 2 Complete When

- [ ] Can run shell commands in PTY terminal
- [ ] Command boundaries detected via shell hooks
- [ ] Events logged to append-only store
- [ ] Git status captured after git commands
- [ ] Demo: AI can see recent commands and git state

### Phase 3 Complete When

- [ ] Can share session with remote observer
- [ ] Remote commands attributed to remote actor
- [ ] Can replay historical session

### Phase 4 Complete When

- [ ] Can edit files with LSP assistance
- [ ] Syntax highlighting works
- [ ] Can set breakpoints and debug
