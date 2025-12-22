# Internal Commands Reference

This document is the Source of Truth for all internal commands (Workflow Triggers) available in the rstn TUI.

```yaml
# Internal Commands Schema
# - id: Unique identifier used in code
#   display_name: Label shown in the TUI left panel
#   description: Purpose of the command
#   status: active | hidden | deprecated
#   workflow_id: ID of the state machine associated with this command

commands:
  - id: PromptClaude
    display_name: "✨ Prompt Claude"
    description: "Start an interactive session with Claude Code agent"
    status: active
    workflow_id: prompt-claude

  - id: Specify
    display_name: "○ Specify"
    description: "Start SDD Specify phase"
    status: hidden
    workflow_id: sdd-specify

  - id: Plan
    display_name: "○ Plan"
    description: "Start SDD Plan phase"
    status: hidden
    workflow_id: sdd-plan

  - id: IntelligentCommit
    display_name: "• Intelligent Commit"
    description: "AI-powered git commit workflow"
    status: hidden
    workflow_id: git-commit
```

## Note on Hidden Commands
Currently, most SDD and Git commands are set to `status: hidden` to maintain a minimalist UI focused on the core Prompt Claude workflow.
