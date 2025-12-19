<<<<<<< HEAD
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
=======
# Feature 018 Task Breakdown: For Loops

**Feature**: 018-for-loops
**Total Tasks**: 58 tasks across 4 phases
**Estimated Effort**: 40-50 hours

---

## Phase 1: Core Parser & Executor (Days 1-2)

### Parser Foundation
- [ ] **T1.1** Create `crates/rush/src/executor/for_loop.rs` module with module declaration in `executor/mod.rs`
- [ ] **T1.2** Define `ForLoop` struct in `executor/mod.rs` with fields: variable (String), word_list (Vec<String>), body (Vec<Command>)
- [ ] **T1.3** Implement `parse_for_loop(input: &str) -> Result<ForLoop, RushError>` function
- [ ] **T1.4** Add `for` keyword detection in command parser (`executor/execute.rs`)
- [ ] **T1.5** Implement error handling for missing `for` keyword
- [ ] **T1.6** Implement error handling for missing `do` keyword
- [ ] **T1.7** Implement error handling for missing `done` keyword
- [ ] **T1.8** Implement error handling for invalid variable names

### Variable Parsing
- [ ] **T1.9** Parse loop variable name (identifier validation)
- [ ] **T1.10** Implement regex for valid variable names (alphanumeric + underscore, not starting with digit)
- [ ] **T1.11** Handle whitespace before and after variable name
- [ ] **T1.12** Return error for reserved keywords used as variable names

### Word List Parsing
- [ ] **T1.13** Parse optional `in` keyword
- [ ] **T1.14** Parse word list (space-separated words until `do` keyword)
- [ ] **T1.15** Handle empty word list (no `in` keyword)
- [ ] **T1.16** Store word list as simple string vector (no expansion yet)
- [ ] **T1.17** Handle quoted words in word list (preserve quotes for now)

### Body Parsing
- [ ] **T1.18** Implement recursive command list parsing for loop body
- [ ] **T1.19** Parse commands separated by semicolons and newlines
- [ ] **T1.20** Properly detect `done` keyword (boundary detection)
- [ ] **T1.21** Handle nested structures in body (if statements, other loops)

### Basic Execution
- [ ] **T1.22** Implement `ForLoop::execute()` method
- [ ] **T1.23** Iterate over word list
- [ ] **T1.24** Bind loop variable to each word (set environment variable)
- [ ] **T1.25** Execute loop body for each word
- [ ] **T1.26** Capture exit code from last command in body
- [ ] **T1.27** Return 0 if word list is empty (no iterations)
- [ ] **T1.28** Handle exit code propagation correctly

### Phase 1 Tests
- [ ] **T1.29** Unit test: `test_parse_for_basic` - basic for/in/do/done
- [ ] **T1.30** Unit test: `test_parse_for_missing_do` - error handling
- [ ] **T1.31** Unit test: `test_parse_for_missing_done` - error handling
- [ ] **T1.32** Unit test: `test_parse_for_invalid_var_name` - error handling
- [ ] **T1.33** Unit test: `test_parse_for_empty_list` - empty word list
- [ ] **T1.34** Integration test: `test_for_basic_iteration` - execute basic loop
- [ ] **T1.35** Integration test: `test_for_empty_word_list` - no execution
- [ ] **T1.36** Integration test: `test_for_single_word` - single iteration

---

## Phase 2: Word List Expansion & Variable Binding (Days 3-4)

### Word List Expansion
- [ ] **T2.1** Implement `expand_word_list(words: &[String]) -> Result<Vec<String>, RushError>`
- [ ] **T2.2** Add variable expansion support (`$VAR`, `${VAR}`)
- [ ] **T2.3** Add command substitution support (`$(cmd)` and `` `cmd` ``)
- [ ] **T2.4** Handle IFS (Internal Field Separator) for word splitting
- [ ] **T2.5** Implement brace expansion support (if available)
- [ ] **T2.6** Ensure NO globbing occurs in word list (treat patterns literally)
- [ ] **T2.7** Handle empty variable expansion (contributes no words)
- [ ] **T2.8** Handle nested expansions correctly
- [ ] **T2.9** Reuse existing expansion functions from `expansion/mod.rs`

### Quote Handling in Word List
- [ ] **T2.10** Preserve quoted strings: `"hello world"` is one word
- [ ] **T2.11** Handle single quotes: no expansion inside
- [ ] **T2.12** Handle double quotes: allow expansion inside
- [ ] **T2.13** Handle escape sequences: `\"`, `\\`, etc.
- [ ] **T2.14** Properly split quoted vs unquoted words

### Positional Parameters (US2)
- [ ] **T2.15** Detect `for var; do` syntax (without `in` keyword)
- [ ] **T2.16** When `in` omitted, use `$@` (all positional parameters)
- [ ] **T2.17** Handle zero positional parameters (loop doesn't execute)
- [ ] **T2.18** Handle one positional parameter
- [ ] **T2.19** Handle multiple positional parameters

### Variable Binding & Scoping
- [ ] **T2.20** Set loop variable in current shell scope (not subshell)
- [ ] **T2.21** Overwrite existing variable with same name
- [ ] **T2.22** Verify loop variable persists after loop with last value
- [ ] **T2.23** If loop doesn't execute, variable unchanged
- [ ] **T2.24** Handle variable names that shadow function parameters

### Phase 2 Tests
- [ ] **T2.25** Unit test: `test_expand_variable_in_list`
- [ ] **T2.26** Unit test: `test_expand_command_substitution`
- [ ] **T2.27** Unit test: `test_no_globbing_in_word_list`
- [ ] **T2.28** Unit test: `test_quote_handling_in_words`
- [ ] **T2.29** Unit test: `test_empty_expansion`
- [ ] **T2.30** Integration test: `test_for_variable_persistence`
- [ ] **T2.31** Integration test: `test_for_variable_shadowing`
- [ ] **T2.32** Integration test: `test_for_positional_parameters_default`
- [ ] **T2.33** Integration test: `test_for_positional_parameters_multiple`
- [ ] **T2.34** Integration test: `test_for_variable_not_modified_if_empty_list`

---

## Phase 3: Complex Loop Bodies & Nesting (Days 5-7)

### Multiline REPL Support
- [ ] **T3.1** Extend `is_statement_complete()` to detect incomplete for loops
- [ ] **T3.2** Check for matching `for`/`done` pair
- [ ] **T3.3** Track nesting depth (for inside if, etc.)
- [ ] **T3.4** Show continuation prompt `>\> ` for incomplete loops
- [ ] **T3.5** Modify `RushPrompt` struct to support continuation state
- [ ] **T3.6** Accumulate multiline input until `done` is seen
- [ ] **T3.7** Parse and execute complete loop statement

### Complex Command Sequences in Loop Body
- [ ] **T3.8** Support multiple commands separated by semicolons
- [ ] **T3.9** Support pipe operator in loop body
- [ ] **T3.10** Support output redirection (`>`, `>>`, `<`)
- [ ] **T3.11** Support command groups `{ cmd1; cmd2; }`
- [ ] **T3.12** Support subshells `( cmd1; cmd2 )`
- [ ] **T3.13** Proper exit code handling for pipelines in body
- [ ] **T3.14** Proper exit code handling for multiple commands

### Nested Control Structures
- [ ] **T3.15** Support if/then/else/fi inside for loop body
- [ ] **T3.16** Implement depth tracking for nested keywords
- [ ] **T3.17** Only match `done` that closes current for loop
- [ ] **T3.18** Support while/until loops inside for loop body
- [ ] **T3.19** Support nested for loops
- [ ] **T3.20** Proper variable binding in nested contexts
- [ ] **T3.21** Test interaction between loop variable and nested structures

### Break/Continue Preparation
- [ ] **T3.22** Design signal mechanism for loop control (LoopSignal enum)
- [ ] **T3.23** Prepare integration points for Features 022/023
- [ ] **T3.24** Document how break/continue will modify execution
- [ ] **T3.25** Create placeholder for signal handling in loop execution

### Phase 3 Tests
- [ ] **T3.26** Integration test: `test_for_multiline_in_repl`
- [ ] **T3.27** Integration test: `test_for_with_pipes`
- [ ] **T3.28** Integration test: `test_for_with_redirections`
- [ ] **T3.29** Integration test: `test_for_with_command_groups`
- [ ] **T3.30** Integration test: `test_for_with_subshells`
- [ ] **T3.31** Integration test: `test_for_in_if_statement`
- [ ] **T3.32** Integration test: `test_if_in_for_statement`
- [ ] **T3.33** Integration test: `test_nested_for_loops`
- [ ] **T3.34** Integration test: `test_for_with_variable_in_condition`
- [ ] **T3.35** Integration test: `test_for_exit_code_from_pipeline`

---

## Phase 4: Edge Cases & Optimization (Days 8-9)

### Special Character Handling
- [ ] **T4.1** Handle words with spaces: `for w in "hello world" foo`
- [ ] **T4.2** Handle words with special characters: `$`, `*`, `?`, etc.
- [ ] **T4.3** Handle words with newlines from command substitution
- [ ] **T4.4** Handle unicode/UTF-8 in variable names and words
- [ ] **T4.5** Handle backslash escapes in words

### IFS Handling
- [ ] **T4.6** Respect default IFS (space, tab, newline)
- [ ] **T4.7** Handle custom IFS values
- [ ] **T4.8** Proper interaction with quotes and escapes
- [ ] **T4.9** Test IFS with command substitution output

### Large Word Lists & Performance
- [ ] **T4.10** Test loop with 100 iterations (target: <100ms)
- [ ] **T4.11** Test loop with 1000 iterations (target: <500ms)
- [ ] **T4.12** Profile and optimize hot paths
- [ ] **T4.13** Verify no memory leaks with large loops
- [ ] **T4.14** Check resource cleanup after loop completion

### Error Conditions & Recovery
- [ ] **T4.15** Command failure in loop body (exit code propagation)
- [ ] **T4.16** Signal handling (SIGINT during loop)
- [ ] **T4.17** Partial loop execution on signal
- [ ] **T4.18** Proper variable state after signal
- [ ] **T4.19** Handle invalid exit codes in body commands

### POSIX Compliance Verification
- [ ] **T4.20** Test against bash for compatibility
- [ ] **T4.21** Test against dash for POSIX compliance
- [ ] **T4.22** Compare exit codes with reference shells
- [ ] **T4.23** Compare variable persistence with reference shells
- [ ] **T4.24** Compare word expansion behavior with reference shells

### Edge Case Tests
- [ ] **T4.25** Unit test: `test_for_words_with_spaces`
- [ ] **T4.26** Unit test: `test_for_words_with_special_chars`
- [ ] **T4.27** Unit test: `test_for_with_newlines_in_words`
- [ ] **T4.28** Integration test: `test_for_large_word_list_100_items`
- [ ] **T4.29** Integration test: `test_for_large_word_list_1000_items`
- [ ] **T4.30** Integration test: `test_for_with_signals`
- [ ] **T4.31** Integration test: `test_for_command_failure_exit_code`
- [ ] **T4.32** Integration test: `test_custom_ifs_handling`

### Code Quality & Documentation
- [ ] **T4.33** Add inline comments for complex logic
- [ ] **T4.34** Document AST structure in comments
- [ ] **T4.35** Document expansion behavior
- [ ] **T4.36** Update executor module documentation
- [ ] **T4.37** Create example usage in code comments
- [ ] **T4.38** Verify >95% code coverage for for-loop module

### Final Validation
- [ ] **T4.39** Run full test suite (`cargo test`)
- [ ] **T4.40** Run clippy for code quality (`cargo clippy`)
- [ ] **T4.41** Verify all 45+ tests pass
- [ ] **T4.42** Check for any compiler warnings
- [ ] **T4.43** Validate POSIX compliance
- [ ] **T4.44** Performance verification (1000 iterations)

---

## Implementation Dependencies

### Required Before Starting
- ✅ Feature 017 (Conditional Control Flow) - parser patterns
- ✅ Feature 001 (Command Execution) - for body execution
- ✅ Existing expansion utilities - for word list expansion

### Required During Implementation
- Command execution infrastructure (already available)
- Environment variable management (already available)
- Signal handling (already available from Feature 006)

### Integration Points for Future Features
- Feature 019 (while/until) - will reuse loop control infrastructure
- Feature 022 (break) - will hook into loop execution
- Feature 023 (continue) - will hook into loop execution

---

## Success Criteria Checklist

**Phase 1 Complete When**:
- [ ] All parser tests pass (8 tests)
- [ ] Basic loops execute correctly
- [ ] Exit codes match POSIX semantics
- [ ] Code compiles without warnings

**Phase 2 Complete When**:
- [ ] Word expansion tests pass (10 tests)
- [ ] Variable binding tests pass
- [ ] Positional parameters work correctly
- [ ] All Phase 1 + Phase 2 tests pass (18+ total)

**Phase 3 Complete When**:
- [ ] Multiline REPL support works
- [ ] Complex bodies execute correctly
- [ ] Nested structures work
- [ ] All 35+ tests pass

**Phase 4 Complete When**:
- [ ] All 45+ tests pass
- [ ] POSIX compliance verified
- [ ] Performance acceptable (<500ms for 1000 iterations)
- [ ] Code coverage >95%
- [ ] Ready for PR and merge

---

## Testing Strategy

### Unit Tests (15 tests)
- Parser functionality
- Word expansion logic
- Variable binding
- Error conditions

### Integration Tests (30 tests)
- Basic for loops
- Complex bodies
- Nested structures
- POSIX compatibility
- Edge cases
- Performance

### Test Execution
```bash
# Run all tests
cargo test --test '*' for_loop

# Run specific test
cargo test for_loop::test_parse_for_basic

# Check coverage
cargo tarpaulin --out Html
>>>>>>> 025-subshells
```

---

<<<<<<< HEAD
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
=======
## Notes

- Follow Feature 017 pattern for nested keyword detection
- Reuse expansion utilities from `expansion/mod.rs`
- Consider performance implications of word list expansion
- Document all integration points with other features
- Keep implementation modular for easier debugging

---

**Created**: 2025-12-06
**Status**: Task Breakdown Complete
**Estimated Total Duration**: 8-10 development days
**Next Phase**: Begin Phase 1 implementation
>>>>>>> 025-subshells
