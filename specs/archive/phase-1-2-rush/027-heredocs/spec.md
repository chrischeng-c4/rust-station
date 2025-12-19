# Feature Specification: Here Documents (Heredocs)

**Feature Branch**: `027-heredocs`
**Created**: 2025-12-07
**Status**: Draft
**Input**: User description: "Here documents (<<EOF...EOF) - inline document input for commands"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Heredoc Input (Priority: P1)

As a shell user, I want to provide multi-line input to a command inline using heredoc syntax (`<<DELIMITER`), so that I can pass formatted text to commands like `cat`, `mail`, or scripts without needing separate files.

**Why this priority**: This is the core heredoc functionality - without basic heredoc support, no other heredoc features work. Essential for scripting and interactive use.

**Independent Test**: Can be fully tested by running `cat <<EOF\nhello\nworld\nEOF` and verifying "hello\nworld" is output.

**Acceptance Scenarios**:

1. **Given** the user is at a shell prompt, **When** they type `cat <<EOF`, **Then** the shell prompts for additional input with a heredoc continuation prompt
2. **Given** the user is entering heredoc content, **When** they type the delimiter `EOF` on its own line, **Then** the heredoc is complete and the command executes with the collected content as stdin
3. **Given** the user types `cat <<EOF\nhello world\nEOF`, **When** the command executes, **Then** "hello world\n" is output to stdout

---

### User Story 2 - Tab-Stripping Heredocs (Priority: P2)

As a shell script author, I want to use `<<-DELIMITER` to strip leading tabs from heredoc content, so that I can indent heredocs within scripts for readability without affecting the output.

**Why this priority**: Essential for writing readable shell scripts where heredocs appear inside indented code blocks (functions, loops, conditionals).

**Independent Test**: Can be tested by running a heredoc with `<<-EOF` containing tab-indented content and verifying tabs are stripped from output.

**Acceptance Scenarios**:

1. **Given** a heredoc started with `<<-EOF`, **When** content lines have leading tabs, **Then** those leading tabs are stripped from the content passed to the command
2. **Given** a heredoc started with `<<-EOF`, **When** the closing delimiter has leading tabs, **Then** the delimiter is still recognized and heredoc completes
3. **Given** a heredoc started with `<<-EOF`, **When** content lines have leading spaces (not tabs), **Then** those spaces are preserved in the output

---

### User Story 3 - Heredoc in Pipeline (Priority: P2)

As a shell user, I want to use heredocs as input to commands in a pipeline, so that I can combine inline input with command processing.

**Why this priority**: Pipelines are fundamental to shell usage; heredocs must work within them for practical scripting.

**Independent Test**: Can be tested by running `cat <<EOF | grep hello\ntest hello\nEOF` and verifying "test hello" is output.

**Acceptance Scenarios**:

1. **Given** a command with heredoc, **When** piped to another command (e.g., `cat <<EOF | grep pattern`), **Then** the heredoc content flows through the pipeline correctly
2. **Given** a heredoc in the first command of a pipeline, **When** executed, **Then** subsequent pipeline commands receive the heredoc content as their input

---

### User Story 4 - Heredoc with Output Redirection (Priority: P3)

As a shell user, I want to combine heredocs with output redirection, so that I can write inline content directly to files.

**Why this priority**: Common use case for creating configuration files or scripts dynamically.

**Independent Test**: Can be tested by running `cat <<EOF > /tmp/test.txt\nhello\nEOF` and verifying the file contains "hello\n".

**Acceptance Scenarios**:

1. **Given** a command with heredoc, **When** combined with output redirection (e.g., `cat <<EOF > file.txt`), **Then** the heredoc content is written to the file
2. **Given** a command with heredoc and append redirection, **When** executed with `>>`, **Then** content is appended to the file

---

### User Story 5 - Empty Heredoc (Priority: P3)

As a shell user, I want heredocs to handle edge cases like empty content, so that scripts work predictably.

**Why this priority**: Edge case handling ensures robust behavior.

**Independent Test**: Can be tested by running `cat <<EOF\nEOF` and verifying empty output.

**Acceptance Scenarios**:

1. **Given** a heredoc where the delimiter immediately follows the command, **When** executed, **Then** the command receives empty stdin
2. **Given** a heredoc with only whitespace between start and delimiter, **When** executed, **Then** whitespace is passed to the command

---

### Edge Cases

- What happens when the user presses Ctrl+D during heredoc input? Shell aborts heredoc collection and returns to prompt
- What happens when the user presses Ctrl+C during heredoc input? Shell cancels and returns to prompt
- How does the shell handle a heredoc delimiter that is never provided? Shell keeps prompting until Ctrl+D/Ctrl+C
- What happens with a delimiter that contains special characters? Delimiter is matched literally
- What happens if heredoc content contains the delimiter as part of a line? Does not match - delimiter must be on its own line exactly

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST recognize `<<WORD` as the heredoc operator, where WORD is the delimiter
- **FR-002**: Shell MUST recognize `<<-WORD` as the tab-stripping heredoc operator
- **FR-003**: Shell MUST display a continuation prompt (e.g., `heredoc> `) while collecting heredoc content
- **FR-004**: Shell MUST collect all lines until a line exactly matching the delimiter is encountered
- **FR-005**: Shell MUST pass the collected content (excluding the delimiter line) to the command's stdin
- **FR-006**: For `<<-` operator, shell MUST strip leading tabs from each content line
- **FR-007**: For `<<-` operator, shell MUST recognize the delimiter even with leading tabs
- **FR-008**: Shell MUST support heredocs in pipelines (e.g., `cat <<EOF | grep pattern`)
- **FR-009**: Shell MUST support heredocs combined with output redirection (e.g., `cat <<EOF > file`)
- **FR-010**: Shell MUST handle Ctrl+C during heredoc input by aborting and returning to prompt
- **FR-011**: Shell MUST handle Ctrl+D during heredoc input by aborting heredoc collection

### Key Entities

- **Heredoc**: A multi-line input construct with a delimiter, content, and optional tab-stripping flag
- **Delimiter**: The word/string that marks the end of heredoc content (matched literally)
- **Heredoc Content**: The lines between the heredoc operator and the closing delimiter

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete basic heredoc input in a single interactive session without errors
- **SC-002**: Tab-stripping heredocs correctly remove all leading tabs from content lines
- **SC-003**: Heredocs work correctly within pipelines, with content flowing to subsequent commands
- **SC-004**: Heredocs combined with file redirection correctly write content to files
- **SC-005**: Shell gracefully handles interrupts (Ctrl+C, Ctrl+D) during heredoc input
- **SC-006**: Heredoc execution completes within the same time as equivalent file-based input

## Assumptions

- Variable expansion within heredoc content is out of scope for Phase 1 (content is treated as literal)
- Quoted delimiters (`<<'EOF'` or `<<"EOF"`) to prevent expansion are out of scope for Phase 1
- Multiple heredocs in a single command are out of scope for Phase 1

## Dependencies

- Feature 005 (output-redirection): Required for `>` and `>>` with heredocs
- Feature 007 (stderr-redirection): Required for `2>` with heredocs
- Feature 004 (pipes): Required for heredocs in pipelines
