# Tasks: Read Builtin

**Input**: Design documents from `/specs/030-read-builtin/`
**Prerequisites**: plan.md, spec.md, research.md

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup

**Purpose**: Register read builtin module in the builtins system

- [ ] T001 Add `pub mod read;` to crates/rush/src/executor/builtins/mod.rs
- [ ] T002 Add match case for "read" command in execute_builtin function in crates/rush/src/executor/builtins/mod.rs

---

## Phase 2: Foundational (Core Read Implementation)

**Purpose**: Implement basic read functionality that all user stories depend on

- [ ] T003 Create crates/rush/src/executor/builtins/read.rs with module structure and imports
- [ ] T004 Implement ReadOptions struct to hold parsed options (-p, -s, -r, -d, -n, -t)
- [ ] T005 Implement parse_options() function to extract options from args
- [ ] T006 Implement read_line_from_stdin() basic input function
- [ ] T007 Implement split_input_by_ifs() for word splitting with IFS handling
- [ ] T008 Implement assign_to_variables() to set variable values via executor

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Basic Variable Input (Priority: P1) 🎯 MVP

**Goal**: Read user input and store in a variable

**Independent Test**: Run `read name` with input "Alice", verify `$name` equals "Alice"

### Implementation for User Story 1

- [ ] T009 [US1] Implement execute() main function with basic read-to-REPLY in crates/rush/src/executor/builtins/read.rs
- [ ] T010 [US1] Add support for reading into named variable(s) in crates/rush/src/executor/builtins/read.rs
- [ ] T011 [US1] Handle empty input (just Enter pressed) returning empty string
- [ ] T012 [US1] Implement exit status (0 success, 1 EOF)

**Checkpoint**: Basic `read var` functionality working

---

## Phase 4: User Story 2 - Multiple Variables (Priority: P1)

**Goal**: Read space-separated values into multiple variables

**Independent Test**: Run `read a b` with input "foo bar", verify both variables set

### Implementation for User Story 2

- [ ] T013 [US2] Extend assign_to_variables() to handle multiple variable names in crates/rush/src/executor/builtins/read.rs
- [ ] T014 [US2] Implement "last variable gets remainder" logic for excess words

**Checkpoint**: Multiple variable assignment working

---

## Phase 5: User Story 3 - Custom Prompt (Priority: P2)

**Goal**: Display prompt before reading input

**Independent Test**: Run `read -p "Name: " name`, verify prompt displays

### Implementation for User Story 3

- [ ] T015 [US3] Implement -p option handling in parse_options() in crates/rush/src/executor/builtins/read.rs
- [ ] T016 [US3] Print prompt string before reading input (no newline after prompt)

**Checkpoint**: Prompt display working

---

## Phase 6: User Story 4 - Silent Password Input (Priority: P2)

**Goal**: Read input without echoing characters

**Independent Test**: Run `read -s password`, verify no echo, value captured

### Implementation for User Story 4

- [ ] T017 [US4] Implement -s option handling with crossterm terminal control in crates/rush/src/executor/builtins/read.rs
- [ ] T018 [US4] Disable echo before reading, restore after reading

**Checkpoint**: Silent mode working

---

## Phase 7: User Story 5 - Custom Delimiter (Priority: P3)

**Goal**: Read until a specific character instead of newline

**Independent Test**: Run `read -d ":" value` with input "hello:", verify `$value` equals "hello"

### Implementation for User Story 5

- [ ] T019 [US5] Implement -d option handling for custom delimiter in crates/rush/src/executor/builtins/read.rs
- [ ] T020 [US5] Read character-by-character until delimiter reached

**Checkpoint**: Custom delimiter working

---

## Phase 8: User Story 6 - Additional Options (Priority: P3)

**Goal**: Support -r (raw), -n (count), -t (timeout) options

### Implementation for User Story 6

- [ ] T021 [P] [US6] Implement -r option (no backslash interpretation) in crates/rush/src/executor/builtins/read.rs
- [ ] T022 [P] [US6] Implement -n option (read COUNT characters) in crates/rush/src/executor/builtins/read.rs
- [ ] T023 [US6] Implement -t option (timeout in seconds) in crates/rush/src/executor/builtins/read.rs

**Checkpoint**: All options working

---

## Phase 9: Integration Tests

**Purpose**: Verify all functionality with integration tests

- [ ] T024 [P] Create crates/rush/tests/integration/read_builtin_tests.rs test file
- [ ] T025 [P] Add test_read_basic_input test
- [ ] T026 [P] Add test_read_multiple_variables test
- [ ] T027 [P] Add test_read_with_prompt test
- [ ] T028 [P] Add test_read_default_reply test
- [ ] T029 [P] Add test_read_raw_mode test

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup and validation

- [ ] T030 Add unit tests in crates/rush/src/executor/builtins/read.rs
- [ ] T031 Run cargo clippy and fix any warnings
- [ ] T032 Run cargo fmt
- [ ] T033 Run full test suite (cargo test)
- [ ] T034 Update specs/features.json to mark 030 as complete

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
- **Integration Tests (Phase 9)**: Depends on Phase 3 minimum
- **Polish (Phase 10)**: Depends on all implementation phases

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies
- **User Story 2 (P1)**: Can start after US1 (extends assign logic)
- **User Story 3 (P2)**: Can start after Foundational - Independent
- **User Story 4 (P2)**: Can start after Foundational - Independent
- **User Story 5 (P3)**: Can start after Foundational - Independent
- **User Story 6 (P3)**: Can start after Foundational - Independent

### Parallel Opportunities

- US3, US4, US5, US6 can all be implemented in parallel after US1+US2
- All integration tests (T024-T029) can run in parallel
- T021, T022 can run in parallel (different options, same file but different sections)

---

## Implementation Strategy

### MVP First (User Stories 1+2)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (basic read)
4. Complete Phase 4: User Story 2 (multiple vars)
5. **STOP and VALIDATE**: Test basic read functionality
6. Add remaining user stories incrementally

### Total Task Count

- **Setup**: 2 tasks
- **Foundational**: 6 tasks
- **User Stories**: 15 tasks (across 6 stories)
- **Integration Tests**: 6 tasks
- **Polish**: 5 tasks
- **TOTAL**: 34 tasks

---

## Notes

- [P] tasks = different files/sections, no dependencies
- [Story] label maps task to specific user story
- Commit after each task or logical group
- Array support (-a) deferred until arrays feature (037) is implemented
