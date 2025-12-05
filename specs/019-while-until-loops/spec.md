# Feature Specification: While and Until Loops

**Feature**: 019 | **Priority**: P0 | **Dependencies**: Feature 017 (if/then/else)
**Status**: Draft | **Created**: 2025-12-06

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic While Loops (Priority: P1)

Users need condition-based iteration; while loops repeat as long as condition is true.

**Acceptance Scenarios**:
1. `while true; do echo x; done` loops (until Ctrl+C)
2. `while [ $x -lt 5 ]; do echo $x; ((x++)); done` counts 0-4
3. `while [ -f /tmp/lock ]; do sleep 1; done` waits for file removal

### User Story 2 - Until Loops (Priority: P1)

Users need inverse condition loops; until continues until condition becomes true.

**Acceptance Scenarios**:
1. `until [ $x -gt 5 ]; do echo $x; ((x++)); done` counts 0-5
2. Semantically equivalent to: `while ! [ $x -gt 5 ]`

### User Story 3 - Break Statement (Priority: P1)

Users need to exit loops early without running remaining iterations.

**Acceptance Scenarios**:
1. `while true; do read x; [ "$x" = quit ] && break; echo $x; done`
2. Break only affects innermost loop

### User Story 4 - Continue Statement (Priority: P1)

Users need to skip current iteration and continue with next.

**Acceptance Scenarios**:
1. `for i in {1..5}; do [ $((i%2)) -eq 0 ] && continue; echo $i; done` (output: 1, 3, 5)
2. Continue only affects innermost loop

### User Story 5 - Nested Loops (Priority: P2)

Users need nested while/until with proper variable scoping.

### User Story 6 - Loop Variables (Priority: P1)

Loop variables must be modifiable within loop body with changes persisting.

### Edge Cases

- Empty loop body (valid, no-op)
- Infinite loops (Ctrl+C must work)
- Break/continue outside loops (error)
- Loop condition variable modifications

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST implement `while` keyword that evaluates condition and loops while true
- **FR-002**: Shell MUST implement `until` keyword that loops until condition becomes true
- **FR-003**: Shell MUST implement `break` statement to exit current loop immediately
- **FR-004**: Shell MUST implement `continue` statement to skip to next iteration
- **FR-005**: Loop condition re-evaluated at start of each iteration
- **FR-006**: Loop variable modifications persist across iterations
- **FR-007**: Break/continue only affect innermost enclosing loop
- **FR-008**: Support nested while/until with proper scoping
- **FR-009**: Parser recognizes while/until/do/done/break/continue as keywords
- **FR-010**: `until` is equivalent to `while ! condition`

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 6 user stories have passing acceptance scenarios
- **SC-002**: 100% POSIX while/until syntax compliance
- **SC-003**: Parse while loops <1ms, execute <5ms overhead per iteration
- **SC-004**: Zero regressions in existing test suite
- **SC-005**: Support unlimited loop nesting

## Assumptions

- Condition evaluation uses if/then logic (feature 017)
- Variable expansion works (feature 014)
- Test command available (feature 062)

## Constraints & Dependencies

- **Depends on**: Feature 017 (if/then/else), feature 001 (MVP)
- **Blocks**: Complex scripting
- **Scope**: while/until/do/done/break/continue keywords only
