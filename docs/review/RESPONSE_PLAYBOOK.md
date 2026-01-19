# Review Response Playbook

This playbook defines how AI agents (and human contributors) should respond to PR review feedback in the Paradiddle.rs project.

## Core Principles

1. **Human-in-the-loop**: Humans have final merge authority
2. **No self-approval**: AI cannot approve its own changes
3. **Transparency**: All decisions must be documented
4. **Minimal scope**: Address feedback without expanding PR scope

---

## Phase 1: Review Ingestion (NO CODING YET)

When review comments arrive:

1. **Read all comments** - Fetch via `gh api` or `gh pr view`
2. **Extract issues** - Create numbered list (R1, R2, R3, ...)
3. **Classify each issue**:

| Classification | Meaning | Action |
|----------------|---------|--------|
| **Accept** | Valid, will fix now | Proceed to patch |
| **Defer** | Valid, out of scope | Document where it will be handled |
| **Reject** | Incorrect or misunderstood | Explain why with evidence |

4. **Print classification table and STOP**
5. **Wait for human approval** before proceeding

---

## Phase 2: Patch Planning

For each **accepted** issue:

1. List files to change
2. Describe fix type (logic / tests / docs)
3. Specify verification command(s)
4. Keep changes minimal and focused

---

## Phase 3: Implementation

1. Apply fixes for **accepted issues only**
2. Keep diffs readable and reviewable
3. Add/update tests when behavior changes
4. Update docs only if guarantees changed

---

## Phase 4: Quality Gate

Run before committing:

```bash
cargo fmt --all
cargo test --all --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Fix only failures caused by your changes.

---

## Phase 5: Commit & PR Update

1. Commit with focused message:
   ```
   fix: address review feedback on <component>
   ```

2. Push to PR branch

3. Post resolution comment with table:

| Review ID | Status | Commit | Notes |
|-----------|--------|--------|-------|
| R1 | ✅ Fixed | abc123 | Added null check |
| R2 | ⏸️ Deferred | - | Tracked in roadmap.md |
| R3 | ❌ Rejected | - | See explanation below |

---

## Phase 6: Stop

- **DO NOT merge** - Human decides when to merge
- **DO NOT start new work** - Wait for further instructions
- **DO NOT expand scope** - Stay within PR boundaries

---

## Documentation References

When deferring work, reference:
- `docs/milestone1/roadmap.md` - Add as future task
- `docs/milestone1/journal.md` - Log the decision

When making architectural decisions:
- Consider if an ADR is needed (see `docs/DECISION_POLICY.md`)

---

## Anti-Patterns

| Don't | Do Instead |
|-------|------------|
| Fix unrelated code | Stay in scope |
| Add features during review | Create separate PR |
| Merge without human approval | Wait for maintainer |
| Ignore deferred items | Document in roadmap |
| Make assumptions | Ask if blocked |

---

## Example Workflow

```
1. Review arrives: "Event system has race condition"

2. Classification:
   R1: Race condition in emit() - ACCEPT (valid bug)
   R2: Add async support - DEFER (feature, not bug)

3. Human approves classification

4. Patch plan:
   - File: cli-ide-base/src/event.rs
   - Fix: Add mutex guard around subscriber iteration
   - Verify: cargo test -p cli-ide-base

5. Implement, test, commit

6. Post resolution table

7. STOP - await merge decision
```
