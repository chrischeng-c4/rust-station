# Feature Specification: Fix I/O Redirection Bug

**Feature Branch**: `006-fix-io-redirection`
**Created**: 2025-11-22
**Status**: Draft
**Input**: User description: "Fix I/O redirection bug where redirection operators are not stored in PipelineSegment and redirections are not being executed"

## User Scenarios & Testing

### User Story 1 - Output Redirection (Priority: P1)

Users can redirect command output to files using the `>` operator to save results for later use or processing.

**Why this priority**: Core functionality that was supposed to work in PR #7 but is completely broken. Users expect this basic shell feature.

**Independent Test**: Run `echo hello > /tmp/test.txt` and verify the file contains "hello", not the literal string "hello > /tmp/test.txt".

**Acceptance Scenarios**:

1. **Given** a command with `>` operator, **When** executed, **Then** output is written to the specified file (creating it if needed, truncating if exists)
2. **Given** an existing file, **When** redirecting output with `>`, **Then** the file is truncated and new content replaces old content
3. **Given** output redirection to a path that doesn't exist, **When** executed, **Then** parent directories remain unchanged (no automatic directory creation) and error is shown if parent doesn't exist

---

### User Story 2 - Append Redirection (Priority: P1)

Users can append command output to existing files using the `>>` operator to accumulate results over multiple commands.

**Why this priority**: Equally critical as output redirection - users need to append to logs and accumulate data.

**Independent Test**: Run `echo first >> /tmp/test.txt` then `echo second >> /tmp/test.txt` and verify both lines exist in the file.

**Acceptance Scenarios**:

1. **Given** a command with `>>` operator, **When** executed, **Then** output is appended to the file (creating if needed)
2. **Given** an existing file with content, **When** appending output with `>>`, **Then** new content is added after existing content
3. **Given** a non-existent file, **When** using `>>`, **Then** file is created and content is written (same as `>` for new files)

---

### User Story 3 - Input Redirection (Priority: P2)

Users can provide file contents as input to commands using the `<` operator.

**Why this priority**: Less commonly used than output redirection, but still expected shell functionality.

**Independent Test**: Create a file with content, run `cat < /tmp/test.txt` and verify it reads from the file.

**Acceptance Scenarios**:

1. **Given** a command with `<` operator and existing file, **When** executed, **Then** file contents are provided as stdin to the command
2. **Given** input redirection from non-existent file, **When** executed, **Then** error message is shown and command fails
3. **Given** input redirection from a directory, **When** executed, **Then** error message indicates it's a directory and command fails

---

### Edge Cases

- What happens when redirecting to a file without write permissions? (Show permission denied error)
- How does system handle redirection to `/dev/null`? (Should work normally - discard output)
- What happens with multiple redirections in one command like `echo test > file1.txt > file2.txt`? (Use industry standard behavior - last redirection wins)
- How are redirections handled in pipelines like `ls | grep txt > results.txt`? (Redirection applies to final command in pipeline)
- What happens with empty redirection targets like `echo test >`? (Parser error - missing file path)

## Requirements

### Functional Requirements

- **FR-001**: System MUST store redirection information in PipelineSegment structure so it's available during execution
- **FR-002**: System MUST apply output redirection (`>`) by truncating/creating target file and redirecting stdout
- **FR-003**: System MUST apply append redirection (`>>`) by opening target file in append mode and redirecting stdout
- **FR-004**: System MUST apply input redirection (`<`) by opening source file and redirecting it to stdin
- **FR-005**: System MUST preserve redirection information through the entire pipeline parsing and execution chain
- **FR-006**: System MUST handle file errors gracefully (permission denied, file not found, is directory) with clear error messages
- **FR-007**: System MUST support multiple redirections in a single command using standard shell semantics (last redirection wins for same stream)
- **FR-008**: System MUST integrate redirections with pipeline execution (redirections apply to appropriate command in pipeline)

### Key Entities

- **PipelineSegment**: Represents a single command in a pipeline. Must store: program name, arguments, and **redirections** (currently missing this field)
- **Redirection**: Represents a single I/O redirection. Attributes: type (Output/Append/Input), file path, file descriptor (stdout/stdin)

## Success Criteria

### Measurable Outcomes

- **SC-001**: All integration tests for I/O redirection pass (output, append, input)
- **SC-002**: Manual test `echo hello > /tmp/test.txt` creates file with correct content in under 100ms
- **SC-003**: Redirection works correctly in combination with pipelines (e.g., `ls | grep txt > results.txt`)
- **SC-004**: Error messages for redirection failures are clear and actionable (specify which file and why it failed)

## Assumptions

- Redirection syntax has already been implemented in the parser (verified - parser correctly identifies `>`, `>>`, `<`)
- Standard Unix file permissions apply (no special permission handling needed beyond OS errors)
- Redirection follows POSIX shell conventions (last redirection for same stream wins)
- File creation uses default permissions (0644 for created files)

## Out of Scope

- Advanced redirections (2>, 2>>, &>, etc.) - only stdout/stdin redirections
- File descriptor manipulation beyond basic stdin/stdout
- Here-documents (`<<`) and here-strings (`<<<`)
- Process substitution (`<(command)`, `>(command)`)
- Redirection to file descriptors (`>&2`)
