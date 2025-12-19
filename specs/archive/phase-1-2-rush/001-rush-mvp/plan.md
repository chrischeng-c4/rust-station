# Implementation Plan: rush Shell MVP

**Branch**: `001-rush-mvp` | **Date**: 2025-11-14 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/001-rush-mvp/spec.md`

## Summary

rush is a modern, high-performance shell for macOS that combines fish-like user experience with Rust's speed and safety. The MVP delivers an interactive REPL with real-time syntax highlighting, autosuggestions from history, tab completions, persistent command history, job control, and script execution. Built on Rust's async ecosystem, rush achieves sub-100ms startup, sub-16ms input responsiveness, and <10MB memory footprint while requiring zero configuration.

**Technical Approach**: Pure Rust implementation using reedline for REPL (modern, flexible, built for shells), tokio async runtime, and TOML parsing. Custom shell-specific lexer for syntax highlighting. File-based history storage with atomic writes for persistence. macOS-specific job control using process groups and signal handling via nix crate.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: reedline (REPL with line editing, highlighting, completions), crossterm (terminal control), tokio (async runtime), toml (config parsing), nix (Unix system calls for job control), rexpect (integration testing), criterion (benchmarks)
**Storage**: Flat file for command history (~/.config/rush/history), TOML for optional config (~/.config/rush/rush.toml)
**Testing**: cargo test (standard Rust testing), criterion (performance benchmarks), integration tests via expect-style testing
**Target Platform**: macOS 11.0+ (Big Sur and newer)
**Project Type**: Single binary project (shell application)
**Performance Goals**:
- <100ms cold startup (constitution requirement)
- <16ms keystroke-to-render (60 FPS, constitution requirement)
- <10MB baseline memory (constitution requirement)
- <5ms command execution overhead vs direct spawn (constitution requirement)
**Constraints**:
- Zero configuration required (constitution: Zero-Config Philosophy)
- No blocking I/O in REPL loop (constitution: Performance-First)
- Pure Rust, minimal FFI (constitution: Rust-Native)
- macOS-only for MVP (spec boundary)
**Scale/Scope**:
- 10,000 history entries (default limit from spec)
- Support directories with 1,000+ files for completions
- Handle command output up to terminal buffer limits
- Single-user, single-machine (no network features in MVP)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Performance-First ✅

- **Startup**: Targeting <100ms aligns with constitution requirement
- **Responsiveness**: 16ms target for syntax highlighting aligns with 60 FPS requirement
- **Memory**: 10MB baseline target matches constitution
- **Async I/O**: tokio usage ensures no blocking operations
- **Overhead**: <5ms execution overhead target matches constitution

**Status**: PASS - All performance targets directly from constitution

### Principle II: Zero-Config Philosophy ✅

- **Defaults**: All features work without config file
- **Fish-like UX**: Syntax highlighting, autosuggestions, completions enabled by default
- **Optional config**: TOML file only for customization, never required
- **30-second productivity**: User can launch and start working immediately

**Status**: PASS - Zero configuration required, TOML purely optional

### Principle III: Progressive Complexity ✅

- **Simple by default**: Basic REPL works with zero learning curve
- **Opt-in features**: Job control, scripting available but not forced
- **Discoverability**: Tab completions reveal available commands
- **No complexity cost**: Users not using job control don't pay for it

**Status**: PASS - Layered functionality, simple defaults

### Principle IV: Modern UX ✅

- **Syntax highlighting**: Real-time, 5 color categories
- **Autosuggestions**: Ghost text from history
- **Smart completions**: Commands, paths, flags
- **Visual feedback**: Clear prompt, job status indicators

**Status**: PASS - All modern UX features in MVP

### Principle V: Rust-Native ✅

- **Pure Rust**: No FFI except for minimal macOS system calls (job control)
- **Ecosystem**: Using tokio, crossterm, toml crates (mature, idiomatic)
- **Memory safety**: No unsafe code planned (will justify if needed)
- **Idiomatic**: Following Rust API guidelines

**Status**: PASS - Pure Rust with minimal FFI

### Overall Gate Status: ✅ PASS

All constitution principles satisfied. Ready to proceed with Phase 0 research.

## Project Structure

### Documentation (this feature)

```text
specs/001-rush-mvp/
├── plan.md              # This file
├── research.md          # Phase 0: Technology decisions
├── data-model.md        # Phase 1: Core entities (Command, History, Job, Config)
├── quickstart.md        # Phase 1: Development setup guide
├── contracts/           # Phase 1: Internal module contracts
│   ├── repl.md         # REPL module interface
│   ├── history.md      # History module interface
│   ├── completion.md   # Completion engine interface
│   └── executor.md     # Command executor interface
└── tasks.md             # Phase 2: Implementation tasks (created by /speckit.tasks)
```

### Source Code (repository root)

```text
crates/rush/
├── src/
│   ├── main.rs          # Entry point, CLI setup
│   ├── repl/
│   │   ├── mod.rs       # REPL orchestration
│   │   ├── input.rs     # Input handling and line editing
│   │   ├── highlight.rs # Syntax highlighting engine
│   │   └── suggest.rs   # Autosuggestion engine
│   ├── history/
│   │   ├── mod.rs       # History management
│   │   └── storage.rs   # File persistence
│   ├── completion/
│   │   ├── mod.rs       # Completion orchestration
│   │   ├── command.rs   # Command completions from PATH
│   │   ├── path.rs      # Filesystem path completions
│   │   └── flag.rs      # Flag completions
│   ├── executor/
│   │   ├── mod.rs       # Command execution
│   │   ├── job.rs       # Job control (bg, fg, jobs)
│   │   └── script.rs    # Script execution
│   ├── config/
│   │   ├── mod.rs       # Configuration loading
│   │   └── defaults.rs  # Default settings
│   └── lib.rs           # Library interface (for testing)
│
├── tests/
│   ├── integration/
│   │   ├── repl_test.rs        # End-to-end REPL tests
│   │   ├── history_test.rs     # History persistence tests
│   │   ├── completion_test.rs  # Completion behavior tests
│   │   └── job_control_test.rs # Job control tests
│   └── fixtures/
│       └── test_scripts/       # Sample shell scripts for testing
│
├── benches/
│   ├── startup.rs       # Startup time benchmarks
│   └── input.rs         # Input responsiveness benchmarks
│
└── Cargo.toml           # Dependencies and metadata
```

**Structure Decision**: Single binary project structure. rush is a standalone application, not a library, but exposes a lib.rs interface to enable comprehensive integration testing. All code lives under `crates/rush/` following the workspace structure. Modules organized by functional area (repl, history, completion, executor, config) for clear separation of concerns.

## Complexity Tracking

> No constitution violations - complexity tracking not needed for this feature.

---

## Phase 0: Research & Technology Decisions

### Research Tasks

The following unknowns from Technical Context were researched:

1. ✅ **REPL Library Choice**: **reedline** (modern, flexible, built for shells)
2. ✅ **Syntax Highlighting Approach**: **Custom lexer** (simple, fast, shell-specific)
3. ✅ **Terminal Control**: **crossterm** (integrates with reedline, cross-platform ready)
4. ✅ **History Storage Format**: **Plain text** (simple, fast startup, human-readable)
5. ✅ **Job Control Implementation**: **Process Groups + Signals** (standard Unix approach)
6. ✅ **Testing Strategy**: **Layered** (unit tests + rexpect + criterion)

**Output**: [research.md](research.md) with all decisions documented and justified

---

## Phase 1: Design & Contracts

### Artifacts Generated

1. ✅ **[data-model.md](data-model.md)** - Core entities and relationships
   - Command, HistoryEntry, Job, Config, CompletionResult
   - Validation rules, state transitions, persistence strategy

2. ✅ **[contracts/repl.md](contracts/repl.md)** - REPL module interface
   - Line editing, syntax highlighting, autosuggestions, history navigation
   - Performance requirements, error handling, testing contract

3. ✅ **[contracts/history.md](contracts/history.md)** - History module interface
   - Persistence, loading, appending, searching
   - Crash-safe atomic writes, concurrent access handling

4. ✅ **[contracts/completion.md](contracts/completion.md)** - Completion engine interface
   - Command, path, and flag completion
   - Ranking algorithm, caching strategy, performance targets

5. ✅ **[contracts/executor.md](contracts/executor.md)** - Command executor interface
   - Job control, signal handling, script execution
   - Process group management, macOS-specific implementation

6. ✅ **[quickstart.md](quickstart.md)** - Development setup guide
   - Prerequisites, build commands, testing workflow
   - Performance verification, IDE setup, troubleshooting

7. ✅ **Agent Context Updated** - CLAUDE.md updated with technology stack

### Re-evaluation: Constitution Check

*All principles remain satisfied with final technology choices:*

- ✅ **Performance-First**: reedline + custom lexer + plain text history = <100ms startup
- ✅ **Zero-Config**: No setup required, TOML purely optional
- ✅ **Progressive Complexity**: Simple REPL → job control → scripting
- ✅ **Modern UX**: reedline provides highlighting, suggestions, completions
- ✅ **Rust-Native**: Pure Rust stack (reedline, crossterm, tokio, nix)

**Final Gate Status**: ✅ PASS - Ready for task breakdown

---

## Planning Phase Complete

All design artifacts generated. Technology decisions finalized. Module contracts defined.

**Next Step**: Run `/speckit.tasks` to generate actionable implementation tasks.
