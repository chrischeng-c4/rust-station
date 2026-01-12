# rustation - GPUI Native Desktop App

# Build the GPUI application
build:
    cargo build --workspace

# Run the GPUI application
dev:
    cargo run -p rstn

# Run the GPUI application (release mode)
run:
    cargo run -p rstn --release

# Run all Rust tests
test:
    cargo test --workspace

# Run Rust unit tests only
test-unit:
    cargo test --workspace --lib

# Run clippy linter
lint:
    cargo clippy --workspace -- -D warnings

# Format all Rust code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Build release binary
build-release:
    cargo build --workspace --release

# Clean build artifacts
clean:
    cargo clean

# Install rstn to ~/.local/bin
install: build-release
    mkdir -p ~/.local/bin
    cp target/release/rstn ~/.local/bin/rstn
    @echo "Installed rstn to ~/.local/bin/rstn"
    @echo "Usage: rstn  # Open current directory"

# Build and run in one command
dev-build:
    cargo build -p rstn && cargo run -p rstn

# Watch for changes and rebuild (requires cargo-watch)
watch:
    cargo watch \
        --delay 0.5 \
        --ignore target \
        --ignore .git \
        --ignore "*.plist" \
        -x 'build -p rstn' \
        -x 'run -p rstn'

# Watch with debug logging
watch-debug:
    RUST_LOG=debug RUST_BACKTRACE=1 \
    cargo watch \
        --delay 0.5 \
        --ignore target \
        -x 'build -p rstn' \
        -x 'run -p rstn'

# Watch and run tests on changes
watch-test:
    cargo watch -x 'test --package rstn-core --lib'

# Run fast unit tests (rstn-core only, ~0.5s)
test-fast:
    cargo test --package rstn-core --lib

# Run specific test by name
test-name NAME:
    cargo test --package rstn-core --lib {{NAME}} -- --nocapture

# Run tests with verbose output
test-verbose:
    cargo test --package rstn-core --lib -- --nocapture --test-threads=1

# Run all tests with timing info
test-all:
    #!/bin/bash
    if xcrun --find metal &>/dev/null; then
        echo "✅ Xcode available - running all tests including UI"
        cargo test --workspace
    else
        echo "⚠️  Xcode not found - running unit tests only"
        cargo test --package rstn-core --lib
    fi
