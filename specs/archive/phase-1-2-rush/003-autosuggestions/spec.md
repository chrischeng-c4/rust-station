# Feature Specification: History-Based Autosuggestions

**Feature Branch**: `003-autosuggestions`
**Created**: 2025-11-17
**Status**: Draft
**Input**: User description: "Add autosuggestions from history"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Inline Suggestion Display (Priority: P1)

As a rush shell user, when I start typing a command, I want to see a suggestion from my command history displayed inline (ghosted text after my cursor), so I can quickly reuse commands I've typed before without typing them completely.

**Why this priority**: This is the core value of autosuggestions - showing users their past commands as they type. Without this, the feature provides no value.

**Independent Test**: Can be fully tested by typing the first few characters of a previously-used command and verifying that a grayed-out suggestion appears. Delivers immediate value by reducing typing effort for repeated commands.

**Acceptance Scenarios**:

1. **Given** I previously ran "git status", **When** I type "git s", **Then** I see "tatus" displayed as grayed-out suggestion text after my cursor
2. **Given** I previously ran "cargo build --release", **When** I type "cargo b", **Then** I see "uild --release" as a grayed-out suggestion
3. **Given** I type "xyz" which matches no history, **When** I continue typing, **Then** no suggestion is displayed
4. **Given** I have multiple history entries starting with "git", **When** I type "git", **Then** I see the most recent matching command as the suggestion

---

### User Story 2 - Accept Suggestion with Arrow Key (Priority: P2)

As a rush shell user, when I see an autosuggestion, I want to accept it by pressing the Right Arrow key, so I can complete the command quickly without typing the rest.

**Why this priority**: Suggestions are only useful if users can accept them. This is the primary interaction mechanism.

**Independent Test**: Can be tested by triggering a suggestion (from US1) and pressing Right Arrow to verify the suggestion becomes actual input. Delivers value by allowing users to act on suggestions.

**Acceptance Scenarios**:

1. **Given** I type "git s" and see suggestion "tatus", **When** I press Right Arrow, **Then** the full command "git status" is accepted into the input buffer
2. **Given** I accept a suggestion with Right Arrow, **When** the cursor moves to end of line, **Then** I can continue typing or press Enter to execute
3. **Given** I accept a partial suggestion, **When** I continue typing, **Then** new suggestions appear based on the updated input

---

### User Story 3 - Accept Partial Suggestion (Priority: P3)

As a rush shell user, when I see a multi-word autosuggestion, I want to accept just the next word by pressing Alt+Right Arrow (or Ctrl+Right Arrow), so I can incrementally accept suggestions when I only want part of the suggested command.

**Why this priority**: Power users often want to accept suggestions word-by-word rather than all-or-nothing. This enhances flexibility but is not essential for basic functionality.

**Independent Test**: Can be tested independently by triggering a multi-word suggestion and pressing the word-forward key to verify only one word is accepted. Delivers value for users who want more granular control.

**Acceptance Scenarios**:

1. **Given** I type "git" and see suggestion " commit -m 'message'", **When** I press Alt+Right Arrow, **Then** only "commit" is accepted (not "-m 'message'")
2. **Given** I accept one word of a suggestion, **When** the suggestion updates, **Then** I see the remaining part of the original suggestion or a new matching suggestion
3. **Given** I press Alt+Right Arrow on a single-word suggestion, **When** accepting, **Then** the entire suggestion is accepted

---

### Edge Cases

- **Empty history**: What happens when the user has no command history yet?
  → No suggestions are shown, shell behaves normally

- **No matching history**: What happens when user input doesn't match any history entry?
  → No suggestion is displayed, shell behaves normally

- **Exact match exists**: What happens when user has typed a complete command that exists in history?
  → No suggestion shown (input already complete), or suggest the next command in the sequence if pattern detected

- **Multiple matches**: When multiple history entries match the current input, which one is suggested?
  → Most recent matching entry is suggested (fish shell behavior)

- **Cursor position**: What happens if user moves cursor to middle of line?
  → Suggestions should only appear when cursor is at end of line (industry standard)

- **Very long suggestions**: How to handle suggestions longer than terminal width?
  → Truncate suggestion display at terminal boundary, full text still available for acceptance

- **Special characters in history**: How to handle commands with quotes, escapes, or newlines?
  → Display special characters correctly in grayed-out format, preserve exact command on acceptance

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST display autosuggestions as grayed-out (dimmed) text immediately after the cursor position
- **FR-002**: System MUST search command history for entries that start with the current user input
- **FR-003**: System MUST suggest the most recent matching history entry when multiple matches exist
- **FR-004**: System MUST update suggestions in real-time as the user types each character
- **FR-005**: System MUST only display suggestions when the cursor is at the end of the input line
- **FR-006**: System MUST allow users to accept the full suggestion using Right Arrow key
- **FR-007**: System MUST allow users to accept suggestions word-by-word using Alt+Right Arrow (or Ctrl+Right Arrow on some platforms)
- **FR-008**: System MUST clear the suggestion when user input no longer matches any history entry
- **FR-009**: System MUST handle empty history gracefully (no suggestions, no errors)
- **FR-010**: System MUST preserve exact command text (including quotes, escapes) when accepting suggestions
- **FR-011**: System MUST render special characters in suggestions correctly (quotes, backslashes, etc.)
- **FR-012**: System MUST truncate suggestion display at terminal width boundary without breaking character rendering

### Key Entities

- **Suggestion**: Represents an autocomplete suggestion derived from command history
  - Source command: The full historical command being suggested
  - Display text: The portion of the command shown as grayed-out text
  - Match position: Where in the history the suggestion was found
  - Acceptance state: Full or partial acceptance

- **Match**: Represents a history entry that matches current user input
  - History entry: The complete command from history
  - Match score: Recency-based ranking (most recent = highest)
  - Prefix match: The portion of input that matches the history entry

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can accept a suggested command by pressing Right Arrow, reducing typing time by at least 50% for repeated commands
- **SC-002**: Suggestions appear within 50 milliseconds of user input, providing instant feedback
- **SC-003**: 90% of users successfully accept at least one suggestion within their first 10 commands after feature activation
- **SC-004**: Users report reduced typing effort for frequently-used commands in user testing
- **SC-005**: Feature works correctly with history files containing 10,000+ entries without noticeable lag
- **SC-006**: Suggestions correctly handle commands with special characters (quotes, escapes) in 100% of test cases
