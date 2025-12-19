# Feature Specification: rush Shell MVP

**Feature Branch**: `001-rush-mvp`
**Created**: 2025-11-14
**Status**: Draft
**Input**: User description: "rush shell MVP - modern, fast, fish-like shell with zero-config philosophy, syntax highlighting, autosuggestions, tab completions, command history, job control, and script execution on macOS"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Interactive Command Execution (Priority: P1)

A developer opens rush for the first time and immediately starts typing commands. The shell responds instantly with visual feedback (syntax highlighting showing commands in one color, paths in another). They can execute basic commands, use arrow keys to edit input, and see command output. The experience feels as fast as their terminal emulator itself.

**Why this priority**: This is the absolute core of any shell. Without basic REPL functionality, nothing else matters. This must work perfectly for rush to be viable.

**Independent Test**: Can be fully tested by launching rush, typing `ls -la`, editing with arrow keys, pressing Enter, and seeing output. Delivers immediate value as a usable shell.

**Acceptance Scenarios**:

1. **Given** rush is launched, **When** user types `ls -la`, **Then** command text appears with syntax highlighting showing `ls` in command color and `-la` in flag color
2. **Given** user has typed a command, **When** user presses Enter, **Then** command executes and output appears immediately
3. **Given** user has typed text, **When** user presses left/right arrow keys, **Then** cursor moves correctly through the input
4. **Given** user has typed text, **When** user presses backspace, **Then** character before cursor is deleted
5. **Given** command fails, **When** execution completes, **Then** user sees error message and shell remains responsive for next command

---

### User Story 2 - Command History Navigation (Priority: P1)

A developer wants to re-run a command they executed earlier in the session. They press the up arrow and see their previous commands. They can cycle through history with up/down arrows, edit any command, and execute it. When they close and reopen rush, their history persists across sessions.

**Why this priority**: History is essential for shell productivity. Developers constantly re-run commands with small modifications. This is a must-have for day-one viability.

**Independent Test**: Can be tested by running several commands (`ls`, `pwd`, `date`), pressing up arrow to see previous commands, cycling through history, executing one, then closing and reopening rush to verify persistence.

**Acceptance Scenarios**:

1. **Given** user has executed commands in current session, **When** user presses up arrow, **Then** previous command appears in input area
2. **Given** user is viewing a history entry, **When** user presses up arrow again, **Then** next older command appears
3. **Given** user is viewing history, **When** user presses down arrow, **Then** next newer command appears (or empty input if at end)
4. **Given** user has closed and reopened rush, **When** user presses up arrow, **Then** commands from previous session appear in history
5. **Given** user views a history entry, **When** user edits and executes it, **Then** the edited version is added as a new history entry

---

### User Story 3 - Syntax Highlighting (Priority: P1)

A developer types commands and immediately sees visual feedback. Command names appear in one color, file paths in another, flags in a third, and strings in a fourth. This happens as they type, with zero latency. If they type an invalid command, it appears in a distinct color to indicate it won't be found.

**Why this priority**: Syntax highlighting is core to rush's "fish-like" UX promise. It provides immediate visual feedback and prevents errors. This is a key differentiator from basic shells and must be in MVP.

**Independent Test**: Can be tested by typing various commands and observing real-time color changes: `ls -la /etc` (command, flag, path), `echo "hello"` (command, string), `nonexistentcmd` (error color).

**Acceptance Scenarios**:

1. **Given** user starts typing, **When** user types a valid command name (e.g., `ls`), **Then** command appears in command color
2. **Given** user has typed a command, **When** user adds a flag (e.g., `-la`), **Then** flag appears in flag color
3. **Given** user is typing, **When** user types a file path that exists, **Then** path appears in valid path color
4. **Given** user types, **When** user types a quoted string (e.g., `"hello"`), **Then** string appears in string color
5. **Given** user types, **When** user types a command that doesn't exist, **Then** text appears in error color

---

### User Story 4 - Autosuggestions (Priority: P2)

A developer starts typing `ls -` and sees ghost text suggesting `-la` based on their command history. They can press right arrow to accept the suggestion, or continue typing to ignore it. Suggestions appear instantly and never interfere with typing. This saves time on frequently-used commands.

**Why this priority**: Autosuggestions are a beloved fish feature that dramatically improves productivity. While not essential for basic shell usage, it's important for the "fish-like" promise. P2 because shell is usable without it.

**Independent Test**: Can be tested by running `ls -la` several times, then typing just `ls` and observing ghost text suggestion. Pressing right arrow should complete the suggestion.

**Acceptance Scenarios**:

1. **Given** user has executed `ls -la` previously, **When** user types `ls`, **Then** ghost text ` -la` appears as a suggestion
2. **Given** a suggestion is shown, **When** user presses right arrow, **Then** suggestion is accepted and becomes part of input
3. **Given** a suggestion is shown, **When** user continues typing different text, **Then** suggestion disappears or updates
4. **Given** user types a command prefix, **When** multiple matching history entries exist, **Then** most recent match is suggested
5. **Given** no matching history exists, **When** user types, **Then** no suggestion appears

---

### User Story 5 - Tab Completions (Priority: P2)

A developer types `ls /etc/p` and presses Tab. rush shows a list of matching paths (`/etc/passwd`, `/etc/paths`, etc.). They can cycle through options with Tab, or start typing to narrow the list. Completions work for commands (from PATH), file paths, and common command flags.

**Why this priority**: Tab completion is expected in modern shells and significantly improves UX. While developers can type full paths manually, this slows them down. P2 because it's important but not blocking basic usage.

**Independent Test**: Can be tested by typing `ls /et` and pressing Tab to see `/etc` completed, or typing `ec` and Tab to see `echo` suggested, or typing `ls -` and Tab to see flag suggestions.

**Acceptance Scenarios**:

1. **Given** user types a partial command name, **When** user presses Tab, **Then** matching commands from PATH are displayed
2. **Given** user types a partial file path, **When** user presses Tab, **Then** matching paths in filesystem are displayed
3. **Given** user types a command and `-`, **When** user presses Tab, **Then** common flags for that command are suggested
4. **Given** multiple completions match, **When** user presses Tab repeatedly, **Then** cycles through matching options
5. **Given** only one completion matches, **When** user presses Tab, **Then** completion is automatically applied

---

### User Story 6 - Job Control (Priority: P3)

A developer runs a long-running process (e.g., `npm install`) and wants to suspend it to run another command. They press Ctrl+Z to suspend it, run other commands, then type `fg` to bring it back. They can also run commands with `&` to background them immediately, use `jobs` to list running jobs, and use `bg` to resume suspended jobs in background.

**Why this priority**: Job control is essential for professional development workflows, but not needed for basic command execution. Developers can work around its absence in MVP by opening multiple terminal windows. P3 because it's important but not blocking launch.

**Independent Test**: Can be tested by running `sleep 30`, pressing Ctrl+Z to suspend, running `jobs` to see it listed, running `fg` to resume, or running `sleep 30 &` to background immediately.

**Acceptance Scenarios**:

1. **Given** a command is running, **When** user presses Ctrl+Z, **Then** process is suspended and user returns to prompt
2. **Given** jobs exist, **When** user types `jobs`, **Then** list of jobs with IDs and status is displayed
3. **Given** a job is suspended, **When** user types `fg`, **Then** job resumes in foreground
4. **Given** a job is suspended, **When** user types `bg`, **Then** job resumes in background
5. **Given** user types a command ending with `&`, **When** command starts, **Then** runs in background and prompt returns immediately

---

### User Story 7 - Script Execution (Priority: P3)

A developer has a shell script file `build.sh` with multiple commands. They make it executable (`chmod +x build.sh`) and run it (`./build.sh`). rush executes each command in the script sequentially, showing output as each completes. Scripts can include basic features like command chaining (`&&`, `||`, `;`) and output redirection (`>`, `>>`).

**Why this priority**: Script execution enables automation and workflows beyond interactive use. However, most developers can use bash for scripts initially while using rush interactively. P3 because it expands rush's utility but isn't essential for MVP launch.

**Independent Test**: Can be tested by creating a file with commands (`echo "line1"\necho "line2"`), making it executable, running it, and verifying both lines are printed.

**Acceptance Scenarios**:

1. **Given** an executable script file exists, **When** user runs `./script.sh`, **Then** commands in script execute sequentially
2. **Given** script contains multiple commands, **When** script runs, **Then** output from each command appears in order
3. **Given** script contains `command1 && command2`, **When** command1 succeeds, **Then** command2 executes
4. **Given** script contains `command1 && command2`, **When** command1 fails, **Then** command2 does not execute
5. **Given** script contains `echo "text" > file.txt`, **When** script runs, **Then** output is redirected to file

---

### Edge Cases

- What happens when terminal window is resized while a command is running?
- What happens when user tries to run a command that doesn't exist in PATH?
- What happens when history file is corrupted or has invalid entries?
- What happens when user presses Ctrl+C during command execution?
- What happens when a command produces more output than terminal buffer can hold?
- What happens when user types non-ASCII characters (emojis, CJK characters)?
- What happens when configuration file has syntax errors?
- What happens when multiple instances of rush try to write to history simultaneously?
- What happens when executed command tries to read from stdin?
- What happens when user's PATH environment variable is empty or invalid?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST display a command prompt immediately upon launch (within 100ms on modern hardware)
- **FR-002**: Shell MUST accept user input character by character and display it in real-time
- **FR-003**: Shell MUST execute commands when user presses Enter and display their output
- **FR-004**: Shell MUST support line editing with arrow keys (left, right), backspace, and delete
- **FR-005**: Shell MUST persist command history to disk after each command execution
- **FR-006**: Shell MUST load command history from disk on startup
- **FR-007**: Shell MUST allow navigation through history using up/down arrow keys
- **FR-008**: Shell MUST highlight syntax in real-time as user types, distinguishing commands, flags, paths, strings, and invalid commands
- **FR-009**: Shell MUST show autosuggestions as ghost text based on command history
- **FR-010**: Shell MUST allow accepting autosuggestions with right arrow key
- **FR-011**: Shell MUST provide tab completions for commands from PATH
- **FR-012**: Shell MUST provide tab completions for file paths from filesystem
- **FR-013**: Shell MUST provide tab completions for common command flags
- **FR-014**: Shell MUST support suspending running processes with Ctrl+Z
- **FR-015**: Shell MUST provide `jobs` command to list active jobs
- **FR-016**: Shell MUST provide `fg` command to resume suspended job in foreground
- **FR-017**: Shell MUST provide `bg` command to resume suspended job in background
- **FR-018**: Shell MUST support background job execution with `&` operator
- **FR-019**: Shell MUST execute shell script files when invoked directly
- **FR-020**: Shell MUST support command chaining operators: `&&` (and), `||` (or), `;` (sequence)
- **FR-021**: Shell MUST support output redirection: `>` (overwrite), `>>` (append)
- **FR-022**: Shell MUST support interrupting running commands with Ctrl+C
- **FR-023**: Shell MUST load optional TOML configuration from `~/.config/rush/rush.toml`
- **FR-024**: Shell MUST work perfectly with zero configuration (no config file required)
- **FR-025**: Shell MUST run on macOS (later versions can expand to other platforms)

### Key Entities

- **Command**: A user-entered instruction consisting of a command name, optional flags, and optional arguments. Attributes include: command name (string), arguments (list of strings), exit code (integer), execution timestamp.

- **History Entry**: A record of a previously executed command. Attributes include: command text (string), execution timestamp, working directory when executed, exit code, session ID.

- **Job**: A running or suspended process managed by the shell. Attributes include: job ID (integer), process ID (integer), command text (string), status (running/suspended/completed), background flag (boolean).

- **Configuration Setting**: A user preference loaded from TOML file. Attributes include: setting key (string), setting value (string/integer/boolean), section (string for grouped settings like `[appearance]` or `[behavior]`).

- **Completion Result**: A potential completion match for user input. Attributes include: completion text (string), completion type (command/path/flag), description (optional helper text), match score (for ranking multiple results).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Shell launches and displays prompt in under 100ms on a modern Mac (M1 or newer)
- **SC-002**: User can execute 10 basic commands (ls, cd, pwd, echo, cat) without any configuration
- **SC-003**: Syntax highlighting updates appear within 16ms of keystroke (maintains 60 FPS feel)
- **SC-004**: Command history persists across sessions - commands from yesterday are accessible today
- **SC-005**: User can complete a full interactive session (10+ commands) and report that shell "feels fast"
- **SC-006**: Autosuggestions appear within 50ms of user stopping typing
- **SC-007**: Tab completions display within 100ms of Tab keypress for common scenarios (< 100 matches)
- **SC-008**: Shell memory usage stays under 10MB during typical interactive session
- **SC-009**: User can suspend a job, run another command, and resume the first job successfully
- **SC-010**: Scripts with 100+ lines execute correctly from start to finish

## Assumptions

- **Platform**: MVP targets macOS only; terminal emulation standards are consistent on modern macOS versions
- **Terminal**: Users run rush in standard terminal emulators (Terminal.app, iTerm2) that support ANSI escape codes
- **Filesystem**: Standard Unix filesystem semantics apply; paths use forward slashes
- **Shell conventions**: Users expect familiar shell behavior (Ctrl+C to interrupt, Ctrl+Z to suspend, etc.)
- **History size**: Default history limit of 10,000 entries is reasonable for most users
- **Config location**: `~/.config/rush/` follows XDG Base Directory specification and is appropriate for macOS
- **Performance**: "Modern hardware" means Mac M1 or newer, or Intel Mac from 2018 or later
- **Syntax highlighting**: Five color categories (command, flag, path, string, error) are sufficient for MVP
- **Completion sources**: Commands come from PATH, paths from filesystem, flags from common command databases

## Out of Scope for MVP

The following features are explicitly excluded from v0.1:

- **Cross-platform support**: Linux and Windows deferred to post-MVP
- **Advanced scripting**: Functions, conditionals, loops, variables deferred
- **Custom themes**: Color customization deferred (one default theme only)
- **Rhai scripting**: Configuration scripting deferred to post-MVP
- **Plugin system**: Extension mechanisms deferred
- **Network features**: SSH integration, remote completion deferred
- **Advanced completions**: Command-specific flag parsing, context-aware suggestions deferred
- **Advanced job control**: Job groups, terminal multiplexing, job monitoring deferred
- **Shell integration**: Git prompt, directory shortcuts, custom widgets deferred
- **Performance monitoring**: Built-in profiling, command timing deferred
- **Session management**: Tmux-like session persistence deferred
