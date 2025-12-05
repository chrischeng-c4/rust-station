# Task Breakdown: For Loop Iteration (for/in/do/done)

**Feature**: 018 - for/in/do/done loops | **Priority**: P0
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)
**Total Tasks**: 58 | **Phases**: 9 | **Est. Duration**: 7-9 weeks

---

## Dependency Graph & MVP Strategy

```
Phase 1: Setup
    ↓
Phase 2: Foundation (lexer + AST keywords + parser)
    ↓ [blocks all user story phases]
    ├─→ Phase 3: US1 (basic for/in/do/done) [P0 blocking]
    │       ↓
    │   Phase 4: US2 (command subst + globbing)
    │       ↓
    │   Phase 5: US3 (nested loops)
    │
    ├─→ Phase 6: US4 (variable expansion) [parallelizable after Foundation]
    ├─→ Phase 7: US5 (range expansion) [parallelizable, can defer to feature 030]
    └─→ Phase 8: US6 (arrays) [parallelizable]
            ↓
        Phase 9: Polish (benchmarks + regression)

**MVP Scope**: Phases 1-5 (Foundation → US1 → US2 → US3 enables most scripts)
**Full Feature**: Phases 1-9
```

---

# PHASE 1: Setup

- [ ] T001 Verify Rust environment and dependencies current
- [ ] T002 Create test fixture directory at `crates/rush/tests/fixtures/scripts/for-loops/`
- [ ] T003 Create test template at `crates/rush/tests/fixtures/scripts/for-loops/template.sh`
- [ ] T004 Verify existing test suite passes (baseline: `cargo test`)
- [ ] T005 Document development quickstart in `specs/018-for-loops/quickstart.md`
- [ ] T006 Update CI/CD config for for-loop feature tests

---

# PHASE 2: Foundation - Parser & Lexer

**Goal**: Lexer recognizes for/in/do/done keywords; parser can create ForStatement AST nodes

- [ ] T007 [P] Update lexer to recognize keywords: `for`, `in`, `do`, `done` in `crates/rush/src/parser/lexer.rs`
- [ ] T008 [P] Add ForStatement AST node to `crates/rush/src/parser/ast.rs`:
  - `struct ForStatement { var_name: String, list_expr: CommandList, body: CommandList }`
- [ ] T009 [P] Implement AST Display trait for ForStatement (debugging output)
- [ ] T010 [P] Add parser function `parse_for_statement()` in `crates/rush/src/parser/parser.rs`
- [ ] T011 [P] Parser: Handle `in` keyword as optional (implicit arg expansion if omitted)
- [ ] T012 Update parser main dispatcher to call `parse_for_statement()` on `for` keyword
- [ ] T013 Add unit tests in `crates/rush/tests/unit/parser/for_loops.rs`:
  - Test: Basic for/in/do/done parsing
  - Test: For without explicit `in` list
  - Test: Malformed for statements (missing do, missing done, orphaned done)
- [ ] T014 Verify full test suite passes: `cargo test`

---

# PHASE 3: US1 - Basic List Iteration

**Goal**: Execute basic for loops over simple lists; `for x in a b c; do echo $x; done` works

**Independent Test Criteria**: All 3 US1 acceptance scenarios pass

- [ ] T015 [US1] Add ForStatement handler in executor `crates/rush/src/executor/lib.rs`
- [ ] T016 [US1] Implement `evaluate_for_loop()` function:
  - Parse loop variable name
  - Expand list expression into word list
  - Iterate: set loop var, execute body, advance to next item
  - Return exit code from last body execution
- [ ] T017 [US1] Update executor's command dispatcher to handle ForStatement AST node
- [ ] T018 [US1] REPL support: Detect `for` keyword, buffer input until `done`, parse multiline statement in `crates/rush/src/shell/repl.rs`
- [ ] T019 [US1] Create integration test `crates/rush/tests/integration/for_loops_basic.rs`:
  - Test: `for x in a b c; do echo $x; done` outputs a,b,c
  - Test: `for x in; do echo $x; done` (empty list) → no output
  - Test: `for x in $HOME /tmp; do echo $x; done` → vars expanded
- [ ] T020 [US1] Create shell script fixtures:
  - `basic_iteration.sh`: simple word list
  - `empty_list.sh`: empty list behavior
  - `variable_in_list.sh`: variable expansion in list
- [ ] T021 [US1] Verify all tests pass and no regressions
- [ ] T022 [US1] Performance test: <1ms parse, <5ms exec overhead per item

---

# PHASE 4: US2 - Command Substitution & Globbing

**Goal**: For loops work with command output `$(...)` and globs `*.txt`

**Independent Test Criteria**: US2 acceptance scenarios pass

- [ ] T023 [US2] Update list expansion in `evaluate_for_loop()` to support:
  - Command substitution: `$(find /tmp -type f)`
  - Globbing: `/etc/*.conf`
  - Combined: `$dir/*.sh` (variable + glob)
- [ ] T024 [US2] Ensure word splitting respects quoting (e.g., "hello world" = 1 item)
- [ ] T025 [US2] Create integration test `crates/rush/tests/integration/for_loops_command_subst.rs`:
  - Test: `for f in $(find /tmp -type f); do echo $f; done`
  - Test: `for f in *.sh; do echo $f; done`
  - Test: Combined glob + vars
- [ ] T026 [US2] Create fixtures:
  - `command_substitution.sh`
  - `globbing.sh`
  - `combined_expansion.sh`
- [ ] T027 [US2] Verify all tests pass

---

# PHASE 5: US3 - Nested Loops

**Goal**: For loops can be nested; proper variable scoping for each nesting level

**Independent Test Criteria**: Nested loop acceptance scenarios pass

- [ ] T028 [P] [US3] Parser: Recursive for statement parsing (should work already; verify)
- [ ] T029 [US3] Executor: Ensure nested loops have proper variable scoping:
  - Outer loop variable accessible in inner loop
  - Inner loop variable doesn't overwrite outer
  - Variable restored after inner loop exits
- [ ] T030 [US3] Create integration test `crates/rush/tests/integration/for_loops_nested.rs`:
  - Test: Nested for in then block
  - Test: Nested for in loop body
  - Test: Triple nested (for → for → for)
  - Test: Variable isolation between nesting levels
- [ ] T031 [US3] Create fixtures:
  - `nested_basic.sh`
  - `nested_complex.sh`
  - `variable_isolation.sh`
- [ ] T032 [US3] Verify all tests pass

---

# PHASE 6: US4 - Loop Variable Expansion

**Goal**: Loop variable expands correctly in all contexts (commands, strings, paths)

- [ ] T033 [US4] Update variable expansion logic to handle loop variables in all contexts:
  - Direct: `$x` → value
  - In strings: `"$x.txt"` → value.txt
  - In command args: `touch /tmp/file$x` → creates file with value
  - Multiple refs: `echo $x $x` → value value
- [ ] T034 [US4] Create integration test `crates/rush/tests/integration/for_loops_variable_expansion.rs`:
  - Test: `for f in a b; do echo $f; done`
  - Test: `for f in a b; do echo "$f.txt"; done` → a.txt, b.txt
  - Test: `for f in 1 2; do touch /tmp/$f; done` → creates /tmp/1, /tmp/2
- [ ] T035 [US4] Verify all tests pass

---

# PHASE 7: US5 - Range Expansion (OPTIONAL - Consider deferring to feature 030)

**Goal**: Brace range syntax `{1..5}`, `{a..c}` expands in for loops

- [ ] T036 [US5] Add range expansion to list evaluation: `{1..10}`, `{1..10..2}`, `{a..z}`
- [ ] T037 [US5] Integrate with existing brace expansion (feature 029 when available)
- [ ] T038 [US5] Create integration test `crates/rush/tests/integration/for_loops_ranges.rs`:
  - Test: `for i in {1..5}; do echo $i; done`
  - Test: `for c in {a..c}; do echo $c; done`
- [ ] T039 [US5] Verify tests pass

---

# PHASE 8: US6 - Array Iteration

**Goal**: For loops iterate over arrays created with feature 011

- [ ] T040 [US6] Update for loop to handle array variables:
  - Recognize `"${arr[@]}"` syntax
  - Iterate over array elements as list items
- [ ] T041 [US6] Create integration test `crates/rush/tests/integration/for_loops_arrays.rs`:
  - Test: `arr=(a b c); for x in "${arr[@]}"; do echo $x; done`
  - Test: Array with spaces: `arr=("hello world" "foo"); for x in "${arr[@]}"; do echo $x; done`
- [ ] T042 [US6] Verify tests pass

---

# PHASE 8: Edge Cases

- [ ] T043 Create edge case tests `crates/rush/tests/integration/for_loops_edge_cases.rs`:
  - Empty loop body (valid no-op)
  - Comments inside loop
  - Multiline formatting variations
  - Loop variable conflict resolution
- [ ] T044 Create fixtures for edge cases
- [ ] T045 Error handling:
  - Invalid loop variable names
  - Command failures in list expansion
  - Syntax errors (missing do, done)
- [ ] T046 Verify comprehensive error messages

---

# PHASE 9: Polish & Validation

- [ ] T047 Performance benchmarks: Parse 100 for loops, avg <1ms each
- [ ] T048 Performance benchmarks: Execute 1000-item for loop, verify <5ms/item overhead
- [ ] T049 Run full suite: `cargo test --all-targets --all-features`
- [ ] T050 Run clippy: `cargo clippy --all-targets --all-features` (fix all warnings)
- [ ] T051 Code formatter: `cargo fmt`
- [ ] T052 Calculate code coverage (target ≥80% for new code)
- [ ] T053 Update documentation:
  - Add for/in/do/done to CLAUDE.md
  - Create docs/018-FOR-LOOPS.md with examples
  - Update README.md with usage examples
- [ ] T054 Final validation: All tests pass, zero regressions
- [ ] T055 Document any performance characteristics or limitations
- [ ] T056 Create final completion report

---

## Summary

| Phase | Focus | Tasks | Status |
|-------|-------|-------|--------|
| 1 | Setup | 6 | Pending |
| 2 | Foundation | 8 | Pending |
| 3 | US1 (Basic) | 8 | Pending |
| 4 | US2 (Expansion) | 5 | Pending |
| 5 | US3 (Nesting) | 4 | Pending |
| 6 | US4 (Variables) | 3 | Pending |
| 7 | US5 (Ranges) | 4 | Pending |
| 8 | US6 (Arrays) | 3 | Pending |
| 9 | Polish | 10 | Pending |

**Total**: 58 tasks
**MVP Path**: Phases 1-5 (39 tasks, ~4-5 weeks)
**Full Feature**: All phases (~7-9 weeks)
