# Implementation Plan: Glob Pattern Expansion

**Feature**: 009-globbing
**Date**: 2025-11-27
**Spec Version**: 1.0

## Constitution Alignment Check

| Principle | Alignment | Notes |
|-----------|-----------|-------|
| Performance-First | ✅ | Use efficient glob crate, lazy directory reading |
| Zero-Config | ✅ | Works out of the box, POSIX-compliant defaults |
| Progressive Complexity | ✅ | Basic globs first, extended later if needed |
| Correctness | ✅ | Match bash/zsh behavior for standard patterns |
| Rust-Native | ✅ | Use `glob` crate (pure Rust, well-maintained) |

## Technical Approach

### Architecture Decision: Use `glob` Crate

**Decision**: Use the `glob` crate rather than manual implementation.

**Rationale**:
- Well-tested (10+ years, millions of downloads)
- Handles edge cases (unicode, symlinks, permissions)
- Pure Rust, no system dependencies
- Follows POSIX semantics
- Saves ~500 lines of complex pattern matching code

**Alternative Considered**: Manual implementation
- Pro: No new dependency
- Con: High complexity, edge case risk, maintenance burden
- Rejected: Risk outweighs benefit for this scope

### Integration Architecture

```
Input: "ls *.rs file?.txt"
         ↓
    Tokenization (existing)
         ↓
    Quote Processing (existing)
         ↓
  → Glob Expansion (NEW) ←
         ↓
    Variable Expansion (existing)
         ↓
    Command Execution
```

### New Module: `crates/rush/src/executor/glob.rs`

```rust
//! Glob pattern expansion module
//!
//! Expands wildcard patterns in command arguments to matching file paths.

use glob::{glob, GlobError, Pattern};
use std::path::PathBuf;

/// Expand glob patterns in a list of arguments
pub fn expand_globs(args: &[String]) -> Vec<String> {
    args.iter()
        .flat_map(|arg| expand_single_glob(arg))
        .collect()
}

/// Expand a single argument if it contains glob patterns
fn expand_single_glob(arg: &str) -> Vec<String> {
    // Check if argument is quoted (don't expand)
    // Check if argument contains glob characters
    // If yes, expand; if no, return as-is
    // If no matches, return literal pattern (POSIX)
}

/// Check if string contains unescaped glob characters
pub fn contains_glob_chars(s: &str) -> bool {
    // Check for *, ?, [
}
```

### Parser Integration

Modify `parse_command_line()` in `parser.rs`:

```rust
pub fn parse_command_line(line: &str) -> Result<(String, Vec<String>)> {
    let tokens = tokenize(line)?;
    let expanded_vars = expand_variables(&tokens)?;
    let expanded_globs = glob::expand_globs(&expanded_vars);  // NEW
    // ... rest of parsing
}
```

## Data Model

### GlobResult

```rust
/// Result of glob expansion for a single pattern
pub enum GlobResult {
    /// Pattern matched one or more files
    Matches(Vec<PathBuf>),
    /// Pattern matched nothing, return literal
    NoMatch(String),
    /// Pattern was quoted, don't expand
    Literal(String),
}
```

### Quoting State

Track quote state during tokenization to preserve literal patterns:

```rust
pub struct Token {
    pub value: String,
    pub quoted: bool,  // If true, don't expand globs
}
```

## Implementation Phases

### Phase 1: Foundation (US1 partial)
- Add `glob` dependency to Cargo.toml
- Create `glob.rs` module
- Implement `contains_glob_chars()`
- Implement basic `expand_single_glob()` for `*`
- Unit tests for basic expansion

### Phase 2: Full Wildcards (US1, US2)
- Complete `*` pattern handling
- Implement `?` single-character matching
- Handle no-match case (return literal)
- Integration with parser
- Unit tests for all wildcard patterns

### Phase 3: Character Classes (US3)
- Implement `[...]` pattern matching
- Implement `[^...]` negation
- Implement `[a-z]` ranges
- Unit tests for character classes

### Phase 4: Special Cases (US4, US5)
- Hidden file exclusion by default
- `.*` pattern for hidden files
- Directory glob patterns (`src/*.rs`)
- Quote preservation (don't expand quoted patterns)

### Phase 5: Polish
- Performance optimization
- Error handling refinement
- Integration tests
- Documentation

## File Changes

| File | Change Type | Description |
|------|-------------|-------------|
| `Cargo.toml` | Modify | Add `glob` dependency |
| `executor/mod.rs` | Modify | Add `pub mod glob;` |
| `executor/glob.rs` | Create | New glob expansion module |
| `executor/parser.rs` | Modify | Integrate glob expansion |

## Testing Strategy

### Unit Tests (in `glob.rs`)
- `test_contains_glob_chars()`
- `test_expand_star_pattern()`
- `test_expand_question_mark()`
- `test_expand_character_class()`
- `test_no_match_returns_literal()`
- `test_quoted_pattern_not_expanded()`
- `test_hidden_files_excluded()`

### Integration Tests (in `feature_test.rs`)
- `test_ls_star_expansion()`
- `test_glob_with_directory()`
- `test_glob_in_pipeline()`

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Performance with large directories | Low | Medium | Lazy iteration, limit results |
| Edge cases in pattern matching | Low | Low | Use well-tested `glob` crate |
| Breaking existing commands | Low | High | Extensive testing, preserve POSIX |

## Dependencies

```toml
[dependencies]
glob = "0.3"  # Stable, widely used
```

## Rollback Plan

If issues discovered:
1. Feature flag to disable glob expansion
2. Revert to passing literal patterns
3. No data persistence affected

## Success Metrics

- All unit tests pass
- All integration tests pass
- `ls *.rs` works correctly in rush shell
- No performance regression in command execution
- Coverage maintained at 85%+
