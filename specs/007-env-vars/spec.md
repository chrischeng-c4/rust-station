# Feature Specification: Environment Variables

**Feature Branch**: `007-env-vars`
**Created**: 2025-11-26
**Status**: Draft
**Input**: User description: "Environment Variables - Add $VAR expansion, export and set builtins for rush shell"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Variable Expansion in Commands (Priority: P1)

As a shell user, I want to use environment variables in my commands so that I can reference dynamic values like `$HOME`, `$PATH`, and `$USER` without hardcoding paths or values.

**Why this priority**: This is the most fundamental use case. Without variable expansion, users cannot run basic commands like `cd $HOME` or `echo $PATH`. This blocks all other environment variable functionality and makes the shell impractical for daily use.

**Independent Test**: Can be fully tested by typing `echo $HOME` and verifying the user's home directory is printed. Delivers immediate value for path navigation and command construction.

**Acceptance Scenarios**:

1. **Given** the system has a HOME environment variable set, **When** user types `echo $HOME`, **Then** the shell prints the value of HOME (e.g., `/Users/username`)

2. **Given** the system has a PATH environment variable, **When** user types `echo $PATH`, **Then** the shell prints the full PATH value with colon-separated directories

3. **Given** a variable FOO does not exist, **When** user types `echo $FOO`, **Then** the shell prints an empty string (no error)

4. **Given** a variable HOME exists, **When** user types `ls $HOME/Documents`, **Then** the shell lists the contents of the user's Documents folder

5. **Given** a variable with special characters in value, **When** user references it with `${VAR}`, **Then** the braces disambiguate the variable name from surrounding text (e.g., `${HOME}_backup` expands correctly)

---

### User Story 2 - Set Environment Variables with Export (Priority: P2)

As a shell user, I want to set environment variables using `export` so that my custom variables are available to child processes I spawn.

**Why this priority**: After reading variables, users need to set them. This enables workflow customization, passing configuration to scripts, and modifying PATH for the session. Depends on P1 (expansion) being complete.

**Independent Test**: Can be tested by running `export MY_VAR=hello` followed by `echo $MY_VAR` and verifying "hello" is printed. Child process inheritance can be tested with `export FOO=bar && sh -c 'echo $FOO'`.

**Acceptance Scenarios**:

1. **Given** an empty shell session, **When** user types `export MY_VAR=hello`, **Then** `echo $MY_VAR` prints "hello"

2. **Given** user has set `export PATH=$PATH:/custom/bin`, **When** user types `echo $PATH`, **Then** the PATH includes `/custom/bin` at the end

3. **Given** user has exported a variable, **When** user runs a child process, **Then** the child process can access the exported variable

4. **Given** an existing variable, **When** user types `export VAR=new_value`, **Then** the variable is updated to the new value

5. **Given** user types `export VAR=value with spaces`, **When** expansion occurs, **Then** the entire "value with spaces" is the variable value (quoted properly)

---

### User Story 3 - List and Manage Variables with Set (Priority: P3)

As a shell user, I want to use `set` to view all shell variables and manage them so I can inspect the current environment and troubleshoot issues.

**Why this priority**: This is an inspection/debugging feature. Users can be productive without it using `export` and `echo`, but it's valuable for understanding the shell state. Lower priority than core expansion and setting.

**Independent Test**: Can be tested by running `set` and verifying a list of variables is displayed. Filtering can be tested with `set | grep PATH`.

**Acceptance Scenarios**:

1. **Given** the shell has inherited system environment, **When** user types `set`, **Then** all environment variables are listed in NAME=value format

2. **Given** user has exported custom variables, **When** user types `set`, **Then** both inherited and user-defined variables appear in the list

3. **Given** a large environment, **When** user types `set`, **Then** output can be piped to other commands (e.g., `set | grep PATH`)

---

### User Story 4 - Inherit System Environment on Startup (Priority: P1)

As a shell user, I want rush to automatically inherit environment variables from my system so that my PATH, HOME, USER, and other standard variables work immediately.

**Why this priority**: Without environment inheritance, the shell would be isolated and unusable. Commands wouldn't be found (no PATH), home directory unknown (no HOME). This is foundational and tied with P1.

**Independent Test**: Start rush shell and immediately run `echo $PATH`. If PATH contains system directories (like /usr/bin), inheritance is working.

**Acceptance Scenarios**:

1. **Given** the user's system has PATH set, **When** rush starts, **Then** `echo $PATH` shows the inherited PATH value

2. **Given** the user has custom environment variables in their system, **When** rush starts, **Then** those variables are available in rush

3. **Given** a fresh shell session, **When** user runs `which ls`, **Then** the command is found because PATH is inherited

---

### Edge Cases

- **Undefined variable**: `$UNDEFINED_VAR` expands to empty string, not an error
- **Escaped dollar sign**: `\$HOME` prints literal "$HOME", not the variable value
- **Variable at end of word**: `echo foo$BAR` expands BAR correctly
- **Empty value**: `export EMPTY=` sets variable to empty string (different from unset)
- **Invalid variable name**: `export 123=value` produces an error (names must start with letter/underscore)
- **Recursive expansion**: `export A=$B` where B=hello sets A to "hello" (expands at assignment time)
- **Special variables**: $? (exit code), $$ (PID) - marked as future enhancement, not in initial scope
- **Nested braces**: `${${VAR}}` is not supported (single level only)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST inherit all environment variables from the parent process on startup
- **FR-002**: Shell MUST expand `$VAR` syntax to the variable's value in commands and arguments
- **FR-003**: Shell MUST expand `${VAR}` syntax to support disambiguation from surrounding text
- **FR-004**: Shell MUST expand undefined variables to empty string (no error)
- **FR-005**: Shell MUST support `export VAR=value` to set environment variables
- **FR-006**: Shell MUST pass exported variables to child processes
- **FR-007**: Shell MUST support `export VAR=$OTHER_VAR` with expansion at assignment time
- **FR-008**: Shell MUST support `set` builtin to list all environment variables
- **FR-009**: Shell MUST support escaped dollar sign `\$` to print literal dollar sign
- **FR-010**: Shell MUST reject invalid variable names (names must match `[a-zA-Z_][a-zA-Z0-9_]*`)
- **FR-011**: Shell MUST support variable expansion in any argument position, not just standalone
- **FR-012**: Shell MUST preserve variable values with spaces when properly quoted

### Key Entities

- **Environment Variable**: A name-value pair where name follows `[a-zA-Z_][a-zA-Z0-9_]*` pattern and value is a string. Variables have scope (inherited, session-local, exported to children).
- **Variable Reference**: A `$NAME` or `${NAME}` pattern in user input that triggers expansion to the variable's current value.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can run `echo $HOME` and see their home directory path within 1 second of shell startup
- **SC-002**: Variable expansion adds less than 1ms overhead to command execution (aligns with Performance-First principle)
- **SC-003**: 100% of system environment variables are accessible in rush immediately after startup
- **SC-004**: Users can set and use a custom variable in under 5 seconds (`export FOO=bar && echo $FOO`)
- **SC-005**: Child processes receive all exported variables (verifiable with `sh -c 'echo $VAR'`)

## Assumptions

- Variable names follow POSIX conventions: `[a-zA-Z_][a-zA-Z0-9_]*`
- Values are strings (no typed variables in MVP)
- No universal/global variable scoping (fish-style) in MVP - all variables are session-scoped
- Special variables ($?, $$, $!, etc.) are out of scope for this feature - will be separate specification
- No array variables in MVP
- No variable arithmetic expansion (`$((1+2))`) in MVP
- `unset` command is out of scope for MVP (can be added later)

## Out of Scope

- Special shell variables ($?, $$, $!, $0, $1-$9, $@, $*)
- Array variables
- Arithmetic expansion
- Variable typing
- Universal variables (fish-style persistent variables)
- Variable event handlers
- `unset` command
- `env` command (use system `env`)
