# Keybinding Reference

This document is the **canonical source of truth** for all keybindings in Paradiddle.rs.

## Rules

1. **No undocumented bindings** - Every keybinding in code MUST be listed here.
2. **No unlisted implementations** - Do not implement a keybinding without adding it to this document first.
3. **Review gate** - Any PR that changes keybinding behavior MUST update this document.
4. **CI enforcement** - The `keybinding-doc-sync` workflow fails if keybinding code changes without updating this file.

## Current Bindings

These bindings are implemented and active in the current release.

| Key | Context | Action | Since |
|-----|---------|--------|-------|
| `q` | Global | Quit application | PR #5 |
| `Esc` | Global | Quit application | PR #5 |
| `Tab` | Global | Toggle focus between panes | PR #5 |

### Binding Details

#### Quit (`q` / `Esc`)
- **Context**: Global (works regardless of focused pane)
- **Action**: Sets `running = false`, triggering graceful shutdown
- **Implementation**: `KeybindingRouter::new()` registers `AppKey::Q` and `AppKey::Esc` → `Action::Quit`

#### Toggle Focus (`Tab`)
- **Context**: Global
- **Action**: Cycles focus between Editor and Terminal panes
- **Implementation**: `KeybindingRouter::new()` registers `AppKey::Tab` → `Action::ToggleFocus`

## Reserved Bindings (Not Yet Implemented)

These keys are reserved for future implementation. They are NOT active but should not be used for other purposes.

| Key | Planned Action | Target PR |
|-----|----------------|-----------|
| `Shift+Tab` | Reverse focus cycle | PR #7 |
| `Ctrl+C` | Interrupt / Cancel | PR #7 |
| `h` / `j` / `k` / `l` | Vim-style navigation | PR #7 |
| `Ctrl+W` + direction | Window management | PR #6 |
| `Ctrl+T` | New tab/pane | Future |
| `:` | Command palette | Future |
| `/` | Search | Future |
| `?` | Help overlay | PR #7 |

## Action Reference

Available actions that can be bound to keys:

| Action | Description |
|--------|-------------|
| `Quit` | Exit the application |
| `ToggleFocus` | Switch focus to the next pane |
| `FocusNext` | Move focus forward (same as ToggleFocus with 2 panes) |
| `FocusPrev` | Move focus backward |
| `None` | Key handled but no action taken |

## Adding a New Binding

1. Add the binding to this document FIRST (in "Reserved" if planning, "Current" if implementing)
2. Update `AppKey` enum if a new key type is needed (`cli-ide-workbench/src/input.rs`)
3. Register the binding in `KeybindingRouter::new()` (`cli-ide-workbench/src/keybinding.rs`)
4. Add tests verifying the binding works
5. Update the "Since" column in this document

## Removing a Binding

1. Move the binding from "Current" to a "Deprecated" section (or remove entirely if never released)
2. Remove registration from `KeybindingRouter::new()`
3. Update tests
4. Document the removal in CHANGELOG.md

## Implementation Files

Keybinding-related code lives in:

- `cli-ide-workbench/src/input.rs` - `AppKey` and `AppEvent` enums
- `cli-ide-workbench/src/keybinding.rs` - `KeybindingRouter` and `Action` enum
- `cli-ide-workbench/src/app.rs` - Event dispatch and action execution
- `cli-ide-demo/src/main.rs` - Crossterm key translation
