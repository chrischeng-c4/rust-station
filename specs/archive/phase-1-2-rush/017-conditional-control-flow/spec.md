# Feature Specification: Conditional Control Flow

**Feature Branch**: `017-conditional-control-flow`
**Created**: 2025-12-06
**Status**: Draft
**Input**: User description: "Conditional Control Flow (if/then/else/elif/fi)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Simple Conditional Execution (Priority: P1)

As a shell user, I want to execute commands conditionally based on whether a previous command succeeded or failed, so that I can create scripts that respond to different situations automatically.

**Why this priority**: This is the fundamental use case for conditional control flow. Without basic if/then/fi support, no other conditional features are useful. It enables the most common scripting pattern: "do X, and if it works, do Y."

**Independent Test**: Can be fully tested by writing a simple `if command; then echo "success"; fi` and verifying correct branch execution based on command exit status.

**Acceptance Scenarios**:

1. **Given** the shell is running, **When** user enters `if true; then echo "yes"; fi`, **Then** "yes" is printed to stdout
2. **Given** the shell is running, **When** user enters `if false; then echo "yes"; fi`, **Then** nothing is printed (command exits silently)
3. **Given** the shell is running, **When** user enters `if ls /existing-dir; then echo "found"; fi`, **Then** directory contents are listed and "found" is printed
4. **Given** the shell is running, **When** user enters `if ls /nonexistent; then echo "found"; fi`, **Then** error message is shown but "found" is NOT printed

---

### User Story 2 - Conditional with Alternative (Priority: P2)

As a shell user, I want to specify an alternative action when my condition fails, so that I can handle both success and failure cases in a single construct.

**Why this priority**: The else clause is the second most common conditional pattern. It completes the basic binary decision-making capability needed for robust scripts.

**Independent Test**: Can be fully tested by writing `if false; then echo "yes"; else echo "no"; fi` and verifying the else branch executes.

**Acceptance Scenarios**:

1. **Given** the shell is running, **When** user enters `if true; then echo "yes"; else echo "no"; fi`, **Then** only "yes" is printed
2. **Given** the shell is running, **When** user enters `if false; then echo "yes"; else echo "no"; fi`, **Then** only "no" is printed
3. **Given** the shell is running, **When** user enters a multiline if/then/else/fi block, **Then** the construct executes correctly after fi is entered

---

### User Story 3 - Multiple Condition Branches (Priority: P3)

As a shell user, I want to chain multiple conditions together, so that I can handle more than two possible outcomes without deeply nesting if statements.

**Why this priority**: The elif clause enables cleaner multi-way branching. While less common than if/else, it's essential for readable scripts that handle multiple cases.

**Independent Test**: Can be fully tested by writing an if/elif/elif/else/fi chain and verifying the correct branch executes based on which condition is true first.

**Acceptance Scenarios**:

1. **Given** the shell is running, **When** user enters `if false; then echo "1"; elif true; then echo "2"; fi`, **Then** only "2" is printed
2. **Given** the shell is running, **When** user enters `if false; then echo "1"; elif false; then echo "2"; else echo "3"; fi`, **Then** only "3" is printed
3. **Given** the shell is running, **When** user enters multiple elif clauses, **Then** only the first matching condition's block executes
4. **Given** the shell is running with `if true; then echo "1"; elif true; then echo "2"; fi`, **When** executed, **Then** only "1" is printed (first match wins)

---

### User Story 4 - Nested Conditionals (Priority: P4)

As a shell user, I want to nest if statements inside other if statements, so that I can express complex decision trees.

**Why this priority**: Nesting is a natural extension of basic conditionals. While users often prefer elif for readability, nested conditionals are sometimes necessary for complex logic.

**Independent Test**: Can be fully tested by writing `if true; then if true; then echo "nested"; fi; fi` and verifying "nested" is printed.

**Acceptance Scenarios**:

1. **Given** the shell is running, **When** user enters `if true; then if true; then echo "inner"; fi; echo "outer"; fi`, **Then** both "inner" and "outer" are printed
2. **Given** the shell is running, **When** user enters `if true; then if false; then echo "inner"; fi; echo "outer"; fi`, **Then** only "outer" is printed
3. **Given** the shell is running, **When** user enters deeply nested conditionals (3+ levels), **Then** all levels are parsed and executed correctly

---

### User Story 5 - Multiline Interactive Entry (Priority: P5)

As a shell user entering commands interactively, I want the shell to prompt me for continuation when I start an if statement, so that I can write readable multiline conditional blocks.

**Why this priority**: Interactive usability matters for day-to-day shell use. Users expect to enter conditionals across multiple lines with appropriate continuation prompts.

**Independent Test**: Can be fully tested by entering `if true; then` on one line and verifying the shell displays a continuation prompt rather than executing immediately.

**Acceptance Scenarios**:

1. **Given** the shell is in interactive mode, **When** user enters `if true; then` and presses Enter, **Then** shell displays a continuation prompt (e.g., `> `)
2. **Given** the shell is waiting for if completion, **When** user enters `echo "hello"` then `fi`, **Then** shell executes the complete construct and returns to normal prompt
3. **Given** the shell is waiting for if completion, **When** user enters invalid syntax like `fi fi`, **Then** shell displays a syntax error and allows user to continue

---

### Edge Cases

- What happens when `if` is used without `then`? Shell should display a syntax error indicating `then` is expected.
- What happens when `fi` is missing? Shell should continue waiting for input in interactive mode, or report error in script mode.
- What happens when `elif` appears without a preceding `if`? Shell should display a syntax error.
- What happens when `else` appears after `else`? Shell should display a syntax error (only one else allowed).
- What happens when the condition command doesn't exist? Shell should report command not found and treat it as failure (else branch executes).
- What happens with empty `then` blocks? Shell should accept them (no-op) - `if true; then; fi` is valid.
- What happens when `then` and `fi` appear on the same line as commands? Shell should parse correctly: `if true; then echo "a"; echo "b"; fi`

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST recognize `if`, `then`, `elif`, `else`, and `fi` as reserved keywords when they appear as the first word of a command or after `;`
- **FR-002**: Shell MUST evaluate the exit status of the condition command (0 = true/success, non-zero = false/failure)
- **FR-003**: Shell MUST execute the `then` block only when the condition command exits with status 0
- **FR-004**: Shell MUST execute the `else` block only when all conditions (if and all elif) exit with non-zero status
- **FR-005**: Shell MUST support multiple `elif` clauses between `if` and `else`/`fi`
- **FR-006**: Shell MUST stop evaluating conditions after the first one that succeeds (short-circuit evaluation)
- **FR-007**: Shell MUST support nesting of if/then/else/elif/fi constructs to arbitrary depth
- **FR-008**: Shell MUST parse if constructs whether written on single line (semicolon-separated) or multiple lines
- **FR-009**: Shell MUST display a continuation prompt when an if construct is incomplete in interactive mode
- **FR-010**: Shell MUST report syntax errors for malformed conditionals with meaningful error messages indicating what was expected
- **FR-011**: Shell MUST set the exit status of the entire if construct to the exit status of the last executed command within the chosen branch
- **FR-012**: Shell MUST treat a nonexistent command in the condition position as a failed condition (non-zero exit status)
- **FR-013**: Shell MUST allow empty blocks (e.g., `if true; then; fi` with no commands in the then block)
- **FR-014**: Shell MUST allow compound commands (pipelines, command lists with `&&` and `||`) as conditions

### Key Entities

- **Conditional Block**: A complete if/then/[elif/then]*/[else]/fi construct. Contains one or more condition-action pairs and optionally a default action.
- **Condition Command**: Any command whose exit status determines which branch executes. Can be a simple command, pipeline, or compound command.
- **Then Block**: A sequence of commands executed when the associated condition succeeds. May be empty.
- **Else Block**: A sequence of commands executed when all conditions fail. Optional, may be empty if present.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All POSIX-compliant if/then/else/elif/fi scripts execute with identical behavior to bash (as reference implementation)
- **SC-002**: Syntax errors in conditional constructs are reported with clear messages indicating the expected token and location
- **SC-003**: Nested conditionals up to 10 levels deep execute correctly without stack overflow or parsing errors
- **SC-004**: Users can write and execute conditional scripts without referencing documentation for basic if/then/else/elif/fi syntax
- **SC-005**: Interactive multiline entry of conditionals works seamlessly with appropriate continuation prompts

## Assumptions

- Exit status semantics follow POSIX convention: 0 = success/true, non-zero = failure/false
- The `test` and `[` commands are out of scope for this feature (separate feature for test expressions)
- Arithmetic conditionals like `(( ))` are out of scope (separate feature)
- Pattern matching with `[[` is out of scope (separate feature)
- Command substitution in conditions works if already implemented in the shell
- The shell already supports basic command execution with exit status tracking
