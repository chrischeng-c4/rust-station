# Feature Specification: Stderr Redirection (2>, 2>>)

**Feature Branch**: `007-stderr-redirection`
**Created**: 2025-11-30
**Status**: Draft
**Input**: User description: "Implement stderr redirection operators (2> and 2>>) to redirect error output from commands to files, matching the behavior of bash/zsh. Users should be able to redirect stderr separately from stdout, redirect both to the same file, and use both > and >> modes."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Redirect Stderr to File (Priority: P1)

Users want to capture error output from commands without mixing it with standard output. This is essential for debugging and logging error streams separately. A developer running a command that produces both stdout and stderr needs to redirect only the error output to a file while keeping normal output on the terminal.

**Why this priority**: This is the core functionality of stderr redirection. Without this, users cannot separate error streams from output streams, making error tracking and logging difficult.

**Independent Test**: Can be tested by executing any command that produces stderr and verifying that `2>` redirects only error output to a file while stdout appears on terminal.

**Acceptance Scenarios**:

1. **Given** a command that produces both stdout and stderr, **When** user executes `command 2> error.txt`, **Then** stderr is written to `error.txt` and stdout appears on terminal
2. **Given** a file that already exists, **When** user executes `command 2> file.txt`, **Then** the file is overwritten (not appended)

---

### User Story 2 - Append Stderr to File (Priority: P1)

Users want to append error output to existing files without losing previous content. This is critical for maintaining logs of errors that accumulate over time.

**Why this priority**: Appending (2>>) is as essential as basic redirection for log management and multi-command workflows.

**Independent Test**: Can be tested by executing a command with `2>>` and verifying the output appends rather than overwrites.

**Acceptance Scenarios**:

1. **Given** a file with existing content, **When** user executes `command 2>> file.txt`, **Then** stderr is appended to the file preserving existing content
2. **Given** a file that doesn't exist, **When** user executes `command 2>> file.txt`, **Then** the file is created and stderr is written to it

---

### User Story 3 - Redirect Both Stdout and Stderr to Same File (Priority: P2)

Users want to capture both standard output and error output to a single file. This is useful when the distinction between output and error is less important than capturing everything that the command produces.

**Why this priority**: This is a common use case but can be achieved by combining individual redirections. Less critical than basic stderr redirection.

**Independent Test**: Can be tested by executing a command with combined redirections and verifying both stdout and stderr appear in the output file.

**Acceptance Scenarios**:

1. **Given** a command that produces both stdout and stderr, **When** user executes `command > output.txt 2>&1`, **Then** both stdout and stderr are written to `output.txt`
2. **Given** a command with stdout and stderr, **When** user executes `command 2>&1`, **Then** stderr is redirected to wherever stdout is currently going

---

### User Story 4 - Combine Stderr Redirection with Pipes (Priority: P2)

Users want to redirect stderr while piping stdout to other commands. This allows sophisticated command chains where error logging is separated from output piping.

**Why this priority**: Enables advanced workflows combining pipes and redirection. Important but less fundamental than basic redirection.

**Independent Test**: Can be tested by piping stdout while redirecting stderr and verifying correct streams reach their destinations.

**Acceptance Scenarios**:

1. **Given** a command with stdout and stderr, **When** user executes `command 2> error.txt | next_command`, **Then** stdout is piped to `next_command` and stderr is written to `error.txt`
2. **Given** a piped command, **When** user executes `command1 | command2 2> error.txt`, **Then** stderr from `command2` is redirected and stdout is piped

---

### User Story 5 - Redirect Stdout and Stderr to Different Files (Priority: P2)

Users want to redirect stdout and stderr to different files independently. This allows complete separation of output streams for logging and debugging.

**Why this priority**: Provides fine-grained control over output streams. Useful for production logging scenarios.

**Independent Test**: Can be tested by redirecting both streams to different files and verifying correct content in each file.

**Acceptance Scenarios**:

1. **Given** a command with stdout and stderr, **When** user executes `command > output.txt 2> error.txt`, **Then** stdout goes to `output.txt` and stderr goes to `error.txt`
2. **Given** the above, **When** examining both files, **Then** each contains only its respective stream

### Edge Cases

- What happens when the file cannot be written (permission denied)?
- How does the system handle invalid file paths in redirection targets?
- What if stderr is redirected to the same location as stdout multiple times in one command?
- How are special characters in filenames handled (spaces, quotes, etc.)?
- What happens when redirecting to a directory instead of a file?
- How does the system handle closed or unwritable file descriptors?

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST support the `2>` operator to redirect stderr to a file (overwrite mode)
- **FR-002**: System MUST support the `2>>` operator to redirect stderr to a file (append mode)
- **FR-003**: System MUST preserve stdout unaffected when redirecting stderr with `2>` or `2>>`
- **FR-004**: System MUST support combining stderr redirection with pipes (e.g., `command 2> file | next_command`)
- **FR-005**: System MUST support combining stderr and stdout redirection (e.g., `command > out.txt 2> err.txt`)
- **FR-006**: System MUST support `2>&1` to redirect stderr to stdout
- **FR-007**: System MUST create the target file if it doesn't exist (for both `2>` and `2>>`)
- **FR-008**: System MUST handle absolute and relative file paths in redirection targets
- **FR-009**: System MUST report an error if redirection target file cannot be written
- **FR-010**: System MUST handle stderr from piped commands correctly (each command in pipeline can have independent stderr redirection)

### Key Entities

- **File Descriptor**: Represents the output stream (stderr = fd 2, stdout = fd 1)
- **Redirection Target**: The file path where output is redirected
- **Redirection Mode**: Overwrite (>) or append (>>)

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: Users can redirect stderr to a file using `2>` operator with 100% compatibility with bash/zsh behavior
- **SC-002**: Users can append stderr to a file using `2>>` operator with 100% compatibility
- **SC-003**: Stdout remains unaffected when stderr is redirected (verified by no stdout appearing in error file)
- **SC-004**: Error redirection works correctly in pipelines (stderr from any command in the pipeline can be redirected independently)
- **SC-005**: All documented edge cases produce appropriate error messages or handle gracefully
- **SC-006**: Feature passes all acceptance scenarios defined in user stories

## Assumptions

- The underlying operating system supports file descriptor manipulation (all modern systems do)
- File redirection follows POSIX semantics similar to bash/zsh
- The `2>&1` construct is handled as a special case that redirects stderr to the current stdout target
- Permission errors when writing files should produce user-friendly error messages
- Redirections can be chained in a single command (e.g., `2> file1 2> file2` - the last one wins)

## Dependencies

- Builds on **Feature 005** (Output Redirection) - extends the existing `>` and `>>` redirection implementation
- Depends on command parsing infrastructure to recognize and handle `2>` and `2>>` tokens
- Requires file I/O capabilities already present in rush
- Requires integration with job control for proper file descriptor handling

## Notes

- This feature extends the existing output redirection feature (005) with error stream handling
- Implementation should follow POSIX shell semantics for maximum compatibility
- The `&>` operator (redirect both stdout and stderr) is a bash extension and marked as optional
- Error messages should help users understand common mistakes (e.g., `command 2 > file` vs `command 2> file`)
