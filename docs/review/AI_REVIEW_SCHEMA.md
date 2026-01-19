# AI Review Schema

This document defines the structure and rules for AI-generated code reviews (Codex, etc.).

## Review Output Format

AI reviews should follow this structure:

```markdown
## Summary
Brief description of what the PR does.

## Issues Found

### R1: [Priority] Title
- **File**: path/to/file.rs:line
- **Issue**: Description of the problem
- **Suggestion**: Recommended fix
- **Rationale**: Why this matters

### R2: [Priority] Title
...

## Approval Status
- [ ] Approve
- [ ] Request Changes
- [ ] Comment Only
```

## Priority Classification

AI reviewers must classify issues using the same rubric as human reviewers:

| Priority | Meaning | Example |
|----------|---------|---------|
| P0 | Blocking - must fix | Security flaw, missing keybinding doc |
| P1 | Important - should fix | Performance issue, unclear code |
| P2 | Nice to have | Style suggestion, refactoring idea |

## Mandatory Checks

AI reviewers MUST check these items and flag violations as P0:

### 1. Keybinding Documentation Sync

**Rule**: Any PR that changes keybinding behavior MUST update `docs/input/keybindings.md`.

**Files to check for keybinding changes**:
- `cli-ide-workbench/src/input.rs` (AppKey, AppEvent)
- `cli-ide-workbench/src/keybinding.rs` (KeybindingRouter, Action)
- `cli-ide-workbench/src/app.rs` (key handling)
- `cli-ide-workbench/src/focus.rs` (focus-related bindings)
- `cli-ide-demo/src/main.rs` (key translation)

**If keybinding changes detected AND `docs/input/keybindings.md` not modified**:
```markdown
### R1: [P0] Missing keybinding documentation update
- **File**: docs/input/keybindings.md
- **Issue**: Keybinding-related code changed but documentation not updated
- **Suggestion**: Update docs/input/keybindings.md to reflect the new/changed bindings
- **Rationale**: All keybindings must be documented per project policy
```

### 2. Security Checks

Flag any:
- Unsanitized user input
- Command injection vectors
- Hardcoded credentials
- Unsafe unwrap on external data

### 3. Test Coverage

Flag if:
- New public function has no tests
- Behavior change doesn't update tests
- Test assertions are weak or missing

### 4. CI Status

Do not approve if CI is failing.

## Review Scope

AI reviewers should focus on:

- **In scope**: Code correctness, security, tests, documentation
- **Out of scope**: Subjective style preferences, architecture decisions (unless obviously wrong)

## Deference to Human Judgment

AI reviewers should:

1. Present findings clearly
2. Explain rationale
3. Defer final decision to human reviewers
4. Not block on P2 issues alone

## Response Handling

When humans respond to AI reviews:

1. AI findings may be accepted, deferred, or rejected
2. Humans must justify rejections
3. AI should not argue with human decisions
4. Track resolutions in the resolution table

## Example Review

```markdown
## Summary
This PR adds a new keybinding for help overlay.

## Issues Found

### R1: [P0] Missing keybinding documentation update
- **File**: docs/input/keybindings.md
- **Issue**: New `?` → Help binding added but not documented
- **Suggestion**: Add row to Current Bindings table in docs/input/keybindings.md
- **Rationale**: Keybinding contract requires all bindings be documented

### R2: [P1] Missing test for new binding
- **File**: cli-ide-workbench/src/keybinding.rs
- **Issue**: No test verifies the new `?` binding
- **Suggestion**: Add test_help_binding() to keybinding::tests
- **Rationale**: All bindings should have test coverage

## Approval Status
- [ ] Approve
- [x] Request Changes (P0 issue present)
- [ ] Comment Only
```
