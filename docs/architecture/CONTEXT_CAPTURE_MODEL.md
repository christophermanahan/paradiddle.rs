# CLI Context Capture Model

This document defines the architecture for capturing, ingesting, and sharing CLI context in Paradiddle.rs. It establishes the mental model that will guide implementation in Phase 2 and Phase 3.

## 1. Purpose & Scope

### Why This Model Exists

CLI tools generate rich context: commands executed, their outputs, working directories, environment state, and tool-specific semantics. This context is valuable for:

- **AI assistance**: Providing LLMs with accurate, bounded context for code generation and debugging
- **Collaboration**: Sharing terminal sessions with consistent state across participants
- **Auditability**: Maintaining a clear record of what happened during a session

Without a formal model, context capture becomes ad-hoc, inconsistent, and difficult to extend.

### What This Model Covers

- Event-driven context ingestion architecture
- Taxonomy of captured events
- Capability levels for different tools and integrations
- Sharing and remote context semantics
- Security and trust boundaries

### Explicit Non-Goals

- **CRDTs for code editing**: Code synchronization uses different primitives (LSP, file watching); this model focuses on terminal/CLI context
- **Screen scraping as primary signal**: PTY output parsing is a fallback, not the primary capture mechanism
- **Real-time streaming to external services**: Context is captured locally first; sharing is explicit and controlled

## 2. High-Level Model

```
┌─────────────────────────────────────────────────────────────────┐
│                        Terminal Session                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Shell Hooks │  │ Tool Wrappers│  │ PTY Output (fallback)  │  │
│  └──────┬──────┘  └──────┬──────┘  └───────────┬─────────────┘  │
│         │                │                      │                │
│         └────────────────┼──────────────────────┘                │
│                          │                                       │
│                          ▼                                       │
│               ┌─────────────────────┐                            │
│               │   Event Ingestion   │                            │
│               │   (Single Writer)   │                            │
│               └──────────┬──────────┘                            │
│                          │                                       │
│                          ▼                                       │
│               ┌─────────────────────┐                            │
│               │  Append-Only Log    │                            │
│               │  (Event Store)      │                            │
│               └──────────┬──────────┘                            │
│                          │                                       │
│                          ▼                                       │
│               ┌─────────────────────┐                            │
│               │  LLM Context Views  │                            │
│               │  (Materialized)     │                            │
│               └─────────────────────┘                            │
└─────────────────────────────────────────────────────────────────┘
```

### Core Principles

1. **Event-driven ingestion**: All context enters the system as discrete events with timestamps and provenance
2. **Single-writer daemon**: One authority process owns the event log; clients publish events to it
3. **Append-only log**: Events are immutable once written; corrections are new events
4. **Derived views**: "LLM context" is a materialized summary computed from the event log, not the log itself

## 3. Event Taxonomy

Events are organized into three layers, from most primitive to most semantic.

### Layer 1: Primitive Events

Low-level signals from the terminal environment.

| Event | Description | Source |
|-------|-------------|--------|
| `CommandStarted` | A command began execution | Shell hook |
| `CommandFinished` | A command completed with exit code | Shell hook |
| `CwdChanged` | Working directory changed | Shell hook |
| `EnvChanged` | Environment variable modified | Shell hook |
| `PtyOutput` | Raw terminal output (bounded) | PTY |

### Layer 2: State Probes

Periodic or triggered snapshots of external state.

| Event | Description | Source |
|-------|-------------|--------|
| `GitStatus` | Repository state (branch, dirty, ahead/behind) | git adapter |
| `GitDiff` | Changes since last probe | git adapter |
| `K8sContext` | Current cluster, namespace, context | kubectl adapter |
| `CloudContext` | Active project/region/account | cloud CLI adapter |
| `BuildState` | Build system status (clean, dirty, errors) | build adapter |

### Layer 3: Semantic Events

Higher-level interpretations derived from primitives or adapters.

| Event | Description | Derived From |
|-------|-------------|--------------|
| `GitCheckout` | Branch was switched | CommandFinished + GitStatus |
| `K8sExec` | Interactive exec into pod | CommandStarted pattern match |
| `DeployStarted` | Deployment initiated | Adapter-specific |
| `TestRun` | Test suite executed with results | CommandFinished + output parsing |

## 4. Capability Levels

Not all tools provide the same level of integration. This ladder defines what we can capture at each level.

### Level 0: Metadata Only

- Command text (redacted if sensitive)
- Exit code
- Timing (start, end, duration)
- Working directory

**Available for**: Any command run through the shell.

### Level 1: Context Probes

- Tool-specific state snapshots
- Structured diffs between probes
- Triggered by relevant commands

**Available for**: Tools with query commands (git status, kubectl get, etc.).

### Level 2: Structured Outputs

- Machine-readable output (JSON, YAML)
- Parsed into typed events
- Direct tool invocation

**Available for**: Tools with `--output json` or equivalent.

### Level 3: Semantic Adapters

- Command intent detection
- Cross-command correlation
- Domain-specific event generation

**Available for**: Tools with dedicated adapters (git, k8s, major build systems).

### Level 4: Deep Integration

- Plugin/extension API integration
- Bidirectional communication
- Real-time state synchronization

**Available for**: Rare; requires tool-specific investment (LSP, DAP).

## 5. Generic Terminal Support

### PTY-Backed Sessions

Every terminal window in Paradiddle.rs is backed by a PTY (pseudo-terminal). This provides:

- Full terminal emulation
- Support for interactive programs (vim, htop, etc.)
- Standard I/O capture

### Shell Integration

Shell hooks provide the primary structured signal:

```
┌────────────────────────────────────────────────────┐
│                    Shell Process                    │
│                                                     │
│  PROMPT_COMMAND / precmd:                          │
│    - Emit CwdChanged if cwd differs                │
│    - Emit CommandFinished for previous command     │
│                                                     │
│  preexec / DEBUG trap:                             │
│    - Emit CommandStarted with command text         │
│                                                     │
│  Markers in PS1:                                   │
│    - OSC sequences for command boundaries          │
│    - Exit code in prompt                           │
└────────────────────────────────────────────────────┘
```

### Why Shell Hooks Over PTY Parsing

| Approach | Pros | Cons |
|----------|------|------|
| Shell hooks | Precise boundaries, structured data, exit codes | Requires shell integration |
| PTY parsing | Works without shell cooperation | Ambiguous boundaries, no exit codes, expensive |

**Decision**: Shell hooks are the primary mechanism. PTY parsing is opt-in fallback for unsupported shells.

## 6. Tool Wrappers & Adapters

### Generic Wrapper Contract

Any tool can be wrapped to emit events:

```
wrapper(tool, args):
  emit CommandStarted(tool, args, cwd)
  result = execute(tool, args)
  emit CommandFinished(tool, exit_code, duration)
  if adapter_exists(tool):
    emit adapter.semantic_events(tool, args, result)
  return result
```

### Tool Adapters vs Domain Adapters

| Type | Scope | Example |
|------|-------|---------|
| Tool Adapter | Single tool | `git` adapter |
| Domain Adapter | Category of tools | `version-control` adapter (git, hg, svn) |

**Priority**: Domain adapters over tool adapters. We optimize for coverage, not depth.

### Prioritized Domains

1. **Version Control** (git, hg, svn)
2. **Container Orchestration** (kubectl, docker, podman)
3. **Build Systems** (cargo, npm, make, gradle)
4. **Cloud CLIs** (aws, gcloud, az)
5. **Package Managers** (apt, brew, pip, cargo)

## 7. Fallback Strategy

### When Tools Are Unsupported

For tools without adapters:

1. **Level 0 always available**: Command, exit code, timing
2. **Level 1 if queryable**: Probe scripts can run after commands
3. **PTY parsing opt-in**: User can enable output capture for specific commands

### PTY Parsing as Last Resort

PTY output parsing is:

- **Opt-in**: User must explicitly enable
- **Bounded**: Limited buffer, automatic truncation
- **Redacted**: Sensitive patterns removed before storage
- **Expensive**: Consumes memory and CPU

### Security Implications

- Secrets may appear in terminal output
- Redaction patterns are best-effort
- Users control what gets captured
- Audit log shows what was captured (and redacted)

## 8. Shared / Remote Context

### When Context Can Be Shared

Context sharing is explicit and controlled:

| Sharing Mode | Description | Use Case |
|--------------|-------------|----------|
| Read-only view | Observer sees context, cannot modify | Pair debugging |
| Collaborative | Multiple actors, attributed events | Pair programming |
| Replay | Historical session playback | Training, review |

### Single Authority Model

Even in collaborative mode, one daemon owns the log:

```
┌─────────────┐     ┌─────────────┐
│  Local CLI  │────▶│             │
└─────────────┘     │             │
                    │   Daemon    │────▶ Event Log
┌─────────────┐     │  (Owner)    │
│ Remote CLI  │────▶│             │
└─────────────┘     └─────────────┘
```

### Actor Attribution

Every event includes:

- `actor_id`: Who generated the event
- `source`: Local, remote, or system
- `timestamp`: When it was recorded (not when it occurred)

### Mental Model

This mirrors familiar collaboration tools:

- **Live Share**: Single host, guests connect
- **tmux**: Session owner, attached clients
- **Google Docs**: One document, multiple cursors

## 9. Security & Trust

### Redaction at Ingestion

Before events are logged:

1. **Pattern matching**: Known secret formats removed
2. **Environment filtering**: Sensitive env vars excluded
3. **User rules**: Custom redaction patterns

### Bounded Transcripts

- Maximum event log size per session
- Automatic rotation and pruning
- Oldest events removed first (FIFO)

### User Opt-In

Users explicitly control:

- Which sessions are captured
- Which tools have adapters enabled
- Whether PTY output is captured
- Whether context can be shared

### Auditability

The system maintains:

- Log of what was captured
- Log of what was redacted (without content)
- Log of what was shared and with whom

## 10. Relationship to Roadmap

This document informs the design and implementation of:

| PR | Topic | Relevance |
|----|-------|-----------|
| PR #7 | Layout Primitives | Window management for terminal panes |
| PR #8 | PTY-Backed Terminals | Core terminal emulation layer |
| PR #9 | Daemon + IPC | Event ingestion infrastructure |
| PR #10+ | Adapters | Tool-specific event generation |

### PR Number Shift Note

This documentation PR (#5.1) was inserted after PR #5 to clarify the context capture model before implementation begins. As a result:

- Previously planned PR #6 (Layout Primitives) → now PR #7
- Previously planned PR #7 (Configurable Keybindings) → now PR #8
- Future PRs shift by +1

Historical PR numbers (#1–#5) are unchanged.

---

## Appendix: Glossary

| Term | Definition |
|------|------------|
| **Context** | The state and history relevant to understanding a CLI session |
| **Event** | A discrete, timestamped record of something that happened |
| **Adapter** | Code that translates tool-specific signals into semantic events |
| **Probe** | A query that captures external state at a point in time |
| **Daemon** | The single-writer process that owns the event log |
| **View** | A derived, materialized summary of the event log |
