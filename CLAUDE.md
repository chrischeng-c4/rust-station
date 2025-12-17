# CLAUDE.md

<language>
Respond in English (U.S.) by default. Use Traditional Chinese only when user writes in Traditional Chinese.
</language>

---

<chain-of-thought>
Before starting ANY feature work, work through these steps IN ORDER:

<step number="1" name="WHAT">
  - Feature number: ___
  - Feature name: ___
  - User-facing outcome: ___
</step>

<step number="2" name="WHERE in SDD">
  - spec.md exists? YES/NO
  - plan.md exists? YES/NO
  - tasks.md exists? YES/NO
  - Current phase: SPECIFY | PLAN | TASKS | IMPLEMENT | TEST
</step>

<step number="3" name="NEXT ACTION">
  - If no spec → /speckit.specify
  - If no plan → /speckit.plan
  - If no tasks → /speckit.tasks
  - If tasks exist → implement next task
  - If implemented → dispatch to tui-tester
</step>

<step number="4" name="TESTS NEEDED">
  - Unit tests: ___
  - Integration tests: ___
  - TUI e2e tests: ___ (dispatch to tui-tester)
</step>

<step number="5" name="COMPLETE?">
  - All tasks done? YES/NO
  - All tests pass? YES/NO
  - cargo clippy clean? YES/NO
  - Ready for PR? YES/NO
</step>

You MUST write out these 5 steps before writing any code.
</chain-of-thought>

---

<decision-trees>

<tree name="SDD Workflow">
START: New feature request
│
├─► Does spec.md exist?
│   ├─ NO → Run /speckit.specify, then /speckit.clarify if needed
│   └─ YES → Continue
│
├─► Does plan.md exist?
│   ├─ NO → Run /speckit.plan
│   └─ YES → Continue
│
├─► Does tasks.md exist?
│   ├─ NO → Run /speckit.tasks
│   └─ YES → Continue
│
├─► Are all tasks complete?
│   ├─ NO → Implement next task, then dispatch to tui-tester
│   └─ YES → Continue
│
├─► Do all tests pass?
│   ├─ NO → Fix failing tests
│   └─ YES → Continue
│
└─► END: Create PR with /speckit.review
</tree>

<tree name="Dispatch to tui-tester">
START: Need TUI testing?
│
├─► Does feature touch TUI code?
│   ├─ NO → Skip tui-tester, use regular unit tests
│   └─ YES → Continue
│
├─► What TUI component?
│   ├─ Mouse handling → Dispatch with mouse context
│   ├─ Keyboard handling → Dispatch with keyboard context
│   ├─ View/Focus → Dispatch with state context
│   └─ Widget rendering → Dispatch with render context
│
└─► Prepare context, then dispatch:
    Task(subagent_type="tui-tester", prompt="<context>...</context>")
</tree>

</decision-trees>

---

<few-shot-examples>

<example name="Complete SDD Workflow">
User: "Add click support for tabs"

<thinking>
STEP 1: WHAT
- Feature: 001-click-function
- Outcome: Users can click tabs to switch views

STEP 2: WHERE in SDD
- spec.md: NO
- Current phase: SPECIFY

STEP 3: NEXT ACTION
→ Run /speckit.specify

STEP 4: TESTS NEEDED
- Unit: test_mouse_click_settings_tab
- TUI e2e: Dispatch to tui-tester

STEP 5: COMPLETE?
- All tasks done: NO
- Tests pass: NO
- clippy clean: NO
</thinking>

Action: /speckit.specify
</example>

<example name="Dispatching to tui-tester">
After implementing mouse click handling:

Task(
  subagent_type="tui-tester",
  prompt="""
<context>
  <feature>001-click-function</feature>
  <files>crates/rstn/src/tui/app.rs</files>
  <functions>handle_mouse_event()</functions>
  <state-changes>current_view switches between Worktree/Settings/Dashboard</state-changes>
  <layout>tab_bar_rect stores clickable area (populated after render)</layout>
</context>

<requirements>
  <test>Click on each tab switches view</test>
  <test>Right-click is ignored</test>
  <test>Click outside tab bar is ignored</test>
</requirements>

<helpers>
  <helper>render_app_to_test_backend(app, width, height)</helper>
  <helper>mouse_click(col, row) -> MouseEvent</helper>
  <helper>key_event(code) -> KeyEvent</helper>
</helpers>
"""
)
</example>

<example name="Commit Format">
feat(001): add mouse click support for tabs
fix(001): enable mouse capture in terminal setup
test(001): add e2e tests for tab click handling
</example>

</few-shot-examples>

---

<grounding>

<repository-structure>
rustation/
├── Cargo.toml              # Workspace root
├── CLAUDE.md               # This file (main thread instructions)
├── .claude/agents/
│   └── tui-tester.md       # TUI testing subagent
├── crates/
│   ├── rush/               # Shell implementation
│   └── rstn/src/tui/
│       ├── app.rs          # App, handle_mouse_event, handle_key_event
│       ├── event.rs        # Event types
│       ├── views/          # WorktreeView, SettingsView, Dashboard
│       └── widgets/        # InputDialog, TextInput, OptionPicker
├── specs/{NNN}-{name}/
│   ├── spec.md
│   ├── plan.md
│   └── tasks.md
└── target/                 # Build output (gitignored)
</repository-structure>

<sdd-commands>
| Command | Output | Purpose |
|---------|--------|---------|
| /speckit.specify | spec.md | Define requirements |
| /speckit.clarify | refine spec | Ask clarifying questions |
| /speckit.plan | plan.md | Design architecture |
| /speckit.tasks | tasks.md | Generate task breakdown |
| /speckit.implement | code+tests | Implement feature |
| /speckit.review | PR review | Verify against spec |
| /spec-status | status | Full SDD status |
| /spec-check | quick check | Quick status |
</sdd-commands>

<build-commands>
cargo build -p rstn           # Build TUI app
cargo test -p rstn            # Run all tests
cargo test -p rstn test_mouse # Run mouse tests
cargo clippy -p rstn          # Lint check
</build-commands>

<tui-tester-context-template>
When dispatching to tui-tester, ALWAYS use this structure:

<context>
  <feature>{NNN}-{name}</feature>
  <files>{list of changed files}</files>
  <functions>{entry point functions}</functions>
  <state-changes>{what fields change}</state-changes>
  <layout>{which rects are involved}</layout>
</context>

<requirements>
  <test>{specific test case 1}</test>
  <test>{specific test case 2}</test>
  <test>{edge cases}</test>
</requirements>

<helpers>
  <helper>render_app_to_test_backend(app, width, height)</helper>
  <helper>mouse_click(col, row) -> MouseEvent</helper>
  <helper>key_event(code) -> KeyEvent</helper>
  <helper>key_event_with_mod(code, modifiers) -> KeyEvent</helper>
</helpers>
</tui-tester-context-template>

</grounding>

---

<negative-constraints>

<rule severity="NEVER">Skip SDD phases → Leads to misaligned code → Follow specify → plan → tasks → implement</rule>
<rule severity="NEVER">Implement without spec → No traceability → Run /speckit.specify first</rule>
<rule severity="NEVER">Dispatch to tui-tester without context → Agent lacks info → Use context template</rule>
<rule severity="NEVER">Hardcode test coordinates → Breaks on resize → Calculate from layout rects</rule>
<rule severity="NEVER">Forget EnableMouseCapture → Mouse events won't work → Add to terminal setup</rule>
<rule severity="NEVER">Commit without running tests → Broken code enters repo → Run cargo test first</rule>
<rule severity="NEVER">Skip clippy → Lints accumulate → Run cargo clippy before commit</rule>

<bad-example name="No context dispatch">
Task(subagent_type="tui-tester", prompt="Write mouse tests")
</bad-example>

<bad-example name="Missing file info">
Task(subagent_type="tui-tester", prompt="Test click on tabs")
</bad-example>

</negative-constraints>

---

<delimiters>
Use these markers in workflow updates:

<marker name="SDD STATUS">
Feature: 001-click-function
Phase: IMPLEMENT
Tasks: 3/5 complete
</marker>

<marker name="IMPLEMENTING">
Task: Add mouse click handler
File: crates/rstn/src/tui/app.rs
</marker>

<marker name="DISPATCHING TEST">
Agent: tui-tester
Focus: Mouse click on tab bar
</marker>

<marker name="BUILD CHECK">
cargo build: PASS
cargo test: PASS
cargo clippy: PASS
</marker>

<marker name="READY FOR PR">
All tasks complete, tests pass
</marker>
</delimiters>

---

<output-structure>
After each work session, report in this format:

<report>
  <feature>{NNN}-{name}</feature>

  <sdd-phase>
    <phase name="Specify" status="DONE"/>
    <phase name="Plan" status="DONE"/>
    <phase name="Tasks" status="DONE"/>
    <phase name="Implement" status="IN PROGRESS" progress="3/5"/>
    <phase name="Test" status="PENDING"/>
    <phase name="Review" status="PENDING"/>
  </sdd-phase>

  <tasks-completed>
    <task status="done">Task 1: Description</task>
    <task status="done">Task 2: Description</task>
    <task status="next">Task 3: Next up</task>
  </tasks-completed>

  <tests>
    <test name="test_mouse_click_tab" status="PASS" agent="tui-tester"/>
    <test name="test_right_click_ignored" status="PASS" agent="tui-tester"/>
  </tests>

  <build-status>
    <check name="cargo build" status="PASS"/>
    <check name="cargo test" status="PASS" note="7 new tests"/>
    <check name="cargo clippy" status="PASS"/>
  </build-status>

  <next-steps>
    <step>Implement Task 4</step>
    <step>Dispatch to tui-tester for pane click tests</step>
    <step>Run /speckit.review</step>
  </next-steps>
</report>
</output-structure>

---

<self-correction>
Before committing or creating PR, verify ALL items:

<checklist name="SDD Compliance">
  <item>spec.md exists and is current?</item>
  <item>plan.md exists and was followed?</item>
  <item>tasks.md exists and all tasks complete?</item>
  <item>Implementation matches spec?</item>
</checklist>

<checklist name="Code Quality">
  <item>cargo build passes?</item>
  <item>cargo test passes?</item>
  <item>cargo clippy clean?</item>
  <item>No unwrap() in production code?</item>
</checklist>

<checklist name="Testing">
  <item>Unit tests written?</item>
  <item>TUI e2e tests dispatched to tui-tester?</item>
  <item>All tests pass?</item>
  <item>Edge cases covered?</item>
</checklist>

<checklist name="Commit">
  <item>Commit message format: feat(NNN): description?</item>
  <item>Changes are focused (not mixed features)?</item>
  <item>PR size reasonable (&lt;500 lines ideal)?</item>
</checklist>

If ANY item is NO, fix it before proceeding.
</self-correction>

---

<quick-reference>
SDD WORKFLOW:
  1. /speckit.specify → spec.md
  2. /speckit.plan → plan.md
  3. /speckit.tasks → tasks.md
  4. Implement each task
  5. Dispatch TUI tests to tui-tester
  6. /speckit.review → PR

DISPATCH TO TUI-TESTER:
  Task(subagent_type="tui-tester", prompt="<context>...</context><requirements>...</requirements><helpers>...</helpers>")

BUILD CYCLE:
  cargo build -p rstn
  cargo test -p rstn
  cargo clippy -p rstn

COMMIT FORMAT:
  feat(NNN): description
  fix(NNN): description
  test(NNN): description
</quick-reference>

---

<technologies>
  <tech>Rust 1.75+ (edition 2021)</tech>
  <tech>ratatui 0.29+ (TUI framework)</tech>
  <tech>crossterm 0.28 (terminal I/O)</tech>
  <tech>reedline 0.26+ (line editing)</tech>
  <tech>tokio (async runtime)</tech>
  <tech>tracing (logging)</tech>
</technologies>

<debugging>
  <logs>~/.rustation/logs/</logs>
  <command>tail -f ~/.rustation/logs/rstn.log</command>
  <command>grep -i "keyword" ~/.rustation/logs/rstn.log</command>
  <tip>Use `just install-dev` for hot reload symlinks</tip>
</debugging>

---

## MCP Architecture (Features 060-065)

rstn uses a **dual-channel architecture** for Claude Code communication, replacing fragile text-based status parsing with robust MCP tools.

### Architecture Overview

```
Claude Code (claude CLI)
    ↓ (display channel)
    ├── stream-json: Real-time text output
    │   ├── Cost tracking (total_cost_usd)
    │   ├── Session management (session_id)
    │   └── Display rendering
    │
    ↓ (control channel)
    └── MCP/SSE: Structured tool calls
        ├── rstn_report_status
        ├── rstn_read_spec
        ├── rstn_get_context
        └── rstn_complete_task
```

### Display Channel (stream-json)

Purpose: Real-time text rendering and metadata tracking

- **Format**: JSONL (one JSON object per line)
- **Transport**: stdout from `claude --output-format stream-json`
- **Content**: Assistant responses, cost data, session IDs

### Control Channel (MCP over SSE)

Purpose: State transitions and structured communication

- **Protocol**: JSON-RPC 2.0 (Model Context Protocol)
- **Transport**: Server-Sent Events (SSE) over HTTP
- **Port**: 19560 (default, auto-increments if busy)
- **URL**: `http://127.0.0.1:19560`

### Available MCP Tools

| Tool | Purpose | Key Parameters |
|------|---------|----------------|
| `rstn_report_status` | Report status changes | status: "needs_input" \| "completed" \| "error" |
| `rstn_read_spec` | Read spec artifacts | artifact: "spec" \| "plan" \| "tasks" \| "checklist" \| "analysis" |
| `rstn_get_context` | Get feature context | (none - returns current feature metadata) |
| `rstn_complete_task` | Mark task complete | task_id: "T001", skip_validation: bool |

### Tool Usage Example

**Old Way (Removed in Feature 064):**
```
❌ Output status block:
```rscli-status
{"status":"needs_input","prompt":"Describe the feature"}
```
```

**New Way (MCP Tools):**
```
✅ Call MCP tool:
rstn_report_status({
  "status": "needs_input",
  "prompt": "Describe the feature"
})
```

### Implementation Details

**Server Startup** (`main.rs`):
```rust
let (mcp_event_tx, _mcp_event_rx) = mpsc::channel(100);
let mcp_config = McpServerConfig::default();
let mcp_handle = mcp_server::start_server(mcp_config, mcp_event_tx).await?;

// Write config for Claude Code to discover
mcp_server::write_mcp_config(mcp_handle.port())?;
```

**Tool Handler Pattern** (`mcp_server.rs`):
```rust
struct MyToolHandler {
    event_tx: mpsc::Sender<Event>,
}

#[async_trait::async_trait]
impl ToolHandler for MyToolHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        // 1. Extract and validate arguments
        let arg = arguments.get("field")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::validation("Missing 'field'"))?;

        // 2. Send event to TUI
        self.event_tx.send(Event::MyEvent { arg }).await?;

        // 3. Return success
        Ok(ToolResult {
            content: vec![ContentBlock::text("Success")],
            is_error: Some(false),
            ..Default::default()
        })
    }
}
```

**Event Handling** (`app.rs`):
```rust
Event::McpStatus { status, prompt, message } => {
    match status.as_str() {
        "needs_input" => {
            self.input_dialog = Some(InputDialog::new("Input", prompt));
            self.input_mode = true;
        }
        "completed" => {
            self.worktree_view.command_done();
            self.status_message = Some("Completed".to_string());
        }
        "error" => {
            self.status_message = Some(format!("Error: {}", message));
        }
        _ => {}
    }
}
```

### Configuration

rstn automatically writes `~/.config/claude-code/mcp_servers.json` with:

```json
{
  "rstn": {
    "transport": "sse",
    "url": "http://127.0.0.1:19560"
  }
}
```

Claude Code auto-discovers this and connects to the MCP server.

### Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| MCP server not starting | Port 19560 busy | Server auto-increments port. Check logs for actual port. |
| Tool not found | Outdated rstn version | Ensure rstn >= 0.2.0 with Features 060-065 |
| Connection refused | Server not running | Verify `rstn` TUI is active. MCP server starts with TUI. |
| Tools not discovered | Config file missing | Check `~/.config/claude-code/mcp_servers.json` exists |
| Event not received | Channel closed | Check for panics in tool handlers or event loop |

### Testing

**Run MCP integration tests:**
```bash
cargo test -p rstn --test mcp_server_test
```

**Test coverage:**
- Server lifecycle (startup/shutdown)
- Tool registration
- Event dispatch (status, task completion)
- Error handling
- State updates

### Migration Notes

**Feature 064 removed text-based parsing:**
- ❌ Removed: `RscliStatus` struct, `parse_status()` methods
- ❌ Removed: `STATUS_BLOCK_START/END` constants
- ❌ Removed: Text-based fallback heuristics
- ✅ Replaced: All status communication now via MCP tools

**System prompt updated:**
Claude is instructed to use MCP tools instead of status blocks:
```
Use these MCP tools to communicate status and task progress:
- rstn_report_status: Report task status changes
- rstn_complete_task: Mark tasks complete
- rstn_read_spec: Read spec artifacts
- rstn_get_context: Get current feature context
```

### Further Reading

- **Tool Schemas**: See `docs/mcp-tools.md` for detailed tool reference
- **MCP Spec**: [Model Context Protocol](https://modelcontextprotocol.io/)
- **SSE Spec**: [Server-Sent Events](https://html.spec.whatwg.org/multipage/server-sent-events.html)

---

## Active Technologies
- Rust 1.75+ (edition 2021) + okio, serde_json, thiserror (all already in workspace) (052-internalize-spec-generation)
- File system (`specs/` directory, `features.json`) (052-internalize-spec-generation)
- Rust 1.75+ (edition 2021) + okio, serde, serde_json, thiserror, regex (all in workspace) (053-internalize-clarify)
- File system (`specs/{NNN}-{name}/spec.md`) (053-internalize-clarify)
- Rust 1.75+ (edition 2021) + okio (async runtime), serde_json (JSON parsing), thiserror (error handling), which (CLI detection) - all already in workspace (054-internalize-plan)
- File system - `specs/{NNN}-{name}/` directory structure (054-internalize-plan)

## Recent Changes
- 052-internalize-spec-generation: Added Rust 1.75+ (edition 2021) + okio, serde_json, thiserror (all already in workspace)
