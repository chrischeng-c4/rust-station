# rustation - Electron Desktop App

# Install all dependencies and build core
setup:
    pnpm install --recursive
    cd desktop && node node_modules/electron/install.js
    cd packages/core && pnpm build

# Run Electron dev server
dev:
    @test -d desktop/node_modules || (echo "Missing dependencies. Run: just setup" && exit 1)
    cd desktop && pnpm dev

# Build Electron app
build:
    cd desktop && pnpm build

# Run all tests (unit + e2e)
test: test-rust test-e2e
    @echo "All tests passed!"

# Run Rust unit tests
test-rust:
    cargo test

# Run e2e tests
test-e2e:
    cd desktop && pnpm test:e2e

# Build napi-rs module
build-core:
    cd packages/core && pnpm build

# Build distributable app (.app bundle for macOS)
build-app: build-core build
    cd desktop && pnpm build:mac

# Install rstn CLI to ~/.local/bin
install: build-app
    mkdir -p ~/.local/bin
    ln -sf {{justfile_directory()}}/desktop/bin/rstn ~/.local/bin/rstn
    @echo "Installed rstn to ~/.local/bin/rstn"
    @echo "Usage: rstn .  # Open current directory"
