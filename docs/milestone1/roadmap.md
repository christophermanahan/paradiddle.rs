# Milestone 1 Roadmap

This roadmap enumerates the tasks for Phase 1 of the Paradiddle.rs implementation. Tasks are grouped by priority:

* **P0** – critical items that must be completed before progressing.
* **P1** – important tasks that enable the demo and usability.
* **P2** – enhancements, tests and documentation that can be completed after P0 and P1 tasks.

## P0 Tasks (Completed ✅)

1. ~~**Set up Cargo workspace**~~ ✅ – Define a multi-crate Rust workspace with `cli-ide-base`, `cli-ide-platform`, `cli-ide-workbench` and `cli-ide-demo`.
2. ~~**Implement `Event<T>` system**~~ ✅ – Create an event type with `map`, `filter` and `debounce` transformations, inspired by VS Code.
3. ~~**Implement `ServiceContainer`**~~ ✅ – Provide a basic dependency injection container with `register` and `resolve` methods.
4. ~~**Create `Window` trait**~~ ✅ – Define an interface for drawable windows that accept a `Frame` and `Rect`.

## P1 Tasks (Completed ✅)

1. ~~**Implement `EditorWindow`**~~ ✅ – A minimal editor component that displays static text; later will use a rope data structure.
2. ~~**Implement `TerminalWindow`**~~ ✅ – A stub terminal component showing placeholder output, to be backed by a PTY in later phases.
3. ~~**Basic rendering loop**~~ ✅ – Use the `ratatui` crate to create a horizontal split and render both windows side-by-side.
4. ~~**Demo application**~~ ✅ – Provide a binary crate (`cli-ide-demo`) that launches the demo, enabling users to see the editor and terminal together.

## P2 Tasks

1. ~~**Write unit tests**~~ ✅ – Cover the event system and service container with unit tests. (17 tests total)
2. **Add logging/instrumentation** – Introduce logging statements to aid debugging.
3. ~~**Improve README and documentation**~~ ✅ – Flesh out usage instructions and architecture diagrams.
4. **Explore reference implementations** – Use `_refs` to store code and ideas from other projects (e.g., VS Code, Shaku, Ratatui examples).
5. ~~**Set up CI/CD pipeline**~~ ✅ – Add GitHub Actions for automated testing, linting, and builds. (PR #2)

## PR #2: Governance & CI/CD (Completed ✅)

1. ~~**Governance documentation**~~ ✅ – Human-in-the-loop policy, merge gates, ADR requirements.
2. ~~**Review process documentation**~~ ✅ – Roles, checklists, resolution tables.
3. ~~**Decision policy**~~ ✅ – ADR triggers and process.
4. ~~**Review response playbook**~~ ✅ – How to respond to PR feedback.
5. ~~**ADR system**~~ ✅ – Template and directory structure.
6. ~~**GitHub templates**~~ ✅ – PR template, issue templates (bug, feature, design).
7. ~~**Developer experience**~~ ✅ – DEV.md guide, CHANGELOG, justfile.
8. ~~**CI workflows**~~ ✅ – fmt, clippy, test, build jobs.
9. ~~**Security workflows**~~ ✅ – gitleaks, cargo-audit, cargo-deny.
10. ~~**Coverage workflow**~~ ✅ – cargo-llvm-cov integration.
11. ~~**Performance workflow**~~ ✅ – Benchmark infrastructure.

## PR #3: UI Snapshot Tests & Performance Baselines (Completed ✅)

1. ~~**UI snapshot test harness**~~ ✅ – Render windows to offscreen buffer using TestBackend.
2. ~~**EditorWindow snapshot tests**~~ ✅ – Verify title, borders, and content.
3. ~~**TerminalWindow snapshot tests**~~ ✅ – Verify title, borders, and placeholder text.
4. ~~**Split layout tests**~~ ✅ – Verify both panes render without overlap.
5. ~~**Edge case tests**~~ ✅ – Minimum size rendering, various terminal sizes.
6. ~~**Event system benchmarks**~~ ✅ – Criterion benches for emit/subscribe/map.
7. ~~**Render benchmarks**~~ ✅ – Criterion benches for window rendering at various sizes.
8. ~~**Documentation**~~ ✅ – DEV.md updated with test and bench instructions.

## PR #4: App Core + Event Loop + Deterministic Tests (In Progress)

See `docs/milestone1/pr4-plan.md` for full planning document.

1. ~~**App struct**~~ ✅ – Owns windows, state (running, size, focused pane).
2. ~~**AppEvent/AppKey enums**~~ ✅ – Decoupled from crossterm for testability.
3. ~~**App::handle_event()**~~ ✅ – Process quit, resize, focus toggle.
4. ~~**App::render()**~~ ✅ – Render both windows with layout.
5. ~~**Demo event loop**~~ ✅ – Real poll-based loop with keyboard/resize handling.
6. ~~**Terminal guard**~~ ✅ – RAII cleanup pattern for terminal restoration.
7. ~~**Non-TTY App tests**~~ ✅ – Integration tests driving AppEvent.
8. ~~**App benchmarks**~~ ✅ – handle_event and render perf baselines.
9. ~~**Documentation**~~ ✅ – DEV.md, roadmap, journal updated.

## Future Work

### PR #5: Keybinding Router (Planned)
- Configurable key→action mapping
- Modal support (normal/insert/command)
- Help overlay

### PR #6: Layout Primitives (Planned)
- Split containers (horizontal/vertical)
- Resize splits with keyboard
- Layout serialization

### Deferred Items (From Reviews)

- **True debounce semantics** – Current implementation is leading-edge throttle; trailing-edge debounce requires async timers.

---

## Completed Items Summary

| Category | Completed | Remaining |
|----------|-----------|-----------|
| P0 Tasks | 4/4       | 0         |
| P1 Tasks | 4/4       | 0         |
| P2 Tasks | 4/5       | 1         |
| PR #2    | 11/11     | 0         |
| PR #3    | 8/8       | 0         |
| PR #4    | 9/9       | 0         |

---

Update this roadmap as tasks are completed or as new tasks are discovered during development.
