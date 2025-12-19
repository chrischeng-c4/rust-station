# Technology Research: rush Shell MVP

**Date**: 2025-11-14
**Feature**: rush Shell MVP
**Purpose**: Resolve technology choices for implementation

---

## Research Task 1: REPL Library Choice

**Question**: Should we use rustyline, reedline, or custom implementation?

### Decision: **reedline**

### Rationale:

**reedline** is the clear choice for rush because:

1. **Modern Architecture**: Built specifically for nu shell with lessons learned from rustyline
2. **Performance**: Designed for real-time syntax highlighting without blocking
3. **Flexibility**: Highly customizable highlighters, validators, and completers
4. **Active Development**: Maintained by nushell team, regular updates
5. **Cross-term Integration**: Works seamlessly with crossterm (our terminal library choice)

### Alternatives Considered:

**rustyline** (rejected):
- ❌ Older codebase, less flexible architecture
- ❌ Syntax highlighting is more limited, can cause input lag
- ❌ Harder to customize for shell-specific needs
- ✅ More mature, stable
- ✅ Simpler API

**Why rejected**: rustyline's architecture predates modern requirements for real-time highlighting. reedline was specifically built to solve these limitations.

**Custom implementation** (rejected):
- ✅ Total control over behavior
- ✅ Could optimize for exact use case
- ❌ 2-4 months development time
- ❌ Reinventing mature functionality
- ❌ Ongoing maintenance burden
- ❌ Edge cases (Unicode, terminal quirks) already solved in reedline

**Why rejected**: Violates constitution Principle V (Rust-Native: prefer mature crates). reedline already solves our problems.

### Implementation Notes:

```rust
// Basic reedline setup for rush
use reedline::{Reedline, Signal, DefaultPrompt};

let mut line_editor = Reedline::create()
    .with_highlighter(Box::new(RushHighlighter::new()))
    .with_completer(Box::new(RushCompleter::new()))
    .with_hinter(Box::new(RushHinter::new()))
    .with_history(Box::new(FileBackedHistory::new()));
```

---

## Research Task 2: Syntax Highlighting Approach

**Question**: tree-sitter vs regex-based vs custom lexer?

### Decision: **Custom Lexer (shell-specific)**

### Rationale:

**Custom shell lexer** provides the best balance for rush:

1. **Performance**: Simple lexer <1ms for typical commands, meets 16ms requirement
2. **Simplicity**: Shell syntax is relatively simple (compared to programming languages)
3. **Control**: Exact highlighting rules for shell patterns
4. **Size**: Minimal code, no heavy dependencies
5. **Zero-config**: No grammar files, just Rust code

### Alternatives Considered:

**tree-sitter** (rejected):
- ✅ Robust parsing, handles complex syntax
- ✅ Incremental parsing for large inputs
- ❌ Heavyweight dependency (~500KB+ for shell grammar)
- ❌ Overkill for shell syntax (not a programming language)
- ❌ Slower startup due to grammar loading
- ❌ Violates Principle I (Performance-First: minimize overhead)

**Why rejected**: Shell syntax is simpler than programming languages. tree-sitter is designed for complex languages (Rust, JavaScript) with deep nesting and complex scoping. Shells have flat syntax (command + args). Custom lexer is faster and lighter.

**Regex-based** (rejected):
- ✅ Simple to implement
- ❌ Performance unpredictable with complex patterns
- ❌ Hard to maintain as syntax grows
- ❌ Regex backtracking can cause lag on pathological inputs

**Why rejected**: Regex can have performance issues. Custom lexer with hand-written state machine is faster and more predictable.

### Implementation Approach:

```rust
// Simplified shell lexer
enum TokenType {
    Command,      // First word or word after |, &&, ||, ;
    Flag,         // Starts with - or --
    Path,         // Contains / or looks like file
    String,       // Quoted with " or '
    Operator,     // |, &&, ||, ;, >, >>
    Invalid,      // Command not in PATH
}

// Fast single-pass lexer
fn lex_command(input: &str, path_checker: &PathChecker) -> Vec<(Range<usize>, TokenType)> {
    // State machine: ~50 lines, <1ms for typical input
}
```

**Token categories (5 colors as per spec)**:
1. Command (valid: green, invalid: red)
2. Flag (blue)
3. Path (cyan)
4. String (yellow)
5. Operator (white/default)

---

## Research Task 3: Terminal Control Library

**Question**: crossterm vs termion vs rustyline's built-in?

### Decision: **crossterm**

### Rationale:

**crossterm** is the optimal choice:

1. **Cross-platform Foundation**: Even though MVP is macOS-only, using crossterm prepares for Linux/Windows
2. **reedline Integration**: reedline is built on crossterm, natural fit
3. **Full-featured**: All terminal capabilities we need (colors, cursor, events)
4. **Active Maintenance**: Regular updates, responsive maintainers
5. **Pure Rust**: No unsafe bindings, aligns with Principle V

### Alternatives Considered:

**termion** (rejected):
- ✅ Lightweight, fast
- ✅ Unix-focused (good for macOS)
- ❌ Unix-only (blocks future Windows support)
- ❌ Less active development than crossterm
- ❌ Doesn't integrate with reedline

**Why rejected**: Since we chose reedline (which uses crossterm), termion would create dependency conflicts. crossterm's cross-platform nature is valuable for post-MVP.

**rustyline's built-in** (rejected):
- ✅ Integrated solution
- ❌ We're not using rustyline (using reedline instead)
- ❌ Less flexible for custom terminal control

**Why rejected**: N/A - we're using reedline, not rustyline.

---

## Research Task 4: History Storage Format

**Question**: Plain text vs SQLite vs custom binary?

### Decision: **Plain text (newline-delimited)**

### Rationale:

**Plain text format** is the pragmatic choice:

1. **Simplicity**: One command per line, append-only
2. **Fast Startup**: No database initialization overhead
3. **Human-readable**: Users can inspect/edit history file
4. **Atomic Writes**: Use temp file + rename for crash safety
5. **Lightweight**: No SQLite dependency (~1MB)
6. **Fish Compatibility**: Similar to fish shell's approach

### Alternatives Considered:

**SQLite** (rejected):
- ✅ Powerful querying (timestamps, working directory, exit codes)
- ✅ ACID guarantees
- ✅ Handles corruption better
- ❌ Adds 1MB+ to binary size
- ❌ Slower startup (database initialization)
- ❌ Overkill for simple command list
- ❌ Violates Principle I (Performance-First: minimize startup time)

**Why rejected**: For MVP, we only need "list of previous commands". SQLite's benefits (complex queries, relationships) aren't needed. Can add metadata in future with extended plain text format (e.g., `command|timestamp|exit_code`).

**Custom binary format** (rejected):
- ✅ Compact, fast parsing
- ❌ Not human-readable
- ❌ Development time to design format
- ❌ Need tooling to inspect/debug

**Why rejected**: Plain text is "good enough" and debuggable. Binary format is premature optimization.

### Implementation Approach:

```rust
// History file: ~/.config/rush/history
// Format: One command per line, newest at bottom

struct HistoryManager {
    path: PathBuf,
    entries: VecDeque<String>,  // In-memory cache
    max_entries: usize,         // 10,000 default
}

impl HistoryManager {
    fn append(&mut self, cmd: &str) -> Result<()> {
        // 1. Add to in-memory cache
        self.entries.push_back(cmd.to_string());

        // 2. Write to temp file
        let temp = self.path.with_extension("tmp");
        fs::write(&temp, self.entries.join("\n"))?;

        // 3. Atomic rename (crash-safe)
        fs::rename(&temp, &self.path)?;

        Ok(())
    }
}
```

**Crash Safety**: Use temp file + atomic rename. If crash occurs mid-write, original history file remains intact.

---

## Research Task 5: Job Control Implementation (macOS)

**Question**: How to implement job control on macOS?

### Decision: **Process Groups + SIGTSTP/SIGCONT**

### Rationale:

Standard Unix job control mechanisms work on macOS:

1. **Process Groups**: Each job gets its own process group ID (PGID)
2. **Signals**: SIGTSTP (suspend), SIGCONT (continue), SIGCHLD (status changes)
3. **Terminal Control**: tcsetpgrp() to switch foreground process group
4. **No FFI**: Rust's std::process and nix crate provide safe wrappers

### Implementation Approach:

```rust
use nix::sys::signal::{self, Signal};
use nix::unistd::{setpgid, tcsetpgrp, Pid};

struct Job {
    id: usize,
    pid: Pid,
    pgid: Pid,
    command: String,
    state: JobState,
}

enum JobState {
    Running,
    Suspended,
    Completed(i32),  // exit code
}

impl JobManager {
    fn spawn_foreground(&mut self, cmd: &str) -> Result<i32> {
        let child = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()?;

        let pid = Pid::from_raw(child.id() as i32);

        // Put child in its own process group
        setpgid(pid, pid)?;

        // Give terminal control to child
        tcsetpgrp(STDIN_FILENO, pid)?;

        // Wait for completion
        let status = child.wait()?;

        // Take back terminal control
        tcsetpgrp(STDIN_FILENO, rush_pgid)?;

        Ok(status.code().unwrap_or(1))
    }

    fn suspend_foreground(&mut self) {
        // Triggered by Ctrl+Z
        signal::kill(current_fg_pid, Signal::SIGTSTP)?;
        tcsetpgrp(STDIN_FILENO, rush_pgid)?;
    }

    fn resume_foreground(&mut self, job_id: usize) {
        let job = &mut self.jobs[job_id];
        signal::kill(job.pgid, Signal::SIGCONT)?;
        tcsetpgrp(STDIN_FILENO, job.pgid)?;
        job.state = JobState::Running;
    }

    fn resume_background(&mut self, job_id: usize) {
        let job = &mut self.jobs[job_id];
        signal::kill(job.pgid, Signal::SIGCONT)?;
        job.state = JobState::Running;
        // Don't give terminal control
    }
}
```

**Key macOS Considerations**:
- Use `nix` crate for signal handling (pure Rust, no unsafe)
- Handle SIGCHLD to detect job completion
- Properly restore terminal state to rush after job suspend/complete
- Background jobs: spawn in separate process group, don't call tcsetpgrp

---

## Research Task 6: Testing Strategy

**Question**: How to test interactive shell behavior?

### Decision: **Layered Testing Strategy**

### Rationale:

Different test types for different concerns:

1. **Unit Tests**: Test individual modules (lexer, history, completions) with cargo test
2. **Integration Tests**: Test combined behavior using expect-style automation
3. **Benchmark Tests**: Measure performance with criterion

### Implementation Approach:

#### Unit Tests (cargo test)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_command() {
        let tokens = lex_command("ls -la /etc", &PathChecker::new());
        assert_eq!(tokens[0].1, TokenType::Command);
        assert_eq!(tokens[1].1, TokenType::Flag);
        assert_eq!(tokens[2].1, TokenType::Path);
    }

    #[test]
    fn test_history_persistence() {
        let mut hist = HistoryManager::new_temp()?;
        hist.append("echo hello")?;
        hist.append("ls -la")?;

        // Reload from disk
        let hist2 = HistoryManager::load(hist.path())?;
        assert_eq!(hist2.get(0), Some("echo hello"));
        assert_eq!(hist2.get(1), Some("ls -la"));
    }
}
```

#### Integration Tests (rexpect)

```rust
// Use rexpect crate for expect-style testing
use rexpect::spawn;

#[test]
fn test_repl_command_execution() {
    let mut p = spawn("./target/debug/rush", Some(5000)).unwrap();

    // Wait for prompt
    p.exp_string("$ ").unwrap();

    // Send command
    p.send_line("echo hello").unwrap();

    // Verify output
    p.exp_string("hello").unwrap();
    p.exp_string("$ ").unwrap();
}

#[test]
fn test_history_navigation() {
    let mut p = spawn("./target/debug/rush", Some(5000)).unwrap();

    p.exp_string("$ ").unwrap();
    p.send_line("echo first").unwrap();

    p.exp_string("$ ").unwrap();
    p.send_line("echo second").unwrap();

    p.exp_string("$ ").unwrap();
    p.send("\x1b[A").unwrap();  // Up arrow
    p.exp_string("echo second").unwrap();
}
```

#### Benchmark Tests (criterion)

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_startup(c: &mut Criterion) {
    c.bench_function("startup", |b| {
        b.iter(|| {
            let start = Instant::now();
            let output = Command::new("./target/release/rush")
                .arg("-c")
                .arg("exit")
                .output()
                .unwrap();
            let duration = start.elapsed();
            assert!(duration < Duration::from_millis(100),
                   "Startup took {:?}, exceeds 100ms requirement", duration);
        })
    });
}

fn bench_syntax_highlighting(c: &mut Criterion) {
    let highlighter = RushHighlighter::new();
    c.bench_function("highlight_command", |b| {
        b.iter(|| {
            highlighter.highlight(
                black_box("ls -la /etc/passwd | grep root | wc -l"),
                0
            )
        })
    });
}

criterion_group!(benches, bench_startup, bench_syntax_highlighting);
criterion_main!(benches);
```

**Testing Tools**:
- `cargo test` - unit and integration tests
- `rexpect` - expect-style interactive testing
- `criterion` - performance benchmarking
- `tempfile` - temporary test fixtures

---

## Summary of Decisions

| Decision Point | Choice | Key Reason |
|----------------|--------|------------|
| REPL Library | **reedline** | Modern, flexible, built for shells |
| Syntax Highlighting | **Custom Lexer** | Simple, fast, shell-specific |
| Terminal Control | **crossterm** | Integrates with reedline, cross-platform ready |
| History Storage | **Plain Text** | Simple, fast startup, human-readable |
| Job Control | **Process Groups + Signals** | Standard Unix approach, works on macOS |
| Testing | **Layered Strategy** | Unit tests + integration (rexpect) + benchmarks (criterion) |

All decisions align with constitution principles:
- ✅ Performance-First: Fast startup, minimal dependencies
- ✅ Rust-Native: Pure Rust, using mature crates
- ✅ Zero-Config: Simple defaults, no setup required

---

## Updated Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**:
- `reedline` - REPL with line editing, highlighting, completions
- `crossterm` - Terminal control (colors, cursor, events)
- `tokio` - Async runtime for non-blocking I/O
- `toml` - Configuration file parsing
- `nix` - Unix system calls (job control, signals)
- `rexpect` - Integration testing (dev dependency)
- `criterion` - Performance benchmarks (dev dependency)

**Storage**: Plain text file for history (~/.config/rush/history), TOML for optional config (~/.config/rush/rush.toml)
**Testing**: cargo test (unit), rexpect (integration), criterion (benchmarks)

All "NEEDS CLARIFICATION" items resolved. Ready to proceed to Phase 1: Design & Contracts.
