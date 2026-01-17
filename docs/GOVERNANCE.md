# Governance

This document defines the governance model for Paradiddle.rs development.

## Human-in-the-Loop Policy

**All changes require human approval before merge.**

This project uses AI assistance for:
- Code implementation
- Review response
- Documentation
- Test generation

However, AI agents **cannot**:
- Self-approve their own PRs
- Merge without human authorization
- Override human decisions
- Skip required review steps

## Merge Gates

Every PR must pass before merge:

### Required CI Checks
- [ ] `fmt` - Code formatting (rustfmt)
- [ ] `clippy` - Linting with warnings as errors
- [ ] `test` - All tests pass
- [ ] `build` - Release build succeeds

### Required Reviews
- [ ] At least one human approval
- [ ] All review comments addressed (fixed or explicitly deferred)
- [ ] Resolution table posted for any AI-addressed feedback

### Required Documentation
- [ ] PR description explains what and why
- [ ] Tests added for behavior changes
- [ ] Docs updated if public API changed
- [ ] ADR created for architectural decisions (see `DECISION_POLICY.md`)

## Decision Authority

| Decision Type | Authority |
|---------------|-----------|
| Merge PR | Human maintainer |
| Approve PR | Human reviewer |
| Defer review item | AI (with justification) + Human approval |
| Reject review item | AI (with evidence) + Human approval |
| Create ADR | Anyone, approved by maintainer |
| Release | Human maintainer |

## Branch Protection (Recommended)

After PR #2 merges, enable on `main`:

```
✓ Require pull request reviews before merging
  - Required approving reviews: 1
  - Dismiss stale reviews when new commits pushed

✓ Require status checks to pass before merging
  - Required checks: fmt, clippy, test, build

✓ Require conversation resolution before merging

✓ Do not allow bypassing the above settings
```

## Roles

### Maintainer
- Final merge authority
- Release management
- Branch protection configuration
- ADR approval

### Contributor (Human or AI)
- Implement features/fixes
- Respond to reviews
- Create PRs
- Propose ADRs

### Reviewer
- Code review
- Approve/request changes
- Verify CI passes
- Check documentation

## Escalation

If AI and human disagree:
1. AI documents its reasoning
2. Human makes final decision
3. Decision is logged in journal
4. No further argument from AI

## Amendments

Changes to governance require:
1. PR with proposed changes
2. Discussion in PR comments
3. Human maintainer approval
