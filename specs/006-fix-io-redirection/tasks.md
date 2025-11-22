# Tasks: Fix I/O Redirection Bug

**Input**: Design documents from `/specs/006-fix-io-redirection/`
**Prerequisites**: plan.md (complete), spec.md (complete), data-model.md (complete)

**Tests**: Included - tests are critical for verifying bug fix

**Organization**: Tasks organized by user story to enable independent testing of each redirection type (>, >>, <)

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Repository structure (Rust monorepo):
- Source: `crates/rush/src/`
- Tests: `crates/rush/tests/`
- Integration tests: `crates/rush/tests/integration/`

---

## Phase 1: Setup (Structural Changes)

**Purpose**: Add missing field to PipelineSegment struct and update constructor

- [ ] T001 Add `redirections: Vec<Redirection>` field to PipelineSegment struct in crates/rush/src/executor/mod.rs
- [ ] T002 Update PipelineSegment::new() constructor to accept redirections parameter in crates/rush/src/executor/mod.rs
- [ ] T003 Update all existing PipelineSegment::new() calls in tests to pass empty vec![] for redirections

---

## Phase 2: Foundational (Parser Updates)

**Purpose**: Make parser populate redirections field when creating segments

**⚠️ CRITICAL**: This phase must complete before user story implementation - redirections must flow from parser to executor

- [ ] T004 Read existing parse_command_with_redirections() logic in crates/rush/src/executor/parser.rs to understand redirection extraction
- [ ] T005 Update split_into_segments() function in crates/rush/src/executor/parser.rs to extract redirections from tokens
- [ ] T006 Pass extracted redirections to PipelineSegment::new() in split_into_segments() function
- [ ] T007 Add unit test for parse_pipeline() with redirections in crates/rush/src/executor/parser.rs (verify segments contain redirections)

**Checkpoint**: Parser now populates redirections field - ready for executor implementation

---

## Phase 3: User Story 1 - Output Redirection (Priority: P1) 🎯 MVP

**Goal**: Users can redirect command output to files using `>` operator

**Independent Test**: Run `echo hello > /tmp/test.txt` and verify file contains "hello" (not literal "> /tmp/test.txt")

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T008 [P] [US1] Add integration test for output redirection in crates/rush/tests/feature_test.rs (test_output_redirection already exists - verify it still fails)
- [ ] T009 [P] [US1] Add test for output redirection with existing file (truncation) in crates/rush/tests/feature_test.rs
- [ ] T010 [P] [US1] Add test for output redirection error cases (permission denied, is directory) in crates/rush/tests/integration/redirection_errors.rs

### Implementation for User Story 1

- [ ] T011 [US1] Update execute_single_command() in crates/rush/src/executor/pipeline.rs to use segment.redirections instead of extract_redirections_from_args()
- [ ] T012 [US1] Verify RedirectionType::Output handling in pipeline.rs applies File::create() correctly (already implemented - just needs to be called)
- [ ] T013 [US1] Remove extract_redirections_from_args() call from execute_single_command() in crates/rush/src/executor/pipeline.rs (now redundant)
- [ ] T014 [US1] Run cargo test --test feature_test::test_output_redirection and verify it now PASSES
- [ ] T015 [US1] Test output redirection manually: echo hello > /tmp/rush_test.txt

**Checkpoint**: Output redirection (>) fully functional and independently testable

---

## Phase 4: User Story 2 - Append Redirection (Priority: P1)

**Goal**: Users can append command output to files using `>>` operator

**Independent Test**: Run `echo first >> /tmp/test.txt` then `echo second >> /tmp/test.txt` and verify both lines exist

### Tests for User Story 2

- [ ] T016 [P] [US2] Add integration test for append redirection in crates/rush/tests/feature_test.rs (test_append_redirection already exists - verify it fails then passes)
- [ ] T017 [P] [US2] Add test for append to non-existent file (should create) in crates/rush/tests/feature_test.rs
- [ ] T018 [P] [US2] Add test for multiple appends to same file in crates/rush/tests/feature_test.rs

### Implementation for User Story 2

- [ ] T019 [US2] Verify RedirectionType::Append handling in crates/rush/src/executor/pipeline.rs uses OpenOptions::append() correctly (already implemented - just needs segment.redirections)
- [ ] T020 [US2] Run cargo test --test feature_test::test_append_redirection and verify it now PASSES
- [ ] T021 [US2] Test append redirection manually: echo line1 >> /tmp/test.txt && echo line2 >> /tmp/test.txt

**Checkpoint**: Append redirection (>>) fully functional and independently testable

---

## Phase 5: User Story 3 - Input Redirection (Priority: P2)

**Goal**: Users can provide file contents as input to commands using `<` operator

**Independent Test**: Create file with content, run `cat < /tmp/test.txt` and verify it reads from file

### Tests for User Story 3

- [ ] T022 [P] [US3] Add integration test for input redirection in crates/rush/tests/integration/redirection_test.rs
- [ ] T023 [P] [US3] Add test for input redirection from non-existent file (should error) in crates/rush/tests/integration/redirection_test.rs
- [ ] T024 [P] [US3] Add test for input redirection from directory (should error) in crates/rush/tests/integration/redirection_test.rs

### Implementation for User Story 3

- [ ] T025 [US3] Verify RedirectionType::Input handling in crates/rush/src/executor/pipeline.rs opens file for reading correctly (already implemented - just needs segment.redirections)
- [ ] T026 [US3] Run cargo test --test redirection_test and verify input redirection tests PASS
- [ ] T027 [US3] Test input redirection manually: echo test > /tmp/in.txt && cat < /tmp/in.txt

**Checkpoint**: Input redirection (<) fully functional and independently testable

---

## Phase 6: Polish & Integration

**Purpose**: Cross-cutting improvements, edge cases, and documentation

- [ ] T028 [P] Test combinations: redirections with pipelines (ls | grep txt > results.txt) in crates/rush/tests/integration/pipeline_redirection_test.rs
- [ ] T029 [P] Test combinations: redirections with background jobs (sleep 1 > /tmp/out.txt &) in crates/rush/tests/integration/background_redirection_test.rs
- [ ] T030 [P] Test edge case: multiple redirections same type (last wins) in crates/rush/tests/integration/redirection_test.rs
- [ ] T031 [P] Test edge case: redirection to /dev/null in crates/rush/tests/integration/redirection_test.rs
- [ ] T032 Update TEST_COVERAGE.md with new test counts and redirection test coverage in crates/rush/TEST_COVERAGE.md
- [ ] T033 Run full test suite: cargo test (verify all 247+ existing tests still pass)
- [ ] T034 Run cargo clippy and fix any warnings related to changes
- [ ] T035 Run cargo fmt on modified files

**Checkpoint**: All tests passing, code quality verified, ready for PR

---

## Dependencies & Execution Order

### Critical Path (Must be sequential)
```
Phase 1 (Setup) → Phase 2 (Parser) → Phase 3 (US1 Output) → Phase 4 (US2 Append) → Phase 5 (US3 Input) → Phase 6 (Polish)
```

### Parallel Opportunities

**Within Phase 3 (US1)**:
- T008, T009, T010 can run in parallel (different test files)
- T011-T013 must be sequential (same file)

**Within Phase 4 (US2)**:
- T016, T017, T018 can run in parallel (different tests)

**Within Phase 5 (US3)**:
- T022, T023, T024 can run in parallel (different tests)

**Within Phase 6 (Polish)**:
- T028, T029, T030, T031, T032 can all run in parallel
- T033-T035 must be sequential (validation steps)

### User Story Independence

- **US1** (Output) can be tested independently after Phase 2
- **US2** (Append) can be tested independently after Phase 2
- **US3** (Input) can be tested independently after Phase 2

All three user stories share the same structural change (redirections field) but test different code paths in pipeline.rs.

---

## MVP Scope

**Minimum Viable Product**: Phase 1 + Phase 2 + Phase 3 (US1 Output Redirection)

This gives users the most commonly used redirection feature (>) with ~25 tasks.

**Incremental Delivery**:
1. **MVP**: US1 Output (>) - ~25 tasks
2. **V2**: + US2 Append (>>) - ~31 tasks total
3. **V3**: + US3 Input (<) - ~35 tasks total
4. **Full**: + Polish - ~35 tasks total

---

## Task Summary

- **Total Tasks**: 35
- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 4 tasks
- **Phase 3 (US1 Output)**: 8 tasks
- **Phase 4 (US2 Append)**: 6 tasks
- **Phase 5 (US3 Input)**: 6 tasks
- **Phase 6 (Polish)**: 8 tasks

**Parallel Tasks**: 18 marked with [P] (51% can run in parallel)

**Estimated Effort**: ~2-3 hours for experienced Rust developer
- Setup: 30min
- Parser updates: 45min
- US1 implementation + tests: 30min
- US2 implementation + tests: 20min
- US3 implementation + tests: 20min
- Polish + validation: 30min

---

## Implementation Strategy

### Test-Driven Approach

For each user story:
1. Write tests FIRST (they should fail - demonstrating the bug)
2. Implement fix
3. Run tests (they should now pass)
4. Verify manually

### Verification Checklist

After completing each phase:
- [ ] All new tests pass
- [ ] All existing tests still pass (no regressions)
- [ ] Manual testing confirms expected behavior
- [ ] No clippy warnings
- [ ] Code formatted with cargo fmt

### Risk Mitigation

- **Risk**: Breaking existing pipeline/job control
  - **Mitigation**: Run full test suite after each phase
- **Risk**: Missing edge cases
  - **Mitigation**: Comprehensive test coverage in Phase 6
- **Risk**: Performance regression
  - **Mitigation**: Vec<Redirection> is zero-cost when empty

---

## Success Criteria (from spec.md)

- [x] **SC-001**: All integration tests for I/O redirection pass → T008-T027
- [x] **SC-002**: `echo hello > /tmp/test.txt` works <100ms → T015 manual test
- [x] **SC-003**: Works with pipelines → T028
- [x] **SC-004**: Clear error messages → T010, T024 (verify existing error handling)

All success criteria mapped to specific tasks above.
