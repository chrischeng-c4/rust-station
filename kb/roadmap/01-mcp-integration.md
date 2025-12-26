---
title: "Native MCP Server"
description: "Spec: Embedded Rust MCP Server for AI Context"
category: roadmap
status: planned
version: 1.1.0
---

# Feature Spec: Native MCP Server

## 1. Overview

**Goal**: Embed a Model Context Protocol (MCP) server within the Rust backend to expose project-specific context to AI clients (Claude Desktop, Claude Code, or our own Chat UI).
**Core Value**: Allows AI agents to "see" and "act" within the worktree safely and accurately.

## 2. Architecture

### Server Model
- **Cardinality**: One MCP server instance per **Worktree**.
- **Transport**: SSE (Server-Sent Events) over HTTP.
- **Port**: Auto-assigned (e.g., 3000, 3001...) or configured.

### State Topology
```rust
pub struct WorktreeState {
    // ...
    pub mcp_server: McpState,
}

pub struct McpState {
    pub status: ServerStatus, // Stopped, Running(port), Error
    pub clients: usize,       // Connection count
}
```

## 3. Tools Specification

The server will expose the following MCP Tools:

### `read_file`
- **Args**: `path: string`
- **Behavior**: Reads file content.
- **Security**: Sandboxed to worktree root. Deny absolute paths outside root.

### `list_directory`
- **Args**: `path: string`
- **Behavior**: Lists files/folders. respecting `.gitignore`.

### `get_project_context`
- **Args**: None
- **Behavior**: Returns high-level summary (open tabs, active tasks, docker status).

### `run_just_task`
- **Args**: `task_name: string`
- **Behavior**: Executes a Just task and returns output.

## 4. Implementation Details

### Tech Stack
- **Library**: `axum` (for HTTP/SSE) or `mcp-sdk-rs` (if available/mature).
- **Async**: Runs in background `tokio` task.

### Lifecycle
1. **Start**: When worktree is opened (or manual toggle).
2. **Stop**: When worktree is closed.
3. **Restart**: On configuration change.

## 5. Actions & API

| Action | Payload | Description |
|--------|---------|-------------|
| `StartMcpServer` | `{ worktree_id }` | Spawns server task |
| `StopMcpServer` | `{ worktree_id }` | Aborts server task |
| `SetMcpStatus` | `{ status, port }` | Backend -> Frontend update |

## 6. Testing Strategy

- **Integration**:
  - Start server.
  - Use `curl` or MCP client to call tools.
  - Verify JSON-RPC responses.
- **Security**:
  - Attempt `read_file("../../../secret")` -> Expect Error.