# Claude Code Communication Channels

**Last Updated**: 2024-12-20
**Status**: Reference Documentation

---

## Overview

When rstn runs Claude Code programmatically (headless mode with `claude -p`), it uses **3 communication channels simultaneously** for different purposes:

```
┌─────────────────────────────────────────────────────────────┐
│                    rstn (TUI Process)                        │
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │ JSONL Parser │    │ Hook Handler │    │  MCP Server  │  │
│  │ (stdout)     │    │ (.claude/)   │    │ (HTTP :port) │  │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘  │
└─────────┼──────────────────┼──────────────────┼────────────┘
          │                  │                  │
          │ 1. stream-json   │ 2. bash scripts  │ 3. HTTP POST
          ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────────┐
│              Claude Code (Subprocess via -p)                 │
│  claude -p "prompt" --output-format stream-json             │
│         --allowedTools "Bash,Read,Write,..."                │
│         --mcp-config ~/.rstn/mcp-session.json               │
└─────────────────────────────────────────────────────────────┘
```

Each channel serves a distinct purpose and operates independently.

---

## Channel 1: stream-json I/O (Real-time Output Streaming)

### Purpose
Get Claude's **response text** in real-time as it's being generated (streaming output like ChatGPT's typing effect).

### Direction
`Claude Code → rstn` (unidirectional, output only)

### How It Works

**Command Line**:
```bash
claude -p "your prompt" \
  --output-format stream-json \    # Output newline-delimited JSON
  --verbose \                       # Required with -p + stream-json
  --include-partial-messages        # Show incremental updates
```

**Output Format**: JSONL (JSON Lines) - one event per line to stdout

```json
{"type":"assistant","text":"I'll","session_id":"abc123"}
{"type":"assistant","text":" help","session_id":"abc123"}
{"type":"assistant","text":" you","session_id":"abc123"}
{"type":"result","success":true,"session_id":"abc123"}
```

### rstn Implementation

**File**: `crates/rstn/src/runners/cargo.rs:497-530`

```rust
// Read stdout line by line (JSONL format)
let reader = BufReader::new(stdout);
let mut lines = reader.lines();

while let Ok(Some(line)) = lines.next_line().await {
    // Parse each JSON line
    if let Ok(msg) = serde_json::from_str::<ClaudeStreamMessage>(&line) {
        // Track session ID for resumption
        if msg.session_id.is_some() {
            result.session_id = msg.session_id.clone();
        }

        // Forward text to TUI for display
        if msg.msg_type == "assistant" {
            if let Some(ref sender) = sender {
                sender.send(Event::ClaudeStream(msg))?;
            }
        }
    }
}
```

### Message Types

Defined in `crates/rstn/src/tui/claude_stream.rs`:

| Type | Description | Example |
|------|-------------|---------|
| `"assistant"` | Text response from Claude | `{"type":"assistant","text":"Here's the plan..."}` |
| `"tool_use"` | Claude requesting tool execution | `{"type":"tool_use","name":"Read","input":{...}}` |
| `"tool_result"` | Result from tool execution | `{"type":"tool_result","content":"..."}` |
| `"result"` | Final completion status | `{"type":"result","success":true}` |

### When Used
- ✅ **Always** when running `claude -p` from rstn
- ✅ For displaying Claude's response in the TUI Output panel
- ✅ For capturing `session_id` to resume conversations

### Limitations
- **Output only** - cannot send multi-turn conversations
- **Text input only** - uses simple string argument: `claude -p "prompt"`
- For complex input, see **stream-json Input** below

---

## stream-json Input Format (Advanced)

### Purpose
Send **multi-turn conversation history** to Claude instead of a single text prompt.

### Direction
`rstn → Claude Code` (input only, via stdin)

### How It Works

**Command Line**:
```bash
echo '{"role":"user","content":"What is 2+2?"}
{"role":"assistant","content":"4"}
{"role":"user","content":"Multiply by 3"}' | \
claude -p --input-format stream-json --output-format stream-json
```

**Input Format**: JSONL (JSON Lines) via stdin - one message per line

```json
{"role":"user","content":"What is 2+2?"}
{"role":"assistant","content":"2+2 equals 4"}
{"role":"user","content":"Now multiply that by 3"}
```

### Message Structure

#### Role Field

**Type**: String enum (only 2 values allowed)

```typescript
role: "user" | "assistant"
```

**There is NO `"tool"` or `"system"` role in messages.**

#### Basic Message

```json
{
  "role": "user",        // or "assistant"
  "content": "string or array"
}
```

#### Message with Tool Use (Assistant)

```json
{
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I'll read the file for you."
    },
    {
      "type": "tool_use",
      "id": "toolu_01A09q90qw90lq917835lq9",
      "name": "Read",
      "input": {
        "file_path": "spec.md"
      }
    }
  ]
}
```

#### Message with Tool Result (User)

**Important**: Tool results use `"role":"user"`, not `"role":"tool"`!

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "# Spec\n\nThis is the spec content..."
    }
  ]
}
```

### Rules

1. **Must alternate**: `user` → `assistant` → `user` → `assistant`
2. **Consecutive same-role messages are merged** into a single turn
3. **Last message can be assistant** (prompt continuation):
   ```json
   {"role":"assistant","content":"Here's a poem:\n\nRoses are"}
   ```
   Claude will continue from "Roses are..."
4. **Max 100,000 messages** per request

### Use Cases

| Scenario | Use Text Input (`-p "prompt"`) | Use stream-json Input |
|----------|-------------------------------|----------------------|
| Single prompt | ✅ Simple and sufficient | ❌ Overkill |
| Continue conversation | ⚠️ Need `--continue` flag | ✅ Send full history |
| Resume after error | ⚠️ Claude forgets context | ✅ Include error in history |
| Multi-step workflow | ❌ Multiple spawns needed | ✅ Single spawn with conversation |
| Piping from script | ⚠️ Hard to structure | ✅ Natural JSONL format |
| Tool use replay | ❌ Can't show tool history | ✅ Include tool_use/tool_result |

### rstn Status

**Not currently implemented** - rstn uses text input (`-p "prompt"`) + session management (`--continue` flag).

Potential future use:
- Task resumption with full context
- Error recovery workflows
- Multi-round clarification sessions

---

## Channel 2: Hooks (Permission Control)

### Purpose
Control **what Claude can do** by intercepting tool calls and approving/denying them.

### Direction
`Claude Code ↔ Bash Scripts` (bidirectional)

### How It Works

Hooks are **bash scripts** in `.claude/hooks/` that run at specific lifecycle events.

**Hook Types**:
```
.claude/hooks/
├── pre-tool-use         # Before Claude uses a tool
├── permission-request   # When Claude asks for permission
├── post-tool-use       # After tool execution
└── user-prompt-submit  # After user submits a prompt
```

**Example Hook** (`.claude/hooks/post-tool-use`):
```bash
#!/bin/bash
# This hook runs AFTER Claude uses a tool

# Environment variables available:
# - $TOOL_NAME (e.g., "Write", "Bash")
# - $TOOL_RESULT (output from tool execution)

# Auto-rebuild rstn if code changed
if echo "$TOOL_RESULT" | grep -q "crates/rstn/"; then
    cargo build -p rstn
fi
```

**Permission Hook** (`.claude/hooks/permission-request`):
```bash
#!/bin/bash
# Auto-approve specific tools

if [[ "$TOOL_NAME" == "Read" ]]; then
    echo '{"permissionDecision":"allow"}'
    exit 0
fi

if [[ "$TOOL_NAME" == "Bash" ]] && [[ "$TOOL_ARGS" == *"cargo build"* ]]; then
    echo '{"permissionDecision":"allow"}'
    exit 0
fi

# Ask for everything else
echo '{"permissionDecision":"ask"}'
```

### Hook vs --allowedTools

| Method | Granularity | Performance | Use Case |
|--------|-------------|-------------|----------|
| **Hooks** | Very fine (inspect each call) | Slower (bash script per call) | Complex approval logic |
| **--allowedTools** | Tool-level (e.g., "all Bash") | Fast (no script execution) | ✅ **rstn uses this** |

### rstn's Approach

**Instead of hooks**, rstn uses `--allowedTools` flag for better performance:

**File**: `crates/rstn/src/runners/cargo.rs:407-418`

```rust
// Add allowed tools for autonomous operation (instead of hooks)
if !options.allowed_tools.is_empty() {
    let tools_str = options.allowed_tools.join(",");
    cmd.arg("--allowedTools").arg(&tools_str);
    tracing::debug!("Added --allowedTools: {}", tools_str);
}
```

**Allowed Tools** (defined in `app.rs:616-624`):
```rust
allowed_tools: vec![
    "Bash".to_string(),      // All Bash commands
    "Read".to_string(),      // Read files
    "Write".to_string(),     // Write files
    "Edit".to_string(),      // Edit files
    "Glob".to_string(),      // File pattern matching
    "Grep".to_string(),      // Content search
    "Task".to_string(),      // Subagent tasks
]
```

This tells Claude: "You can use these tools **without asking**."

### When Used

- ❌ **Not used for permission control** in rstn (replaced by `--allowedTools`)
- ✅ Used for **side effects** (like auto-rebuilding after file changes)
- ✅ Useful for **global** Claude Code behavior across all sessions

---

## Channel 3: MCP (Bidirectional State Communication)

### Purpose
Let Claude **control rstn's TUI** and **read rstn's state** (specs, tasks, feature context).

### Direction
`Claude Code ↔ rstn HTTP Server` (bidirectional)

### Architecture

rstn runs an **embedded HTTP server** using Axum that Claude calls via MCP (Model Context Protocol).

**Server Startup** (`main.rs`):
```rust
let mcp_state = Arc::new(Mutex::new(McpState::default()));
let (mcp_event_tx, _mcp_event_rx) = mpsc::channel(100);
let mcp_config = McpServerConfig::default(); // port: 0 = auto-assign
let mcp_handle = mcp_server::start_server(
    mcp_config,
    mcp_event_tx,
    mcp_state.clone()
).await?;

// Write config for Claude Code to discover
mcp_server::write_mcp_config(mcp_handle.port())?;
```

**MCP Config** (`~/.rstn/mcp-session.json`):
```json
{
  "mcpServers": {
    "rstn": {
      "type": "http",
      "url": "http://127.0.0.1:44832/mcp"
    }
  }
}
```

**Claude Discovery**:
```bash
claude -p "prompt" --mcp-config ~/.rstn/mcp-session.json
```

### Available MCP Tools

**File**: `crates/rstn/src/mcp_server.rs`

| Tool | Purpose | Parameters | Response |
|------|---------|------------|----------|
| `rstn_report_status` | Report status changes, ask for user input | `status`: "needs_input" \| "completed" \| "error"<br>`prompt`: Question for user<br>`message`: Status message | Blocks until user responds |
| `rstn_complete_task` | Mark task as complete | `task_id`: "T001"<br>`skip_validation`: bool | Success/error |
| `rstn_read_spec` | Read spec artifacts | `artifact`: "spec" \| "plan" \| "tasks" \| "checklist" \| "analysis" | Artifact content |
| `rstn_get_context` | Get current feature context | _(none)_ | Feature number, name, branch, spec_dir |

### Example Flow

**User presses 'p' (Prompt Claude)** and types: "Create a login feature"

#### 1. rstn spawns Claude
```rust
claude -p "Create a login feature" \
  --output-format stream-json \
  --mcp-config ~/.rstn/mcp-session.json \
  --allowedTools "Bash,Read,Write,Edit,Glob,Grep,Task"
```

#### 2. Claude gets feature context via MCP
```http
POST http://127.0.0.1:44832/mcp
Content-Type: application/json

{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "rstn_get_context"
  }
}
```

**rstn responds**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "feature_number": "042",
    "feature_name": "login-system",
    "branch": "042-login-system",
    "spec_dir": "/path/to/specs/042-login-system"
  }
}
```

#### 3. Claude reads spec via MCP
```http
POST http://127.0.0.1:44832/mcp

{
  "method": "tools/call",
  "params": {
    "name": "rstn_read_spec",
    "arguments": {
      "artifact": "spec"
    }
  }
}
```

**rstn responds** with spec.md content.

#### 4. Claude needs clarification
```http
POST http://127.0.0.1:44832/mcp

{
  "method": "tools/call",
  "params": {
    "name": "rstn_report_status",
    "arguments": {
      "status": "needs_input",
      "prompt": "Should login support OAuth or just username/password?"
    }
  }
}
```

**rstn**:
1. Shows InputDialog in TUI
2. **Blocks the HTTP request** (doesn't respond yet)
3. Waits for user to type answer
4. User types: "Google OAuth"
5. rstn sends HTTP response:
   ```json
   {"result": "User response: Google OAuth"}
   ```

#### 5. Claude streams response via stream-json
```json
{"type":"assistant","text":"I'll implement Google OAuth login..."}
```

### MCP HTTP Handler

**File**: `crates/rstn/src/mcp_server.rs`

```rust
/// MCP endpoint - handles all JSON-RPC requests
async fn mcp_handler(
    State(state): State<AppState>,
    Json(request): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let response = match request.method.as_str() {
        "initialize" => handle_initialize(&state, &request).await,
        "tools/list" => handle_tools_list(&request).await,
        "tools/call" => handle_tools_call(&state, &request).await,
        _ => JsonRpcResponse::error(request.id, -32601, "Method not found"),
    };
    Json(response)
}

async fn handle_report_status(
    state: &AppState,
    args: HashMap<String, Value>
) -> ToolResult {
    match status.as_str() {
        "needs_input" => {
            // Create oneshot channel for response
            let (tx, rx) = oneshot::channel();

            // Store sender and push event to TUI
            {
                let mut mcp_state = state.mcp_state.lock().await;
                mcp_state.input_response_tx = Some(tx);
                mcp_state.push_tui_event(Event::McpStatus {
                    status,
                    prompt,
                    message
                });
            }

            // Block HTTP request until user responds via TUI
            match rx.await {
                Ok(response) => ToolResult::text(&format!(
                    "User response: {}",
                    response
                )),
                Err(_) => ToolResult::error("Input request was cancelled"),
            }
        }
        _ => { /* handle completed/error */ }
    }
}
```

### When Used

- ✅ When Claude needs to **interact** with rstn (ask questions, mark tasks done)
- ✅ When Claude needs to **read** rstn's state (specs, tasks, feature info)
- ✅ **Replaces text-based parsing** (Feature 064 removed status blocks like `rscli-status`)

---

## Communication Model: Client/Server Analogy

### Traditional API Usage

```
┌──────────────┐                    ┌──────────────┐
│   Client     │ ──── "user" ────>  │   Claude     │
│  (Your App)  │                    │   (Server)   │
│              │ <── "assistant" ── │              │
└──────────────┘                    └──────────────┘
```

### rstn Model

```
┌──────────────────────────┐        ┌──────────────┐
│   Client Side            │        │   Server     │
│   (role: "user")         │        │              │
│                          │        │              │
│  ┌────────┐  ┌────────┐ │        │              │
│  │  User  │  │  rstn  │ │────────│    Claude    │
│  │ (Human)│  │  (TUI) │ │  JSON  │     Code     │
│  └───┬────┘  └───┬────┘ │        │              │
│      │           │       │        │              │
│      └───────────┘       │        │              │
│    Both send as          │        │              │
│    "role": "user"        │        │              │
│                          │        │              │
│  - Prompts               │        │ role:        │
│  - Tool results          │        │ "assistant"  │
│  - MCP responses         │        │              │
└──────────────────────────┘        └──────────────┘
```

**Key Insight**: rstn is a "super client" that:
1. Forwards human prompts (`"user"`)
2. Executes tools on behalf of Claude (`"user"` with tool results)
3. Handles MCP interactions (`"user"` with rstn_* tool results)

All of these are sent as `"role":"user"` because they're all happening on the **client side** of the Claude API.

---

## Summary Table

| Channel | Direction | Purpose | Data Format | When Used |
|---------|-----------|---------|-------------|-----------|
| **stream-json Output** | Claude → rstn | Stream Claude's response text | JSONL (stdout) | ✅ Always (real-time output) |
| **stream-json Input** | rstn → Claude | Send conversation history | JSONL (stdin) | ❌ Not used yet (future: task resumption) |
| **Hooks** | rstn ↔ Bash | Control permissions | Bash exit codes | ❌ Replaced by `--allowedTools` |
| **--allowedTools** | rstn → Claude | Grant tool permissions | CLI flag | ✅ Always (permission control) |
| **MCP** | Claude ↔ rstn | Bidirectional state access | JSON-RPC over HTTP | ✅ When Claude needs rstn's state |

---

## Current rstn Implementation

**File**: `crates/rstn/src/runners/cargo.rs:395-454`

```rust
pub async fn run_claude_command_streaming(
    command: &str,
    options: &ClaudeCliOptions,
    sender: Option<mpsc::Sender<Event>>,
) -> Result<ClaudeResult> {
    // 1. Find Claude binary
    let claude_path = crate::claude_discovery::ClaudeDiscovery::find_claude().await?;
    let mut cmd = Command::new(&claude_path);

    // 2. PERMISSIONS (--allowedTools replaces hooks)
    if !options.allowed_tools.is_empty() {
        let tools_str = options.allowed_tools.join(",");
        cmd.arg("--allowedTools").arg(&tools_str);
        tracing::debug!("Added --allowedTools: {}", tools_str);
    }

    // Legacy fallback
    if options.skip_permissions {
        cmd.arg("--dangerously-skip-permissions");
        tracing::warn!("Using skip_permissions (consider --allowedTools instead)");
    }

    // 3. SESSION MANAGEMENT
    if let Some(ref session) = options.session_id {
        cmd.arg("--resume").arg(session);
    } else if options.continue_session {
        cmd.arg("--continue");
    }

    // 4. STREAM-JSON OUTPUT
    cmd.arg("-p").arg(command);
    cmd.arg("--output-format").arg("stream-json");
    cmd.arg("--verbose");
    cmd.arg("--include-partial-messages");

    // 5. MCP CONFIG (points to rstn's HTTP server)
    if let Some(mcp_config_path) = crate::domain::paths::mcp_config_path() {
        if std::path::Path::new(&mcp_config_path).exists() {
            cmd.arg("--mcp-config").arg(&mcp_config_path);
        }
    }

    // 6. SPAWN AND PARSE STREAM
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take()?;
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    let mut result = ClaudeResult::default();

    while let Ok(Some(line)) = lines.next_line().await {
        // Parse JSONL and forward to TUI
        if let Ok(msg) = serde_json::from_str::<ClaudeStreamMessage>(&line) {
            if let Some(ref s) = sender {
                s.send(Event::ClaudeStream(msg))?;
            }
        }
    }

    Ok(result)
}
```

**All 3 methods work together**:
- `--allowedTools` = Permission control (replaces hooks)
- `--output-format stream-json` = Real-time text streaming
- `--mcp-config` = Bidirectional state access

---

## Future Enhancements

### Potential Use of stream-json Input

**Use Case 1: Task Resumption with Full Context**

Instead of spawning Claude multiple times:

```rust
// Current approach (multiple spawns):
run_claude("Implement T001").await?;
run_claude("Implement T002").await?;
run_claude("Implement T003").await?; // If this fails, context is lost

// With stream-json input (single spawn):
let conversation = vec![
    json!({"role":"user","content":"/speckit.implement"}),
    json!({"role":"assistant","content":"Starting T001..."}),
    json!({"role":"tool","content":"T001 completed"}),
    json!({"role":"assistant","content":"Starting T002..."}),
    json!({"role":"tool","content":"T002 completed"}),
    json!({"role":"assistant","content":"Starting T003..."}),
    json!({"role":"tool","content":"Error: Test failed"}),
    json!({"role":"user","content":"Fix the test error and continue"}),
];

run_claude_with_jsonl_input(&conversation).await?;
```

**Use Case 2: Error Recovery**

```rust
// First attempt failed
let error = run_claude("Implement feature").await.unwrap_err();

// Retry with error context
let conversation = vec![
    json!({"role":"user","content":"Implement feature"}),
    json!({"role":"assistant","content":"Building..."}),
    json!({"role":"tool","content":format!("Build failed: {}", error)}),
    json!({"role":"user","content":"Fix the build error"}),
];

run_claude_with_jsonl_input(&conversation).await?;
```

---

## References

- [Anthropic Messages API](https://platform.claude.com/docs/en/api/messages) - Official API specification
- [Claude Code CLI Reference](https://code.claude.com/docs/en/cli-reference) - CLI flags and options
- [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) - MCP specification
- Claude Code Issue [#5034](https://github.com/anthropics/claude-code/issues/5034) - stream-json input bug

---

## Related Documentation

- `kb/03-api-reference/mcp-tools.md` - Detailed MCP tool schemas
- `kb/03-api-reference/claude-code-cli-reference.md` - Complete CLI reference
- `kb/03-api-reference/claude-code-headless.md` - Headless mode patterns
- `kb/03-api-reference/claude-code-hooks.md` - Hook system documentation
- `CLAUDE.md` - MCP Architecture section (lines 250-450)
