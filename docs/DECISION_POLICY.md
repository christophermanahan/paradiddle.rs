# Decision Policy

This document defines when Architecture Decision Records (ADRs) are required.

## What is an ADR?

An Architecture Decision Record captures:
- **Context**: Why we faced this decision
- **Decision**: What we chose
- **Consequences**: Trade-offs and implications

ADRs live in `docs/adr/` and are numbered sequentially.

## ADR Triggers

Create an ADR when:

### Dependencies
- [ ] Adding a new external crate
- [ ] Upgrading a crate with breaking changes
- [ ] Removing a widely-used dependency
- [ ] Choosing between competing crates

### Architecture
- [ ] Creating new crate boundaries
- [ ] Changing module structure significantly
- [ ] Introducing new abstractions
- [ ] Modifying public API contracts

### Runtime Choices
- [ ] Async runtime selection (tokio, async-std, etc.)
- [ ] Storage backend decisions
- [ ] IPC mechanisms
- [ ] Serialization formats

### Security
- [ ] Authentication/authorization approaches
- [ ] Cryptographic choices
- [ ] Secret management
- [ ] Input validation strategies

### Performance
- [ ] Caching strategies
- [ ] Concurrency models
- [ ] Memory management approaches
- [ ] Algorithm selections for hot paths

## ADR Exemptions

An ADR is **not** required for:
- Bug fixes
- Test additions
- Documentation updates
- Formatting changes
- Minor refactoring
- Dependency patches (non-breaking)

## ADR Process

1. **Propose**: Create draft ADR in PR
2. **Discuss**: Review in PR comments
3. **Decide**: Maintainer approves or rejects
4. **Record**: Merge ADR with implementation

## ADR Lifecycle

```
PROPOSED → ACCEPTED → IMPLEMENTED
              ↓
         SUPERSEDED (by newer ADR)
              ↓
         DEPRECATED (no longer applies)
```

## Template

Use `docs/adr/0000-template.md` as the starting point.

## Naming Convention

```
docs/adr/NNNN-short-title.md
```

Where NNNN is a zero-padded sequential number.

Examples:
- `0001-use-crossbeam-channels.md`
- `0002-ratatui-for-tui.md`
- `0003-async-runtime-selection.md`

## Retroactive ADRs

For decisions made before this policy:
- Create ADRs when the decision is revisited
- Mark as "RETROACTIVE" in status
- Document original reasoning if known
