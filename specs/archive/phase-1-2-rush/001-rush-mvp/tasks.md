# Tasks: rush Shell MVP

**Input**: Design documents from `specs/001-rush-mvp/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL for this feature - only included if explicitly needed for validation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `crates/rush/src/`, `crates/rush/tests/` at repository root
- Paths shown below are for rush shell project in workspace

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Cargo.toml for rush crate in crates/rush/Cargo.toml with dependencies: reedline, crossterm, tokio, toml, nix
- [x] T002 Create main.rs entry point in crates/rush/src/main.rs
- [x] T003 Create lib.rs library interface in crates/rush/src/lib.rs
- [x] T004 [P] Create module structure: crates/rush/src/repl/mod.rs
- [x] T005 [P] Create module structure: crates/rush/src/history/mod.rs
- [x] T006 [P] Create module structure: crates/rush/src/completion/mod.rs
- [x] T007 [P] Create module structure: crates/rush/src/executor/mod.rs
- [x] T008 [P] Create module structure: crates/rush/src/config/mod.rs
- [x] T009 [P] Configure clippy and rustfmt in crates/rush/.rustfmt.toml and crates/rush/.clippy.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T010 [P] Create Config struct and defaults in crates/rush/src/config/defaults.rs (history_size, prompt, theme, timeouts)
- [ ] T011 [P] Implement TOML config loader in crates/rush/src/config/mod.rs with optional file loading from ~/.config/rush/rush.toml
- [ ] T012 [P] Create Command data structure in crates/rush/src/executor/mod.rs (program, args, background, operators, redirects)
- [ ] T013 [P] Create HistoryEntry data structure in crates/rush/src/history/mod.rs (command, timestamp, exit_code, working_dir)
- [ ] T014 [P] Create Job data structure and JobState enum in crates/rush/src/executor/job.rs
- [ ] T015 [P] Create CompletionResult data structure in crates/rush/src/completion/mod.rs (text, type, description, score)
- [ ] T016 [P] Create error types in crates/rush/src/lib.rs using thiserror crate

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Interactive Command Execution (Priority: P1) 🎯 MVP

**Goal**: User can launch rush, type commands, execute them, and see output with real-time visual feedback

**Independent Test**: Launch rush, type `ls -la`, press Enter, see directory listing. Type `echo hello`, press Enter, see "hello". Use arrow keys to edit input. Verify syntax highlighting shows commands in green, flags in blue.

### Implementation for User Story 1

- [ ] T017 [US1] Implement basic REPL loop skeleton in crates/rush/src/repl/mod.rs using reedline's Reedline::create()
- [ ] T018 [US1] Implement input handling in crates/rush/src/repl/input.rs for keystroke capture and line editing (left, right, backspace, delete)
- [ ] T019 [US1] Create RushHighlighter implementing reedline::Highlighter trait in crates/rush/src/repl/highlight.rs
- [ ] T020 [US1] Implement custom shell lexer in crates/rush/src/repl/highlight.rs to tokenize commands (Command, Flag, Path, String, Operator tokens)
- [ ] T021 [US1] Implement syntax coloring logic in RushHighlighter using crossterm colors (command=green, flag=blue, path=cyan, string=yellow, error=red)
- [ ] T022 [US1] Implement command validator in crates/rush/src/repl/input.rs to check if command exists in PATH (for error color)
- [ ] T023 [US1] Implement basic command executor in crates/rush/src/executor/mod.rs using std::process::Command to spawn and wait for processes
- [ ] T024 [US1] Connect REPL to executor in crates/rush/src/repl/mod.rs - parse input, execute, display output
- [ ] T025 [US1] Implement signal handler for Ctrl+C in crates/rush/src/repl/mod.rs to cancel input (not exit shell)
- [ ] T026 [US1] Implement signal handler for Ctrl+D in crates/rush/src/repl/mod.rs to exit shell only if input is empty
- [ ] T027 [US1] Add error display in crates/rush/src/repl/mod.rs for command not found and execution failures

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. User can use rush as a basic shell with syntax highlighting.

---

## Phase 4: User Story 2 - Command History Navigation (Priority: P1)

**Goal**: User can navigate previous commands with up/down arrows, and history persists across sessions

**Independent Test**: Run several commands (`ls`, `pwd`, `echo test`), press up arrow to see `echo test`, up again to see `pwd`, down to see `echo test` again. Exit rush and relaunch, press up arrow, verify commands from previous session appear.

### Implementation for User Story 2

- [ ] T028 [P] [US2] Create HistoryManager struct in crates/rush/src/history/mod.rs with VecDeque for in-memory storage
- [ ] T029 [P] [US2] Implement history file operations in crates/rush/src/history/storage.rs for atomic writes (temp file + rename pattern)
- [ ] T030 [US2] Implement HistoryManager::load() in crates/rush/src/history/mod.rs to read from ~/.config/rush/history on startup
- [ ] T031 [US2] Implement HistoryManager::append() in crates/rush/src/history/mod.rs to add command and persist to disk synchronously
- [ ] T032 [US2] Implement history search methods in crates/rush/src/history/mod.rs (get, recent, search_prefix) for navigation
- [ ] T033 [US2] Integrate HistoryManager with reedline in crates/rush/src/repl/mod.rs using FileBackedHistory or custom implementation
- [ ] T034 [US2] Implement max_entries enforcement in crates/rush/src/history/mod.rs (default 10,000, remove oldest when exceeded)
- [ ] T035 [US2] Handle corrupted history file in crates/rush/src/history/storage.rs (skip invalid lines, log warning, continue)
- [ ] T036 [US2] Create ~/.config/rush directory on first run in crates/rush/src/history/storage.rs if it doesn't exist

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. User can execute commands with syntax highlighting AND navigate history across sessions.

---

## Phase 5: User Story 3 - Syntax Highlighting (Priority: P1)

**Goal**: Real-time syntax highlighting as user types, with distinct colors for commands, flags, paths, strings, and errors

**Independent Test**: Type `ls -la /etc` and observe: `ls` in green, `-la` in blue, `/etc` in cyan. Type `echo "hello"` and observe quotes in yellow. Type `invalidcmd` and observe in red.

**Note**: Core highlighting implemented in User Story 1 (T019-T022). This phase adds refinements and edge cases.

### Implementation for User Story 3

- [ ] T037 [US3] Enhance lexer in crates/rush/src/repl/highlight.rs to detect quoted strings (both " and ')
- [ ] T038 [US3] Enhance lexer in crates/rush/src/repl/highlight.rs to detect file paths (containing / or matching filesystem)
- [ ] T039 [US3] Implement path validation in crates/rush/src/repl/highlight.rs to check if paths exist (valid=cyan, invalid=default)
- [ ] T040 [US3] Optimize highlighting performance in crates/rush/src/repl/highlight.rs to complete in <16ms (60 FPS requirement)
- [ ] T041 [US3] Handle Unicode characters in lexer in crates/rush/src/repl/highlight.rs (emojis, CJK characters)
- [ ] T042 [US3] Add operator highlighting in crates/rush/src/repl/highlight.rs (|, &&, ||, ;, >, >>)

**Checkpoint**: All P1 stories (US1, US2, US3) are complete. rush provides a fully functional basic shell with execution, history, and real-time highlighting.

---

## Phase 6: User Story 4 - Autosuggestions (Priority: P2)

**Goal**: Display ghost text suggesting commands based on history as user types

**Independent Test**: Execute `ls -la` multiple times. Type just `ls` and observe ghost text ` -la` appear. Press right arrow to accept suggestion. Type `git` and observe most recent git command suggested.

### Implementation for User Story 4

- [ ] T043 [US4] Create RushHinter implementing reedline::Hinter trait in crates/rush/src/repl/suggest.rs
- [ ] T044 [US4] Implement suggestion matching logic in crates/rush/src/repl/suggest.rs using HistoryManager::search_prefix()
- [ ] T045 [US4] Configure RushHinter to show most recent match in crates/rush/src/repl/suggest.rs (reverse chronological order)
- [ ] T046 [US4] Add suggestion debouncing in crates/rush/src/repl/suggest.rs to wait 50ms after last keystroke (per spec SC-006)
- [ ] T047 [US4] Style ghost text in crates/rush/src/repl/suggest.rs using dimmed/gray color
- [ ] T048 [US4] Integrate RushHinter with reedline in crates/rush/src/repl/mod.rs using with_hinter()
- [ ] T049 [US4] Handle suggestion acceptance on right arrow in crates/rush/src/repl/mod.rs (reedline handles this automatically)

**Checkpoint**: User Stories 1-4 complete. rush now provides execution, history, highlighting, AND autosuggestions.

---

## Phase 7: User Story 5 - Tab Completions (Priority: P2)

**Goal**: Tab key provides smart completions for commands from PATH, filesystem paths, and common flags

**Independent Test**: Type `ec` and press Tab, see `echo` completed. Type `ls /et` and Tab, see `/etc` completed. Type `ls -` and Tab, see list of flags (`-l`, `-a`, `-h`, etc.).

### Implementation for User Story 5

- [ ] T050 [P] [US5] Create CommandCompleter in crates/rush/src/completion/command.rs with PATH scanning and caching
- [ ] T051 [P] [US5] Create PathCompleter in crates/rush/src/completion/path.rs with filesystem traversal
- [ ] T052 [P] [US5] Create FlagCompleter in crates/rush/src/completion/flag.rs with common flag database
- [ ] T053 [US5] Implement PATH caching in crates/rush/src/completion/command.rs to scan all PATH directories on startup
- [ ] T054 [US5] Implement command completion matching in crates/rush/src/completion/command.rs (prefix match, score by relevance)
- [ ] T055 [US5] Implement path completion in crates/rush/src/completion/path.rs handling absolute paths, relative paths, and ~/ expansion
- [ ] T056 [US5] Implement flag completion database in crates/rush/src/completion/flag.rs for common commands (ls, git, cargo, etc.)
- [ ] T057 [US5] Create CompletionEngine orchestrator in crates/rush/src/completion/mod.rs to route completions to correct completer
- [ ] T058 [US5] Implement completion type detection logic in crates/rush/src/completion/mod.rs (first word = command, contains / = path, starts with - = flag)
- [ ] T059 [US5] Create RushCompleter implementing reedline::Completer trait in crates/rush/src/completion/mod.rs
- [ ] T060 [US5] Integrate RushCompleter with reedline in crates/rush/src/repl/mod.rs using with_completer()
- [ ] T061 [US5] Optimize completion performance in crates/rush/src/completion/mod.rs to complete in <100ms (spec SC-007)
- [ ] T062 [US5] Handle large directories efficiently in crates/rush/src/completion/path.rs (limit results to first 100 matches, or use streaming)

**Checkpoint**: All P1 and P2 stories complete (US1-US5). rush provides full interactive experience with execution, history, highlighting, suggestions, AND completions.

---

## Phase 8: User Story 6 - Job Control (Priority: P3)

**Goal**: User can suspend jobs with Ctrl+Z, resume with fg/bg, and run background jobs with &

**Independent Test**: Run `sleep 30`, press Ctrl+Z to suspend. Type `jobs` to see job listed. Type `fg` to resume. Press Ctrl+C to kill. Run `sleep 30 &` and observe job runs in background while prompt returns.

### Implementation for User Story 6

- [ ] T063 [US6] Create JobManager struct in crates/rush/src/executor/job.rs with jobs HashMap and next_job_id counter
- [ ] T064 [US6] Implement process group creation in crates/rush/src/executor/job.rs using nix::unistd::setpgid()
- [ ] T065 [US6] Implement terminal foreground control in crates/rush/src/executor/job.rs using nix::unistd::tcsetpgrp()
- [ ] T066 [US6] Implement foreground job spawning in crates/rush/src/executor/job.rs (spawn, setpgid, tcsetpgrp, wait)
- [ ] T067 [US6] Implement background job spawning in crates/rush/src/executor/job.rs (spawn with &, don't give terminal control)
- [ ] T068 [US6] Implement SIGTSTP handler in crates/rush/src/executor/job.rs for Ctrl+Z (suspend foreground job)
- [ ] T069 [US6] Implement SIGCONT handling in crates/rush/src/executor/job.rs for fg and bg commands
- [ ] T070 [US6] Implement SIGCHLD handler in crates/rush/src/executor/job.rs to detect job completion
- [ ] T071 [US6] Implement `jobs` builtin command in crates/rush/src/executor/job.rs to list active jobs
- [ ] T072 [US6] Implement `fg` builtin command in crates/rush/src/executor/job.rs to resume job in foreground
- [ ] T073 [US6] Implement `bg` builtin command in crates/rush/src/executor/job.rs to resume job in background
- [ ] T074 [US6] Integrate JobManager with REPL in crates/rush/src/repl/mod.rs for Ctrl+Z signal handling
- [ ] T075 [US6] Ensure terminal state restoration in crates/rush/src/executor/job.rs after job suspend/complete (tcsetpgrp back to shell)

**Checkpoint**: User Stories 1-6 complete. rush now supports full job control for professional development workflows.

---

## Phase 9: User Story 7 - Script Execution (Priority: P3)

**Goal**: Execute shell scripts from files with command chaining (&&, ||, ;) and output redirection (>, >>)

**Independent Test**: Create file `test.sh` with `echo "line1"\necho "line2"`, make executable with `chmod +x test.sh`, run `./test.sh`, verify both lines print. Test `echo "test" > file.txt && cat file.txt` and verify output.

### Implementation for User Story 7

- [ ] T076 [US7] Implement command parser in crates/rush/src/executor/script.rs to parse chaining operators (&&, ||, ;, |)
- [ ] T077 [US7] Implement command parser in crates/rush/src/executor/script.rs to parse redirection operators (>, >>)
- [ ] T078 [US7] Implement script file reader in crates/rush/src/executor/script.rs to load and parse multi-line scripts
- [ ] T079 [US7] Implement && operator logic in crates/rush/src/executor/script.rs (run next command only if previous succeeded)
- [ ] T080 [US7] Implement || operator logic in crates/rush/src/executor/script.rs (run next command only if previous failed)
- [ ] T081 [US7] Implement ; operator logic in crates/rush/src/executor/script.rs (run next command regardless of previous exit code)
- [ ] T082 [US7] Implement > redirection in crates/rush/src/executor/script.rs (redirect stdout to file, overwrite mode)
- [ ] T083 [US7] Implement >> redirection in crates/rush/src/executor/script.rs (redirect stdout to file, append mode)
- [ ] T084 [US7] Implement pipe operator | in crates/rush/src/executor/script.rs (pipe stdout of one command to stdin of next)
- [ ] T085 [US7] Add script execution support to main executor in crates/rush/src/executor/mod.rs (detect .sh files, invoke script runner)
- [ ] T086 [US7] Handle shebang lines in crates/rush/src/executor/script.rs (#!/usr/bin/env rush or #!/bin/sh)

**Checkpoint**: All user stories (US1-US7) complete. rush is a fully-functional shell supporting all MVP features.

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories, final validation, and performance verification

- [ ] T087 [P] Add benchmark for startup time in crates/rush/benches/startup.rs using criterion (verify <100ms requirement)
- [ ] T088 [P] Add benchmark for input responsiveness in crates/rush/benches/input.rs (verify <16ms keystroke latency)
- [ ] T089 [P] Add benchmark for syntax highlighting in crates/rush/benches/highlight.rs (verify <16ms per keystroke)
- [ ] T090 [P] Add integration test for full REPL session in crates/rush/tests/integration/repl_test.rs using rexpect
- [ ] T091 [P] Add integration test for history persistence in crates/rush/tests/integration/history_test.rs
- [ ] T092 [P] Add integration test for job control workflow in crates/rush/tests/integration/job_control_test.rs
- [ ] T093 [P] Add memory usage verification script in crates/rush/benches/memory.rs (verify <10MB baseline)
- [ ] T094 Code cleanup and refactoring: Remove dead code, add documentation comments to public APIs
- [ ] T095 Run clippy and fix all warnings in crates/rush/
- [ ] T096 Run cargo fmt on entire codebase
- [ ] T097 [P] Create example scripts in crates/rush/examples/ directory for testing script execution
- [ ] T098 Update workspace Cargo.toml to include rush as workspace member if not already included
- [ ] T099 Verify all success criteria from spec.md are met (SC-001 through SC-010)
- [ ] T100 Run quickstart.md manual test checklist to validate all features

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-9)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 10)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P1)**: Builds on User Story 1 (enhances existing highlighting) - Should follow US1
- **User Story 4 (P2)**: Depends on User Story 2 (needs history for suggestions) - Should follow US2
- **User Story 5 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 6 (P3)**: Depends on User Story 1 (needs basic executor) - Should follow US1
- **User Story 7 (P3)**: Depends on User Story 1 (needs basic executor) - Should follow US1

### Within Each User Story

- Tasks within a story marked [P] can run in parallel
- Tasks without [P] depend on previous tasks in the same story
- Story complete when all tasks checked off

### Parallel Opportunities

- All Setup tasks (T004-T009) can run in parallel
- All Foundational tasks (T010-T016) can run in parallel
- Within User Story 2: T028 and T029 can run in parallel
- Within User Story 5: T050, T051, T052 can run in parallel
- Polish phase: T087-T093, T097 can run in parallel

---

## Parallel Example: User Story 2 (History)

```bash
# Launch these model tasks together:
Task T028: "Create HistoryManager struct in crates/rush/src/history/mod.rs"
Task T029: "Implement history file operations in crates/rush/src/history/storage.rs"

# After both complete, proceed sequentially with T030-T036
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Interactive Command Execution)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready - this is a usable shell!

**This gives you a working shell with syntax highlighting in ~20-30 tasks.**

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (MVP + History!)
4. Add User Story 3 → Test independently → Deploy/Demo (Enhanced highlighting!)
5. Add User Story 4 → Test independently → Deploy/Demo (+ Autosuggestions!)
6. Add User Story 5 → Test independently → Deploy/Demo (+ Tab completions!)
7. Add User Story 6 → Test independently → Deploy/Demo (+ Job control!)
8. Add User Story 7 → Test independently → Deploy/Demo (Full MVP!)
9. Polish phase → Performance validation → Release v0.1

Each story adds value without breaking previous stories.

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Interactive Execution)
   - Developer B: User Story 2 (History Navigation)
   - Developer C: User Story 5 (Tab Completions)
3. After US1 completes:
   - Developer A moves to User Story 3 (Syntax Highlighting - enhances US1)
   - Developer D joins: User Story 6 (Job Control - depends on US1)
4. After US2 completes:
   - Developer B moves to User Story 4 (Autosuggestions - depends on US2)
   - Developer E joins: User Story 7 (Script Execution - depends on US1)
5. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies within story
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

---

## Task Count Summary

- **Total Tasks**: 100
- **Setup Phase**: 9 tasks
- **Foundational Phase**: 7 tasks
- **User Story 1 (P1)**: 11 tasks
- **User Story 2 (P1)**: 9 tasks
- **User Story 3 (P1)**: 6 tasks
- **User Story 4 (P2)**: 7 tasks
- **User Story 5 (P2)**: 13 tasks
- **User Story 6 (P3)**: 13 tasks
- **User Story 7 (P3)**: 11 tasks
- **Polish Phase**: 14 tasks

**MVP Scope** (just User Story 1): Setup (9) + Foundational (7) + US1 (11) = **27 tasks** for a working shell with syntax highlighting!

**Parallel Opportunities**: 15 tasks can run in parallel (marked with [P])

**Independent Test Criteria**: Each user story has clear test scenario for validation
