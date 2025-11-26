# Specification: Glob Pattern Expansion

**Feature**: 009-globbing
**Status**: Draft
**Priority**: P1 (Essential)
**Created**: 2025-11-27

## Overview

Implement glob pattern expansion (wildcards) in the rush shell, enabling users to match multiple files with patterns like `*.rs`, `file?.txt`, and `[abc]*`.

## Problem Statement

Currently, rush does not expand glob patterns in command arguments. When a user types `ls *.rs`, the literal string `*.rs` is passed to `ls` rather than a list of matching files. This breaks expected shell behavior and makes file operations tedious.

## User Stories

### US1: Basic Wildcard Matching (Priority: P1)

**As a** shell user
**I want** to use `*` to match any sequence of characters
**So that** I can operate on multiple files without typing each name

**Acceptance Criteria:**
- `ls *.rs` lists all `.rs` files in current directory
- `cat file*` matches `file1.txt`, `file2.txt`, `filename.rs`, etc.
- `*` alone matches all non-hidden files in current directory
- Pattern with no matches passes literal pattern to command (POSIX behavior)

### US2: Single Character Wildcard (Priority: P1)

**As a** shell user
**I want** to use `?` to match exactly one character
**So that** I can match files with predictable naming patterns

**Acceptance Criteria:**
- `ls file?.txt` matches `file1.txt`, `fileA.txt`, but not `file10.txt`
- `?` can be combined with `*`: `?.rs` matches `a.rs` but not `ab.rs`
- Multiple `?` work: `???` matches exactly 3-character names

### US3: Character Class Matching (Priority: P2)

**As a** shell user
**I want** to use `[...]` to match specific characters
**So that** I can match files with character variations

**Acceptance Criteria:**
- `[abc]` matches exactly one of: a, b, or c
- `[a-z]` matches any lowercase letter
- `[0-9]` matches any digit
- `[!abc]` or `[^abc]` matches any character except a, b, c
- `file[0-9].txt` matches `file0.txt` through `file9.txt`

### US4: Hidden File Handling (Priority: P2)

**As a** shell user
**I want** glob patterns to exclude hidden files by default
**So that** I don't accidentally operate on dotfiles

**Acceptance Criteria:**
- `*` does NOT match files starting with `.`
- `.*` explicitly matches hidden files
- `.?*` matches hidden files with at least one character after dot

### US5: Directory Glob (Priority: P3)

**As a** shell user
**I want** to match files in subdirectories
**So that** I can operate on nested file structures

**Acceptance Criteria:**
- `src/*.rs` matches `.rs` files in `src/` directory
- `*/Cargo.toml` matches `Cargo.toml` in any immediate subdirectory
- Patterns respect directory boundaries (`*` doesn't match `/`)

## Functional Requirements

### FR1: Pattern Expansion Location
- Glob expansion MUST occur in the parser/executor before command execution
- Each glob pattern argument MUST be expanded to zero or more file paths
- Expansion MUST happen after variable expansion but before command execution

### FR2: Match Ordering
- Matched files MUST be returned in alphabetical order (locale-aware)
- Directory entries MUST be sorted case-insensitively on case-insensitive filesystems

### FR3: No Match Behavior (POSIX)
- If a pattern matches no files, the literal pattern MUST be passed to the command
- This follows POSIX shell behavior (bash default without `nullglob`)

### FR4: Quoting Escapes Globbing
- Patterns in single quotes MUST NOT be expanded: `'*.rs'` → literal `*.rs`
- Patterns in double quotes MUST NOT be expanded: `"*.rs"` → literal `*.rs`
- Backslash escapes glob characters: `\*.rs` → literal `*.rs`

### FR5: Special Characters
- `*` - Match zero or more characters (except `/`)
- `?` - Match exactly one character (except `/`)
- `[...]` - Match one character from set
- `[!...]` or `[^...]` - Match one character NOT in set
- `[a-z]` - Match character range

### FR6: Performance
- Glob expansion SHOULD complete in <100ms for directories with <10,000 entries
- Results SHOULD be cached for repeated patterns in same command line
- Directory reads SHOULD be lazy (don't read entire filesystem tree)

## Non-Functional Requirements

### NFR1: Compatibility
- Behavior MUST match bash/zsh glob semantics for basic patterns
- No extended glob (`**`, `?(pattern)`, etc.) in initial implementation

### NFR2: Error Handling
- Permission denied errors SHOULD be silently skipped (match continues)
- Broken symlinks SHOULD be included in matches (target doesn't need to exist)

## Out of Scope

- Extended globbing (`**` recursive, `?(pattern)`, `+(pattern)`)
- Brace expansion (`{a,b,c}`)
- Tilde expansion in glob patterns (already handled by cd/shell)

## Dependencies

- Existing parser infrastructure (`crates/rush/src/executor/parser.rs`)
- Filesystem access via `std::fs`
- Consider using `glob` crate or implementing manually

## Technical Notes

### Implementation Options

1. **Use `glob` crate**: Mature, well-tested, handles edge cases
2. **Manual implementation**: More control, fewer dependencies, educational

Recommendation: Use `glob` crate for correctness, wrap with rush-specific behavior.

### Integration Point

Glob expansion should occur in `parse_command_line()` or a new `expand_globs()` function called after tokenization but before command struct creation.

## Test Scenarios

1. `ls *.rs` in directory with `main.rs`, `lib.rs` → expands to both files
2. `echo *` in empty directory → outputs literal `*`
3. `ls '*.rs'` → passes literal `*.rs` to ls
4. `ls file?.txt` with `file1.txt`, `file22.txt` → only `file1.txt`
5. `ls [a-c]*` with `apple`, `banana`, `cherry`, `date` → `apple`, `banana`, `cherry`
6. `ls .*` → lists hidden files only
7. `ls src/*.rs` → lists `.rs` files in src directory

## References

- POSIX Shell Command Language: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html
- Bash Pattern Matching: https://www.gnu.org/software/bash/manual/html_node/Pattern-Matching.html
- Rust `glob` crate: https://docs.rs/glob/latest/glob/
