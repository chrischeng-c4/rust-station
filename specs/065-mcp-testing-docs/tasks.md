# Tasks: MCP Testing & Documentation

**Feature**: 065-mcp-testing-docs
**Created**: 2024-12-17
**Status**: Ready for implementation

## Phase 1: Unit Tests - Server

- [ ] T001 Add test_server_startup()
- [ ] T002 Add test_server_shutdown()
- [ ] T003 Add test_tool_registration()

## Phase 2: Unit Tests - Tools

- [ ] T004 Add test_report_status_needs_input()
- [ ] T005 Add test_report_status_completed()
- [ ] T006 Add test_report_status_error()
- [ ] T007 Add test_read_spec_success()
- [ ] T008 Add test_read_spec_not_found()
- [ ] T009 Add test_get_context_success()
- [ ] T010 Add test_complete_task_success()
- [ ] T011 Add test_complete_task_invalid_id()

## Phase 3: Integration Tests

- [ ] T012 Create tests/mcp_integration_test.rs
- [ ] T013 Add test_full_status_flow()
- [ ] T014 Add test_task_completion_flow()
- [ ] T015 Add test_error_recovery()

## Phase 4: Documentation - CLAUDE.md

- [ ] T016 Add MCP Architecture section header
- [ ] T017 Document dual-channel architecture
- [ ] T018 Document available tools table
- [ ] T019 Add tool usage examples
- [ ] T020 Add troubleshooting guide

## Phase 5: Documentation - Tool Reference

- [ ] T021 Create docs/mcp-tools.md
- [ ] T022 Document rstn_report_status schema
- [ ] T023 Document rstn_read_spec schema
- [ ] T024 Document rstn_get_context schema
- [ ] T025 Document rstn_complete_task schema

## Phase 6: Verification

- [ ] T026 Run all tests
- [ ] T027 Verify documentation renders correctly
- [ ] T028 Manual review of docs accuracy

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
