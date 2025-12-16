# Tasks: MCP Migration & Cleanup

**Feature**: 064-mcp-migration-cleanup
**Created**: 2024-12-17
**Status**: Ready for implementation

## Phase 1: Remove Status Parsing

- [ ] T001 Remove STATUS_BLOCK_START constant
- [ ] T002 Remove STATUS_BLOCK_END constant
- [ ] T003 Remove RscliStatus struct
- [ ] T004 Remove parse_status() method
- [ ] T005 Remove needs_input() method
- [ ] T006 Remove is_completed() method
- [ ] T007 Remove has_error() method
- [ ] T008 Remove get_input_prompt() method
- [ ] T009 Remove get_error_message() method

## Phase 2: Simplify Display

- [ ] T010 Simplify get_display_text() to passthrough

## Phase 3: Update Event Handling

- [ ] T011 Remove text-based fallback in handle_claude_completed()
- [ ] T012 Verify MCP-only control path works

## Phase 4: Update System Prompt

- [ ] T013 Update RSCLI_SYSTEM_PROMPT in cargo.rs
- [ ] T014 Add MCP tool usage instructions

## Phase 5: Verification

- [ ] T015 Run cargo build -p rstn
- [ ] T016 Run cargo test -p rstn
- [ ] T017 Run cargo clippy -p rstn
- [ ] T018 Manual test full SDD workflow

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
