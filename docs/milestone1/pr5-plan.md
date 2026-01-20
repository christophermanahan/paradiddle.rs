# PR #5: Focus Management + Keybinding Router + Window Registry

## Goal

Introduce the minimum infrastructure required to manage multiple windows reliably:
- **Focus state**: Track which window is active and provide visual indicators
- **Key routing**: Route key events to the appropriate window based on focus
- **Window registration**: Maintain a registry of windows with unique identifiers

This PR establishes the foundation for future window management features (tiling, splits, floating windows) without implementing them.

## Definition of Done

- [ ] `WindowId` type exists with unique ID generation
- [ ] `WindowRegistry` can register, unregister, and lookup windows by ID
- [ ] `FocusManager` tracks focused window and provides focus change API
- [ ] `KeybindingRouter` routes key events based on context (global vs window-specific)
- [ ] Windows display visual focus indicators (border style change)
- [ ] App integrates all components
- [ ] All existing tests pass
- [ ] New tests cover:
  - WindowId uniqueness
  - WindowRegistry CRUD operations
  - FocusManager state transitions
  - KeybindingRouter dispatch logic
  - Focus indicator rendering
- [ ] `cargo clippy -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] Documentation updated (DEV.md, roadmap.md, journal.md)

## Scope

### In Scope

1. **WindowId** (`cli-ide-workbench/src/window/window_id.rs`)
   - Newtype wrapper around `u64`
   - Thread-safe atomic counter for generation
   - `PartialEq`, `Eq`, `Hash`, `Clone`, `Copy`, `Debug`

2. **WindowRegistry** (`cli-ide-workbench/src/registry.rs`)
   - `register(window) -> WindowId`
   - `unregister(id)`
   - `get(id) -> Option<&dyn Window>`
   - `get_mut(id) -> Option<&mut dyn Window>`
   - `iter() -> impl Iterator<Item = (WindowId, &dyn Window)>`
   - Thread-safe (wrapped in appropriate sync primitives)

3. **FocusManager** (`cli-ide-workbench/src/focus.rs`)
   - `focused() -> Option<WindowId>`
   - `set_focus(id)`
   - `clear_focus()`
   - `is_focused(id) -> bool`
   - Emits focus change events (using existing Event<T> system)

4. **KeybindingRouter** (`cli-ide-workbench/src/keybinding.rs`)
   - Global keybindings (always active: quit, help)
   - Context-aware routing (focused window receives keys)
   - `register_global(key, action)`
   - `dispatch(key) -> Option<Action>`
   - Action enum for supported actions

5. **Focus Indicators**
   - Focused window: `BorderType::Thick` or distinct color
   - Unfocused window: `BorderType::Plain`
   - Update `Window` trait if needed to support focus state

6. **App Integration**
   - Replace hardcoded `FocusedPane` with `FocusManager`
   - Use `KeybindingRouter` for key dispatch
   - Use `WindowRegistry` for window management

### Out of Scope (Deferred)

- PTY integration
- IPC/daemon mode
- Floating window geometry
- Tiling/layout tree (PR #6)
- Configurable keybindings from file
- Modal keybindings (vim modes)
- Mouse focus
- Window titles from content
- Multi-monitor support

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                          App                                 │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ FocusManager │  │WindowRegistry│  │ KeybindingRouter │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
│         │                 │                   │             │
│         │    ┌────────────┼───────────────────┘             │
│         │    │            │                                 │
│         ▼    ▼            ▼                                 │
│  ┌────────────────────────────────────┐                     │
│  │          Window instances          │                     │
│  │  ┌────────────┐  ┌──────────────┐  │                     │
│  │  │EditorWindow│  │TerminalWindow│  │                     │
│  │  └────────────┘  └──────────────┘  │                     │
│  └────────────────────────────────────┘                     │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Key Event arrives** → App receives `AppEvent::Key(key)`
2. **Router dispatch** → `KeybindingRouter::dispatch(key)`
   - If global binding matches → Execute global action (quit, toggle focus)
   - Else → Forward to focused window (future: window handles key)
3. **Focus change** → `FocusManager::set_focus(id)`
   - Emits `FocusChanged` event
   - Windows query `is_focused()` during render

### WindowId Generation

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u64);

impl WindowId {
    pub fn new() -> Self {
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
```

## Test Strategy

### Unit Tests

1. **WindowId Tests** (`window_id.rs`)
   - `test_window_id_unique`: Multiple calls return distinct IDs
   - `test_window_id_equality`: Same ID equals itself
   - `test_window_id_hash`: Can be used as HashMap key

2. **WindowRegistry Tests** (`registry.rs`)
   - `test_register_returns_id`
   - `test_unregister_removes_window`
   - `test_get_returns_registered_window`
   - `test_get_returns_none_for_unknown_id`
   - `test_iter_visits_all_windows`

3. **FocusManager Tests** (`focus.rs`)
   - `test_initial_focus_is_none`
   - `test_set_focus_updates_focused`
   - `test_clear_focus_sets_none`
   - `test_is_focused_returns_correct_state`
   - `test_focus_change_emits_event`

4. **KeybindingRouter Tests** (`keybinding.rs`)
   - `test_global_binding_registered`
   - `test_dispatch_returns_global_action`
   - `test_dispatch_returns_none_for_unbound`
   - `test_global_bindings_take_precedence`

### Integration Tests

1. **Focus Indicator Tests** (`tests/focus_tests.rs`)
   - `test_focused_window_has_thick_border`
   - `test_unfocused_window_has_plain_border`
   - `test_focus_toggle_changes_borders`

2. **App Integration Tests** (extend `app_tests.rs`)
   - `test_app_uses_focus_manager`
   - `test_app_routes_keys_through_router`
   - `test_tab_changes_focus_via_manager`

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Window trait changes break existing code | Medium | Medium | Keep trait additions additive; default impls where possible |
| Thread-safety overhead in registry | Low | Low | Profile if needed; single-threaded UI is acceptable for Phase 1 |
| Focus events create coupling | Low | Medium | Keep event payloads minimal; use weak references if needed |
| Border style not visible in all terminals | Low | Low | Fall back to character indicators if needed |

## Follow-ups (Future PRs)

- **PR #6**: Layout Primitives (split containers, resize)
- **PR #7**: Configurable keybindings from file
- **PR #8**: Modal keybindings (vim modes)
- **PR #9**: Window-specific key handlers

## Implementation Order

1. `WindowId` type (no dependencies)
2. `FocusManager` (depends on WindowId, Event<T>)
3. `KeybindingRouter` (depends on AppKey)
4. `WindowRegistry` (depends on WindowId, Window trait)
5. Update `Window` trait for focus state
6. Update `EditorWindow` and `TerminalWindow` for focus indicators
7. Update `App` to integrate all components
8. Tests and documentation
