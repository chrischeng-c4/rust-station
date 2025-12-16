# Tasks: MCP Task Completion

**Feature**: 063-mcp-task-completion
**Created**: 2024-12-17
**Status**: Ready for implementation

## Phase 1: Tool Definition

- [ ] T001 Create CompleteTaskArgs struct
- [ ] T002 Define JSON schema for rstn_complete_task

## Phase 2: Event Type

- [ ] T003 Add McpTaskCompleted variant to Event enum

## Phase 3: Task Completion Logic

- [ ] T004 Add complete_task_by_id() to SpecifyState
- [ ] T005 Implement task lookup by ID
- [ ] T006 Mark task complete and save to file
- [ ] T007 Get next incomplete task info

## Phase 4: Tool Handler

- [ ] T008 Implement handle_complete_task async function
- [ ] T009 Send McpTaskCompleted event
- [ ] T010 Return response with next_task info
- [ ] T011 Register rstn_complete_task tool

## Phase 5: Event Handling

- [ ] T012 Handle McpTaskCompleted in app.rs
- [ ] T013 Refresh worktree view task list
- [ ] T014 Update progress indicator

## Phase 6: Testing

- [ ] T015 Unit test: task marked complete
- [ ] T016 Unit test: file updated
- [ ] T017 Unit test: next task returned
- [ ] T018 Integration test: full flow

## Dependencies

```
T001 → T002
T003
T004 → T005 → T006 → T007
T002, T007 → T008 → T009 → T010 → T011
T003, T011 → T012 → T013 → T014
T014 → T015 → T016 → T017 → T018
```

## Notes

- Validation is optional (skip_validation flag)
- Reuse existing task list infrastructure
- Progress format: "X/Y tasks complete"
