# PR #3: UI/E2E Testing & Performance Harness Plan

This document outlines the planned UI testing and performance infrastructure for Paradiddle.rs.

## Overview

PR #3 will establish testing infrastructure for:
1. Snapshot-based TUI testing
2. Performance benchmarking
3. Future PTY-based integration testing

## Snapshot-Based TUI Testing

### Strategy

Render the TUI to an offscreen buffer and compare against expected snapshots.

### Implementation Approach

```rust
// Offscreen backend for testing
use ratatui::backend::TestBackend;
use ratatui::Terminal;

fn render_to_string(width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        // Render components
    }).unwrap();

    terminal.backend().buffer().to_string()
}
```

### Test Categories

1. **Component Snapshots**
   - EditorWindow renders correctly
   - TerminalWindow renders correctly
   - Layout splits work as expected

2. **Interaction Snapshots**
   - After keypress sequences
   - After resize events
   - After content changes

3. **Edge Cases**
   - Very small terminal sizes
   - Very large terminal sizes
   - Unicode content
   - Long lines / wrapping

### Tooling Options

| Tool | Pros | Cons |
|------|------|------|
| `insta` | Inline snapshots, cargo integration | Requires review workflow |
| Custom | Full control | More code to maintain |
| `expect_test` | Lightweight | Less features |

**Recommendation**: Use `insta` crate for snapshot testing.

## Offscreen Buffer Rendering

### Benefits

- No TTY required for tests
- Deterministic output
- Fast execution
- CI-compatible

### ratatui TestBackend

```rust
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;

// Create 80x24 test terminal
let backend = TestBackend::new(80, 24);
let mut terminal = Terminal::new(backend)?;

// Render
terminal.draw(|f| { /* ... */ })?;

// Assert on buffer content
let buffer = terminal.backend().buffer();
assert_eq!(buffer.get(0, 0).symbol(), "E"); // Top-left char
```

## Performance Focus Areas

### Metrics to Track

1. **Render Time**
   - Time to render full frame
   - Time to render diff (delta)

2. **Memory Usage**
   - Heap allocations per frame
   - Peak memory during operation

3. **Event Throughput**
   - Events processed per second
   - Event latency (emit to receive)

### Benchmark Structure

```rust
// benches/render_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_render(c: &mut Criterion) {
    c.bench_function("render_80x24", |b| {
        b.iter(|| {
            // Render frame
        })
    });
}

criterion_group!(benches, benchmark_render);
criterion_main!(benches);
```

### Performance Targets (Phase 1)

| Metric | Target |
|--------|--------|
| Frame render | < 16ms (60fps capable) |
| Event latency | < 1ms |
| Memory per frame | < 1MB allocation |

## Future PTY-Based Testing

### Use Cases

- Test actual terminal interaction
- Verify ANSI escape sequences
- Test shell integration

### Approach

```rust
use portable_pty::{native_pty_system, PtySize, CommandBuilder};

fn test_with_pty() {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        ..Default::default()
    })?;

    // Spawn paradiddle in PTY
    // Send input
    // Verify output
}
```

### When to Use PTY Tests

- Integration tests only
- Not for unit tests (too slow)
- CI matrix: run on real terminals when available

## Implementation Priority

### PR #3 Scope

1. **P0 - Must Have**
   - [ ] Add `insta` dependency
   - [ ] Create snapshot test infrastructure
   - [ ] Add EditorWindow snapshot tests
   - [ ] Add TerminalWindow snapshot tests

2. **P1 - Should Have**
   - [ ] Add `criterion` benchmarks
   - [ ] Benchmark render performance
   - [ ] Add to CI (perf workflow already exists)

3. **P2 - Nice to Have**
   - [ ] Layout combination tests
   - [ ] Memory profiling setup
   - [ ] PTY test scaffolding

## Dependencies

```toml
[dev-dependencies]
insta = "1.34"
criterion = "0.5"
# Future: portable-pty = "0.8"
```

## CI Integration

The `perf.yml` workflow (added in PR #2) already handles benchmarks:
- Runs on schedule and manual trigger
- Gracefully skips if no benchmarks exist
- Will automatically pick up new benchmarks

## Notes

- Snapshot tests should be reviewed on update
- Performance regressions should block PRs (future)
- PTY tests may be platform-specific
