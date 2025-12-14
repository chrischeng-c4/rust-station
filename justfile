# Build release binary
build:
    cargo build --release

# Install to ~/.local/bin
install: build
    mkdir -p ~/.local/bin
    cp target/release/rstn ~/.local/bin/
    cp target/release/rush ~/.local/bin/

# Build and install
all: install
