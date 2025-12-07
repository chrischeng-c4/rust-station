# Feature Specification: Read Builtin

**Feature Branch**: `030-read-builtin`
**Created**: 2025-12-07
**Status**: Draft
**Input**: User description: "read builtin for user input"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Variable Input (Priority: P1)

A user wants to prompt for input and store the result in a variable for use in their script.

**Why this priority**: Core functionality - reading user input into a variable is the fundamental purpose of the `read` builtin.

**Independent Test**: Can be tested by running `read var` and typing input, then verifying `$var` contains the input.

**Acceptance Scenarios**:

1. **Given** a shell prompt, **When** user runs `read name` and types "Alice", **Then** variable `$name` equals "Alice"
2. **Given** a running script, **When** `read response` is executed and user enters "yes", **Then** `$response` equals "yes"

---

### User Story 2 - Reading Multiple Variables (Priority: P1)

A user wants to read multiple space-separated values into multiple variables in a single read operation.

**Why this priority**: Essential for parsing structured input like "first last" into separate variables.

**Independent Test**: Run `read first last` with input "John Doe" and verify both variables are set correctly.

**Acceptance Scenarios**:

1. **Given** shell prompt, **When** user runs `read a b` and enters "foo bar", **Then** `$a` equals "foo" and `$b` equals "bar"
2. **Given** input with more words than variables, **When** user runs `read a b` and enters "one two three", **Then** `$a` equals "one" and `$b` equals "two three" (last variable gets remainder)

---

### User Story 3 - Custom Prompt Display (Priority: P2)

A user wants to display a prompt message before reading input for better UX.

**Why this priority**: Improves usability but basic read works without it.

**Independent Test**: Run `read -p "Enter name: " name` and verify prompt appears before cursor.

**Acceptance Scenarios**:

1. **Given** shell, **When** user runs `read -p "Enter name: " name`, **Then** "Enter name: " is displayed and input is read into `$name`

---

### User Story 4 - Silent Password Input (Priority: P2)

A user wants to read sensitive input (like passwords) without echoing characters to the screen.

**Why this priority**: Security feature for password entry scripts.

**Independent Test**: Run `read -s password` and type, verify characters are not displayed but value is captured.

**Acceptance Scenarios**:

1. **Given** shell, **When** user runs `read -s secret` and types "password123", **Then** no characters are echoed and `$secret` equals "password123"

---

### User Story 5 - Reading with Delimiter (Priority: P3)

A user wants to read input until a specific delimiter character instead of newline.

**Why this priority**: Advanced use case for parsing special input formats.

**Independent Test**: Run `read -d ":" var` with input "hello:world" and verify `$var` equals "hello".

**Acceptance Scenarios**:

1. **Given** shell, **When** user runs `read -d ":" value` and enters "path:", **Then** `$value` equals "path"

---

### User Story 6 - Reading into Array (Priority: P3)

A user wants to read input and split it into an array variable.

**Why this priority**: Useful for processing lists but requires array support.

**Independent Test**: Run `read -a arr` with input "one two three" and verify array elements.

**Acceptance Scenarios**:

1. **Given** shell, **When** user runs `read -a items` and enters "a b c", **Then** `${items[0]}` equals "a", `${items[1]}` equals "b", `${items[2]}` equals "c"

---

### Edge Cases

- What happens when user presses Ctrl+D (EOF) without entering input?
- How does read handle empty input (just pressing Enter)?
- What happens when reading from a pipe instead of terminal?
- How are backslash escapes handled with and without -r flag?
- What happens with -n (character count) limit?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST implement `read` as a shell builtin command
- **FR-002**: System MUST store input into the specified variable(s)
- **FR-003**: System MUST split input on whitespace when multiple variables are provided
- **FR-004**: System MUST assign remaining input to the last variable when there are more words than variables
- **FR-005**: System MUST return exit status 0 on successful read, non-zero on EOF or error
- **FR-006**: System MUST support the `-p PROMPT` option to display a prompt string
- **FR-007**: System MUST support the `-s` option for silent input (no echo)
- **FR-008**: System MUST support the `-r` option for raw input (no backslash interpretation)
- **FR-009**: System MUST support the `-d DELIM` option for custom delimiter
- **FR-010**: System MUST support the `-n COUNT` option to read exactly COUNT characters
- **FR-011**: System MUST support the `-a ARRAY` option to read into an array
- **FR-012**: System MUST support the `-t TIMEOUT` option with timeout in seconds
- **FR-013**: System MUST use the REPLY variable when no variable name is specified
- **FR-014**: System MUST strip leading/trailing IFS whitespace from input by default
- **FR-015**: System MUST handle EOF (Ctrl+D) gracefully with appropriate exit status

### Key Entities

- **Variable**: Shell variable that stores the read input value
- **REPLY**: Default variable used when no variable name is provided
- **IFS**: Internal Field Separator used for word splitting (default: space, tab, newline)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All standard `read` options work as documented in POSIX and bash
- **SC-002**: Scripts using `read` for user prompts work correctly
- **SC-003**: Password input with `-s` does not echo characters
- **SC-004**: Multiple variable assignment correctly splits input
- **SC-005**: EOF handling returns correct exit status (1)
- **SC-006**: Timeout option correctly times out after specified seconds
