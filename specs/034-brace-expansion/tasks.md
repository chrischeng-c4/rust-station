# Tasks: Brace Expansion

**Input**: Design documents from `/specs/034-brace-expansion/`
**Prerequisites**: plan.md, spec.md, research.md

**Tests**: Tests included - comprehensive test coverage is standard for rush shell features.

**Organization**: Tasks grouped by functional capability to enable incremental implementation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which capability group this task belongs to (US1=Lists, US2=Sequences, US3=Nesting, US4=Edge Cases)
- Include exact file paths in descriptions

## Path Conventions

Per plan.md, this is a single crate project:
- Source: `crates/rush/src/executor/brace/`
- Tests: `crates/rush/src/executor/brace/` (inline mod tests)
- Integration point: `crates/rush/src/executor/execute.rs`

---

## Phase 1: Setup (Module Structure)

**Purpose**: Create brace expansion module skeleton following arithmetic expansion pattern

- [x] T001 Create brace module directory at crates/rush/src/executor/brace/
- [x] T002 Create module coordination file crates/rush/src/executor/brace/mod.rs with submodule declarations
- [x] T003 [P] Create expander stub crates/rush/src/executor/brace/expander.rs with expand_brace() signature
- [x] T004 [P] Create lexer stub crates/rush/src/executor/brace/lexer.rs with BraceToken enum
- [x] T005 [P] Create parser stub crates/rush/src/executor/brace/parser.rs with BraceExpr enum
- [x] T006 Register brace module in crates/rush/src/executor/mod.rs

**Checkpoint**: Module compiles with stub implementations

---

## Phase 2: Foundational (Core Infrastructure)

**Purpose**: Quote/escape handling and pattern detection - MUST complete before expansion logic

**⚠️ CRITICAL**: All expansion capabilities depend on correct quote/escape detection

- [ ] T007 Implement quote state tracking (single/double/escape) in crates/rush/src/executor/brace/lexer.rs
- [ ] T008 Implement find_matching_brace() to locate valid brace pairs in crates/rush/src/executor/brace/lexer.rs
- [ ] T009 Implement is_valid_brace_pattern() to check if content is expandable in crates/rush/src/executor/brace/parser.rs
- [ ] T010 Add BraceExpr enum variants (List, NumericSeq, CharSeq, Literal) in crates/rush/src/executor/brace/parser.rs
- [ ] T011 Implement basic expand_brace() loop that finds and processes brace patterns in crates/rush/src/executor/brace/expander.rs

**Checkpoint**: Foundation ready - can detect brace patterns while respecting quotes/escapes

---

## Phase 3: User Story 1 - Comma-Separated Lists (Priority: P1) 🎯 MVP

**Goal**: Expand `{a,b,c}` to separate words, with preamble/postscript support

**Independent Test**: `echo {cat,dog}` outputs `cat dog`

### Tests for User Story 1

- [ ] T012 [P] [US1] Add tests for basic comma expansion `{a,b,c}` in crates/rush/src/executor/brace/expander.rs
- [ ] T013 [P] [US1] Add tests for preamble/postscript `pre{a,b}post` in crates/rush/src/executor/brace/expander.rs
- [ ] T014 [P] [US1] Add tests for empty alternatives `{a,,b}` in crates/rush/src/executor/brace/expander.rs

### Implementation for User Story 1

- [ ] T015 [US1] Implement split_on_commas() respecting nested braces in crates/rush/src/executor/brace/lexer.rs
- [ ] T016 [US1] Implement parse_list() returning BraceExpr::List in crates/rush/src/executor/brace/parser.rs
- [ ] T017 [US1] Implement expand_list() generating word alternatives in crates/rush/src/executor/brace/expander.rs
- [ ] T018 [US1] Implement preamble/postscript combination logic in crates/rush/src/executor/brace/expander.rs
- [ ] T019 [US1] Handle empty alternatives producing empty strings in crates/rush/src/executor/brace/expander.rs

**Checkpoint**: Comma-separated lists work - MVP functional

---

## Phase 4: User Story 2 - Numeric & Character Sequences (Priority: P2)

**Goal**: Expand `{1..10}` and `{a..z}` with step and padding support

**Independent Test**: `echo {1..5}` outputs `1 2 3 4 5`

### Tests for User Story 2

- [ ] T020 [P] [US2] Add tests for numeric sequences `{1..5}` in crates/rush/src/executor/brace/expander.rs
- [ ] T021 [P] [US2] Add tests for reverse sequences `{5..1}` in crates/rush/src/executor/brace/expander.rs
- [ ] T022 [P] [US2] Add tests for step increment `{1..10..2}` in crates/rush/src/executor/brace/expander.rs
- [ ] T023 [P] [US2] Add tests for zero-padding `{01..05}` in crates/rush/src/executor/brace/expander.rs
- [ ] T024 [P] [US2] Add tests for character sequences `{a..e}` in crates/rush/src/executor/brace/expander.rs
- [ ] T025 [P] [US2] Add tests for negative numbers `{-5..5}` in crates/rush/src/executor/brace/expander.rs

### Implementation for User Story 2

- [ ] T026 [US2] Implement parse_sequence() detecting `..` pattern in crates/rush/src/executor/brace/parser.rs
- [ ] T027 [US2] Implement parse_numeric_sequence() returning BraceExpr::NumericSeq in crates/rush/src/executor/brace/parser.rs
- [ ] T028 [US2] Implement parse_char_sequence() returning BraceExpr::CharSeq in crates/rush/src/executor/brace/parser.rs
- [ ] T029 [US2] Implement expand_numeric_seq() with forward/reverse/step logic in crates/rush/src/executor/brace/expander.rs
- [ ] T030 [US2] Implement zero-padding calculation (width from widest number) in crates/rush/src/executor/brace/expander.rs
- [ ] T031 [US2] Implement expand_char_seq() with ASCII iteration in crates/rush/src/executor/brace/expander.rs
- [ ] T032 [US2] Handle step increment for both numeric and character sequences in crates/rush/src/executor/brace/expander.rs

**Checkpoint**: All sequence types work including reverse, step, and padding

---

## Phase 5: User Story 3 - Nested Braces & Cartesian Product (Priority: P3)

**Goal**: Expand nested `{a,b{1,2}}` and adjacent `{a,b}{1,2}` patterns

**Independent Test**: `echo {a,b{1,2}}` outputs `a b1 b2`

### Tests for User Story 3

- [ ] T033 [P] [US3] Add tests for nested braces `{a,{b,c}}` in crates/rush/src/executor/brace/expander.rs
- [ ] T034 [P] [US3] Add tests for nested with postscript `{a,b{1,2},c}` in crates/rush/src/executor/brace/expander.rs
- [ ] T035 [P] [US3] Add tests for adjacent braces `{a,b}{1,2}` (Cartesian product) in crates/rush/src/executor/brace/expander.rs
- [ ] T036 [P] [US3] Add tests for deeply nested `{a,{b,{c,d}}}` in crates/rush/src/executor/brace/expander.rs

### Implementation for User Story 3

- [ ] T037 [US3] Implement recursive expansion for nested braces in crates/rush/src/executor/brace/expander.rs
- [ ] T038 [US3] Implement expand_word() to find and process leftmost brace first in crates/rush/src/executor/brace/expander.rs
- [ ] T039 [US3] Implement Cartesian product for adjacent brace patterns in crates/rush/src/executor/brace/expander.rs
- [ ] T040 [US3] Ensure innermost braces expand before outer in crates/rush/src/executor/brace/expander.rs

**Checkpoint**: Complex nested patterns and Cartesian products work correctly

---

## Phase 6: User Story 4 - Edge Cases & Error Handling (Priority: P4)

**Goal**: Handle invalid patterns gracefully, return literals for non-expanding cases

**Independent Test**: `echo {a}` outputs literal `{a}` (no expansion)

### Tests for User Story 4

- [ ] T041 [P] [US4] Add tests for single element `{a}` (no expansion) in crates/rush/src/executor/brace/expander.rs
- [ ] T042 [P] [US4] Add tests for empty braces `{}` (no expansion) in crates/rush/src/executor/brace/expander.rs
- [ ] T043 [P] [US4] Add tests for unmatched braces `{a,b` (literal) in crates/rush/src/executor/brace/expander.rs
- [ ] T044 [P] [US4] Add tests for escaped braces `\{a,b\}` (literal) in crates/rush/src/executor/brace/expander.rs
- [ ] T045 [P] [US4] Add tests for quoted braces `'{a,b}'` and `"{a,b}"` (literal) in crates/rush/src/executor/brace/expander.rs
- [ ] T046 [P] [US4] Add tests for mixed type sequences `{a..5}` (literal) in crates/rush/src/executor/brace/expander.rs
- [ ] T047 [P] [US4] Add tests for invalid sequences `{a..z` (literal) in crates/rush/src/executor/brace/expander.rs

### Implementation for User Story 4

- [ ] T048 [US4] Implement single-element detection returning BraceExpr::Literal in crates/rush/src/executor/brace/parser.rs
- [ ] T049 [US4] Implement empty braces detection returning BraceExpr::Literal in crates/rush/src/executor/brace/parser.rs
- [ ] T050 [US4] Ensure escaped braces bypass expansion in crates/rush/src/executor/brace/lexer.rs
- [ ] T051 [US4] Ensure quoted braces bypass expansion in crates/rush/src/executor/brace/lexer.rs
- [ ] T052 [US4] Implement mixed-type sequence detection returning BraceExpr::Literal in crates/rush/src/executor/brace/parser.rs
- [ ] T053 [US4] Implement graceful handling of malformed patterns in crates/rush/src/executor/brace/expander.rs

**Checkpoint**: All edge cases handled gracefully without errors

---

## Phase 7: Pipeline Integration

**Purpose**: Integrate brace expansion into the main execution pipeline

- [ ] T054 Import brace module in crates/rush/src/executor/execute.rs
- [ ] T055 Add expand_brace() call after alias expansion, before variable expansion in crates/rush/src/executor/execute.rs
- [ ] T056 Add integration tests for pipeline order (brace before glob) in crates/rush/src/executor/brace/expander.rs
- [ ] T057 Add integration test for combined expansions `{a,b}$VAR` in crates/rush/src/executor/brace/expander.rs
- [ ] T058 Add integration test for brace+glob `*.{js,ts}` in crates/rush/src/executor/brace/expander.rs

**Checkpoint**: Brace expansion integrated and working in full pipeline

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup and validation

- [ ] T059 [P] Run cargo clippy and fix all warnings in crates/rush/src/executor/brace/
- [ ] T060 [P] Run cargo fmt on all brace module files
- [ ] T061 [P] Add doc comments to public functions in crates/rush/src/executor/brace/
- [ ] T062 Run full test suite `cargo test` to verify no regressions
- [ ] T063 Verify all success criteria from spec.md are met
- [ ] T064 Update features.json status to "complete" for feature 034

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **US1 Lists (Phase 3)**: Depends on Foundational - MVP milestone
- **US2 Sequences (Phase 4)**: Depends on Foundational - can parallel with US1 if needed
- **US3 Nesting (Phase 5)**: Depends on US1 (uses list expansion logic)
- **US4 Edge Cases (Phase 6)**: Depends on Foundational - can parallel with US1-US3
- **Integration (Phase 7)**: Depends on US1-US4 completion
- **Polish (Phase 8)**: Depends on Integration

### User Story Dependencies

- **US1 (Lists)**: Foundation only - core MVP, must complete first
- **US2 (Sequences)**: Foundation only - independent of US1
- **US3 (Nesting)**: Requires US1 (recursive list expansion)
- **US4 (Edge Cases)**: Foundation only - independent, can be done early

### Within Each User Story

- Tests written first (TDD approach)
- Lexer/parser work before expander
- Unit tests before integration

### Parallel Opportunities

**Phase 1 (Setup)**:
```
T003, T004, T005 can run in parallel (different files)
```

**Phase 3-6 Tests**:
```
All test tasks marked [P] within each phase can run in parallel
```

**Cross-Phase**:
```
US2 and US4 can run in parallel with US1 (no dependencies)
```

---

## Parallel Example: User Story 2 Tests

```bash
# Launch all US2 tests in parallel:
T020: tests for numeric sequences {1..5}
T021: tests for reverse sequences {5..1}
T022: tests for step increment {1..10..2}
T023: tests for zero-padding {01..05}
T024: tests for character sequences {a..e}
T025: tests for negative numbers {-5..5}
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T011)
3. Complete Phase 3: User Story 1 - Lists (T012-T019)
4. **STOP and VALIDATE**: Test `echo {a,b,c}` works
5. Can integrate early with T054-T055 for demo

### Incremental Delivery

1. Setup + Foundational → Module structure ready
2. Add US1 (Lists) → Basic brace expansion works
3. Add US2 (Sequences) → Numeric/char ranges work
4. Add US3 (Nesting) → Complex patterns work
5. Add US4 (Edge Cases) → Robust error handling
6. Integration + Polish → Production ready

### Single Developer Strategy

Recommended order for sequential implementation:
1. T001-T011 (Setup + Foundation)
2. T012-T019 (US1 - Lists)
3. T020-T032 (US2 - Sequences)
4. T033-T040 (US3 - Nesting)
5. T041-T053 (US4 - Edge Cases)
6. T054-T064 (Integration + Polish)

---

## Notes

- All tests use inline `#[cfg(test)]` modules per Rust convention
- Follow existing arithmetic expansion module as reference
- Commit after completing each phase
- Run `cargo test` after each implementation task to catch regressions
- Target ~800 lines total (within 1,500 line PR limit)

### Pull Request Strategy

**Single PR** for this feature (well-scoped at ~800 lines):
1. Complete all phases
2. Run full test suite
3. Create PR with all changes
4. Verify `git diff --stat main` shows ≤1,500 lines

---

## Summary

| Metric | Value |
|--------|-------|
| Total Tasks | 64 |
| Setup Tasks | 6 |
| Foundational Tasks | 5 |
| US1 (Lists) Tasks | 8 |
| US2 (Sequences) Tasks | 13 |
| US3 (Nesting) Tasks | 8 |
| US4 (Edge Cases) Tasks | 13 |
| Integration Tasks | 5 |
| Polish Tasks | 6 |
| Parallel Opportunities | 31 tasks marked [P] |
| MVP Scope | Phases 1-3 (19 tasks) |
