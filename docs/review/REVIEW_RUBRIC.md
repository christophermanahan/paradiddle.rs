# Review Rubric

This document defines priority levels and mandatory rules for code reviews.

## Priority Levels

### P0 - Blocking (Must Fix Before Merge)

Issues that MUST be resolved before the PR can be merged:

1. **Security vulnerabilities** - Any code that introduces security risks
2. **Data loss or corruption** - Code that could cause irreversible damage
3. **Breaking changes without migration** - API changes that break existing users
4. **CI failures** - All checks must pass
5. **Missing required tests** - New features must have test coverage
6. **Keybinding documentation sync** - Any PR that changes keybinding behavior MUST update `docs/input/keybindings.md`

### P1 - Important (Should Fix)

Issues that should be addressed but can be deferred with justification:

1. **Performance regressions** - Significant slowdowns
2. **Code quality issues** - Unclear logic, poor naming
3. **Missing edge case handling** - Obvious gaps in error handling
4. **Documentation gaps** - Public APIs without docs

### P2 - Nice to Have (Consider Fixing)

Suggestions that improve code but are not required:

1. **Style preferences** - Alternative approaches that are equally valid
2. **Refactoring opportunities** - Code that works but could be cleaner
3. **Future-proofing** - Changes that only matter for hypothetical scenarios
4. **Optimization suggestions** - Performance improvements for non-critical paths

## Mandatory Rules

These rules always apply and cannot be waived:

### Documentation Sync Rules

| Change Type | Required Documentation Update |
|-------------|------------------------------|
| Keybinding added/modified/removed | `docs/input/keybindings.md` |
| Public API changed | Module/function docstrings |
| Architecture decision | `docs/adr/NNNN-*.md` |
| Breaking change | `CHANGELOG.md` |

### Keybinding Documentation Rule (P0)

**Any PR that changes keybinding behavior MUST update `docs/input/keybindings.md`.**

This includes:
- Adding a new keybinding
- Removing an existing keybinding
- Changing the action a key triggers
- Changing the context in which a binding applies
- Modifying `AppKey`, `Action`, or `KeybindingRouter`

The PR should be blocked until the documentation is updated.

### Test Coverage Rules

- New public functions must have at least one test
- Bug fixes should include a regression test
- Behavior changes should update existing tests

## Review Workflow

```
1. Author opens PR
   ↓
2. CI runs (must pass)
   ↓
3. Reviewer checks P0 rules
   ↓
4. If P0 violations → Request Changes
   ↓
5. Review P1/P2 items
   ↓
6. Author addresses feedback
   ↓
7. Reviewer verifies fixes
   ↓
8. Approve + Merge
```

## Escalation

If author and reviewer disagree:

1. Document both positions in PR comments
2. Request maintainer input
3. Maintainer makes final decision
4. Document decision rationale
