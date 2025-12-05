# Implementation Plan: Conditional Control Flow (if/then/else/elif/fi)

**Branch**: `017-if-then-else` | **Date**: 2025-12-06 | **Spec**: [Feature Specification](spec.md)
**Feature Number**: 017 | **Priority**: P0 | **Category**: Control Flow (017-026)

## Summary

Implement conditional control flow statements (if/then/else/elif/fi) in the rush shell to enable users to create branching logic in shell scripts and interactive commands. This is a foundational feature for scripting support, allowing users to execute different code paths based on command exit status or test conditions.

**Technical Approach**: Extend the existing parser and AST from feature 001 (MVP) to recognize and parse if/then/else/elif/fi keywords. Add corresponding AST nodes and implement evaluation logic in the executor that checks exit codes and controls flow accordingly.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**:
  - Existing parser and AST infrastructure (feature 001)
  - reedline (line editing) - for REPL
  - tokio (async runtime) - for job control integration
  - No new external dependencies required

**Storage**: N/A
**Testing**: cargo test (unit + integration tests using rush REPL harness)
**Target Platform**: macOS (MVP scope from constitution)
**Project Type**: Single monorepo (crates/rush/)
**Performance Goals**:
  - Parse if/elif/else/fi: <1ms per statement
  - Execute if statement: <5ms overhead vs direct command (aligns with constitution principle I)
  - Support unlimited nesting depth (limited only by available memory)

**Constraints**:
  - Must not break existing test suite (100% regression test pass rate required)
  - Must comply with POSIX shell if/then/else/elif/fi syntax
  - Parser must provide clear error messages for malformed constructs

**Scale/Scope**:
  - Support basic if statements (if/then/fi)
  - Support else clauses
  - Support elif chains (unlimited)
  - Support nested if statements
  - Integrate with existing test command `[...]`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Principle I: Performance-First** - PASS
- No blocking I/O introduced
- Execution overhead target: <5ms (well within limits)
- Parser optimization: single-pass, no backtracking needed for if/elif chains

✅ **Principle II: Zero-Config Philosophy** - PASS
- No configuration required for if/then/else/elif/fi
- Works immediately; defaults to POSIX behavior

✅ **Principle III: Progressive Complexity** - PASS
- Basic if/then/fi is simple (2 keywords + condition)
- Advanced features (elif chains, nesting) are opt-in but discoverable
- Complexity naturally follows shell scripting patterns

✅ **Principle IV: Modern UX** - N/A
- Syntax highlighting for if/then/else/elif/fi keywords (existing reedline support)
- Error messages for malformed constructs required

✅ **Principle V: Rust-Native** - PASS
- Pure Rust implementation
- Uses existing parser infrastructure (no new dependencies)
- No unsafe code required

## Project Structure

### Documentation (this feature)

```text
specs/017-if-then-else/
├── spec.md                      # Feature specification (requirements, user stories)
├── plan.md                      # This file (implementation plan)
├── research.md                  # Phase 0: Technical research and design decisions
├── data-model.md                # Phase 1: AST node definitions and data structures
├── quickstart.md                # Phase 1: Development guide and getting started
├── checklists/
│   └── requirements.md          # Specification quality checklist
└── tasks.md                     # Phase 2: Task breakdown (created by /speckit.tasks)
```

### Source Code (Rust monorepo - crates/rush/)

```text
crates/rush/
├── src/
│   ├── parser/
│   │   ├── lexer.rs            # Tokenization (existing; may need if/then/else keywords)
│   │   ├── ast.rs              # AST nodes (ADD: IfStatement, ElseIfClause variants)
│   │   ├── parser.rs           # Parser logic (ADD: parse_if_statement, parse_elif_chain)
│   │   └── mod.rs
│   ├── executor/
│   │   ├── lib.rs              # Command execution engine (ADD: evaluate_if_statement)
│   │   └── mod.rs
│   ├── shell/
│   │   └── repl.rs             # REPL (existing; handle multiline if/fi)
│   ├── main.rs
│   └── lib.rs
├── tests/
│   ├── integration/
│   │   ├── if_then_else.rs     # Integration tests for if/then/else
│   │   ├── elif_chains.rs      # Tests for elif clause chains
│   │   ├── nested_ifs.rs       # Tests for nested if statements
│   │   └── edge_cases.rs       # Edge case tests
│   └── fixtures/
│       └── scripts/            # Test shell scripts with if/then/else
└── Cargo.toml
```

**Structure Decision**: Single Rust library crate (crates/rush/) with modular parser and executor. No new crates required; extends existing parser and executor modules.

## Complexity Tracking

No violations of constitution principles detected. This feature:
- ✅ Maintains performance targets (parsing <1ms, execution <5ms overhead)
- ✅ Introduces zero configuration overhead
- ✅ Follows progressive complexity (simple if/then/fi → advanced elif chains)
- ✅ Uses pure Rust with existing dependencies
- ✅ Fits within MVP scope

## Deployment Strategy

**How this feature will be delivered incrementally via pull requests.**

### Pull Request Plan

**CRITICAL: Keep PRs small and reviewable (see CLAUDE.md for limits).**

**Strategy**: PR per major user story (Option 2)

**Rationale**: Feature has 6 user stories across multiple priority levels. Core functionality (US1-3) must be delivered first; advanced features (US4-6) can follow. Each US targets ~800-1,200 lines including tests.

### Selected Strategy: PR per User Story

```
PR #1: Foundation - Parser AST & Infrastructure (~400 lines)
  - Add IfStatement and ElseIfClause AST nodes
  - Add parser functions: parse_if_statement(), parse_elif_chain()
  - Update lexer to recognize if/then/else/elif/fi keywords
  - Unit tests for parser
  - Target: ≤ 500 lines

PR #2: User Story 1 - Basic Conditional Execution (~1,000 lines)
  - Implement if/then/fi evaluation logic
  - Support simple command conditions (exit code checking)
  - Handle then block execution
  - Integration tests: if true; then ...; fi
  - Integration tests: if false; then ...; fi
  - Target: ≤ 1,500 lines

PR #3: User Story 2 - Else Clause (~800 lines)
  - Add else block handling to executor
  - Implement alternative code path on condition failure
  - Integration tests: if false; then ...; else ...; fi
  - Integration tests with exit code evaluation
  - Target: ≤ 1,000 lines

PR #4: User Story 3 - Elif Clause (~1,200 lines)
  - Implement elif clause chain evaluation
  - Support multiple sequential conditions
  - Integration tests: multi-elif chains
  - Integration tests: elif with else fallback
  - Target: ≤ 1,500 lines

PR #5: User Story 4 - Compound Conditions (~600 lines)
  - Support && and || operators in if conditions
  - Update executor to handle compound condition evaluation
  - Integration tests: if cmd1 && cmd2; then ...
  - Integration tests: if cmd1 || cmd2; then ...
  - Target: ≤ 800 lines

PR #6: User Story 5 - Test Command Integration (~400 lines)
  - Integrate with [ ] test command for if conditions
  - Ensure proper variable expansion in conditions
  - Integration tests: if [ -f file ]; then ...
  - Integration tests: if [ "$var" = "value" ]; then ...
  - Target: ≤ 500 lines

PR #7: User Story 6 - Nesting & Edge Cases (~600 lines)
  - Support nested if statements
  - Handle edge cases (empty blocks, comments, multiline)
  - Integration tests: nested if structures
  - Integration tests: edge case handling
  - Target: ≤ 800 lines

PR #8: Polish & Documentation (~300 lines)
  - Performance benchmarks validation
  - Comprehensive test coverage review
  - Documentation updates
  - Regression test verification
  - Target: ≤ 500 lines
```

### Merge Sequence

1. **PR #1 (Foundation)** → Merge to main
   - Enables all subsequent features
   - Foundation for parser and AST

2. **PR #2 (US1: Basic if/then)** → Merge to main
   - MVP of conditional execution
   - Provides core value

3. **PR #3 (US2: else)** → Merge to main
   - Completes basic conditional logic
   - Two-way branching support

4. **PR #4 (US3: elif)** → Merge to main
   - Multi-way branching support
   - Essential for real-world scripts

5. **PR #5-7** → Merge to main in order
   - Advanced features building on core

6. **PR #8 (Polish)** → Merge to main
   - Final validation and documentation

**Branch Strategy**:
- Main working branch: `017-if-then-else` (created from feature initialization)
- Sub-branches for PR isolation (optional, only if parallel development needed)
- Example: `017-us1-basic-if`, `017-us2-else`, etc.

### PR Size Validation

Before creating each PR, verify size:
```bash
git diff --stat main  # Check line count
```

**Validation Rules**:
- ✅ All PRs < 1,500 lines of code (including tests and comments)
- ⚠️ Target ideal size: 500-1,000 lines
- ❌ Reject if > 1,500 lines (must split into smaller increments)
- Target test coverage: ≥ 80% of new code

### Implementation Order & Testing

1. Each PR must pass full test suite: `cargo test && cargo clippy`
2. Each PR must maintain 100% regression test pass rate
3. Integration tests added with each functional PR
4. Performance benchmarks validated for <1ms parse time, <5ms execution overhead
