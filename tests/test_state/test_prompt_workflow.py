"""Tests for Prompt Claude workflow state and transitions.
"""

from __future__ import annotations

import uuid
from pathlib import Path

import pytest

from rstn.msg import ClaudeStreamDelta, WorkflowStartRequested
from rstn.reduce import reduce
from rstn.state import AppState
from rstn.state.workflow import WorkflowStatus
from rstn.state.workflows.prompt import PromptClaudeData


def test_prompt_workflow_lifecycle():
    """Test the complete lifecycle of a Prompt Claude workflow."""
    # 1. Initial State
    state = AppState(project_root="/tmp")
    
    # Verify "Prompt Claude" command exists
    assert any(cmd.id == "prompt-claude" for cmd in state.worktree_view.commands)
    
    # 2. Start Workflow
    workflow_id = "test-wf-123"
    prompt = "Hello Claude"
    msg = WorkflowStartRequested(
        workflow_id=workflow_id,
        workflow_type="prompt-claude",
        params=prompt
    )
    
    state, effects = reduce(state, msg)
    
    # Check state
    assert workflow_id in state.active_workflows
    workflow = state.active_workflows[workflow_id]
    assert workflow.status == WorkflowStatus.RUNNING
    assert isinstance(workflow.data, PromptClaudeData)
    assert workflow.data.prompt == prompt
    assert state.worktree_view.active_workflow_id == workflow_id
    
    # Check effects
    assert any(type(e).__name__ == "RunClaudeCli" for e in effects)
    
    # 3. Handle Stream Delta
    delta = "I am "
    msg_delta = ClaudeStreamDelta(workflow_id=workflow_id, delta=delta)
    state, _ = reduce(state, msg_delta)
    
    assert state.active_workflows[workflow_id].data.output == delta
    assert state.worktree_view.workflow_output.endswith(delta)
    
    # Another delta
    delta2 = "thinking."
    msg_delta2 = ClaudeStreamDelta(workflow_id=workflow_id, delta=delta2)
    state, _ = reduce(state, msg_delta2)
    
    assert state.active_workflows[workflow_id].data.output == delta + delta2
    assert state.worktree_view.workflow_output.endswith(delta + delta2)


def test_prompt_claude_data_serialization():
    """Test that PromptClaudeData can be serialized and restored."""
    data = PromptClaudeData(
        prompt="test prompt",
        output="accumulated output",
        claude_session_id="uuid-123",
        cost_usd=0.05
    )
    
    # Round-trip through JSON
    json_str = data.model_dump_json()
    restored = PromptClaudeData.model_validate_json(json_str)
    
    assert restored.prompt == data.prompt
    assert restored.output == data.output
    assert restored.claude_session_id == data.claude_session_id
    assert restored.cost_usd == data.cost_usd
