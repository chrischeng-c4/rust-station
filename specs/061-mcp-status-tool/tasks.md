# Tasks: MCP Status Tool

**Feature**: 061-mcp-status-tool
**Created**: 2024-12-17
**Status**: Ready for implementation

## Phase 1: Tool Definition

- [ ] T001 Create ReportStatusArgs struct in mcp_server.rs
- [ ] T002 Define JSON schema for rstn_report_status tool

## Phase 2: Event Type

- [ ] T003 Add McpStatus variant to Event enum in event.rs

## Phase 3: Tool Handler

- [ ] T004 Implement handle_report_status async function
- [ ] T005 Send McpStatus event via channel
- [ ] T006 Return ToolResult with success message

## Phase 4: Tool Registration

- [ ] T007 Register rstn_report_status in McpServer startup
- [ ] T008 Verify tool appears in tools/list response

## Phase 5: Event Handling

- [ ] T009 Handle McpStatus in app.rs main loop
- [ ] T010 Implement needs_input → InputDialog transition
- [ ] T011 Implement completed → phase done transition
- [ ] T012 Implement error → display error message

## Phase 6: Testing

- [ ] T013 Unit test: tool handler sends correct event
- [ ] T014 Unit test: needs_input shows dialog
- [ ] T015 Integration test: full tool call flow

## Dependencies

```
T001 → T002
T003
T004 → T005 → T006
T002, T006 → T007 → T008
T003, T008 → T009 → T010 → T011 → T012
T012 → T013 → T014 → T015
```

## Notes

- Reuse existing InputDialog widget
- Mirror behavior of current parse_status() logic
- This is the core replacement for rscli-status blocks
