# Feature Specification: Conditional Control Flow (if/then/else/elif/fi)

**Feature Branch**: `015-if-then-else`
**Feature Number**: 017 (Rush Feature Roadmap)
**Created**: 2025-12-06
**Status**: Draft
**Priority**: P0 (Critical - Foundation for scripting)
**Dependencies**: None

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Conditional Execution (Priority: P1)

A user wants to execute different commands based on whether a previous command succeeded or failed. They need to run a command, check its exit status, and execute different code paths accordingly.

**Why this priority**: This is the foundational use case for conditional logic. All scripts need to handle success/failure conditions. Essential for MVP.

**Independent Test**: Can execute `if command; then action1; fi` and verify the action runs when command succeeds.

**Acceptance Scenarios**:

1. **Given** a shell with the if/then/fi construct implemented, **When** executing `if true; then echo "success"; fi`, **Then** "success" is printed to stdout
2. **Given** a shell with if/then/fi implemented, **When** executing `if false; then echo "fail"; fi`, **Then** nothing is printed (command does not execute)
3. **Given** a shell with if/then/fi implemented, **When** executing `if [ -f file.txt ]; then echo "exists"; fi`, **Then** "exists" is printed only if file.txt exists

---

### User Story 2 - Else Clause (Priority: P1)

A user wants to specify an alternative action when the condition fails. They need an `else` clause that executes when the condition is false.

**Why this priority**: Essential control flow; most real scripts need both true and false branches. Enables proper fallback behavior.

**Independent Test**: Can execute `if false; then action1; else action2; fi` and verify action2 executes when condition is false.

**Acceptance Scenarios**:

1. **Given** if/else/fi implemented, **When** executing `if false; then echo "no"; else echo "yes"; fi`, **Then** "yes" is printed
2. **Given** if/else/fi implemented, **When** executing `if true; then echo "yes"; else echo "no"; fi`, **Then** "yes" is printed
3. **Given** if/else/fi implemented, **When** executing `if [ $? -eq 0 ]; then echo "ok"; else echo "error"; fi`, **Then** the appropriate branch executes based on the last exit code

---

### User Story 3 - Elif Clause (Priority: P1)

A user needs to check multiple conditions in sequence. They want `elif` (else if) to avoid deeply nested if statements and improve readability.

**Why this priority**: Required for multi-way branching (common in scripts). Prevents code complexity without this feature.

**Independent Test**: Can execute `if cond1; then act1; elif cond2; then act2; fi` and verify correct branch executes for each condition.

**Acceptance Scenarios**:

1. **Given** if/elif/fi implemented, **When** executing `if false; then echo "1"; elif true; then echo "2"; fi`, **Then** "2" is printed
2. **Given** if/elif/fi implemented, **When** executing `if false; then echo "1"; elif false; then echo "2"; else echo "3"; fi`, **Then** "3" is printed
3. **Given** if/elif/fi implemented, **When** using multiple elif clauses: `if c1; then a1; elif c2; then a2; elif c3; then a3; fi`, **Then** only the first true condition's action executes

---

### User Story 4 - Compound Conditions with &&/|| (Priority: P2)

A user wants to combine conditions using logical AND (`&&`) and OR (`||`) operators to create complex conditional logic without deeply nested if statements.

**Why this priority**: Important for realistic scripts; reduces nesting and improves readability. Secondary but highly valuable.

**Independent Test**: Can execute `if cmd1 && cmd2; then action; fi` and verify action runs only when both commands succeed.

**Acceptance Scenarios**:

1. **Given** if with compound conditions, **When** executing `if [ -f file ] && grep -q "pattern" file; then echo "found"; fi`, **Then** "found" is printed only if file exists AND contains pattern
2. **Given** if with || operator, **When** executing `if command1 || command2; then echo "success"; fi`, **Then** "success" prints if either command succeeds

---

### User Story 5 - Test Command Integration (Priority: P1)

A user wants to use the test command `[...]` or `[[...]]` within if statements to check files, strings, and numeric values. The if statement must properly evaluate test expressions.

**Why this priority**: Essential for any real script. Without test integration, if is severely limited to just exit codes.

**Independent Test**: Can execute `if [ -d "dir" ]; then echo "directory"; fi` and verify proper file/directory testing.

**Acceptance Scenarios**:

1. **Given** if with [ ] test syntax, **When** checking `if [ "$var" = "value" ]; then`, **Then** correct string comparison occurs
2. **Given** if with [ ] test syntax, **When** checking `if [ "$num" -gt 10 ]; then`, **Then** numeric comparison works correctly
3. **Given** if with [ ] test syntax, **When** checking `if [ -z "$empty_var" ]; then`, **Then** empty variable detection works

---

### User Story 6 - Nested If Statements (Priority: P2)

A user needs to nest if statements for more complex control flow within if/else/elif branches.

**Why this priority**: Supports realistic script complexity. Secondary, as elif often reduces nesting needs.

**Independent Test**: Can execute nested if statements and verify outer and inner conditions work correctly together.

**Acceptance Scenarios**:

1. **Given** nested if statements, **When** executing `if true; then if false; then echo "a"; else echo "b"; fi; fi`, **Then** "b" is printed
2. **Given** nested if in else clause, **When** outer condition is false, **Then** inner if in else branch executes correctly

---

### Edge Cases

- What happens when an if statement has no then clause? → Should error
- How does system handle empty if/then/fi blocks? → Should be valid (no-op)
- What if condition command doesn't exist? → Should fail with appropriate error
- What if an elif is used without preceding if? → Should error
- What if fi is missing? → Should error or wait for more input in interactive mode
- How are comments handled inside if blocks? → Should be ignored
- What about if statements spanning multiple lines with improper quoting? → Should error appropriately

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST implement `if` keyword that evaluates a condition command and executes the then block if the command succeeds (exit code 0)
- **FR-002**: Shell MUST implement `then` keyword that marks the beginning of the code block to execute when the condition succeeds
- **FR-003**: Shell MUST implement `else` keyword that marks an alternative block executed when the condition fails (exit code non-zero)
- **FR-004**: Shell MUST implement `elif` keyword (else-if) allowing multiple sequential conditions to be checked
- **FR-005**: Shell MUST implement `fi` keyword that closes an if statement
- **FR-006**: Shell MUST properly evaluate the exit status of condition commands, treating exit code 0 as true and non-zero as false
- **FR-007**: Shell MUST support test commands like `[ ... ]` as conditions within if statements
- **FR-008**: Shell MUST handle compound conditions using `&&` (AND) and `||` (OR) operators within if statements
- **FR-009**: Shell MUST support semicolons and newlines appropriately as statement separators in if blocks
- **FR-010**: Shell MUST allow if statements to be nested (if within if/then/else blocks)
- **FR-011**: Shell MUST properly handle variable expansion and command substitution in condition expressions
- **FR-012**: Shell MUST report appropriate syntax errors when if statements are malformed (missing fi, missing then, etc.)
- **FR-013**: Parser MUST recognize if/then/else/elif/fi as keywords and not interpret them as command names

### Key Entities

- **If Statement**: A control flow construct containing a condition expression, then block, optional else/elif blocks, and fi terminator
  - Attributes: condition (command), then-block (command list), else-block (optional), elif-clauses (optional list)
  - Relationships: can be nested within other if statements, loops, or function bodies

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 6 user stories (basic if, else, elif, compound conditions, test integration, nesting) must have acceptance scenarios that pass
- **SC-002**: The shell can execute bash/sh scripts containing if/then/else/elif/fi constructs with 100% compatibility for standard POSIX conditional syntax
- **SC-003**: Performance: if statement evaluation adds no measurable overhead compared to direct command execution (< 5% latency increase)
- **SC-004**: Parser correctly rejects 100% of malformed if statements (missing fi, missing then, mismatched elif) with clear error messages
- **SC-005**: All existing shell tests continue to pass; no regressions introduced by if/then/else/elif/fi implementation

## Assumptions

- The shell already has command parsing and execution infrastructure (from MVP - feature 001)
- The `[` test command already exists (feature 062 will enhance it with `[[]]`, but basic `[` is available from feature 001)
- Variable expansion and command substitution already work (feature 001, 010, 014)
- The shell properly tracks exit codes of commands (feature 001)
- POSIX-compliant syntax is the target (not bash-specific extensions beyond basic if/elif/else/fi)

## Constraints & Dependencies

- **Depends on**: None (feature 001 MVP provides all base requirements)
- **Blocks**: Features 018-026 (loops, functions, etc. may build on this)
- **Scope**: Limited to if/then/else/elif/fi syntax; test command enhancements are in feature 062
- **Shell Compatibility**: Must support POSIX shell syntax for if statements; bash-isms (like `[[]]`) are out of scope for this feature
