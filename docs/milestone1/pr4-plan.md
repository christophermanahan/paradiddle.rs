# PR #4 — App Core + Event Loop + Deterministic Testing Harness

## Goal

Introduce a testable application core that runs a real event loop, decoupled from terminal I/O. Enable non-TTY testing of app behavior including quit, resize, and rendering.

---

## Definition of Done

- [ ] `App` struct with lifecycle state (running, size, focused pane)
- [ ] `AppEvent` enum decoupled from crossterm (Key, Resize)
- [ ] `AppKey` enum for internal key representation
- [ ] Demo runs in a loop, quits on `q`/`Esc`, handles resize
- [ ] Terminal cleanup guard ensures restore even on panic/error
- [ ] Non-TTY tests: quit event, resize event, render assertions
- [ ] Snapshot tests extended for state transitions
- [ ] Perf benchmarks extended for `handle_event` + `render`
- [ ] `docs/DEV.md` updated with demo and test instructions
- [ ] Milestone docs updated (roadmap.md, journal.md)
- [ ] `cargo fmt/test/clippy` clean

---

## Scope

### In Scope

| Item | Description |
|------|-------------|
| App struct | Owns windows, state, size |
| AppEvent/AppKey | Decoupled input abstraction |
| Event loop | Poll-based loop in demo crate |
| Quit handling | `q` and `Esc` keys |
| Resize handling | Update stored size, re-render |
| Focus indicator | Optional Tab toggle with border marker |
| Terminal guard | RAII cleanup pattern |
| Non-TTY tests | Drive App via AppEvent, assert state |
| Render tests | Buffer assertions for content |

### Out of Scope

| Item | Rationale |
|------|-----------|
| PTY integration | Deferred to later milestone |
| IPC/daemon mode | Not needed for Phase 1 |
| LSP client | Future milestone |
| Tiling/layout tree | PR #6 or later |
| Plugin/config system | Future milestone |
| Complex keybindings | PR #5 keybinding router |
| Mouse events | Keep minimal for now |

---

## Files / Modules

```
cli-ide-workbench/
├── src/
│   ├── lib.rs              # Add pub mod app; pub mod input;
│   ├── app.rs              # NEW: App struct, lifecycle
│   ├── input.rs            # NEW: AppEvent, AppKey enums
│   └── window/             # Existing window code
├── tests/
│   └── app_tests.rs        # NEW: Integration tests
└── benches/
    └── render_bench.rs     # EXTEND: Add app benchmarks

cli-ide-demo/
└── src/
    └── main.rs             # UPDATE: Real event loop
```

---

## Test Strategy

### Unit/Integration Tests

1. **Quit behavior**: Send `AppEvent::Key(AppKey::Q)`, assert `app.running == false`
2. **Resize behavior**: Send `AppEvent::Resize(100, 50)`, verify dimensions stored
3. **Render output**: Render to TestBackend, assert "Editor" and "Terminal" present
4. **Focus toggle** (if implemented): Send Tab, verify focus changes

### Non-TTY Approach

- All tests use `ratatui::backend::TestBackend`
- No crossterm dependency in tests
- No sleeps or polling loops
- Deterministic: same input → same output

### Snapshot Extensions

- Initial state snapshot
- After resize snapshot (different dimensions)
- Focus indicator visibility (if implemented)

---

## Risks / Mitigations

| Risk | Mitigation |
|------|------------|
| Terminal not restored on panic | Use RAII guard pattern with Drop impl |
| Snapshot brittleness | Assert key substrings, not exact output |
| API churn in App design | Keep surface minimal; internal refactors OK |
| crossterm version mismatch | Pin version, abstract behind AppKey |

---

## Follow-ups

### PR #5: Keybinding Router
- Configurable key→action mapping
- Modal support (normal/insert/command)
- Help overlay

### PR #6: Layout Primitives
- Split containers (horizontal/vertical)
- Resize splits with keyboard
- Layout serialization

### PR #7+: PTY Integration
- Shell spawning
- ANSI parsing
- Terminal buffer
