# Development Quickstart: rush Shell MVP

**Feature**: rush Shell MVP
**Purpose**: Get rush development environment set up quickly

---

## Prerequisites

### System Requirements

- **Operating System**: macOS 11.0 (Big Sur) or newer
- **Hardware**: M1 Mac or Intel Mac (2018+)
- **Rust**: 1.75.0 or newer
- **Terminal**: Terminal.app or iTerm2

### Install Rust

```bash
# Install rustup (Rust toolchain manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version  # Should be 1.75.0 or newer
cargo --version
```

---

## Quick Start (5 minutes)

###Step 1: Clone Repository

```bash
git clone git@github.com:chrischeng-c4/rust-station.git
cd rust-station
```

### Step 2: Checkout Feature Branch

```bash
git fetch
git checkout 001-rush-mvp
```

### Step 3: Build rush

```bash
# Debug build (faster compilation, slower runtime)
cargo build -p rush

# Run rush
./target/debug/rush
```

### Step 4: Verify It Works

```bash
# Inside rush shell:
$ echo "hello from rush"
hello from rush

$ ls -la
# Should show directory listing with syntax highlighting

# Press up arrow - should show previous command
# Type "ec" and press Tab - should complete to "echo"

# Exit
$ exit
```

---

## Development Workflow

### Build Commands

```bash
# Debug build (development)
cargo build -p rush

# Release build (performance testing)
cargo build -p rush --release

# Check without building (fast)
cargo check -p rush

# Format code
cargo fmt --all

# Lint with clippy
cargo clippy -p rush --all-targets --all-features
```

### Running rush

```bash
# Run debug build
cargo run -p rush

# Run release build
cargo run -p rush --release

# Run with logging enabled
RUST_LOG=debug cargo run -p rush
```

### Testing

```bash
# Run all tests
cargo test -p rush

# Run specific test file
cargo test -p rush --test history_test

# Run with output visible
cargo test -p rush -- --nocapture

# Run benchmarks
cargo bench -p rush
```

---

## Project Structure

```
crates/rush/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library interface
│   ├── repl/            # REPL module
│   ├── history/         # History management
│   ├── completion/      # Tab completion
│   ├── executor/        # Command execution
│   └── config/          # Configuration
├── tests/
│   └── integration/     # Integration tests
├── benches/             # Performance benchmarks
└── Cargo.toml
```

---

## Development Tips

### Fast Iteration

```bash
# Use cargo-watch for auto-rebuild on file changes
cargo install cargo-watch
cargo watch -x 'run -p rush'
```

### Debugging

```bash
# Run with debug logging
RUST_LOG=rush=debug cargo run -p rush

# Use lldb (macOS debugger)
lldb ./target/debug/rush
(lldb) run
(lldb) bt  # backtrace on crash
```

### Performance Profiling

```bash
# Build with debug symbols for profiling
cargo build -p rush --release --profile=release-with-debug

# Profile with instruments (macOS)
instruments -t "Time Profiler" ./target/release/rush

# Check startup time
time ./target/release/rush -c "exit"
# Should be < 100ms
```

---

## Common Issues

### Issue: Compilation Errors

```
error: failed to compile rush v0.1.0
```

**Solution**: Update Rust toolchain

```bash
rustup update
rustc --version  # Verify >= 1.75.0
```

### Issue: Tests Failing

```
test result: FAILED. 10 passed; 5 failed
```

**Solution**: Check if you're on the correct branch

```bash
git status
git checkout 001-rush-mvp
```

### Issue: rush Doesn't Start

```
Error: Failed to initialize terminal
```

**Solution**: Ensure running in interactive terminal (not CI/script)

```bash
# This will fail:
echo "ls" | ./target/debug/rush

# This works:
./target/debug/rush
```

---

## Performance Targets

Verify your build meets constitution requirements:

### Startup Time

```bash
# Should be < 100ms
time ./target/release/rush -c "exit"
```

### Memory Usage

```bash
# Launch rush and check memory
./target/release/rush &
ps aux | grep rush
# VSZ should be < 10MB for baseline
```

### Input Responsiveness

```bash
# Run benchmarks
cargo bench -p rush --bench input
# Keystroke latency should be < 16ms (60 FPS)
```

---

## IDE Setup

### VS Code

Install extensions:

```bash
# Rust Analyzer (LSP)
code --install-extension rust-lang.rust-analyzer

# TOML support
code --install-extension tamasfe.even-better-toml
```

`.vscode/settings.json`:

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

### CLion / RustRover

1. Open `rust-station` directory
2. CLion will auto-detect Cargo project
3. Run configurations auto-generated
4. Use "rush" run configuration to launch

---

## Testing rush Features

### Manual Test Checklist

**Basic REPL**:
- [ ] Launch rush, see prompt
- [ ] Type `echo hello`, see output
- [ ] Use arrow keys to edit input
- [ ] Press Enter to execute

**Syntax Highlighting**:
- [ ] Type `ls -la /etc` - see colors (command=green, flag=blue, path=cyan)
- [ ] Type `invalidcmd` - see red color (error)
- [ ] Type `echo "string"` - see yellow quotes

**History**:
- [ ] Execute several commands
- [ ] Press up arrow - see previous command
- [ ] Press up again - see older command
- [ ] Press down - see newer command
- [ ] Exit and relaunch - history persists

**Autosuggestions**:
- [ ] Execute `ls -la`
- [ ] Type just `ls` - see ghost text ` -la`
- [ ] Press right arrow - suggestion accepted

**Tab Completion**:
- [ ] Type `ec` and press Tab - completes to `echo`
- [ ] Type `ls /et` and press Tab - completes to `/etc`
- [ ] Type `ls -` and press Tab - shows flag options

**Job Control**:
- [ ] Run `sleep 30`
- [ ] Press Ctrl+Z - job suspended, prompt returns
- [ ] Type `jobs` - see suspended job
- [ ] Type `fg` - job resumes
- [ ] Press Ctrl+C - job terminated

**Script Execution**:
- [ ] Create file: `echo 'echo "line1"\necho "line2"' > test.sh`
- [ ] Make executable: `chmod +x test.sh`
- [ ] Run: `./test.sh`
- [ ] See both lines printed

---

## Next Steps

Once development environment is set up:

1. Read the design docs:
   - [spec.md](spec.md) - Feature specification
   - [plan.md](plan.md) - Implementation plan
   - [research.md](research.md) - Technology decisions
   - [data-model.md](data-model.md) - Core entities
   - [contracts/](contracts/) - Module interfaces

2. Review existing code (when implementation starts):
   ```bash
   # Explore codebase structure
   tree crates/rush/src

   # Read module documentation
   cargo doc -p rush --open
   ```

3. Pick a task from `tasks.md` (created by `/speckit.tasks`)

4. Make changes following spec-driven workflow:
   - All code MUST trace to requirements in spec.md
   - All architecture decisions MUST align with constitution
   - All PRs MUST reference spec artifacts

---

## Getting Help

- **Spec questions**: Check [spec.md](spec.md) for requirements
- **Technical questions**: Check [research.md](research.md) for technology decisions
- **API questions**: Check [contracts/](contracts/) for module interfaces
- **Build issues**: Check this quickstart guide

Remember: rush follows **spec-driven development**. When in doubt, refer to the specifications!
