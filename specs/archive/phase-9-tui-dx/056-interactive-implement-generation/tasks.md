# Tasks: Interactive Implement Generation

**Feature**: 056-interactive-implement-generation
**Created**: 2025-12-17
**Status**: Ready for implementation

## Phase 1: State Setup

- [ ] T001 Create `ImplementState` struct in worktree.rs
- [ ] T002 Add `load_tasks_from_file()` method to load tasks.md
- [ ] T003 Add implement phase to SpecifyState (reuse task_list_state)

## Phase 2: Entry Point

- [ ] T004 Add hotkey 'i' in worktree to start Implement phase
- [ ] T005 Load tasks.md when entering Implement phase
- [ ] T006 Render task list with completion status indicators

## Phase 3: Task Execution

- [ ] T007 [P] Add `execute_task()` async method in app.rs
- [ ] T008 [P] Build task context from spec/plan/tasks
- [ ] T009 Wire Enter key to execute selected task
- [ ] T010 Display execution output in Output pane

## Phase 4: Completion Tracking

- [ ] T011 Mark task complete in memory after execution
- [ ] T012 Add 'x' hotkey for manual completion toggle
- [ ] T013 Implement `save_tasks_to_file()` for persistence
- [ ] T014 Auto-save on task completion

## Phase 5: Polish

- [ ] T015 Add auto-advance to next incomplete task
- [ ] T016 Add progress indicator (X/Y tasks complete)
- [ ] T017 Handle execution errors gracefully

## Phase 6: Testing

- [ ] T018 [P] Unit test: load_tasks_from_file
- [ ] T019 [P] Unit test: save_tasks_to_file
- [ ] T020 [P] Unit test: completion toggle
- [ ] T021 Integration test: execute → complete → save flow

## Dependencies

```
T001 → T002 → T003
T004 → T005 → T006
T007, T008 (parallel) → T009 → T010
T011 → T012 → T013 → T014
T015, T016, T017 (parallel)
T018, T019, T020 (parallel) → T021
```

## Estimates

| Phase | Tasks | Complexity |
|-------|-------|------------|
| State Setup | 3 | Low |
| Entry Point | 3 | Low |
| Task Execution | 4 | Medium |
| Completion Tracking | 4 | Low |
| Polish | 3 | Low |
| Testing | 4 | Low |
| **Total** | **21** | **~300-400 lines** |
