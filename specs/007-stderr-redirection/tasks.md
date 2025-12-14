# Implementation Tasks: Feature 007 - Stderr Redirection (2>, 2>>)

**Feature**: Stderr Redirection (2>, 2>>)
**Branch**: `007-stderr-redirection`
**Created**: 2025-11-30
**Status**: Ready for Implementation

## Overview

Implement stderr redirection operators (2>, 2>>) to allow users to redirect error output to files. Based on the technical plan, this involves extending the existing output redirection system in three phases:
- **Phase 1**: Data structure extension (enum variant)
- **Phase 2**: Parser enhancement (token recognition)
- **Phase 3**: Executor implementation (dup2() redirection)

## User Story Summary

| ID | Story | Priority | Tests | Dependencies |
|----|-------|----------|-------|--------------|
| US1 | Redirect Stderr to File (2>) | P1 | ✓ | Phase 1,2,3 |
| US2 | Append Stderr to File (2>>) | P1 | ✓ | Phase 1,2,3 |
| US3 | Redirect Both Streams (2>&1) | P2 | - | Deferred |
| US4 | Combine with Pipes | P2 | ✓ | Phase 1,2,3 |
| US5 | Separate Stdout/Stderr | P2 | ✓ | Phase 1,2,3 |

## Implementation Strategy

**MVP Scope**: Complete User Stories 1 & 2 (basic 2> and 2>> redirection)
- Covers core functionality: redirect stderr to file with truncate/append
- Simplest path to working feature
- Other stories (pipes, fd duplication) can follow incrementally

**Full Feature**: All 5 user stories (minus 2>&1 which defers to feature 008)
- Comprehensive stderr handling in all contexts
- Estimated 4-6 hours of implementation + testing

**Parallelization**: Minimal - features are sequential due to dependencies
- Phase 1 (structures) must complete before Phase 2 (parsing)
- Phase 2 must complete before Phase 3 (execution)

---

## Phase 1: Foundation - Data Structure Extension

**Goal**: Extend RedirectionType enum to support Stderr variant

**Files Modified**:
- `/Users/chrischeng/projects/rustation/crates/rush/src/main.rs` (lines 40-50)

**Independent Test Criteria**:
- Code compiles with new enum variant
- Pattern matching on RedirectionType includes Stderr case
- Existing Stdout redirections still work

### Tasks

- [ ] T001 [P] Add `Stderr(bool)` variant to RedirectionType enum in main.rs (lines 40-50)
- [ ] T002 [P] Update executor match statement to handle RedirectionType::Stderr case in main.rs (lines 520-560)
- [ ] T003 [P] Verify code compiles: `cargo check -p rush`
- [ ] T004 [P] Run existing redirection tests to ensure no regression: `cargo test -p rush redirect`

---

## Phase 2: Foundation - Parser Enhancement

**Goal**: Enhance parser to recognize and parse 2> and 2>> tokens

**Files Modified**:
- `/Users/chrischeng/projects/rustation/crates/rush/src/main.rs` (lines 150-260)

**Independent Test Criteria**:
- Tokenizer recognizes `2>` and `2>>` as valid tokens
- Parser creates `RedirectionType::Stderr(false)` for 2>
- Parser creates `RedirectionType::Stderr(true)` for 2>>
- Invalid syntax rejected (e.g., `2>` without filename)
- Existing > and >> redirections still work

### Tasks

- [ ] T005 [P] Modify tokenizer to recognize 2> and 2>> tokens in main.rs (lines 150-200)
  - Check if token starts with '2' followed by '>' or '>>'
  - Treat as single token like existing redirections

- [ ] T006 [P] Update parse_redirections() function to handle 2> and 2>> in main.rs (lines 220-260)
  - Add case for RedirectionType::Stderr in match statement
  - Parse following token as filename (reuse existing filename parsing)
  - Set append flag based on 2> (false) vs 2>> (true)

- [ ] T007 [P] Add validation for stderr redirection targets
  - Ensure filename follows 2> or 2>>
  - Report error if token missing or is another operator
  - Error message: "Invalid redirection: 2> requires filename"

- [ ] T008 Add unit tests for parser in main.rs test module
  - Test tokenizer recognizes 2> and 2>> tokens
  - Test parser creates Stderr(false) for 2>
  - Test parser creates Stderr(true) for 2>>
  - Test invalid syntax rejected (2> with no filename)

- [ ] T009 Verify parser changes: `cargo check -p rush`

- [ ] T010 Run parser tests: `cargo test -p rush parser`

---

## Phase 3.1: US1 - Redirect Stderr to File (2>)

**Goal**: Implement basic stderr redirection to file (truncate mode)

**Files Modified**:
- `/Users/chrischeng/projects/rustation/crates/rush/src/main.rs` (lines 480-560)

**Independent Test Criteria**:
- Command: `echo msg >&2 2>error.txt` produces file with stderr content
- Stdout on terminal is empty
- File is created with correct permissions
- Existing file is overwritten (not appended)

### Tasks

- [ ] T011 [US1] Extend file opening logic to handle RedirectionType::Stderr in execute_command() (lines 520-540)
  - Use same file opening flags as stdout: O_WRONLY | O_CREAT | O_TRUNC
  - Set mode to 0o644 (same as stdout)
  - Handle errors gracefully

- [ ] T012 [US1] Implement stderr fd redirection via dup2() in main.rs (lines 540-560)
  - Call `dup2(file_fd, 2)` for stderr (fd 2)
  - Must happen before execve() so subprocess inherits redirected fd
  - Check dup2() return value for errors

- [ ] T013 [US1] Add error handling for dup2() failures
  - Print error message to original stderr: "Failed to redirect stderr: [error]"
  - Don't execute command if dup2() fails
  - Close file descriptor in error path

- [ ] T014 [US1] Write integration test for basic stderr redirection
  - Test script: `echo error >&2 2>test_err.txt`
  - Verify file test_err.txt contains "error"
  - Verify stdout is empty

- [ ] T015 [US1] Write integration test for file overwrite behavior
  - Create file with "old content"
  - Execute: `echo new >&2 2>test_err.txt`
  - Verify file contains only "new" (not "old content\nnew")

- [ ] T016 [US1] Run tests: `cargo test -p rush`

---

## Phase 3.2: US2 - Append Stderr to File (2>>)

**Goal**: Implement stderr append redirection to file

**Files Modified**:
- `/Users/chrischeng/projects/rustation/crates/rush/src/main.rs` (lines 520-540)

**Independent Test Criteria**:
- Command: `echo msg >&2 2>>error.txt` appends to file
- File exists after first command, stderr appends second time
- Existing content preserved (not overwritten)

### Tasks

- [ ] T017 [P] [US2] Modify file opening logic to handle append mode (2>>)
  - Check if RedirectionType::Stderr(true) (append flag set)
  - Use O_APPEND flag instead of O_TRUNC for append mode
  - Create file if doesn't exist (O_CREAT flag)

- [ ] T018 [P] [US2] Write integration test for stderr append
  - Create file with "first line\n"
  - Execute: `echo second >&2 2>>test_err.txt`
  - Verify file contains "first line\nsecond\n"

- [ ] T019 [P] [US2] Write integration test for append to new file
  - Delete file if exists
  - Execute: `echo line1 >&2 2>>test_err.txt && echo line2 >&2 2>>test_err.txt`
  - Verify file contains both lines

- [ ] T020 [US2] Run tests: `cargo test -p rush`

---

## Phase 3.3: US4 - Combine Stderr Redirection with Pipes

**Goal**: Support stderr redirection in pipelines

**Files Modified**:
- `/Users/chrischeng/projects/rustation/crates/rush/src/main.rs` (executor pipeline handling)

**Independent Test Criteria**:
- Command: `cmd 2>err.txt | next_cmd` - stdout piped, stderr redirected
- Command: `cmd1 | cmd2 2>err.txt` - stderr from cmd2 redirected
- Each command in pipeline can have independent stderr redirection

### Tasks

- [ ] T021 [US4] Verify stderr redirection works in pipelines
  - Test: `echo out; echo err >&2 2>err.txt | cat`
  - Verify cat receives stdout, err.txt has stderr

- [ ] T022 [US4] Write integration test for pipeline with stderr redirect
  - Test: `echo msg >&2 2>err.txt | grep msg` (should find nothing, grep gets nothing)
  - Verify stderr in err.txt

- [ ] T023 [US4] Write integration test for second command redirect in pipeline
  - Test: `echo input | cat >&2 2>err.txt`
  - Verify err.txt contains piped input

- [ ] T024 [US4] Run tests: `cargo test -p rush`

---

## Phase 3.4: US5 - Redirect Stdout and Stderr to Different Files

**Goal**: Support separate redirections for stdout and stderr

**Files Modified**:
- No new changes; reuses existing logic from US1+US2

**Independent Test Criteria**:
- Command: `cmd > out.txt 2> err.txt` works correctly
- Stdout in out.txt, stderr in err.txt, each clean (no mixing)

### Tasks

- [ ] T025 [P] [US5] Write integration test for separate stdout/stderr redirection
  - Test: `echo out; echo err >&2 > out.txt 2> err.txt`
  - Verify out.txt contains "out"
  - Verify err.txt contains "err"

- [ ] T026 [P] [US5] Write integration test for overwriting both files
  - Test: First run with output, then run again
  - Verify both files contain only new output (truncated, not appended)

- [ ] T027 [P] [US5] Write integration test for combining truncate and append
  - Test: `echo out > out.txt 2>> err.txt`
  - Verify out.txt truncated, err.txt appended

- [ ] T028 [US5] Run tests: `cargo test -p rush`

---

## Phase 4: Polish & Integration

**Goal**: Final validation, error handling, documentation

**Files Modified**:
- `/Users/chrischeng/projects/rustation/crates/rush/src/main.rs`

### Tasks

- [ ] T029 Run full test suite: `cargo test -p rush`

- [ ] T030 Run clippy for code quality: `cargo clippy -p rush --all-targets --all-features`

- [ ] T031 Test error cases
  - Permission denied: Try to redirect to /root/err.txt (should fail gracefully)
  - Invalid path: Try /invalid/path/file.txt
  - Directory as target: Try 2>/tmp (directory, not file)

- [ ] T032 Verify error messages are user-friendly
  - Print to stderr before executing command
  - Include reason for failure (permission, path not found, etc.)
  - Command doesn't execute if redirection fails

- [ ] T033 Test edge cases
  - Empty stderr: Command with no output
  - Large stderr: Redirect megabytes of error output
  - Special characters in filename: `2>"file with spaces.txt"`
  - Relative and absolute paths: `2>./err.txt` and `2>/tmp/err.txt`

- [ ] T034 Update project documentation (if applicable)
  - Add stderr redirection to shell feature list
  - Document 2> and 2>> operators
  - Note: 2>&1 deferred to feature 008

- [ ] T035 Final integration test: `cargo test -p rush -- --nocapture`

---

## Task Dependencies

```
Phase 1 (Foundation)
├── T001: Add Stderr variant to enum
├── T002: Update executor match
├── T003-T004: Compile & test
└── → Phase 2

Phase 2 (Parser)
├── T005: Tokenizer enhancement
├── T006-T007: Parser updates
├── T008-T010: Testing
└── → Phase 3

Phase 3.1 (US1 - Basic 2>)
├── T011-T013: Implementation
├── T014-T015: Tests
└── T016: Full test suite

Phase 3.2 (US2 - Append 2>>)
├── T017: Append mode logic
├── T018-T019: Tests
└── T020: Full test suite

Phase 3.3 (US4 - Pipes)
├── T021-T023: Tests
└── T024: Full test suite

Phase 3.4 (US5 - Separate Files)
├── T025-T027: Tests
└── T028: Full test suite

Phase 4 (Polish)
├── T029-T035: Validation & cleanup
└── Ready for PR
```

## Parallel Opportunities

**Limited parallelization** due to sequential dependencies:

1. **Phase 1 & 2 Serial**: Enum must exist before parser uses it
2. **Phase 3 Serial**: Parser must work before executor implementation
3. **Within Phase 3**: User stories can be tested independently once base implementation done

**Minimal parallelization possible**:
- T014-T015 (US1 tests) can run in parallel with each other
- T018-T019 (US2 tests) can run in parallel with each other
- Once Phase 3 complete: All story tests can run in parallel

## Quality Criteria

Before marking complete, verify:

- [ ] All tests pass: `cargo test -p rush`
- [ ] No clippy warnings: `cargo clippy -p rush --all-targets --all-features`
- [ ] Code compiles: `cargo check -p rush`
- [ ] Acceptance scenarios pass (all 5 user stories covered)
- [ ] Error cases handled gracefully
- [ ] Existing features not broken (feature 005 tests still pass)

## Estimated Effort

| Phase | Effort | Notes |
|-------|--------|-------|
| Phase 1 | 15 min | Simple enum extension |
| Phase 2 | 30 min | Parser tokenizer changes |
| Phase 3 | 90 min | Implement dup2(), write tests |
| Phase 4 | 30 min | Validation, edge cases |
| **Total** | **~3 hours** | Incremental, testable |

## MVP Deliverable

**Minimum for shipping**: Complete User Stories 1 & 2 (T001-T020)
- Basic 2> redirection
- Basic 2>> append
- ~1.5 hours effort
- Creates independent, testable feature

**Full feature**: All user stories complete (T001-T035)
- Comprehensive stderr handling
- ~3 hours effort
- Production-ready

## Next Steps

1. **Start Phase 1**: Add Stderr enum variant (T001)
2. **Run T003-T004**: Verify compilation
3. **Proceed to Phase 2**: Enhance parser
4. **Iterate through Phase 3**: Implement user stories incrementally
5. **Phase 4**: Final validation
6. **Create PR**: Submit for review

---

**Generated**: 2025-11-30
**Status**: Ready for Implementation
**Total Tasks**: 35
**Estimated Duration**: ~3 hours
