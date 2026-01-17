# PR #2: CI/CD Pipeline Plan

This document outlines the planned GitHub Actions CI/CD pipeline for Paradiddle.rs.

## Overview

A robust CI/CD pipeline for a maintained Rust CLI/TUI application, focusing on code quality, security, and cross-platform compatibility.

## Workflow Files

### 1. `ci.yml` - Main CI Pipeline

**Triggers:**
- `push` to `main` branch
- `pull_request` to `main` branch

**Jobs:**

#### `fmt` - Format Check
```yaml
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
  with:
    components: rustfmt
- run: cargo fmt --all --check
```

#### `clippy` - Lint Check
```yaml
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
  with:
    components: clippy
- uses: Swatinem/rust-cache@v2
- run: cargo clippy --all-targets --all-features -- -D warnings
```

#### `test` - Unit & Integration Tests
```yaml
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- uses: Swatinem/rust-cache@v2
- run: cargo test --all-features
```

#### `build` - Build Check
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
steps:
  - uses: actions/checkout@v4
  - uses: dtolnay/rust-toolchain@stable
  - uses: Swatinem/rust-cache@v2
  - run: cargo build --release
```

### 2. `security.yml` - Security Audits

**Triggers:**
- `push` to `main`
- `schedule: cron: '0 0 * * 0'` (weekly)
- `pull_request` to `main`

**Jobs:**

#### `audit` - Dependency Audit
```yaml
- uses: actions/checkout@v4
- uses: rustsec/audit-check@v2
  with:
    token: ${{ secrets.GITHUB_TOKEN }}
```

#### `deny` - License & Advisory Check
```yaml
- uses: actions/checkout@v4
- uses: EmbarkStudios/cargo-deny-action@v1
  with:
    command: check
    arguments: --all-features
```

### 3. `release.yml` - Release Pipeline (Optional)

**Triggers:**
- `push` tags: `v*.*.*`

**Jobs:**

#### `build-release`
```yaml
strategy:
  matrix:
    include:
      - os: ubuntu-latest
        target: x86_64-unknown-linux-gnu
        artifact: paradiddle-linux-x86_64
      - os: macos-latest
        target: x86_64-apple-darwin
        artifact: paradiddle-macos-x86_64
      - os: macos-latest
        target: aarch64-apple-darwin
        artifact: paradiddle-macos-aarch64
      - os: windows-latest
        target: x86_64-pc-windows-msvc
        artifact: paradiddle-windows-x86_64.exe
steps:
  - uses: actions/checkout@v4
  - uses: dtolnay/rust-toolchain@stable
    with:
      targets: ${{ matrix.target }}
  - run: cargo build --release --target ${{ matrix.target }}
  - uses: actions/upload-artifact@v4
```

#### `create-release`
```yaml
needs: build-release
- uses: softprops/action-gh-release@v1
  with:
    files: artifacts/*
    generate_release_notes: true
```

## Caching Strategy

Use `Swatinem/rust-cache@v2` for all jobs:
- Caches `~/.cargo/registry`, `~/.cargo/git`, and `target/`
- Key based on `Cargo.lock` hash
- Shared cache across jobs on same OS
- Estimated build time reduction: 50-70%

## Cross-Platform Matrix

| OS | Target | Priority |
|----|--------|----------|
| Ubuntu Latest | x86_64-unknown-linux-gnu | P0 |
| macOS Latest | x86_64-apple-darwin | P0 |
| macOS Latest | aarch64-apple-darwin | P1 |
| Windows Latest | x86_64-pc-windows-msvc | P1 |

## Tooling Recommendations

### Required
- **cargo-deny**: License compliance and advisory database checks
  - Config: `deny.toml` with allowed licenses, banned crates
- **rustsec/audit-check**: RUSTSEC advisory database integration

### Recommended
- **cargo-nextest**: Faster test runner with better output
  - Consider for large test suites
- **cargo-tarpaulin** or **llvm-cov**: Code coverage reporting
  - Integration with Codecov/Coveralls

### Optional (Security-Focused)
- **gitleaks**: Secret scanning in commits
- **semgrep**: Static analysis for security patterns
- **cargo-udeps**: Unused dependency detection

## Configuration Files Needed

### `deny.toml`
```toml
[advisories]
db-path = "~/.cargo/advisory-db"
db-urls = ["https://github.com/rustsec/advisory-db"]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Zlib",
]
copyleft = "deny"

[bans]
multiple-versions = "warn"
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
```

### `rust-toolchain.toml` (Optional)
```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

## Implementation Priority

1. **P0 - Must Have (PR #2)**
   - `ci.yml` with fmt, clippy, test, build
   - Basic caching
   - Linux + macOS matrix

2. **P1 - Should Have (PR #2 or #3)**
   - `security.yml` with cargo-deny
   - Windows matrix
   - `deny.toml` configuration

3. **P2 - Nice to Have (Future)**
   - `release.yml` for tagged releases
   - Code coverage reporting
   - Secret scanning

## Estimated CI Time

| Job | Estimated Duration (Cached) |
|-----|----------------------------|
| fmt | ~10s |
| clippy | ~30s |
| test | ~45s |
| build (per OS) | ~1m |
| security audit | ~30s |
| **Total (parallel)** | **~2m** |

## Notes

- All workflows should use `actions/checkout@v4` for security
- Pin action versions to specific SHAs for production
- Consider branch protection rules requiring CI pass
- Add status badges to README after implementation
