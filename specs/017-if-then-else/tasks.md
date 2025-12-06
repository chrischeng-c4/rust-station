# Task Breakdown: Conditional Control Flow (if/then/else/elif/fi)

**Feature**: 017 - if/then/else/elif/fi | **Priority**: P0
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)
**Total Tasks**: 52 | **Phases**: 8 | **Estimated Duration**: ~6-8 weeks

---

## Phase Overview

| Phase | Focus | PR Size | Status |
|-------|-------|---------|--------|
| Phase 1: Setup | Project structure & test harness | - | Pending |
| Phase 2: Foundation | Parser AST & keyword recognition | ≤ 500 lines | Pending |
| Phase 3: US1 | Basic if/then/fi | ≤ 1,500 lines | Pending |
| Phase 4: US2 | Else clause support | ≤ 1,000 lines | Pending |
| Phase 5: US3 | Elif chain support | ≤ 1,500 lines | Pending |
| Phase 6: US4 | Compound conditions (&&/\|\|) | ≤ 800 lines | Pending |
| Phase 7: US5 | Test command integration | ≤ 500 lines | Pending |
| Phase 8: US6 | Nesting & edge cases | ≤ 800 lines | Pending |
| Phase 9: Polish | Benchmarks & documentation | ≤ 500 lines | Pending |

---

## Dependency Graph

```
Phase 1: Setup (test infrastructure)
    ↓
Phase 2: Foundation (parser AST, lexer keywords)
    ↓
    ├─→ Phase 3: US1 (if/then/fi execution) [blocking for all others]
    │       ↓
    │   Phase 4: US2 (else clause) [optional, but P1]
    │       ↓
    │   Phase 5: US3 (elif chain) [optional, but P1]
    │
    ├─→ Phase 6: US4 (compound conditions) [parallelizable with US2-US3]
    ├─→ Phase 7: US5 (test integration) [parallelizable]
    └─→ Phase 8: US6 (nesting) [parallelizable after US1]
            ↓
        Phase 9: Polish (cross-cutting validation)
```

---

## Parallel Execution Strategy

**MVP Scope** (essential for v1.0):
- Phase 1-5: Setup → Foundation → US1 → US2 → US3 (sequential, ~4 weeks)
- Phases 6-8 can run in parallel after Foundation

**Recommended Parallelization**:
- After Phase 5 completes: Launch US4, US5, US6 in parallel (2-3 weeks)
- Phase 9: Final polish after all features merge

---

# PHASE 1: Setup

## Goal
Establish test infrastructure and validate build environment for if/then/else feature development.

## Independent Test Criteria
- Cargo compiles with no warnings
- Test harness runs successfully
- REPL interactive mode works
- All existing tests still pass

---

- [ ] T001 Verify Rust environment (1.75+) and all dependencies installed
- [ ] T002 Create test fixture directory at `crates/rush/tests/fixtures/scripts/if-else/`
- [ ] T003 Create test template file `crates/rush/tests/fixtures/scripts/if-else/template.sh` with sample if statements
- [ ] T004 Verify existing test suite passes with `cargo test` (baseline for regression testing)
- [ ] T005 Update CI/CD configuration to include if/then/else feature tests
- [ ] T006 Document development environment setup in specs/017-if-then-else/quickstart.md

---

# PHASE 2: Foundation - Parser AST & Infrastructure

## Goal
Extend parser and AST to recognize if/then/else/elif/fi keywords and create corresponding AST node structures.

## Independent Test Criteria
- Parser recognizes if/then/else/elif/fi as keywords (not command names)
- Lexer properly tokenizes if statements
- AST parser creates IfStatement nodes without errors
- Parsing errors generate helpful error messages

---

- [ ] T007 [P] Update lexer to recognize keywords: `if`, `then`, `else`, `elif`, `fi` in `crates/rush/src/parser/lexer.rs`
- [ ] T008 [P] Create AST node types in `crates/rush/src/parser/ast.rs`:
  - `struct IfStatement { condition: CommandList, then_block: CommandList, else_if_clauses: Vec<ElseIfClause>, else_block: Option<CommandList> }`
  - `struct ElseIfClause { condition: CommandList, block: CommandList }`
- [ ] T009 [P] Implement AST Display trait for IfStatement and ElseIfClause for debugging
- [ ] T010 [P] Add parser function `parse_if_statement()` to `crates/rush/src/parser/parser.rs`
- [ ] T011 [P] Add parser function `parse_elif_chain()` to handle multiple elif clauses
- [ ] T012 [P] Implement error handling in parser for malformed if statements (missing fi, missing then)
- [ ] T013 Update parser main dispatch to call `parse_if_statement()` when `if` keyword detected
- [ ] T014 Create unit tests in `crates/rush/tests/unit/parser/if_statement_parsing.rs` for:
  - Basic if/then/fi parsing
  - Elif chain parsing
  - Error cases (missing fi, missing then, orphaned elif)
- [ ] T015 Verify `cargo test` passes with new AST node types and parser functions

---

# PHASE 3: US1 - Basic Conditional Execution (if/then/fi)

**User Story 1**: A user wants to execute different commands based on whether a previous command succeeded or failed.

## Independent Test Criteria
- `if true; then echo "success"; fi` prints "success"
- `if false; then echo "fail"; fi` prints nothing
- Exit code of if statement matches exit code of condition
- Interactive REPL properly handles multiline if statements

---

- [ ] T016 [P] [US1] Add IfStatement variant to executor AST handler in `crates/rush/src/executor/mod.rs`
- [ ] T017 [P] [US1] Implement `evaluate_if_statement()` function in `crates/rush/src/executor/lib.rs` that:
  - Executes the condition command and captures exit code
  - If exit code == 0: execute then_block and return its exit code
  - If exit code != 0: return exit code (no output)
  - Properly handle command failure cases
- [ ] T018 [P] [US1] Update executor's main command dispatcher to handle IfStatement AST node
- [ ] T019 [US1] Update REPL to recognize `if` statements spanning multiple lines and buffer input until `fi` is received in `crates/rush/src/shell/repl.rs`
- [ ] T020 [US1] Implement integration test in `crates/rush/tests/integration/if_then_else.rs`:
  - Test: `if true; then echo "success"; fi` → output "success"
  - Test: `if false; then echo "fail"; fi` → output "" (empty)
  - Test: `if [ -f /tmp/test ]; then echo "exists"; fi` → test with existing/non-existing file
- [ ] T021 [US1] Create shell script fixtures in `crates/rush/tests/fixtures/scripts/if-else/`:
  - `basic_true.sh` - simple if true statement
  - `basic_false.sh` - simple if false statement
  - `file_exists.sh` - test with file existence check
- [ ] T022 [US1] Verify all existing tests still pass with `cargo test`
- [ ] T023 [US1] Performance test: Verify if statement parsing + execution < 5ms overhead vs direct command

---

# PHASE 4: US2 - Else Clause

**User Story 2**: A user wants to specify an alternative action when the condition fails.

## Independent Test Criteria
- `if false; then echo "no"; else echo "yes"; fi` prints "yes"
- `if true; then echo "yes"; else echo "no"; fi` prints "yes"
- Else block executes only when condition fails
- Exit code of else block is returned correctly

---

- [ ] T024 [P] [US2] Modify `evaluate_if_statement()` in `crates/rush/src/executor/lib.rs` to handle else block:
  - If condition exit code != 0 AND else_block exists: execute else_block
  - Return else_block's exit code
- [ ] T025 [P] [US2] Update parser's `parse_if_statement()` to recognize and parse `else` keyword and else_block
- [ ] T026 [US2] Integration test in `crates/rush/tests/integration/if_then_else.rs`:
  - Test: `if false; then echo "no"; else echo "yes"; fi` → output "yes"
  - Test: `if true; then echo "yes"; else echo "no"; fi` → output "yes"
  - Test: exit code from else block is preserved
- [ ] T027 [US2] Create shell script fixtures:
  - `else_false_condition.sh` - else executes when condition fails
  - `else_true_condition.sh` - else does not execute when condition succeeds
  - `else_exit_code.sh` - verify else block exit code is returned
- [ ] T028 [US2] Verify all tests pass and no regressions with `cargo test`

---

# PHASE 5: US3 - Elif Clause

**User Story 3**: A user needs to check multiple conditions in sequence using elif.

## Independent Test Criteria
- Multiple elif clauses evaluated in order
- First true condition's block executes
- Subsequent conditions not evaluated after first match
- Final else clause acts as fallback if all conditions false

---

- [ ] T029 [P] [US3] Implement elif clause evaluation in `evaluate_if_statement()`:
  - After condition fails, iterate through elif_clauses
  - Execute first elif condition that succeeds
  - Execute remaining code only for matched clause
  - If no elif matches and else_block exists: execute else_block
- [ ] T030 [P] [US3] Parser already supports elif chains from T011 (verify in tests)
- [ ] T031 [US3] Optimization: Short-circuit evaluation (don't evaluate remaining elif conditions after first match)
- [ ] T032 [US3] Integration tests in `crates/rush/tests/integration/elif_chains.rs`:
  - Test: Multi-elif chain with first condition true
  - Test: Multi-elif chain with middle condition true
  - Test: Multi-elif chain with last condition true
  - Test: Multi-elif chain with no match → else block executes
  - Test: Multi-elif chain without else → no output when no match
- [ ] T033 [US3] Create shell script fixtures:
  - `elif_first_match.sh` - first elif matches
  - `elif_middle_match.sh` - middle elif matches
  - `elif_all_false.sh` - all conditions false, else executes
  - `elif_no_else.sh` - no else clause present
- [ ] T034 [US3] Performance test: Verify elif chain evaluation is O(n) with short-circuit
- [ ] T035 [US3] Verify all tests pass with `cargo test`

---

# PHASE 6: US4 - Compound Conditions (&&/||)

**User Story 4**: A user wants to combine conditions using logical AND and OR operators.

## Independent Test Criteria
- `if cmd1 && cmd2; then` executes then_block only if both succeed
- `if cmd1 || cmd2; then` executes then_block if either succeeds
- Operator precedence respected (short-circuit evaluation)
- Works nested with other if constructs

---

- [ ] T036 [P] [US4] Verify lexer recognizes `&&` and `||` operators (likely already exists from feature 004/006)
- [ ] T037 [P] [US4] Update parser to handle compound conditions in if statements:
  - Parse `&&` and `||` operators as part of condition expression
  - Build command chain AST nodes for compound conditions
- [ ] T038 [P] [US4] Executor: Implement short-circuit evaluation for `&&`:
  - If first command fails (exit != 0): return failure, don't execute second command
  - If first command succeeds: execute second command, return its exit code
- [ ] T039 [P] [US4] Executor: Implement short-circuit evaluation for `||`:
  - If first command succeeds (exit == 0): return success, don't execute second command
  - If first command fails: execute second command, return its exit code
- [ ] T040 [US4] Integration tests in `crates/rush/tests/integration/compound_conditions.rs`:
  - Test: `if true && true; then echo "yes"; fi` → "yes"
  - Test: `if true && false; then echo "yes"; fi` → no output
  - Test: `if true || false; then echo "yes"; fi` → "yes"
  - Test: `if false || true; then echo "yes"; fi` → "yes"
  - Test: Short-circuit behavior (verify second command not executed when unnecessary)
- [ ] T041 [US4] Create shell script fixtures:
  - `compound_and.sh` - && operator behavior
  - `compound_or.sh` - || operator behavior
  - `compound_short_circuit.sh` - verify short-circuit evaluation
- [ ] T042 [US4] Verify all tests pass with `cargo test`

---

# PHASE 7: US5 - Test Command Integration

**User Story 5**: A user wants to use the test command `[...]` within if statements to check files, strings, and numeric values.

## Independent Test Criteria
- `if [ -f file ]; then` correctly detects file existence
- `if [ "$var" = "value" ]; then` correctly compares strings
- `if [ $num -gt 10 ]; then` correctly compares numbers
- Variable expansion works within test conditions
- All test operators (-f, -d, -z, =, -eq, etc.) work in if conditions

---

- [ ] T043 [P] [US5] Integration with test command (feature 062 dependency note):
  - Ensure if statement condition evaluation accepts test command output
  - Test command output exit code determines condition truth
- [ ] T044 [P] [US5] Verify variable expansion in if conditions works:
  - `if [ "$var" = "value" ]; then` - variable substitution
  - `if [ -f "$filename" ]; then` - variable in file path
- [ ] T045 [US5] Integration tests in `crates/rush/tests/integration/test_command_integration.rs`:
  - Test: `if [ -f /tmp/testfile ]; then echo "exists"; fi` - file existence
  - Test: `if [ -d /tmp ]; then echo "dir"; fi` - directory check
  - Test: `if [ "$USER" = "root" ]; then echo "admin"; fi` - string comparison
  - Test: `if [ 5 -gt 3 ]; then echo "yes"; fi` - numeric comparison
  - Test: `if [ -z "$empty_var" ]; then echo "empty"; fi` - empty variable check
- [ ] T046 [US5] Create shell script fixtures:
  - `test_file_exists.sh` - -f operator
  - `test_dir_exists.sh` - -d operator
  - `test_string_compare.sh` - string comparison
  - `test_numeric_compare.sh` - numeric comparisons
  - `test_var_expansion.sh` - variable substitution in conditions
- [ ] T047 [US5] Verify all tests pass with `cargo test`

---

# PHASE 8: US6 - Nesting & Edge Cases

**User Story 6**: A user needs to nest if statements for complex control flow within branches.

## Independent Test Criteria
- Nested if statements execute correctly
- Inner if blocks have correct scope and exit codes
- Comments handled properly within if blocks
- Empty if/then/fi blocks are valid (no-op)
- Multiline if statements with various formatting work

---

- [ ] T048 [P] [US6] Parser: Verify recursive if statement parsing works (should already from recursive design)
- [ ] T049 [P] [US6] Executor: Ensure nested if statements have proper scope isolation
- [ ] T050 [P] [US6] Edge case handling in parser:
  - Empty then block: `if true; then; fi` - valid (no-op)
  - Comments inside if block: `if true; then # comment; echo x; fi`
  - Multiline formatting with various indentation
- [ ] T051 [US6] Integration tests in `crates/rush/tests/integration/nested_ifs.rs`:
  - Test: Nested if in then block
  - Test: Nested if in else block
  - Test: Triple-nested if statements
  - Test: Nested if with elif clauses
  - Test: Nested if with compound conditions
- [ ] T052 [US6] Edge case integration tests in `crates/rush/tests/integration/edge_cases.rs`:
  - Test: Empty if/then/fi blocks (no-op)
  - Test: Comments inside if blocks
  - Test: Multiline if with various formatting
  - Test: If statement at EOF with missing newline
  - Test: If statement with command substitution in condition
- [ ] T053 [US6] Create shell script fixtures:
  - `nested_if_basic.sh` - simple nested structure
  - `nested_if_complex.sh` - multiple nesting levels
  - `edge_case_empty.sh` - empty blocks
  - `edge_case_comments.sh` - comments in if blocks
  - `edge_case_multiline.sh` - formatting variations
- [ ] T054 [US6] Verify all tests pass with `cargo test` and no regressions

---

# PHASE 9: Polish & Documentation

## Goal
Validate performance, complete documentation, ensure 100% regression test pass rate.

## Independent Test Criteria
- Performance benchmarks validate <1ms parsing, <5ms execution overhead
- All documentation updated
- 100% of existing tests pass (zero regressions)
- Code coverage ≥ 80% for new code

---

- [ ] T055 Performance benchmark: Parse 100 if statements, average < 1ms per statement
- [ ] T056 Performance benchmark: Execute 100 if statements, verify < 5ms overhead vs direct commands
- [ ] T057 Run full test suite: `cargo test --all-targets --all-features`
- [ ] T058 Run clippy linter: `cargo clippy --all-targets --all-features` (fix all warnings)
- [ ] T059 Run formatter: `cargo fmt` (ensure all code properly formatted)
- [ ] T060 Calculate code coverage for new if/then/else feature code (target ≥ 80%)
- [ ] T061 Update CLAUDE.md with if/then/else feature documentation and examples
- [ ] T062 Update crates/rush/README.md with if/then/else usage examples
- [ ] T063 Document all public APIs and AST node types in code comments
- [ ] T064 Create features/017-IF-THEN-ELSE.md in docs/ folder with:
  - Feature overview
  - Syntax reference (if/then/else/elif/fi)
  - Usage examples with screenshots/output
  - Limitations and edge cases
  - Performance characteristics
- [ ] T065 Review all 8 PRs for code quality, test coverage, and adherence to CLAUDE.md workflow
- [ ] T066 Final validation: Run all tests and benchmarks, document results in completion report

---

## Summary

**Total Implementation Tasks**: 66 (including documentation)
**Est. Lines of Code**: ~2,000-2,500 (parser + executor + tests)
**Est. Test Cases**: 35+
**Critical Path** (sequential phases for MVP): Phases 1-5 (~4 weeks)
**Full Feature Delivery**: Phases 1-9 (~6-8 weeks)

### Key Deliverables

1. **Parser Extensions** (Phase 2): Lexer keywords + AST nodes + parsing logic
2. **Executor Implementation** (Phases 3-8): If/then/fi → else → elif → compound conditions → test integration → nesting
3. **Test Suite** (Phases 3-8): 35+ integration tests + 20+ unit tests
4. **Documentation** (Phase 9): Code docs + feature guide + examples

### Acceptance Criteria

✅ All 52 tasks completed and passing
✅ All 6 user stories have passing acceptance scenarios
✅ Performance benchmarks: parsing <1ms, execution <5ms
✅ Zero regression test failures
✅ Code coverage ≥ 80%
✅ Full documentation in place
