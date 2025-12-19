# Feature Specification: Parameter Expansion

**Feature Branch**: `032-parameter-expansion`
**Created**: 2025-12-07
**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Default Value (Priority: P1)

Use a default value when a variable is unset or null.

**Acceptance Scenarios**:
1. `${var:-default}` - returns "default" if var is unset/null, else var's value
2. `${var-default}` - returns "default" if var is unset (not null), else var's value

---

### User Story 2 - Assign Default (Priority: P1)

Assign a default value when a variable is unset or null.

**Acceptance Scenarios**:
1. `${var:=default}` - if var is unset/null, set var to "default" and expand
2. `${var=default}` - if var is unset (not null), set var to "default" and expand

---

### User Story 3 - Error on Unset (Priority: P2)

Display an error and exit if variable is unset or null.

**Acceptance Scenarios**:
1. `${var:?error message}` - if var is unset/null, print error and return exit status 1
2. `${var?error message}` - if var is unset (not null), print error and return exit status 1

---

### User Story 4 - Use Alternate Value (Priority: P2)

Use an alternate value only if variable is set and non-null.

**Acceptance Scenarios**:
1. `${var:+alternate}` - if var is set and non-null, expand to "alternate", else empty
2. `${var+alternate}` - if var is set (including null), expand to "alternate", else empty

---

### User Story 5 - String Length (Priority: P2)

Get the length of a variable's value.

**Acceptance Scenarios**:
1. `${#var}` - expands to the length of var's value

---

### User Story 6 - Substring Operations (Priority: P3)

Extract substrings from variable values.

**Acceptance Scenarios**:
1. `${var:offset}` - substring from offset to end
2. `${var:offset:length}` - substring from offset with length

---

### Edge Cases

- Nested expansions: `${var:-${other:-default}}`
- Empty vs unset variables
- Special variables ($?, $$, etc.)
- Array variable lengths

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support `${var:-default}` for default value on unset/null
- **FR-002**: System MUST support `${var-default}` for default value on unset only
- **FR-003**: System MUST support `${var:=default}` for assign default on unset/null
- **FR-004**: System MUST support `${var=default}` for assign default on unset only
- **FR-005**: System MUST support `${var:?message}` for error on unset/null
- **FR-006**: System MUST support `${var?message}` for error on unset only
- **FR-007**: System MUST support `${var:+alternate}` for alternate on set/non-null
- **FR-008**: System MUST support `${var+alternate}` for alternate on set
- **FR-009**: System MUST support `${#var}` for string length
- **FR-010**: System MUST support `${var:offset}` for substring
- **FR-011**: System MUST support `${var:offset:length}` for substring with length

## Success Criteria *(mandatory)*

- **SC-001**: All parameter expansion forms work as documented
- **SC-002**: Empty vs unset distinction handled correctly with colon modifier
- **SC-003**: Default/assign operations set variables correctly
- **SC-004**: Error operations print message and set exit status
