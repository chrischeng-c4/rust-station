# GEMINI.md

<language>
Respond in English (U.S.) by default. Use Traditional Chinese only when user writes in Traditional Chinese.
</language>

---

<core-philosophy>
## KB-First = Spec-First

**Core Development Philosophy**: The entire project architecture and logic can be derived from the Knowledge Base. KB-First equals Spec-First.

### Principle

- **Knowledge Base as Source of Truth**: `kb/` directory contains authoritative documentation.
- **Code Implements KB**: Implementation follows what is specified in the Knowledge Base.
- **Documentation-Driven Development**: All architectural decisions documented before implementation.

## State-First Architecture

- **State is King**: At any time, rstn's entire state MUST be JSON/YAML serializable.
- **UI = render(State)**: UI is a pure function of state.
- **Testing**: Test state transitions, not UI coordinates.

## Simplicity & Minimalism

- **YAGNI**: Start with minimal viable solution.
- **Delete Aggressively**: Remove unused code and UI elements.
</core-philosophy>

---

<current-architecture-focus>
## Workflow-Driven UI (The "n8n" Model)

The TUI is shifting from a static document viewer to a **Workflow Launcher**.

### 1. Command as Workflow Trigger
- **Left Panel (Commands)**: This is NOT a menu. It is a list of available Workflows.
- **Action**: Selecting a command triggers a Workflow (e.g., "Prompt Claude", "Git Commit").
- **Agent Integration**: AI Agents (Claude Code) are invoked *only* when the workflow reaches a specific node requiring intelligence.

### 2. Dynamic Content Area
- **Middle Panel (Content)**: This area is DYNAMIC.
- **Function**: It visualizes the current state of the active Workflow Node.
- **Examples**:
    - *Idle*: "Select a workflow to start..."
    - *Prompting*: Show Input Dialog.
    - *Streaming*: Show Token Stream.
    - *Reviewing*: Show Diff View.
    - *Specifying*: Show Spec Preview.

### 3. Log Obsolescence
- **No Log Panel**: Detailed logs are persisted to `~/.rstn/logs/`.
- **UI Focus**: The UI should only show information critical to the developer's immediate decision-making.

### 4. No Tab Bar
- **Focus**: The interface should be focused on the current task (Worktree).
- **Navigation**: "Settings" or "Dashboard" can be accessed via specific commands or hotkeys, not always-on tabs.
</current-architecture-focus>

---

<action-plan>
1. **Update KB**: Reflect the new Workflow-Driven UI in `kb/`.
2. **Refactor State**: Update `WorktreeViewState` to support dynamic content nodes.
3. **Refactor UI**: Remove Tab Bar, Remove Log Panel, implement Dynamic Content rendering.
</action-plan>

---

## Sources of Truth
*   **Internal Commands**: `kb/04-reference/cli/commands.md` (Defines active/hidden workflows)
*   **TUI Keybindings**: `kb/04-reference/cli/keybindings.md` (Defines all keyboard mappings)

