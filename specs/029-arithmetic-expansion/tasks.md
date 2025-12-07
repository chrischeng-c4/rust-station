# Tasks: Arithmetic Expansion

**Input**: Design documents from `/specs/029-arithmetic-expansion/`
**Prerequisites**: plan.md, spec.md, research.md, quickstart.md

**Tests**: Integration tests are included as this is a shell feature requiring comprehensive testing.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust crate**: `crates/rush/src/` for source, `crates/rush/tests/` for tests

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create arithmetic module structure and basic types

- [ ] T001 Create arithmetic module directory at crates/rush/src/executor/arithmetic/
- [ ] T002 Create arithmetic module entry point with error types in crates/rush/src/executor/arithmetic/mod.rs
- [ ] T003 Add `pub mod arithmetic;` export to crates/rush/src/executor/mod.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core lexer and parser infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Implement Token enum for arithmetic expressions in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T005 Implement Lexer struct with tokenize() function in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T006 [P] Add number literal support (decimal, octal 0o/0, hex 0x) in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T007 [P] Add identifier/variable tokenization in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T008 Implement Expr enum (AST nodes) in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T009 Implement Pratt parser skeleton with precedence handling in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T010 Add unit tests for lexer in crates/rush/src/executor/arithmetic/lexer.rs

**Checkpoint**: Foundation ready - lexer tokenizes, parser skeleton exists

---

## Phase 3: User Story 1+2 - Basic Arithmetic Expansion (Priority: P1)

**Goal**: Implement `$((expression))` syntax with basic arithmetic operators (+, -, *, /, %, **)

**Independent Test**: Run `echo $((2 + 3))` and verify output is `5`

### Tests for User Story 1+2

- [ ] T011 [P] [US1] Create integration test file at crates/rush/tests/integration/arithmetic_expansion_tests.rs
- [ ] T012 [P] [US1] Add tests for basic $((expr)) expansion in crates/rush/tests/integration/arithmetic_expansion_tests.rs

### Implementation for User Story 1+2

- [ ] T013 [US1] Implement parse_primary() for numbers and variables in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T014 [US1] Implement parse_unary() for +/- prefix in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T015 [US1] Implement parse_binary() with precedence for +, -, *, /, % in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T016 [US1] Implement ** (exponentiation) operator with right associativity in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T017 [US1] Implement parentheses grouping in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T018 [US1] Create evaluator skeleton in crates/rush/src/executor/arithmetic/evaluator.rs
- [ ] T019 [US1] Implement evaluate() for Number and Variable nodes in crates/rush/src/executor/arithmetic/evaluator.rs
- [ ] T020 [US1] Implement evaluate() for binary arithmetic operators in crates/rush/src/executor/arithmetic/evaluator.rs
- [ ] T021 [US1] Add division by zero error handling in crates/rush/src/executor/arithmetic/evaluator.rs
- [ ] T022 [US1] Create expander with expand_arithmetic() function in crates/rush/src/executor/arithmetic/expander.rs
- [ ] T023 [US1] Implement $((..)) pattern detection and extraction in crates/rush/src/executor/arithmetic/expander.rs
- [ ] T024 [US1] Integrate expander into expansion pipeline in crates/rush/src/executor/expansion.rs
- [ ] T025 [US1] Add nested $((..)) expansion support in crates/rush/src/executor/arithmetic/expander.rs

**Checkpoint**: `echo $((5 + 3 * 2))` outputs `11`, variables work: `x=5; echo $((x + 1))` outputs `6`

---

## Phase 4: User Story 3 - Comparison and Logical Operators (Priority: P2)

**Goal**: Add comparison (<, >, <=, >=, ==, !=) and logical (&&, ||, !) operators

**Independent Test**: Run `echo $((5 > 3))` and verify output is `1`

### Tests for User Story 3

- [ ] T026 [P] [US3] Add comparison operator tests in crates/rush/tests/integration/arithmetic_operators_tests.rs
- [ ] T027 [P] [US3] Add logical operator tests in crates/rush/tests/integration/arithmetic_operators_tests.rs

### Implementation for User Story 3

- [ ] T028 [US3] Add comparison operator tokens (<, >, <=, >=, ==, !=) to lexer in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T029 [US3] Add logical operator tokens (&&, ||, !) to lexer in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T030 [US3] Implement comparison operators in parser with correct precedence in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T031 [US3] Implement logical operators in parser with correct precedence in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T032 [US3] Implement ! (logical not) as unary operator in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T033 [US3] Evaluate comparison operators (return 1 for true, 0 for false) in crates/rush/src/executor/arithmetic/evaluator.rs
- [ ] T034 [US3] Evaluate logical operators with short-circuit behavior in crates/rush/src/executor/arithmetic/evaluator.rs

**Checkpoint**: `echo $((5 > 3 && 2 < 4))` outputs `1`

---

## Phase 5: User Story 4 - Bitwise Operators (Priority: P2)

**Goal**: Add bitwise operators (&, |, ^, ~, <<, >>)

**Independent Test**: Run `echo $((5 & 3))` and verify output is `1`

### Tests for User Story 4

- [ ] T035 [P] [US4] Add bitwise operator tests in crates/rush/tests/integration/arithmetic_operators_tests.rs

### Implementation for User Story 4

- [ ] T036 [US4] Add bitwise operator tokens (&, |, ^, ~, <<, >>) to lexer in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T037 [US4] Implement bitwise operators in parser with correct precedence in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T038 [US4] Implement ~ (bitwise not) as unary operator in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T039 [US4] Evaluate bitwise operators in crates/rush/src/executor/arithmetic/evaluator.rs

**Checkpoint**: `echo $((1 << 4))` outputs `16`, `echo $((~0))` outputs `-1`

---

## Phase 6: User Story 5 - Variable Assignment in Arithmetic (Priority: P2)

**Goal**: Support assignment operators (=, +=, -=, *=, /=, %=) and increment/decrement (++, --)

**Independent Test**: Run `x=5; echo $((x += 3)); echo $x` and verify outputs `8` and `8`

### Tests for User Story 5

- [ ] T040 [P] [US5] Add assignment operator tests in crates/rush/tests/integration/arithmetic_operators_tests.rs
- [ ] T041 [P] [US5] Add increment/decrement tests in crates/rush/tests/integration/arithmetic_operators_tests.rs

### Implementation for User Story 5

- [ ] T042 [US5] Add assignment operator tokens (=, +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=) to lexer in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T043 [US5] Add increment/decrement tokens (++, --) to lexer in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T044 [US5] Implement assignment operators in parser (right associative) in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T045 [US5] Implement pre-increment/decrement (++x, --x) in parser in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T046 [US5] Implement post-increment/decrement (x++, x--) in parser in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T047 [US5] Evaluate assignment operators with VariableManager integration in crates/rush/src/executor/arithmetic/evaluator.rs
- [ ] T048 [US5] Evaluate increment/decrement with correct return values in crates/rush/src/executor/arithmetic/evaluator.rs

**Checkpoint**: `x=5; echo $((x++)); echo $x` outputs `5` then `6`

---

## Phase 7: User Story 6 - The let Builtin (Priority: P2)

**Goal**: Implement `let` builtin command for arithmetic assignments

**Independent Test**: Run `let x=5+3; echo $x` and verify output is `8`

### Tests for User Story 6

- [ ] T049 [P] [US6] Create let builtin test file at crates/rush/tests/integration/let_builtin_tests.rs
- [ ] T050 [P] [US6] Add tests for let syntax variations in crates/rush/tests/integration/let_builtin_tests.rs

### Implementation for User Story 6

- [ ] T051 [US6] Create let builtin module at crates/rush/src/executor/builtins/let_cmd.rs
- [ ] T052 [US6] Implement execute() function for let builtin in crates/rush/src/executor/builtins/let_cmd.rs
- [ ] T053 [US6] Handle multiple expressions (let x=5 y=10) in crates/rush/src/executor/builtins/let_cmd.rs
- [ ] T054 [US6] Handle quoted expressions with spaces (let "x = 5 + 3") in crates/rush/src/executor/builtins/let_cmd.rs
- [ ] T055 [US6] Implement exit status (0 for non-zero result, 1 for zero) in crates/rush/src/executor/builtins/let_cmd.rs
- [ ] T056 [US6] Register let builtin in crates/rush/src/executor/builtins/mod.rs

**Checkpoint**: `let "x = 0"; echo $?` outputs `1` (false exit status)

---

## Phase 8: User Story 7 - Arithmetic Command (()) (Priority: P3)

**Goal**: Implement `(( expression ))` as a command with exit status

**Independent Test**: Run `(( 5 > 3 )) && echo yes` and verify output is `yes`

### Tests for User Story 7

- [ ] T057 [P] [US7] Create arithmetic command test file at crates/rush/tests/integration/arithmetic_command_tests.rs
- [ ] T058 [P] [US7] Add tests for (()) in conditionals in crates/rush/tests/integration/arithmetic_command_tests.rs

### Implementation for User Story 7

- [ ] T059 [US7] Add (( and )) token recognition in crates/rush/src/executor/parser.rs
- [ ] T060 [US7] Detect (( at command position and parse as arithmetic command in crates/rush/src/executor/parser.rs
- [ ] T061 [US7] Create ArithmeticCommand variant or handler in crates/rush/src/executor/execute.rs
- [ ] T062 [US7] Execute arithmetic command with proper exit status (0=true, 1=false) in crates/rush/src/executor/execute.rs
- [ ] T063 [US7] Integrate with conditional execution (if, while, &&, ||) in crates/rush/src/executor/execute.rs

**Checkpoint**: `if (( x > 5 )); then echo big; fi` works correctly

---

## Phase 9: User Story 8+9 - Ternary and Comma Operators (Priority: P3)

**Goal**: Add ternary operator (?:) and comma operator for expression sequences

**Independent Test**: Run `echo $((5 > 3 ? 10 : 20))` and verify output is `10`

### Tests for User Story 8+9

- [ ] T064 [P] [US8] Add ternary operator tests in crates/rush/tests/integration/arithmetic_operators_tests.rs
- [ ] T065 [P] [US9] Add comma operator tests in crates/rush/tests/integration/arithmetic_operators_tests.rs

### Implementation for User Story 8+9

- [ ] T066 [US8] Add ternary operator tokens (?, :) to lexer in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T067 [US8] Implement ternary operator parsing (right associative) in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T068 [US8] Evaluate ternary with short-circuit (only evaluate chosen branch) in crates/rush/src/executor/arithmetic/evaluator.rs
- [ ] T069 [US9] Add comma operator token to lexer in crates/rush/src/executor/arithmetic/lexer.rs
- [ ] T070 [US9] Implement comma operator parsing (lowest precedence) in crates/rush/src/executor/arithmetic/parser.rs
- [ ] T071 [US9] Evaluate comma operator (evaluate all, return last) in crates/rush/src/executor/arithmetic/evaluator.rs

**Checkpoint**: `echo $((a=1, b=2, a+b))` outputs `3`

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Edge cases, error handling, and comprehensive testing

- [ ] T072 [P] Add edge case tests (empty expr, whitespace, overflow) in crates/rush/tests/integration/arithmetic_expansion_tests.rs
- [ ] T073 [P] Add octal and hexadecimal number tests in crates/rush/tests/integration/arithmetic_expansion_tests.rs
- [ ] T074 [P] Add undefined variable (defaults to 0) tests in crates/rush/tests/integration/arithmetic_expansion_tests.rs
- [ ] T075 [P] Add non-numeric string (defaults to 0) tests in crates/rush/tests/integration/arithmetic_expansion_tests.rs
- [ ] T076 Add error message tests (division by zero, syntax errors) in crates/rush/tests/integration/arithmetic_expansion_tests.rs
- [ ] T077 Run full test suite and fix any failures with cargo test
- [ ] T078 Run clippy and fix any warnings with cargo clippy --all-targets
- [ ] T079 Validate quickstart.md examples work correctly
- [ ] T080 Update features.json to mark 029 as complete

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-9)**: All depend on Foundational phase completion
  - US1+US2 (Phase 3): Basic arithmetic - must complete first
  - US3 (Phase 4): Comparison/logical - depends on US1+US2 parser being complete
  - US4 (Phase 5): Bitwise - can parallel with US3
  - US5 (Phase 6): Assignments - depends on US1+US2 evaluator
  - US6 (Phase 7): let builtin - depends on US5 for assignment support
  - US7 (Phase 8): (()) command - depends on US1+US2 evaluator
  - US8+US9 (Phase 9): Ternary/comma - depends on US1+US2 parser
- **Polish (Phase 10)**: Depends on all user stories being complete

### Within Each User Story

- Tests written first (when included)
- Parser changes before evaluator changes
- Core implementation before integration
- Unit tests alongside implementation

### Parallel Opportunities

- T006 and T007 (lexer features) can run in parallel
- T026 and T027 (US3 tests) can run in parallel
- T040 and T041 (US5 tests) can run in parallel
- US3 (Phase 4) and US4 (Phase 5) can run in parallel after Phase 3
- US7 (Phase 8) and US8+US9 (Phase 9) can run in parallel after Phase 6

---

## Parallel Example: Phase 3 (User Story 1+2)

```bash
# Launch tests in parallel:
Task: "Create integration test file at crates/rush/tests/integration/arithmetic_expansion_tests.rs"
Task: "Add tests for basic $((expr)) expansion"

# After tests, launch parser implementation:
Task: "Implement parse_primary() for numbers and variables"
Task: "Implement parse_unary() for +/- prefix"
# (sequential - each builds on previous)
```

---

## Implementation Strategy

### MVP First (User Stories 1+2 Only)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: Foundational (T004-T010)
3. Complete Phase 3: User Stories 1+2 (T011-T025)
4. **STOP and VALIDATE**: Test `echo $((5 + 3 * 2))` works
5. Merge to main if ready

### Incremental Delivery

1. Setup + Foundational → Core infrastructure ready
2. Add US1+US2 → Basic arithmetic works → **MVP!**
3. Add US3+US4 → Comparison, logical, bitwise operators
4. Add US5 → Assignment operators and increment/decrement
5. Add US6 → `let` builtin available
6. Add US7 → `(())` command available
7. Add US8+US9 → Ternary and comma operators
8. Polish → All edge cases handled

---

## Summary

- **Total tasks**: 80
- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 7 tasks
- **Phase 3 (US1+US2)**: 15 tasks
- **Phase 4 (US3)**: 9 tasks
- **Phase 5 (US4)**: 5 tasks
- **Phase 6 (US5)**: 9 tasks
- **Phase 7 (US6)**: 8 tasks
- **Phase 8 (US7)**: 7 tasks
- **Phase 9 (US8+US9)**: 8 tasks
- **Phase 10 (Polish)**: 9 tasks

**MVP Scope**: Phases 1-3 (25 tasks) delivers working `$((expr))` with basic operators

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently

### Pull Request Strategy

**CRITICAL: Create separate PRs per plan.md deployment strategy.**

**Workflow**:
1. PR #1: Core Infrastructure (Phase 1+2) ~500 lines
2. PR #2: Basic Arithmetic US1+US2 (Phase 3) ~600 lines
3. PR #3: Comparison & Logic US3+US4 (Phases 4-5) ~400 lines
4. PR #4: Assignments US5 (Phase 6) ~300 lines
5. PR #5: let Builtin US6 (Phase 7) ~300 lines
6. PR #6: (()) Command & Extras US7+US8+US9 (Phases 8-9) ~400 lines
7. Final: Polish (Phase 10) as needed

**Before Creating PR**:
- Check line count: `git diff --stat main`
- If >1,500 lines, split further
