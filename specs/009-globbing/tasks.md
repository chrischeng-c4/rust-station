# Tasks: Glob Pattern Expansion

**Input**: Design documents from `/specs/009-globbing/`
**Prerequisites**: plan.md, spec.md

**Tests**: Unit tests included inline with implementation (Rust convention)

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to

---

## Phase 1: Setup

**Purpose**: Add dependency and create module structure

- [ ] T001 Add `glob = "0.3"` dependency to `crates/rush/Cargo.toml`
- [ ] T002 Create `crates/rush/src/executor/glob.rs` module file
- [ ] T003 Add `pub mod glob;` to `crates/rush/src/executor/mod.rs`

---

## Phase 2: Core Functions (US1, US2)

**Purpose**: Implement basic glob detection and expansion

- [ ] T004 [US1] Implement `contains_glob_chars(s: &str) -> bool` in `glob.rs`
- [ ] T005 [P] [US1] Add unit tests for `contains_glob_chars()`
- [ ] T006 [US1] Implement `expand_single_glob(arg: &str) -> Vec<String>` for `*` pattern
- [ ] T007 [US2] Extend `expand_single_glob()` to handle `?` pattern
- [ ] T008 [US1] Implement `expand_globs(args: &[String]) -> Vec<String>` public function
- [ ] T009 [P] [US1] Add unit tests for `*` pattern expansion
- [ ] T010 [P] [US2] Add unit tests for `?` pattern expansion
- [ ] T011 [US1] Handle no-match case (return literal pattern per POSIX)
- [ ] T012 [P] [US1] Add unit test for no-match behavior

**Checkpoint**: Basic wildcards `*` and `?` work in isolation

---

## Phase 3: Character Classes (US3)

**Purpose**: Implement `[...]` pattern matching

- [ ] T013 [US3] Extend pattern expansion to handle `[abc]` character sets
- [ ] T014 [US3] Implement `[a-z]` range syntax support
- [ ] T015 [US3] Implement `[!...]` and `[^...]` negation
- [ ] T016 [P] [US3] Add unit tests for character class patterns
- [ ] T017 [P] [US3] Add unit tests for negated character classes

**Checkpoint**: Character classes `[...]` work in isolation

---

## Phase 4: Special Cases (US4, US5)

**Purpose**: Handle hidden files and directory patterns

- [ ] T018 [US4] Ensure `*` excludes hidden files (starting with `.`)
- [ ] T019 [US4] Implement `.*` pattern to match hidden files explicitly
- [ ] T020 [P] [US4] Add unit tests for hidden file handling
- [ ] T021 [US5] Implement directory glob patterns (`src/*.rs`)
- [ ] T022 [P] [US5] Add unit tests for directory patterns

**Checkpoint**: Hidden files and directory patterns work

---

## Phase 5: Parser Integration

**Purpose**: Integrate glob expansion into command parsing

- [ ] T023 Modify tokenizer to track quoted state for each token
- [ ] T024 Implement quote detection in `expand_single_glob()` (skip expansion)
- [ ] T025 [P] Add unit tests for quoted pattern preservation
- [ ] T026 Integrate `expand_globs()` call in `parse_command_line()` or `execute()`
- [ ] T027 Add integration test: `ls *.rs` in test directory
- [ ] T028 Add integration test: `echo *` with various file patterns

**Checkpoint**: Glob expansion works end-to-end in shell commands

---

## Phase 6: Polish

**Purpose**: Edge cases, documentation, final validation

- [ ] T029 Handle escaped glob characters (`\*` → literal `*`)
- [ ] T030 [P] Add unit tests for escape handling
- [ ] T031 Sort glob results alphabetically
- [ ] T032 [P] Add unit test for result ordering
- [ ] T033 Run `cargo clippy` and fix warnings
- [ ] T034 Run `cargo fmt`
- [ ] T035 Run full test suite `cargo test -p rush`
- [ ] T036 Manual validation: test all acceptance scenarios from spec.md

---

## Dependencies & Execution Order

```
Phase 1: Setup (T001-T003)
    ↓
Phase 2: Core Functions (T004-T012) [MVP]
    ↓
Phase 3: Character Classes (T013-T017)
    ↓
Phase 4: Special Cases (T018-T022)
    ↓
Phase 5: Parser Integration (T023-T028)
    ↓
Phase 6: Polish (T029-T036)
```

### Parallel Opportunities

- T005 and T009, T010, T012 (independent unit tests)
- T016 and T017 (independent character class tests)
- T020 and T022 (independent special case tests)
- T025, T030, T032 (independent polish tests)

---

## PR Strategy

**Single PR** (feature is cohesive, ~400 lines estimated):
- Phase 1-6 as one PR
- All user stories implemented together
- Feature is atomic (partial glob is confusing)

---

## Summary

| Phase | Tasks | Parallel | Story |
|-------|-------|----------|-------|
| Setup | T001-T003 | 0 | - |
| Core | T004-T012 | 4 | US1, US2 |
| Classes | T013-T017 | 2 | US3 |
| Special | T018-T022 | 2 | US4, US5 |
| Integration | T023-T028 | 1 | - |
| Polish | T029-T036 | 3 | - |
| **Total** | **36 tasks** | **12 parallel** | |
