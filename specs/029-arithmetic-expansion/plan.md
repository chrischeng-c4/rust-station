# Implementation Plan: Arithmetic Expansion

**Branch**: `029-arithmetic-expansion` | **Date**: 2025-12-07 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/029-arithmetic-expansion/spec.md`

## Summary

Implement shell arithmetic expansion capabilities including `$((expression))` syntax for inline calculations, the `let` builtin for arithmetic assignments, and `(( ))` arithmetic command for conditionals. The implementation follows the existing expansion pipeline pattern used by command substitution.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: No new dependencies (pure Rust implementation)
**Storage**: N/A (uses existing VariableManager)
**Testing**: cargo test (unit tests in modules, integration tests in tests/)
**Target Platform**: macOS (MVP)
**Project Type**: Single crate (rush shell)
**Performance Goals**: Arithmetic expressions complete instantly (<1ms)
**Constraints**: Integer-only arithmetic (64-bit signed), no floating-point
**Scale/Scope**: ~1500-2000 lines of new code

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Justification |
|-----------|--------|---------------|
| I. Performance-First | PASS | Pure Rust implementation, no external processes, inline evaluation <1ms |
| II. Zero-Config | PASS | Works immediately, no configuration required |
| III. Progressive Complexity | PASS | Basic `$((2+2))` simple; advanced operators available but not forced |
| IV. Modern UX | PASS | Standard bash/POSIX syntax users expect |
| V. Rust-Native | PASS | Pure Rust, no FFI, no new dependencies |

**Gate Status**: PASSED - All principles satisfied

## Project Structure

### Documentation (this feature)

```text
specs/029-arithmetic-expansion/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── quickstart.md        # Phase 1 output
├── checklists/          # Quality checklists
│   └── requirements.md
└── tasks.md             # Phase 2 output (from /speckit.tasks)
```

### Source Code (repository root)

```text
crates/rush/src/executor/
├── arithmetic/              # NEW: Arithmetic expansion module
│   ├── mod.rs              # Module exports, ArithmeticError type
│   ├── lexer.rs            # Tokenize arithmetic expressions
│   ├── parser.rs           # Parse into AST (expression tree)
│   ├── evaluator.rs        # Evaluate AST with variable resolution
│   └── expander.rs         # Integrate with expansion pipeline
├── builtins/
│   ├── mod.rs              # MODIFY: Register let builtin
│   └── let_cmd.rs          # NEW: let builtin implementation
├── expansion.rs            # MODIFY: Call arithmetic expander
├── parser.rs               # MODIFY: Handle (( )) command syntax
└── execute.rs              # MODIFY: Execute (( )) commands

crates/rush/tests/integration/
├── arithmetic_expansion_tests.rs    # NEW: $((expr)) tests
├── arithmetic_operators_tests.rs    # NEW: Operator tests
├── let_builtin_tests.rs             # NEW: let command tests
└── arithmetic_command_tests.rs      # NEW: (( )) tests
```

**Structure Decision**: Follows existing module patterns - arithmetic module parallels substitution module structure, new builtin in builtins/, integration tests in tests/integration/.

## Architecture Design

### Component Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Input Processing                          │
│  "echo $((x + 5))"                                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Arithmetic Expander (expander.rs)              │
│  1. Detect $((..)) patterns                                 │
│  2. Extract inner expression                                │
│  3. Call evaluator                                          │
│  4. Substitute result                                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Arithmetic Lexer (lexer.rs)                    │
│  Tokenize: "x + 5" → [Var("x"), Plus, Num(5)]              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Arithmetic Parser (parser.rs)                  │
│  Build AST with operator precedence                         │
│  Binary(Add, Var("x"), Num(5))                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Arithmetic Evaluator (evaluator.rs)            │
│  1. Resolve variables via VariableManager                   │
│  2. Compute result recursively                              │
│  3. Handle assignments                                      │
│  4. Return i64 result                                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Output: "echo 15"                        │
└─────────────────────────────────────────────────────────────┘
```

### AST Node Types

```rust
pub enum Expr {
    // Literals
    Number(i64),
    Variable(String),

    // Unary operators
    Negate(Box<Expr>),      // -x
    LogicalNot(Box<Expr>),  // !x
    BitwiseNot(Box<Expr>),  // ~x

    // Binary operators
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),

    // Comparison
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),

    // Logical
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),

    // Bitwise
    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    BitXor(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),

    // Assignment
    Assign(String, Box<Expr>),
    AddAssign(String, Box<Expr>),
    SubAssign(String, Box<Expr>),
    // ... other compound assignments

    // Increment/Decrement
    PreIncrement(String),
    PostIncrement(String),
    PreDecrement(String),
    PostDecrement(String),

    // Ternary
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),

    // Comma
    Comma(Vec<Expr>),

    // Grouping
    Group(Box<Expr>),
}
```

### Operator Precedence (C-style, highest to lowest)

| Precedence | Operators | Associativity |
|------------|-----------|---------------|
| 15 | `++ --` (postfix) | Left |
| 14 | `++ --` (prefix), `+ -` (unary), `! ~` | Right |
| 13 | `**` | Right |
| 12 | `* / %` | Left |
| 11 | `+ -` | Left |
| 10 | `<< >>` | Left |
| 9 | `< <= > >=` | Left |
| 8 | `== !=` | Left |
| 7 | `&` | Left |
| 6 | `^` | Left |
| 5 | `\|` | Left |
| 4 | `&&` | Left |
| 3 | `\|\|` | Left |
| 2 | `?:` | Right |
| 1 | `= += -= *= /= %= &= ^= \|= <<= >>=` | Right |
| 0 | `,` | Left |

### Integration Points

1. **expansion.rs**: Call `arithmetic::expand_arithmetic()` after variable expansion, before command substitution
2. **builtins/mod.rs**: Register `"let" => Some(let_cmd::execute(...))`
3. **parser.rs**: Detect `((` at command position, parse as arithmetic command
4. **execute.rs**: Handle arithmetic command execution with exit status

## Complexity Tracking

No constitution violations to justify.

## Deployment Strategy

### Selected Strategy

**Option 2: PR per User Story** - Feature has multiple independent user stories that can be merged incrementally.

**Rationale**: 9 user stories spanning ~2000 lines. Breaking into PRs by priority allows incremental delivery and testing.

### Merge Sequence

1. **PR #1: Core Infrastructure** (~500 lines)
   - Create arithmetic module structure
   - Implement lexer and basic parser
   - Add module exports to executor/mod.rs

2. **PR #2: Basic Arithmetic (US1+US2)** (~600 lines)
   - Complete parser with all operators
   - Implement evaluator for basic operators
   - Add expander integration
   - Tests for $((expr)) with +, -, *, /, %, **

3. **PR #3: Comparison & Logic (US3+US4)** (~400 lines)
   - Add comparison operators to evaluator
   - Add logical operators
   - Add bitwise operators
   - Tests for all comparison/logical/bitwise ops

4. **PR #4: Assignments (US5)** (~300 lines)
   - Implement assignment operators in evaluator
   - Implement increment/decrement
   - Integration with VariableManager
   - Tests for assignments

5. **PR #5: let Builtin (US6)** (~300 lines)
   - Create builtins/let_cmd.rs
   - Register in builtins/mod.rs
   - Tests for let command

6. **PR #6: (()) Command & Extras (US7+US8+US9)** (~400 lines)
   - Add (( )) command parsing
   - Add ternary operator
   - Add comma operator
   - Final integration tests

**Branch Strategy**: Work directly on `029-arithmetic-expansion` branch, merge to main after all PRs.

### PR Size Validation

**Before creating each PR, verify size**:
```bash
git diff --stat main  # Check line count
```

**Size Limits** (from CLAUDE.md):
- ✅ Ideal: ≤ 500 lines
- ⚠️ Maximum: ≤ 1,500 lines
- ❌ Too large: > 3,000 lines (must split)
