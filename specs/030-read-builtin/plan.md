# Implementation Plan: Read Builtin

**Branch**: `030-read-builtin` | **Date**: 2025-12-07 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/030-read-builtin/spec.md`

## Summary

Implement the `read` shell builtin command for reading user input into shell variables. This is a standard POSIX builtin that enables interactive scripts and user prompts.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: reedline (already in project), std::io for terminal I/O
**Storage**: N/A (variables stored in existing VariableManager)
**Testing**: cargo test (unit + integration tests)
**Target Platform**: macOS (MVP), Linux future
**Project Type**: Single CLI project
**Performance Goals**: <5ms read initialization, instant response to input
**Constraints**: Must work in both interactive and non-interactive modes
**Scale/Scope**: Single builtin command with options

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Performance-First | PASS | Simple I/O operation, no heavy processing |
| II. Zero-Config | PASS | Works immediately with sensible defaults |
| III. Progressive Complexity | PASS | Basic read is simple, options add power |
| IV. Modern UX | PASS | Supports prompts, silent mode for passwords |
| V. Rust-Native | PASS | Pure Rust using std::io and crossterm |

## Project Structure

### Documentation (this feature)

```text
specs/030-read-builtin/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── checklists/          # Quality checklists
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
crates/rush/
├── src/
│   ├── executor/
│   │   └── builtins/
│   │       ├── mod.rs           # Add read module export
│   │       └── read.rs          # NEW: read builtin implementation
│   └── ...
└── tests/
    └── integration/
        └── read_builtin_tests.rs  # NEW: integration tests
```

**Structure Decision**: Single project, add new builtin module following existing pattern (see echo.rs, let_cmd.rs)

## Complexity Tracking

No violations - simple builtin following established patterns.

## Deployment Strategy

### Selected Strategy

**Option 1: Single PR** - entire feature ≤ 500 lines

**Rationale**:
- Single file implementation (~150-200 lines)
- Integration tests (~100-150 lines)
- Follows established builtin pattern

### Merge Sequence

1. PR: `feat(030): implement read builtin for user input` → Merge to main

### PR Size Validation

Expected: ~350 lines total (well under 500 limit)

## Implementation Design

### Core Implementation

1. **Create `read.rs` builtin module**:
   - Parse options: -p (prompt), -s (silent), -r (raw), -d (delimiter), -n (count), -a (array), -t (timeout)
   - Read input from stdin
   - Split on IFS (default: space/tab/newline)
   - Assign to variables or REPLY

2. **Register in `mod.rs`**:
   - Add `pub mod read;`
   - Add match case for "read" command

### Option Handling

| Option | Behavior |
|--------|----------|
| -p PROMPT | Display prompt before reading |
| -s | Silent mode (no echo) |
| -r | Raw mode (no backslash interpretation) |
| -d DELIM | Read until DELIM instead of newline |
| -n COUNT | Read exactly COUNT characters |
| -a ARRAY | Read into array variable |
| -t TIMEOUT | Timeout after TIMEOUT seconds |
| (none) | Read line into REPLY or specified vars |

### Key Functions

```rust
pub fn execute(executor: &mut CommandExecutor, args: &[String]) -> Result<i32>
fn parse_options(args: &[String]) -> (Options, Vec<String>)
fn read_input(options: &Options) -> Result<String>
fn split_and_assign(executor: &mut CommandExecutor, input: &str, vars: &[String])
```

### Exit Status

- 0: Success (input read)
- 1: EOF or timeout
- 2: Invalid usage

### Dependencies

Uses existing:
- `crossterm` for terminal control (silent mode)
- `std::io::stdin()` for reading
- `CommandExecutor::variable_manager_mut()` for variable assignment
