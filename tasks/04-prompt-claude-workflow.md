---
title: "Implement Prompt to Claude Workflow"
status: "done"
priority: "high"
last_updated: 2025-12-23
---

# Task: Implement Prompt to Claude Workflow

## Source
- `kb/architecture/10-workflow-prompt-claude.md`
- `kb/architecture/02-state-first-mvi.md`

## Todo List

### Phase 1: State & Types
- [x] Define `PromptClaudeData` in `rstn/state/workflows/prompt.py`.
- [x] Update `AppState` / `WorkflowState` to support structured workflow data.
- [x] Add "Prompt Claude" to the default command list in `WorktreeViewState`.

### Phase 2: Domain & Infrastructure
- [x] Implement `SessionConfigManager` to generate `/tmp/rstn/{session_id}/mcp-config.json`.
- [x] Update `RunClaudeCli` effect in `rstn/effect/__init__.py` to support full parameters (max_turns, etc.).
- [x] Implement raw logging of Claude CLI output in the executor.

### Phase 3: Effect Executor Implementation
- [x] Implement async subprocess execution for `claude` CLI.
- [x] Implement JSONL parser for `stream-json` output.
- [x] Map JSONL events to `ClaudeStreamDelta`, `ClaudeCompleted`, etc.

### Phase 4: Reducer Logic (FSM)
- [x] Handle `WorkflowStartRequested` for `prompt-claude`:
    - Transition status to `RUNNING`.
    - Dispatch `RunClaudeCli` effect.
- [x] Handle `ClaudeStreamDelta`:
    - Append text to `PromptClaudeData.output`.
- [x] Handle `ClaudeCompleted`:
    - Update status to `COMPLETED`.
    - Capture `session_id` and final metrics.

### Phase 5: UI & Interaction
- [x] Implement "Prompt Mode" in `WorktreeView` (triggered by command).
- [x] Map `Enter` key in input mode to dispatch `WorkflowStartRequested`.
- [x] Ensure Content Area scrolls to bottom during streaming output.
- [x] Display cost/session metrics in the status bar or content footer.

### Phase 6: Verification
- [x] Write state transition tests for the workflow lifecycle.
- [x] Verify serialization/deserialization of `PromptClaudeData`.
- [x] Manual test with real `claude` CLI and a mock MCP config.
