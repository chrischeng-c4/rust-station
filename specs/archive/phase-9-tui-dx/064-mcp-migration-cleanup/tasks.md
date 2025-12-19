# Tasks: MCP Migration & Cleanup

**Feature**: 064-mcp-migration-cleanup
**Created**: 2024-12-17
**Status**: Ready for implementation

## Phase 1: Remove Status Parsing

- [x] T001 Remove STATUS_BLOCK_START constant
- [x] T002 Remove STATUS_BLOCK_END constant
- [x] T003 Remove RscliStatus struct
- [x] T004 Remove parse_status() method
- [x] T005 Remove needs_input() method
- [x] T006 Remove is_completed() method
- [x] T007 Remove has_error() method
- [x] T008 Remove get_input_prompt() method
- [x] T009 Remove get_error_message() method

## Phase 2: Simplify Display

- [x] T010 Simplify get_display_text() to passthrough

## Phase 3: Update Event Handling

- [x] T011 Remove text-based fallback in handle_claude_completed()
- [x] T012 Verify MCP-only control path works

## Phase 4: Update System Prompt

- [x] T013 Update RSCLI_SYSTEM_PROMPT in cargo.rs
- [x] T014 Add MCP tool usage instructions

## Phase 5: Verification

- [x] T015 Run cargo build -p rstn
- [x] T016 Run cargo test -p rstn
- [x] T017 Run cargo clippy -p rstn
- [x] T018 Manual test full SDD workflow

## Dependencies

```
T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009
T009 → T010
T010 → T011 → T012
T012 → T013 → T014
T014 → T015 → T016 → T017 → T018
```

## Notes

- This is a hard cutover - no backward compatibility
- Must be done AFTER all MCP tools are working (060-063)
- Keep session_id handling intact
- get_text() stays unchanged for display channel
