# Implementation Plan: Brace Expansion

**Branch**: `034-brace-expansion` | **Date**: 2025-12-08 | **Spec**: [spec.md](./spec.md)

## Summary

Implement brace expansion for the rush shell, supporting comma-separated lists (`{a,b,c}`), numeric sequences (`{1..10}`), character sequences (`{a..z}`), and nested braces. Brace expansion occurs early in the expansion pipeline, before variable expansion.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: None (pure Rust implementation)
**Storage**: N/A
**Testing**: cargo test
**Target Platform**: macOS (MVP)
**Project Type**: Single crate (rush shell)
**Performance Goals**: <1ms for typical expansions, handles large ranges efficiently
**Constraints**: No new dependencies, must respect quotes/escapes
**Scale/Scope**: Single feature addition to existing expansion pipeline

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Performance-First | ✅ PASS | Pure Rust, no external deps, efficient algorithms |
| II. Zero-Config | ✅ PASS | Works immediately, no configuration needed |
| III. Progressive Complexity | ✅ PASS | Basic `{a,b}` simple; sequences/nesting advanced |
| IV. Modern UX | ✅ PASS | Standard shell feature, expected by users |
| V. Rust-Native | ✅ PASS | Pure Rust implementation, no FFI |

**All gates pass. No violations to justify.**

## Project Structure

### Documentation (this feature)

```text
specs/034-brace-expansion/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── checklists/
│   └── requirements.md  # Quality checklist
└── tasks.md             # Phase 2 output (from /speckit.tasks)
```

### Source Code (repository root)

```text
crates/rush/src/executor/
├── brace/                    # NEW: Brace expansion module
│   ├── mod.rs                # Module exports and coordination
│   ├── expander.rs           # Main entry point: expand_brace()
│   ├── lexer.rs              # Tokenize brace patterns
│   └── parser.rs             # Parse into expansion groups
├── execute.rs                # MODIFY: Add brace expansion to pipeline
├── expansion.rs              # Existing variable expansion
├── glob.rs                   # Existing glob expansion
└── arithmetic/               # Existing arithmetic expansion

crates/rush/src/executor/tests/
└── brace_tests.rs            # NEW: Brace expansion tests
```

**Structure Decision**: Follow existing arithmetic expansion module pattern. New `brace/` subdirectory mirrors `arithmetic/` and `substitution/` organization.

## Architecture

### Expansion Pipeline Integration

Current pipeline (in `execute.rs`):
```
1. Alias Expansion (line 137)
2. Variable Expansion (line 140)
3. Arithmetic Expansion (line 143)
4. Glob Expansion (line 146)
5. Command Parsing (line 149)
```

Updated pipeline with brace expansion:
```
1. Alias Expansion
2. BRACE EXPANSION (NEW - between alias and variable)
3. Variable Expansion
4. Arithmetic Expansion
5. Glob Expansion
6. Command Parsing
```

**Rationale**: Brace expansion occurs first (after aliases) per POSIX/bash order. This allows `{a,$VAR}` to work: braces expand to `a $VAR`, then variables expand.

### Module Design

#### `brace/mod.rs`
```rust
mod expander;
mod lexer;
mod parser;

pub use expander::expand_brace;
```

#### `brace/expander.rs`
Main entry point that:
1. Scans input for brace patterns (respecting quotes/escapes)
2. Expands each brace pattern found
3. Generates Cartesian product for adjacent patterns
4. Returns expanded string with multiple words

#### `brace/lexer.rs`
Tokenizes brace content:
- Identifies comma-separated elements: `{a,b,c}`
- Identifies sequences: `{1..10}`, `{a..z}`, `{1..10..2}`
- Handles nested braces: `{a,{b,c}}`
- Tracks escape state and quote context

#### `brace/parser.rs`
Parses tokenized content into expansion types:
- `BraceList { elements: Vec<String> }` - comma separated
- `NumericSeq { start, end, step, width }` - numeric range
- `CharSeq { start, end, step }` - character range

### Data Structures

```rust
/// Represents a parsed brace expression
enum BraceExpr {
    /// Comma-separated list: {a,b,c}
    List(Vec<String>),
    /// Numeric sequence: {1..10..2}
    NumericSeq {
        start: i64,
        end: i64,
        step: i64,
        width: usize,  // For zero-padding
    },
    /// Character sequence: {a..z..2}
    CharSeq {
        start: char,
        end: char,
        step: i64,
    },
    /// Literal (invalid brace pattern, no expansion)
    Literal(String),
}

/// Result of expanding a single word
struct ExpandedWord {
    /// All generated alternatives
    words: Vec<String>,
}
```

### Algorithm

1. **Scan for Braces**: Walk input character by character
   - Track quote state (single, double, none)
   - Track escape state
   - Find matching `{` and `}` pairs at top level

2. **Parse Brace Content**: For each `{...}` found
   - Check for sequence pattern: `start..end[..step]`
   - Otherwise split on commas (respecting nested braces)
   - Recursively expand nested braces

3. **Generate Expansions**:
   - For lists: each element becomes an alternative
   - For sequences: generate all values in range
   - For nested: expand inner first, then outer

4. **Combine Results**:
   - Prepend preamble, append postscript
   - Cartesian product for multiple adjacent braces
   - Return space-separated word list

### Edge Cases Handling

| Case | Behavior |
|------|----------|
| `{a}` | No expansion (single element) |
| `{}` | Literal `{}` |
| `{a,b` | Literal (unmatched) |
| `\{a,b\}` | Literal (escaped) |
| `'{a,b}'` | Literal (single quoted) |
| `"{a,b}"` | Literal (double quoted) |
| `{a..z}` (char) | Expand a b c ... z |
| `{5..1}` | Reverse: 5 4 3 2 1 |
| `{01..05}` | Zero-padded: 01 02 03 04 05 |
| `{-5..5}` | Negative support: -5 -4 ... 5 |
| `{a..5}` | Literal (mixed types) |

## Deployment Strategy

### Selected Strategy

**Option 1: Single PR** - Feature is well-scoped at ~800 lines estimated.

**Rationale**: Brace expansion is a self-contained feature. All functionality is interdependent (can't ship partial brace expansion). Tests and implementation ship together.

### Merge Sequence

1. **PR #1**: Complete brace expansion implementation
   - New `brace/` module with all components
   - Integration into `execute.rs` pipeline
   - Comprehensive test suite
   - ~800 lines estimated

### PR Size Validation

```bash
git diff --stat main  # Target: ≤ 1,500 lines
```

**Size Limits**:
- ✅ Ideal: ≤ 500 lines
- ✅ Expected: ~800 lines
- ⚠️ Maximum: ≤ 1,500 lines

## Testing Strategy

### Unit Tests
- Lexer tests: tokenization of all pattern types
- Parser tests: correct parsing of sequences and lists
- Edge case tests: quotes, escapes, invalid patterns

### Integration Tests
- Pipeline integration: brace expansion before variable expansion
- Combined expansions: `{a,b}$VAR`, `{1..3}*.txt`
- Real command execution: `echo {a,b,c}`, `mkdir -p {src,tests}`

### Test Cases from Spec

```rust
// FR-001: Basic list expansion
assert_expand("{cat,dog,bird}", "cat dog bird");

// FR-002: Preamble and postscript
assert_expand("file{1,2,3}.txt", "file1.txt file2.txt file3.txt");

// FR-004: Numeric sequence
assert_expand("{1..5}", "1 2 3 4 5");

// FR-005: Reverse sequence
assert_expand("{5..1}", "5 4 3 2 1");

// FR-006: Step increment
assert_expand("{0..10..2}", "0 2 4 6 8 10");

// FR-007: Zero padding
assert_expand("{01..05}", "01 02 03 04 05");

// FR-009: Character sequence
assert_expand("{a..e}", "a b c d e");

// FR-012: Nested braces
assert_expand("{a,b{1,2},c}", "a b1 b2 c");

// FR-014: Single element (no expansion)
assert_expand("{a}", "{a}");

// FR-016: Quoted braces
assert_expand("'{a,b}'", "{a,b}");
```

## Performance Considerations

1. **Large Ranges**: `{1..1000000}` should work but generate many words
   - Consider lazy generation for very large ranges
   - Document performance characteristics

2. **Deeply Nested**: `{a,{b,{c,{d,e}}}}`
   - Recursive expansion, stack depth proportional to nesting
   - Typical use cases have shallow nesting (1-2 levels)

3. **Memory**: Expansion can multiply word count significantly
   - `{a..z}{1..9}` = 26 × 9 = 234 words
   - Allocate result vector with estimated capacity

## Dependencies

- **009-globbing**: ✅ Complete - Brace expansion outputs feed into glob expansion
- No other dependencies required
