---
title: "MCP Server Integration"
description: "Planned: Embedded MCP server for Claude integration"
category: roadmap
status: planned
last_updated: 2025-12-26
version: 3.0.0
---

# MCP Server Integration (Planned)

## Overview

Each worktree will have an embedded MCP (Model Context Protocol) server that provides project context to Claude Code.

---

## Planned Architecture

```
┌─────────────────────────────────────────────┐
│              rustation                       │
│  ┌────────────────────────────────────────┐ │
│  │ Worktree: feature/auth                 │ │
│  │  ┌──────────────────────────────────┐  │ │
│  │  │ MCP Server (HTTP)                │  │ │
│  │  │ Port: auto-assigned              │  │ │
│  │  │ Tools: project_context, etc.     │  │ │
│  │  └──────────────────────────────────┘  │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────┐
│            Claude Code CLI                   │
│  --mcp-config ~/.rstn/mcp-session.json      │
└─────────────────────────────────────────────┘
```

---

## Planned Tools

### 1. `project_context`
Return current project state:
- Open files
- Active worktree
- Recent changes
- Task outputs

### 2. `justfile_commands`
List available just commands for the project.

### 3. `docker_services`
Return Docker service status for development environment.

### 4. `git_status`
Return current git state (branch, changes, commits).

---

## MCP Config Format

```json
{
  "mcpServers": {
    "rstn": {
      "type": "http",
      "url": "http://localhost:8765"
    }
  }
}
```

---

## State Extensions

```rust
pub struct McpState {
    pub status: McpStatus,    // Stopped, Starting, Running, Error
    pub port: Option<u16>,
    pub config_path: Option<String>,
    pub error: Option<String>,
}

pub enum McpStatus {
    Stopped,
    Starting,
    Running,
    Error,
}
```

---

## Actions (Planned)

| Action | Description |
|--------|-------------|
| `StartMcpServer` | Start MCP server for active worktree |
| `StopMcpServer` | Stop MCP server |
| `SetMcpStatus` | Update server status |
| `SetMcpPort` | Set assigned port |
| `SetMcpError` | Set error message |

---

## Implementation Notes

### Technology Options

1. **FastAPI (Python)**: Quick to implement, subprocess
2. **Axum (Rust)**: Native, compiled into app
3. **Node.js**: JavaScript MCP SDK available

### Considerations

- Server must be lightweight
- One server per worktree
- Auto-shutdown when worktree closed
- Port conflict handling

---

## References

- [MCP Specification](https://modelcontextprotocol.io/)
- [Architecture Overview](../implemented/00-architecture.md)
