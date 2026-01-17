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

1. ~~**Write unit tests**~~ ✅ – Cover the event system and service container with unit tests. (11 tests added)
2. **Add logging/instrumentation** – Introduce logging statements to aid debugging.
3. ~~**Improve README and documentation**~~ ✅ – Flesh out usage instructions and architecture diagrams.
4. **Explore reference implementations** – Use `_refs` to store code and ideas from other projects (e.g., VS Code, Shaku, Ratatui examples).
5. **Set up CI/CD pipeline** – Add GitHub Actions for automated testing, linting, and builds. (See `docs/milestone1/pr2-ci-plan.md`)

---

## Completed Items Summary

| Category | Completed | Remaining |
|----------|-----------|-----------|
| P0 Tasks | 4/4       | 0         |
| P1 Tasks | 4/4       | 0         |
| P2 Tasks | 2/5       | 3         |

---

Update this roadmap as tasks are completed or as new tasks are discovered during development.
