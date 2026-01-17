# Milestone 1 Journal

This journal is maintained by the AI agent(s) working on Phase 1 of the Paradiddle.rs rewrite.  Use this document to record what you did, what you learned, and any obstacles encountered.  Entries should be timestamped (use ISO 8601 format) and written in the first person.

## Entry Template

- **Date:** 2026-01-17
- **Task:** Brief description of the task you worked on.
- **Notes:** Any discoveries, insights, or problems faced.
- **Next Steps:** What you plan to do next.

---

<!-- Append your journal entries below this line -->

## Entry 1

- **Date:** 2026-01-17
- **Task:** Fix build errors in generated scaffold
- **Notes:** The ChatGPT-generated scaffold had several issues that prevented compilation:
  1. **Outdated ratatui version (0.20)** - The code used `ratatui::prelude::*` which was added in 0.21+. Updated to 0.28.
  2. **Deprecated API** - `Frame::size()` was renamed to `Frame::area()` in newer ratatui versions.
  3. **Invalid workspace key** - The root Cargo.toml had `edition = "2021"` at workspace level, which is not valid (edition must be specified per-crate).
  4. **Ownership bugs in event system** - The `map`, `filter`, and `debounce` methods in `cli-ide-base/src/event.rs` moved `sender` into a thread closure and then tried to use it again in the returned `Event`. Fixed by cloning the sender before moving.
  5. **Unused imports** - Removed unused `Arc` and `Mutex` imports.
- **Next Steps:** Initialize git repository, run the demo in an interactive terminal, and begin P2 tasks (unit tests).

## Entry 2

- **Date:** 2026-01-17
- **Task:** PR #1 - Bootstrap workspace builds + tests
- **Notes:** Prepared the repository for its first PR with the following changes:
  1. **Clippy compliance** - Added `Default` impl for `Event<T>` to satisfy `clippy::new_without_default`.
  2. **Unit tests for Event<T>** - Added 5 tests covering `new`, `default`, `emit/subscribe`, `map`, `filter`, and `debounce` behaviors.
  3. **Unit tests for ServiceContainer** - Added 6 tests covering `register`, `resolve`, `default`, multiple services, overwrite behavior, and thread safety.
  4. **README update** - Added Phase 1 status section, Quick Start commands, and crate overview table.
  5. **Roadmap update** - Marked all P0, P1, and 2/5 P2 tasks as complete.
  6. **Code formatting** - Ran `cargo fmt --all` to ensure consistent formatting.
- **Verification:**
  - `cargo build` - ✅ Clean
  - `cargo test` - ✅ 11 tests passing
  - `cargo clippy -D warnings` - ✅ No warnings
  - `cargo fmt --check` - ✅ Formatted
- **Next Steps:** Create GitHub repository, push PR #1, then plan PR #2 for CI/CD pipeline.

## Entry 3

- **Date:** 2026-01-17
- **Task:** Fix Event<T> broadcast semantics (Codex review feedback)
- **Notes:** The original implementation used `receiver.clone()` which in crossbeam channels results in load-balancing (only ONE receiver gets each message). Fixed to implement true publish-subscribe broadcast semantics:
  1. **New architecture** - Store subscribers as `Arc<Mutex<Vec<Sender<T>>>>` instead of a shared receiver.
  2. **subscribe()** - Now creates a fresh channel per subscriber and registers its sender.
  3. **emit()** - Fan-outs value to ALL subscribers; automatically removes disconnected senders.
  4. **map/filter/debounce** - Updated to use `upstream.subscribe()` and emit to downstream Event.
  5. **Tests** - Added 6 new tests proving broadcast semantics, dropped subscriber handling, and transformed event broadcasting.
- **Verification:**
  - `cargo test -p cli-ide-base` - ✅ 14 tests passing
  - `cargo clippy -D warnings` - ✅ No warnings
- **Lesson Learned:** Crossbeam's MPMC channels share messages among receivers (work-stealing), not broadcast. True pub-sub requires per-subscriber channels.