# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Governance documentation (`docs/GOVERNANCE.md`)
- Review process documentation (`docs/REVIEW_PROCESS.md`)
- Decision policy for ADRs (`docs/DECISION_POLICY.md`)
- Review response playbook (`docs/review/RESPONSE_PLAYBOOK.md`)
- ADR template (`docs/adr/0000-template.md`)
- GitHub issue templates (bug report, feature request, design/ADR)
- Pull request template
- Developer guide (`docs/DEV.md`)
- CI/CD workflows (fmt, clippy, test, build, security, coverage)
- `justfile` for common development commands
- `deny.toml` for dependency auditing

### Changed
- Updated roadmap with PR #2 items

### Fixed
- Event system now uses true broadcast semantics (PR #1)

## [0.1.0] - 2026-01-17

### Added
- Initial project scaffold
- Cargo workspace with 4 crates:
  - `cli-ide-base`: Event system with map/filter/debounce
  - `cli-ide-platform`: ServiceContainer for dependency injection
  - `cli-ide-workbench`: Window trait and implementations
  - `cli-ide-demo`: Demo application
- Unit tests for Event system (11 tests)
- Unit tests for ServiceContainer (6 tests)
- Basic documentation (README, CLAUDE.md)
- Milestone 1 roadmap and journal

### Technical Details
- Rust 2021 edition
- ratatui 0.28 for TUI rendering
- crossbeam for channels
- crossterm for terminal handling

---

[Unreleased]: https://github.com/christophermanahan/paradiddle.rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/christophermanahan/paradiddle.rs/releases/tag/v0.1.0
