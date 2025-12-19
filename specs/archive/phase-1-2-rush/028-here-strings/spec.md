# Feature Specification: Here Strings

**Feature Branch**: `028-here-strings`
**Created**: 2025-12-07
**Status**: Draft
**Input**: User description: "Here strings (<<<'string') - pass a string directly as stdin to a command"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic String Input (Priority: P1)

As a shell user, I want to pass a simple string directly as stdin to a command using `<<<` syntax, so that I can quickly provide input without creating files or using echo pipes.

**Why this priority**: This is the core functionality - the most common use case for here-strings. Without this, the feature has no value.

**Independent Test**: Can be fully tested by running `cat <<<'hello world'` and verifying output is "hello world"

**Acceptance Scenarios**:

1. **Given** a command that reads stdin, **When** I run `cat <<<'hello'`, **Then** the command receives "hello" followed by a newline as stdin and outputs it
2. **Given** a command that reads stdin, **When** I run `cat <<<"hello world"`, **Then** the command receives "hello world" followed by a newline as stdin
3. **Given** a command that reads stdin, **When** I run `cat <<<hello`, **Then** the command receives "hello" followed by a newline as stdin (unquoted word)

---

### User Story 2 - Variable Expansion (Priority: P2)

As a shell user, I want here-strings to expand variables in unquoted or double-quoted strings, so that I can pass dynamic content to commands.

**Why this priority**: Variable expansion makes here-strings practical for scripting. Single-quoted strings deliberately prevent expansion for literal content.

**Independent Test**: Can be tested by setting a variable and running `cat <<<"$VAR"` to verify expansion occurs

**Acceptance Scenarios**:

1. **Given** variable `NAME=world`, **When** I run `cat <<<"hello $NAME"`, **Then** the command receives "hello world" followed by a newline
2. **Given** variable `NAME=world`, **When** I run `cat <<<'hello $NAME'`, **Then** the command receives literal "hello $NAME" followed by a newline (no expansion)
3. **Given** variable `DIR=/tmp`, **When** I run `cat <<<"${DIR}/file"`, **Then** the command receives "/tmp/file" followed by a newline

---

### User Story 3 - Pipeline Integration (Priority: P2)

As a shell user, I want to use here-strings as the first command in a pipeline, so that I can process string input through multiple commands.

**Why this priority**: Pipelines are fundamental to shell usage. Here-strings should integrate seamlessly.

**Independent Test**: Can be tested by running `cat <<<'hello' | wc -c` and verifying character count

**Acceptance Scenarios**:

1. **Given** a pipeline, **When** I run `cat <<<'hello world' | grep hello`, **Then** grep receives the string and matches
2. **Given** a pipeline, **When** I run `cat <<<'line1' | wc -l`, **Then** wc counts 1 line

---

### User Story 4 - Special Characters (Priority: P3)

As a shell user, I want here-strings to handle special characters correctly based on quoting rules, so that I can pass arbitrary content to commands.

**Why this priority**: Handles edge cases for shell metacharacters. Important for robustness but not core functionality.

**Independent Test**: Can be tested by running `cat <<<'$HOME | > < &'` and verifying literal output

**Acceptance Scenarios**:

1. **Given** a single-quoted here-string, **When** I run `cat <<<'$HOME | > <'`, **Then** the command receives the literal string "$HOME | > <" (no interpretation)
2. **Given** a double-quoted here-string with backslash, **When** I run `cat <<<"hello\tworld"`, **Then** the command receives "hello\tworld" (backslash-t, not tab - bash behavior)
3. **Given** whitespace in content, **When** I run `cat <<<"  spaces  "`, **Then** the command receives "  spaces  " preserving internal whitespace

---

### Edge Cases

- What happens with empty string? `cat <<<''` should pass empty stdin (just newline)
- What happens with multiword unquoted? `cat <<<hello world` - only "hello" is the here-string, "world" is an argument to cat
- What happens with newlines in quoted string? `cat <<<"line1\nline2"` - literal backslash-n (no escape interpretation in bash)
- What happens with here-string after other redirects? `cat <<<'hello' >file` should work
- What happens with command substitution? `cat <<<"$(echo hi)"` should expand to "hi"

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST recognize `<<<` as the here-string operator
- **FR-002**: Shell MUST pass the here-string content as stdin to the preceding command
- **FR-003**: Shell MUST append a trailing newline to the here-string content
- **FR-004**: Shell MUST support single-quoted strings (`<<<'text'`) with no expansion
- **FR-005**: Shell MUST support double-quoted strings (`<<<"text"`) with variable expansion
- **FR-006**: Shell MUST support unquoted words (`<<<word`) as here-strings
- **FR-007**: Shell MUST expand variables (`$VAR`, `${VAR}`) in unquoted and double-quoted here-strings
- **FR-008**: Shell MUST expand command substitutions (`$(cmd)`) in unquoted and double-quoted here-strings
- **FR-009**: Shell MUST preserve literal content (no expansion) in single-quoted here-strings
- **FR-010**: Shell MUST allow here-strings combined with output redirection (`cat <<<'x' >file`)
- **FR-011**: Shell MUST allow here-strings in pipelines (`cat <<<'x' | grep x`)

### Key Entities

- **Here-String Token**: The `<<<` operator followed by the string content
- **String Content**: The text to be passed as stdin, which may be quoted or unquoted
- **Quoting Type**: Single quotes (literal), double quotes (expansion), or unquoted (expansion)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 11 functional requirements pass their acceptance tests
- **SC-002**: Here-strings work identically to bash for the supported syntax subset
- **SC-003**: Users can replace `echo "text" | cmd` with `cmd <<<"text"` for common use cases
- **SC-004**: Here-strings integrate with existing pipeline and redirection features without regression

## Scope

### In Scope (Phase 1)
- Basic `<<<word`, `<<<'quoted'`, and `<<<"double quoted"` syntax
- Variable expansion (`$VAR`, `${VAR}`) in appropriate contexts
- Command substitution (`$(cmd)`) in appropriate contexts
- Pipeline and redirection integration
- Trailing newline appended to content

### Out of Scope (Future)
- Here-string with heredoc in same command (multiple input redirections)
- Process substitution within here-string
- ANSI-C quoting (`$'...'`) - different feature

## Dependencies

- Feature 027 (heredocs): Complete - shares stdin redirection infrastructure
- Feature 010 (command-substitution): Complete - needed for `$(cmd)` expansion
- Feature 014 (environment-variables): Complete - needed for variable expansion

## Assumptions

- Bash compatibility is the target behavior for here-strings
- Only one here-string per simple command (standard shell behavior)
- Here-string takes precedence if both heredoc and here-string specified (last one wins)
