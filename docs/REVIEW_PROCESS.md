# Review Process

This document defines how code reviews work in Paradiddle.rs.

## Roles

### Author
- Creates PR with clear description
- Responds to review feedback
- Posts resolution tables
- Requests re-review after changes

### Reviewer
- Reviews code for correctness, style, and design
- Leaves actionable comments
- Approves or requests changes
- Verifies resolution of feedback

### Maintainer
- Final merge authority
- Resolves conflicts between reviewers
- Ensures governance compliance

## Review Checklist

Reviewers should verify:

### Code Quality
- [ ] Code is readable and maintainable
- [ ] No obvious bugs or logic errors
- [ ] Error handling is appropriate
- [ ] No security vulnerabilities introduced

### Testing
- [ ] Tests cover new/changed behavior
- [ ] Tests are deterministic (no flakes)
- [ ] Edge cases considered

### Documentation
- [ ] Public APIs documented
- [ ] Complex logic has comments
- [ ] README updated if needed

### Style
- [ ] Follows Rust idioms
- [ ] Consistent with existing codebase
- [ ] No unnecessary complexity

### CI
- [ ] All checks pass
- [ ] No new warnings introduced

## Consuming Codex Reviews

When Codex (or other AI reviewers) leave feedback:

1. **Don't auto-accept** - Evaluate each suggestion
2. **Check priority** - P1 issues are more critical than P2
3. **Verify correctness** - AI can be wrong
4. **Consider scope** - Some suggestions may be out of scope

### Response Flow

```
Codex Review
    ↓
Extract Issues (R1, R2, ...)
    ↓
Classify (Accept/Defer/Reject)
    ↓
Human Approves Classification
    ↓
Implement Accepted Items
    ↓
Post Resolution Table
    ↓
Human Merges
```

## Resolution Table Requirement

After addressing review feedback, post a comment with:

```markdown
| Review ID | Status | Commit | Notes |
|-----------|--------|--------|-------|
| R1 | ✅ Fixed | abc123 | Description |
| R2 | ⏸️ Deferred | - | Reason + where tracked |
| R3 | ❌ Rejected | - | Explanation |
```

This provides:
- Traceability of all feedback
- Clear status for each item
- Reference commits for fixes
- Justification for deferrals/rejections

## When to Block vs Defer

### Block (Request Changes)
- Security vulnerabilities
- Data loss risks
- Breaking changes without migration
- Missing required tests
- CI failures

### Defer (Approve with Notes)
- Nice-to-have improvements
- Refactoring suggestions
- Performance optimizations (unless critical)
- Feature additions beyond PR scope

## Review Turnaround

Target response times:
- Initial review: 24 hours
- Re-review after changes: 12 hours
- Urgent fixes: 4 hours

## Stale Reviews

If a PR has no activity for 7 days:
1. Author pings reviewers
2. If no response in 48 hours, maintainer may:
   - Assign new reviewer
   - Merge if approved and CI passes
   - Close if abandoned
