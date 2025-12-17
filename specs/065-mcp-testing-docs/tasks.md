# Tasks: MCP Testing & Documentation

**Feature**: 065-mcp-testing-docs
**Created**: 2024-12-17
**Status**: Ready for implementation

## Phase 1: Unit Tests - Server

- [x] T001 Add test_server_startup()
- [x] T002 Add test_server_shutdown()
- [x] T003 Add test_tool_registration()

## Phase 2: Unit Tests - Tools

- [x] T004 Add test_report_status_needs_input()
- [x] T005 Add test_report_status_completed()
- [x] T006 Add test_report_status_error()
- [x] T007 Add test_read_spec_success()
- [x] T008 Add test_read_spec_not_found()
- [x] T009 Add test_get_context_success()
- [x] T010 Add test_complete_task_success()
- [x] T011 Add test_complete_task_invalid_id()

## Phase 3: Integration Tests

- [x] T012 Create tests/mcp_integration_test.rs
- [x] T013 Add test_full_status_flow()
- [x] T014 Add test_task_completion_flow()
- [x] T015 Add test_error_recovery()

## Phase 4: Documentation - CLAUDE.md

- [x] T016 Add MCP Architecture section header
- [x] T017 Document dual-channel architecture
- [x] T018 Document available tools table
- [x] T019 Add tool usage examples
- [x] T020 Add troubleshooting guide

## Phase 5: Documentation - Tool Reference

- [x] T021 Create docs/mcp-tools.md
- [x] T022 Document rstn_report_status schema
- [x] T023 Document rstn_read_spec schema
- [x] T024 Document rstn_get_context schema
- [x] T025 Document rstn_complete_task schema

## Phase 6: Verification

- [x] T026 Run all tests
- [x] T027 Verify documentation renders correctly
- [x] T028 Manual review of docs accuracy

## Dependencies

```
T001 → T002 → T003
T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011
T011 → T012 → T013 → T014 → T015
T015 → T016 → T017 → T018 → T019 → T020
T020 → T021 → T022 → T023 → T024 → T025
T025 → T026 → T027 → T028
```

## Notes

- Unit tests go in mcp_server.rs #[cfg(test)] module
- Integration tests go in tests/ directory
- Keep test names descriptive and specific
- Use mock event channels for unit tests
