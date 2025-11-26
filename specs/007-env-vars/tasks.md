# Tasks: Environment Variables

**Input**: Design documents from `/specs/007-env-vars/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Unit tests included as spec mentions "cargo test with unit tests and integration tests"

**Organization**: Tasks grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust monorepo**: `crates/rush/src/` for source, `crates/rush/tests/` for tests

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and module structure

- [x] T001 Create environment module file at `crates/rush/src/executor/environment.rs`
- [x] T002 Add `pub mod environment;` to `crates/rush/src/executor/mod.rs`
- [x] T003 [P] Add EnvError type to `crates/rush/src/error.rs` for environment-related errors

---

## Phase 2: Foundational (EnvironmentManager - Blocking Prerequisites)

**Purpose**: Core EnvironmentManager struct that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Implement `EnvironmentManager` struct with `HashMap<String, String>` storage in `crates/rush/src/executor/environment.rs`
- [x] T005 Implement `EnvironmentManager::new()` that inherits from `std::env::vars()` in `crates/rush/src/executor/environment.rs`
- [x] T006 Implement `is_valid_variable_name()` validation function (regex: `[a-zA-Z_][a-zA-Z0-9_]*`) in `crates/rush/src/executor/environment.rs`
- [x] T007 Implement `EnvironmentManager::get(&self, name: &str) -> Option<&str>` in `crates/rush/src/executor/environment.rs`
- [x] T008 Implement `EnvironmentManager::set(&mut self, name: String, value: String) -> Result<(), EnvError>` in `crates/rush/src/executor/environment.rs`
- [x] T009 Implement `EnvironmentManager::iter()` and `as_env_map()` methods in `crates/rush/src/executor/environment.rs`
- [x] T010 Add `env_manager: EnvironmentManager` field to `CommandExecutor` struct in `crates/rush/src/executor/execute.rs`
- [x] T011 Initialize `EnvironmentManager::new()` in `CommandExecutor::new()` in `crates/rush/src/executor/execute.rs`
- [x] T012 [P] Add `env_manager()` and `env_manager_mut()` accessor methods to `CommandExecutor` in `crates/rush/src/executor/execute.rs`
- [x] T013 [P] Add unit tests for `EnvironmentManager` and `is_valid_variable_name()` in `crates/rush/src/executor/environment.rs`

**Checkpoint**: Foundation ready - EnvironmentManager exists, inherits system env, accessible from CommandExecutor

---

## Phase 3: User Story 1 + 4 - Variable Expansion + Inheritance (Priority: P1) 🎯 MVP

**Goal**: Users can use `$VAR` and `${VAR}` syntax to expand environment variables in commands. Shell inherits system environment on startup.

**Independent Test**: Run `echo $HOME` and verify home directory is printed. Run `echo $PATH` and verify PATH is displayed.

### Tests for User Stories 1 & 4

- [x] T014 [P] [US1] Add unit test for basic `$VAR` expansion in `crates/rush/src/executor/parser.rs`
- [x] T015 [P] [US1] Add unit test for `${VAR}` braced expansion in `crates/rush/src/executor/parser.rs`
- [x] T016 [P] [US1] Add unit test for undefined variable expanding to empty string in `crates/rush/src/executor/parser.rs`
- [x] T017 [P] [US1] Add unit test for escaped `\$` producing literal dollar sign in `crates/rush/src/executor/parser.rs`
- [x] T018 [P] [US1] Add unit test for variable in middle of string (`foo$VAR bar`) in `crates/rush/src/executor/parser.rs`

### Implementation for User Stories 1 & 4

- [x] T019 [US1] Implement `expand_variables_in_string(input: &str, env: &EnvironmentManager) -> String` function in `crates/rush/src/executor/parser.rs`
- [x] T020 [US1] Handle `$VAR` syntax (simple variable reference) in `expand_variables_in_string` in `crates/rush/src/executor/parser.rs`
- [x] T021 [US1] Handle `${VAR}` syntax (braced variable reference) in `expand_variables_in_string` in `crates/rush/src/executor/parser.rs`
- [x] T022 [US1] Handle `\$` escape sequence (literal dollar sign) in `expand_variables_in_string` in `crates/rush/src/executor/parser.rs`
- [x] T023 [US1] Handle undefined variables (expand to empty string) in `expand_variables_in_string` in `crates/rush/src/executor/parser.rs`
- [x] T024 [US1] Implement `expand_variables(segments: &mut [PipelineSegment], env: &EnvironmentManager)` to expand all args in `crates/rush/src/executor/parser.rs`
- [x] T025 [US1] Also expand variables in `segment.program` (command name) and `redirection.file_path` in `crates/rush/src/executor/parser.rs`
- [x] T026 [US1] Call `expand_variables()` in `CommandExecutor::execute()` after parsing, before builtin check in `crates/rush/src/executor/execute.rs`
- [x] T027 [US1] Verify `cargo test` passes and `echo $HOME` works in manual testing

**Checkpoint**: User Stories 1 & 4 complete - `echo $HOME`, `echo $PATH`, `ls $HOME/Documents` all work

---

## Phase 4: User Story 2 - Export Builtin (Priority: P2)

**Goal**: Users can set environment variables with `export VAR=value` that are passed to child processes.

**Independent Test**: Run `export FOO=bar` then `echo $FOO` and verify "bar" is printed. Run `export TEST=hello && sh -c 'echo $TEST'` and verify child sees variable.

### Tests for User Story 2

- [x] T028 [P] [US2] Add unit test for `export VAR=value` parsing in `crates/rush/src/executor/builtins/export.rs`
- [x] T029 [P] [US2] Add unit test for `export` with no args (list variables) in `crates/rush/src/executor/builtins/export.rs`
- [x] T030 [P] [US2] Add unit test for `export` with invalid variable name in `crates/rush/src/executor/builtins/export.rs`
- [x] T031 [P] [US2] Add unit test for `export VAR=$OTHER` (expansion at assignment) in `crates/rush/src/executor/builtins/export.rs`

### Implementation for User Story 2

- [x] T032 [US2] Create `export.rs` builtin file at `crates/rush/src/executor/builtins/export.rs`
- [x] T033 [US2] Add `pub mod export;` to `crates/rush/src/executor/builtins/mod.rs`
- [x] T034 [US2] Implement `export::execute()` function signature matching other builtins in `crates/rush/src/executor/builtins/export.rs`
- [x] T035 [US2] Parse `VAR=value` syntax (split on first `=`) in `crates/rush/src/executor/builtins/export.rs`
- [x] T036 [US2] Expand variables in value using `expand_variables_in_string()` before setting in `crates/rush/src/executor/builtins/export.rs`
- [x] T037 [US2] Call `env_manager_mut().set(name, value)` to store variable in `crates/rush/src/executor/builtins/export.rs`
- [x] T038 [US2] Handle `export` with no args (list all variables like `set`) in `crates/rush/src/executor/builtins/export.rs`
- [x] T039 [US2] Return appropriate error for invalid variable names in `crates/rush/src/executor/builtins/export.rs`
- [x] T040 [US2] Register `"export"` in `execute_builtin()` match statement in `crates/rush/src/executor/builtins/mod.rs`
- [x] T041 [US2] Modify `PipelineExecutor` to accept environment map parameter in `crates/rush/src/executor/pipeline.rs`
- [x] T042 [US2] Use `cmd.env_clear().envs(env_map)` when spawning child processes in `crates/rush/src/executor/pipeline.rs`
- [x] T043 [US2] Pass `env_manager.as_env_map()` from execute.rs to pipeline spawning in `crates/rush/src/executor/execute.rs`
- [x] T044 [US2] Verify `export FOO=bar && echo $FOO` works, and `sh -c 'echo $FOO'` sees the variable

**Checkpoint**: User Story 2 complete - `export` sets variables, child processes inherit them

---

## Phase 5: User Story 3 - Set Builtin (Priority: P3)

**Goal**: Users can list all environment variables with `set` command.

**Independent Test**: Run `set` and verify all variables are listed in `NAME=value` format, sorted alphabetically.

### Tests for User Story 3

- [x] T045 [P] [US3] Add unit test for `set` output format (NAME=value per line) in `crates/rush/src/executor/builtins/set.rs`
- [x] T046 [P] [US3] Add unit test for `set` output sorted alphabetically in `crates/rush/src/executor/builtins/set.rs`

### Implementation for User Story 3

- [x] T047 [US3] Create `set.rs` builtin file at `crates/rush/src/executor/builtins/set.rs`
- [x] T048 [US3] Add `pub mod set;` to `crates/rush/src/executor/builtins/mod.rs`
- [x] T049 [US3] Implement `set::execute()` function signature in `crates/rush/src/executor/builtins/set.rs`
- [x] T050 [US3] Get all variables via `env_manager().iter()` in `crates/rush/src/executor/builtins/set.rs`
- [x] T051 [US3] Sort variables alphabetically by name in `crates/rush/src/executor/builtins/set.rs`
- [x] T052 [US3] Print each variable as `NAME=value` format with `println!()` in `crates/rush/src/executor/builtins/set.rs`
- [x] T053 [US3] Register `"set"` in `execute_builtin()` match statement in `crates/rush/src/executor/builtins/mod.rs`
- [x] T054 [US3] Verify `set` lists all variables and `set | grep PATH` works

**Checkpoint**: User Story 3 complete - `set` displays all environment variables

---

## Phase 6: Polish & Edge Cases

**Purpose**: Handle edge cases, improve error messages, final validation

- [x] T055 [P] Add integration test for `export PATH=$PATH:/custom/bin` in `crates/rush/src/executor/builtins/export.rs`
- [x] T056 [P] Add integration test for empty value `export EMPTY=` in `crates/rush/src/executor/builtins/export.rs`
- [x] T057 [P] Add test for variable expansion in redirection paths (e.g., `echo test > $HOME/output.txt`) in `crates/rush/src/executor/parser.rs`
- [x] T058 Handle edge case: multiple `$` in sequence (e.g., `$$` just produces empty if not special var) in `crates/rush/src/executor/parser.rs`
- [x] T059 Ensure invalid variable names produce clear error message in `crates/rush/src/executor/builtins/export.rs`
- [x] T060 Run `cargo clippy` and fix any warnings
- [x] T061 Run `cargo fmt` to ensure consistent formatting
- [x] T062 Run full test suite `cargo test -p rush` and verify all tests pass
- [x] T063 Manual validation: test all acceptance scenarios from spec.md
- [x] T064 Verify PR size is within limits: `git diff --stat main`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **US1+US4 (Phase 3)**: Depends on Foundational - MVP milestone
- **US2 (Phase 4)**: Depends on Foundational + US1 (needs expansion working)
- **US3 (Phase 5)**: Depends on Foundational only (can start after Phase 2)
- **Polish (Phase 6)**: Depends on all user stories complete

### User Story Dependencies

```
Setup (Phase 1)
    ↓
Foundational (Phase 2) - EnvironmentManager
    ↓
    ├── US1+US4 (Phase 3) - Variable Expansion [P1] 🎯 MVP
    │       ↓
    │       └── US2 (Phase 4) - Export Builtin [P2]
    │
    └── US3 (Phase 5) - Set Builtin [P3] (independent, can parallel with US2)
            ↓
        Polish (Phase 6)
```

### Within Each User Story

- Tests written FIRST, ensure they FAIL
- Core functions before integration
- Integration into execute.rs last
- Verify manually before marking complete

### Parallel Opportunities

**Phase 1 (Setup)**: T001, T002, T003 are sequential (module depends on file existing)

**Phase 2 (Foundational)**:
- T004-T009 sequential (building EnvironmentManager)
- T010-T012 can parallel after T009
- T013 can parallel with T010-T012

**Phase 3 (US1+US4)**:
- T014-T018 all parallel (independent test files)
- T019-T025 sequential (building expand function)
- T026-T027 sequential (integration)

**Phase 4 (US2)**:
- T028-T031 all parallel (independent tests)
- T032-T044 mostly sequential (building export builtin)

**Phase 5 (US3)**:
- T045-T046 parallel (independent tests)
- T047-T054 mostly sequential
- **US3 can run in parallel with US2** (different files, no cross-dependency)

---

## Parallel Example: Phase 2 Tests

```bash
# Launch foundational tests together:
Task: "Add unit tests for EnvironmentManager in crates/rush/src/executor/environment.rs"
# This can run while accessor methods are being added
```

## Parallel Example: User Story 1 Tests

```bash
# Launch all US1 tests together (T014-T018):
Task: "Add unit test for basic $VAR expansion"
Task: "Add unit test for ${VAR} braced expansion"
Task: "Add unit test for undefined variable"
Task: "Add unit test for escaped \$"
Task: "Add unit test for variable in middle of string"
```

## Parallel Example: US2 and US3

```bash
# After Foundational + US1 complete, these can run in parallel:
# Developer A: Phase 4 (US2 - Export)
# Developer B: Phase 5 (US3 - Set)
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 4)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: Foundational (T004-T013) - **CRITICAL**
3. Complete Phase 3: US1+US4 (T014-T027)
4. **STOP and VALIDATE**: Test `echo $HOME`, `echo $PATH`
5. Create PR #1: Foundation + Variable Expansion (~800 lines)

### Incremental Delivery

1. PR #1: Setup + Foundational + US1+US4 → **MVP: variable expansion works**
2. PR #2: US2 (Export) → **Can set and pass variables to children**
3. PR #3: US3 (Set) → **Can list all variables**
4. PR #4: Polish → **Edge cases and final validation**

### PR Size Validation

Before each PR:
```bash
git diff --stat main
```

Expected sizes:
- PR #1: ~800 lines (US1+US4 + Foundation)
- PR #2: ~400 lines (US2 Export)
- PR #3: ~300 lines (US3 Set)
- PR #4: ~200 lines (Polish)

---

## Summary

| Phase | Tasks | Parallel | Story |
|-------|-------|----------|-------|
| Setup | T001-T003 | 1 | - |
| Foundational | T004-T013 | 3 | - |
| US1+US4 | T014-T027 | 5 | MVP 🎯 |
| US2 | T028-T044 | 4 | Export |
| US3 | T045-T054 | 2 | Set |
| Polish | T055-T064 | 3 | - |
| **Total** | **64 tasks** | **18 parallel** | |

**MVP Scope**: Phases 1-3 (T001-T027) = 27 tasks
**Full Feature**: All 64 tasks
