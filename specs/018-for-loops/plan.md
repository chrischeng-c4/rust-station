# Implementation Plan: For Loop Iteration (for/in/do/done)

**Branch**: `018-for-loops` | **Date**: 2025-12-06 | **Spec**: [Feature Specification](spec.md)
**Feature Number**: 018 | **Priority**: P0 | **Category**: Control Flow (017-026)

## Summary

Extend the rush shell parser and executor to support for/in/do/done loops, enabling users to iterate over lists, command outputs, and file globs. Builds directly on if/then/else (feature 017) to enable complete scripting support. Implements POSIX-compliant for loop syntax with proper variable scoping and list expansion.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**:
  - Existing parser and AST infrastructure (features 001, 017)
  - reedline (line editing) for multiline loop input
  - tokio (async runtime)
  - No new external dependencies

**Storage**: N/A
**Testing**: cargo test (integration tests with test fixtures)
**Target Platform**: macOS (MVP scope)
**Performance Goals**:
  - Parse for loops: <1ms per statement
  - Loop iteration overhead: <5ms per item vs direct commands
  - Support unlimited nesting depth

**Constraints**:
  - 100% regression test pass rate required
  - POSIX for loop syntax compliance
  - Variable expansion must work in all loop contexts

**Scale/Scope**:
  - Support basic for/in lists (words, variables, command output)
  - Support globbing in lists
  - Support nested loops
  - Support proper variable scoping

## Constitution Check

✅ **Principle I: Performance-First** - PASS
- No blocking I/O; execution overhead <5ms per item
- Single-pass parser, no backtracking

✅ **Principle II: Zero-Config Philosophy** - PASS
- For loops work immediately; no configuration needed
- Sensible defaults for variable scoping

✅ **Principle III: Progressive Complexity** - PASS
- Basic for/in/do/done is simple
- Advanced features (range expansion via feature 030) are opt-in

✅ **Principle IV: Modern UX** - N/A
- Syntax highlighting for keywords (via reedline)

✅ **Principle V: Rust-Native** - PASS
- Pure Rust, no new dependencies
- Extends existing parser

## Project Structure

### Documentation
```text
specs/018-for-loops/
├── spec.md
├── plan.md (this file)
├── tasks.md (created by /speckit.tasks)
└── checklists/requirements.md
```

### Source Code (Rust crates/rush/)
```text
crates/rush/src/
├── parser/
│   ├── lexer.rs (ADD: for, in, do, done keywords)
│   ├── ast.rs (ADD: ForStatement node)
│   ├── parser.rs (ADD: parse_for_statement())
│   └── mod.rs
├── executor/
│   └── lib.rs (ADD: evaluate_for_loop())
└── shell/repl.rs (UPDATE: multiline for...done handling)

tests/
├── integration/
│   ├── for_loops_basic.rs (US1)
│   ├── for_loops_command_subst.rs (US2)
│   ├── for_loops_nested.rs (US3)
│   └── for_loops_edge_cases.rs
└── fixtures/scripts/for-loops/
    ├── basic_iteration.sh
    ├── command_substitution.sh
    ├── globbing.sh
    ├── nested_loops.sh
    └── edge_cases.sh
```

**Structure**: Single crate, modular parser/executor extensions.

## Complexity Tracking

No constitution violations. Feature maintains all principles.

## Deployment Strategy

### Selected Strategy: PR per User Story + Nested Loops (Option 2)

Feature has 6 user stories; PR1 is foundation, PR2-7 implement stories sequentially, PR8 is polish.

**PR Structure**:

1. **PR #1**: Foundation - Parser AST & Keywords (~400 lines)
   - Add for/in/do/done keywords to lexer
   - Add ForStatement AST node
   - Add parse_for_statement() parser function
   - Unit tests

2. **PR #2 [US1]**: Basic List Iteration (~900 lines)
   - Executor: evaluate_for_loop() function
   - REPL: multiline for...done support
   - Integration tests: US1 acceptance scenarios

3. **PR #3 [US2]**: Command Substitution & Globbing (~800 lines)
   - List expansion with command substitution
   - Glob pattern support
   - Integration tests: US2 scenarios

4. **PR #4 [US3]**: Nested Loops (~700 lines)
   - Recursive loop parsing and execution
   - Variable scoping for nested contexts
   - Integration tests: nesting scenarios

5. **PR #5 [US4]**: Loop Variable Expansion (~600 lines)
   - Variable substitution in loop contexts
   - Proper quoting/escaping support
   - Integration tests: variable handling

6. **PR #6 [US5]**: Range Expansion (~500 lines) - OPTIONAL or defer to feature 030
   - Brace range expansion {1..5}
   - Character ranges {a..z}

7. **PR #7 [US6]**: Array Iteration (~400 lines)
   - Array element iteration
   - Edge cases and error handling
   - Integration tests

8. **PR #8**: Polish & Validation (~300 lines)
   - Performance benchmarks
   - Documentation
   - Regression testing

**Total**: ~5,200 lines across 8 PRs (MVP: PR1-5 = ~3,400 lines)

### Implementation Order

Sequential for MVP:
1. PR #1: Foundation
2. PR #2: Basic iteration
3. PR #3: Command substitution
4. PR #4: Nesting (enables realistic scripts)

Then parallel or sequential:
5-8. Remaining user stories and polish

**Validation**: `cargo test && cargo clippy` after each PR
**Performance**: Verify <1ms parse, <5ms/item execution overhead
