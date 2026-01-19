# Paradiddle.rs Development Commands
# Run `just --list` to see available commands

# Default recipe: show help
default:
    @just --list

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
test:
    cargo test --all --all-features

# Run tests with output
test-verbose:
    cargo test --all --all-features -- --nocapture

# Run tests for a specific crate
test-crate crate:
    cargo test -p {{crate}}

# Build debug
build:
    cargo build

# Build release
build-release:
    cargo build --release

# Run the demo (requires interactive terminal)
demo:
    cargo run -p cli-ide-demo

# Run full CI suite locally
ci: fmt-check clippy test build-release
    @echo "✅ All CI checks passed!"

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Check for security vulnerabilities
audit:
    cargo audit

# Run cargo-deny checks
deny:
    cargo deny check

# Generate code coverage report
coverage:
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Watch for changes and run tests
watch:
    cargo watch -x test

# Create a new ADR
adr name:
    @echo "Creating ADR: {{name}}"
    @MAX_NUM=$(ls docs/adr/*.md 2>/dev/null | grep -v template | sed -n 's/.*\/\([0-9]\{4\}\)-.*/\1/p' | sort -n | tail -1); \
    NEXT_NUM=$$((10#$${MAX_NUM:-0} + 1)); \
    PADDED=$$(printf "%04d" $$NEXT_NUM); \
    if [ -f "docs/adr/$${PADDED}-{{name}}.md" ]; then \
        echo "Error: docs/adr/$${PADDED}-{{name}}.md already exists"; exit 1; \
    fi; \
    cp docs/adr/0000-template.md "docs/adr/$${PADDED}-{{name}}.md"; \
    echo "Created: docs/adr/$${PADDED}-{{name}}.md"
