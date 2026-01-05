# Workflow Testing Guide

> **Status**: Active
> **Last Updated**: 2025-01-05

## Overview

This document defines the testing strategy for the three workflow state machines:
- `constitution-management` - Constitution Management
- `context-management` - Context Management
- `change-management` - Change Management

## Testing Principles

### 1. Unit Tests: One Transition Per Test

Each unit test covers **exactly one state transition**:
- Input: Current state + Action
- Output: Next state
- Assertion: State changed correctly

```
[State A] --action--> [State B]
     ^                    ^
     |                    |
  1 test case = 1 transition
```

### 2. E2E Tests: Path Coverage

E2E tests cover **complete paths** from entry to terminal states:
- Count = Number of distinct paths through state machine
- Each path = One E2E test
- Cannot exceed branch count
- Cannot be fewer than branch count

```
Entry --> ... --> Terminal
  ^                  ^
  |__________________|
       1 E2E test
```

---

## Constitution Management

### State Machine

```
                         ┌─────────────┐
                         │   Loading   │
                         │ (checking)  │
                         └──────┬──────┘
                                │ CheckConstitutionExists
                ┌───────────────┼───────────────┐
                ▼               ▼               ▼
         ┌────────────┐  ┌────────────┐  ┌────────────┐
         │ ClaudeMd   │  │Constitution│  │Constitution│
         │  Found     │  │  Exists    │  │  Missing   │
         └─────┬──────┘  └────────────┘  └─────┬──────┘
               │                               │
         ┌─────┴─────┐                   ┌─────┴─────┐
         ▼           ▼                   ▼           ▼
     ImportMd    SkipMd             ApplyDefault  StartQA
         │           │                   │           │
         ▼           ▼                   ▼           ▼
      [Exists]   [Missing]           [Exists]   ┌────────┐
                                                │Collecting│
                                                │ Q1→Q2→Q3→Q4
                                                └────┬───┘
                                                     │ Generate
                                                     ▼
                                                ┌──────────┐
                                                │Generating│
                                                └────┬─────┘
                                                     │ Complete
                                                     ▼
                                                ┌──────────┐
                                                │ Complete │
                                                └──────────┘
```

### Unit Tests (State Transitions)

| # | From State | Action | To State | Test Name |
|---|------------|--------|----------|-----------|
| 1 | Loading | CheckConstitutionExists (found) | Exists | `test_check_constitution_exists_found` |
| 2 | Loading | CheckConstitutionExists (missing) | Missing | `test_check_constitution_exists_missing` |
| 3 | Loading | CheckConstitutionExists (claude.md) | ClaudeMdFound | `test_check_constitution_claude_md_found` |
| 4 | ClaudeMdFound | ImportClaudeMd | Exists | `test_import_claude_md` |
| 5 | ClaudeMdFound | SkipClaudeMdImport | Missing | `test_skip_claude_md` |
| 6 | Missing | ApplyDefaultConstitution | Exists | `test_apply_default_constitution` |
| 7 | Missing | StartConstitutionWorkflow | Collecting(Q0) | `test_start_qa_workflow` |
| 8 | Collecting(Qn) | AnswerConstitutionQuestion | Collecting(Qn+1) | `test_answer_question_advance` |
| 9 | Collecting(Q3) | AnswerConstitutionQuestion | Collecting(Q4/Ready) | `test_answer_last_question` |
| 10 | Collecting(Ready) | GenerateConstitution | Generating | `test_start_generation` |
| 11 | Generating | StreamOutput | Generating | `test_stream_output_append` |
| 12 | Generating | CompleteGeneration | Complete | `test_complete_generation` |
| 13 | Exists | StartConstitutionWorkflow | Collecting(Q0) | `test_regenerate_from_exists` |
| 14 | Any | ClearConstitutionWorkflow | (workflow=null) | `test_clear_workflow` |

**Total Unit Tests: 14**

### E2E Tests (Complete Paths)

| # | Path | Test Name |
|---|------|-----------|
| 1 | Loading → Exists (already exists) | `e2e_constitution_already_exists` |
| 2 | Loading → ClaudeMd → Import → Exists | `e2e_import_claude_md` |
| 3 | Loading → ClaudeMd → Skip → Missing → Default → Exists | `e2e_skip_claude_md_apply_default` |
| 4 | Loading → ClaudeMd → Skip → Missing → QA → Complete | `e2e_skip_claude_md_qa_flow` |
| 5 | Loading → Missing → Default → Exists | `e2e_apply_default_template` |
| 6 | Loading → Missing → QA → Collecting → Generate → Complete | `e2e_full_qa_workflow` |
| 7 | Loading → Exists → Regenerate → QA → Complete | `e2e_regenerate_constitution` |

**Total E2E Tests: 7**

---

## Context Management

### State Machine

```
                    ┌─────────────┐
                    │   Loading   │
                    └──────┬──────┘
                           │ RefreshContext
                    ┌──────┴──────┐
                    ▼             ▼
             ┌────────────┐ ┌────────────┐
             │    Not     │ │Initialized │
             │Initialized │ └─────┬──────┘
             └─────┬──────┘       │
                   │        ┌─────┴─────┐
             ┌─────┴─────┐  ▼           ▼
             ▼           ▼ Empty     HasFiles
         GenerateAI  UseTemplate    (Tabs)
             │           │
             ▼           ▼
        ┌──────────┐ ┌──────────┐
        │Generating│ │Initialized│
        └────┬─────┘ │ (empty)  │
             │       └──────────┘
             ▼
        ┌──────────┐
        │HasFiles  │
        └──────────┘
```

### Unit Tests (State Transitions)

| # | From State | Action | To State | Test Name |
|---|------------|--------|----------|-----------|
| 1 | Loading | RefreshContext (not init) | NotInitialized | `test_refresh_not_initialized` |
| 2 | Loading | RefreshContext (has files) | HasFiles | `test_refresh_has_files` |
| 3 | Loading | RefreshContext (empty) | Empty | `test_refresh_empty_context` |
| 4 | NotInitialized | InitializeContext | Initialized(empty) | `test_initialize_templates` |
| 5 | NotInitialized | GenerateContext | Generating | `test_start_ai_generation` |
| 6 | Generating | StreamOutput | Generating | `test_generation_stream` |
| 7 | Generating | CompleteGeneration | HasFiles | `test_generation_complete` |
| 8 | Generating | GenerationError | Error | `test_generation_error` |
| 9 | HasFiles | RefreshContext | HasFiles | `test_refresh_existing_files` |
| 10 | HasFiles | GenerateContext | Generating | `test_regenerate_context` |

**Total Unit Tests: 10**

### E2E Tests (Complete Paths)

| # | Path | Test Name |
|---|------|-----------|
| 1 | Loading → HasFiles (already initialized) | `e2e_context_already_exists` |
| 2 | Loading → NotInit → Template → Empty | `e2e_initialize_with_templates` |
| 3 | Loading → NotInit → AI Generate → HasFiles | `e2e_generate_context_ai` |
| 4 | Loading → HasFiles → AI Refresh → HasFiles | `e2e_refresh_context_ai` |
| 5 | Loading → Empty → AI Generate → HasFiles | `e2e_generate_from_empty` |

**Total E2E Tests: 5**

---

## Change Management

### State Machine

```
                    ┌─────────────┐
                    │   Loading   │
                    └──────┬──────┘
                           │ RefreshChanges
                    ┌──────┴──────┐
                    ▼             ▼
             ┌────────────┐ ┌────────────┐
             │   Empty    │ │ HasChanges │
             │            │ └─────┬──────┘
             └─────┬──────┘       │
                   │        ┌─────┴─────┐
                   │        ▼           ▼
                   │   NoSelection   Selected
                   │                     │
                   │              ┌──────┴──────┐
                   │              ▼             ▼
                   │         ViewDetail    Actions
                   │                      (Propose/Plan)
                   │                           │
                   ▼                           ▼
              CreateChange             StatusTransitions
                   │                    proposed → planning
                   ▼                    planning → planned
              HasChanges                planned → implementing
                                       implementing → done
```

### Unit Tests (State Transitions)

| # | From State | Action | To State | Test Name |
|---|------------|--------|----------|-----------|
| 1 | Loading | RefreshChanges (empty) | Empty | `test_refresh_empty_changes` |
| 2 | Loading | RefreshChanges (has) | HasChanges | `test_refresh_has_changes` |
| 3 | Empty | CreateChange | HasChanges(1) | `test_create_first_change` |
| 4 | HasChanges | CreateChange | HasChanges(n+1) | `test_create_additional_change` |
| 5 | HasChanges | SelectChange | Selected | `test_select_change` |
| 6 | Selected | SelectChange (other) | Selected(other) | `test_select_different_change` |
| 7 | Selected | GenerateProposal | Proposing | `test_start_proposal_generation` |
| 8 | Proposing | CompleteProposal | proposed | `test_complete_proposal` |
| 9 | proposed | GeneratePlan | Planning | `test_start_plan_generation` |
| 10 | Planning | CompletePlan | planned | `test_complete_plan` |
| 11 | planned | StartImplementation | implementing | `test_start_implementation` |
| 12 | implementing | MarkDone | done | `test_mark_done` |
| 13 | Any | CancelChange | cancelled | `test_cancel_change` |
| 14 | Any | ArchiveChange | archived | `test_archive_change` |

**Total Unit Tests: 14**

### E2E Tests (Complete Paths)

| # | Path | Test Name |
|---|------|-----------|
| 1 | Loading → HasChanges (existing) | `e2e_changes_already_exist` |
| 2 | Loading → Empty → Create → HasChanges | `e2e_create_first_change` |
| 3 | Loading → HasChanges → Select → ViewDetail | `e2e_view_change_detail` |
| 4 | Empty → Create → Propose → proposed | `e2e_create_and_propose` |
| 5 | proposed → Plan → planned | `e2e_generate_plan` |
| 6 | planned → implementing → done | `e2e_full_change_lifecycle` |
| 7 | Any → Cancel | `e2e_cancel_change` |

**Total E2E Tests: 7**

---

## Summary

| Workflow | Unit Tests | E2E Tests |
|----------|------------|-----------|
| constitution-management | 14 | 7 |
| context-management | 10 | 5 |
| change-management | 14 | 7 |
| **Total** | **38** | **19** |

---

## File Structure

```
packages/core/src/
├── constitution/
│   ├── mod.rs
│   └── tests.rs          # 14 unit tests
├── context/
│   ├── mod.rs
│   └── tests.rs          # 10 unit tests
└── changes/
    ├── mod.rs
    └── tests.rs          # 14 unit tests

e2e/
├── constitution-management.spec.ts   # 7 E2E tests
├── context-management.spec.ts        # 5 E2E tests
└── change-management.spec.ts         # 7 E2E tests
```

---

## Implementation Priority

1. **P0**: Constitution Management (most mature, already partially tested)
2. **P1**: Context Management (simpler state machine)
3. **P2**: Change Management (most complex, depends on other workflows)

---

## References

- `kb/architecture/11-workflow-system.md` - Workflow architecture
- `kb/workflow/testing-guide.md` - General testing principles
- `kb/architecture/02-state-first-principle.md` - State-first testing
