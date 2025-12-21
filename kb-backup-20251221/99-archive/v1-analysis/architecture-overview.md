# System Architecture Overview

**Last updated**: 2025-12-18

This document provides a high-level overview of the rustation project architecture, including both the rstn TUI and rush shell components.

---

## Project Structure

The rustation project is a Rust workspace containing two main components:

```
rustation/
├── crates/
│   ├── rush/          - Interactive shell implementation
│   └── rstn/          - TUI for specification-driven development
├── specs/             - Feature specifications (SDD workflow)
├── docs/              - Documentation
├── .claude/           - Claude Code integration (agents, commands, hooks)
└── Cargo.toml         - Workspace manifest
```

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Rustation Project                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────────┐          ┌──────────────────────┐   │
│  │    rush (Shell)      │          │    rstn (TUI)        │   │
│  │                      │          │                      │   │
│  │  ┌────────────────┐ │          │  ┌────────────────┐ │   │
│  │  │  Parser        │ │          │  │  TUI Layer     │ │   │
│  │  │  (Lexer+AST)   │ │          │  │  (ratatui)     │ │   │
│  │  └────────────────┘ │          │  └────────────────┘ │   │
│  │          │           │          │          │          │   │
│  │          ▼           │          │          ▼          │   │
│  │  ┌────────────────┐ │          │  ┌────────────────┐ │   │
│  │  │  Executor      │ │          │  │  Domain Layer  │ │   │
│  │  │  (Commands)    │ │          │  │  (Spec, Git)   │ │   │
│  │  └────────────────┘ │          │  └────────────────┘ │   │
│  │          │           │          │          │          │   │
│  │          ▼           │          │          ▼          │   │
│  │  ┌────────────────┐ │          │  ┌────────────────┐ │   │
│  │  │  Builtins      │ │          │  │  MCP Server    │ │   │
│  │  │  (cd,export,..)│ │          │  │  (Axum HTTP)   │ │   │
│  │  └────────────────┘ │          │  └────────────────┘ │   │
│  └──────────────────────┘          └──────────────────────┘   │
│             │                                    │             │
│             └──────────────┬───────────────────┘             │
│                            │                                  │
│                            ▼                                  │
│                  ┌──────────────────┐                         │
│                  │  Claude Code CLI │                         │
│                  │  (Subprocess)    │                         │
│                  └──────────────────┘                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. rush - Interactive Shell

**Purpose**: Rust-based interactive shell with modern features

**Status**: Maintenance mode (Phase 1-6 complete, 35 features)

**Key Modules**:
- `parser/` - Lexer and AST builder
- `executor/` - Command execution engine
- `builtins/` - Built-in commands (cd, export, alias, etc.)
- `completion/` - Tab completion engine
- `history/` - Command history management

**Architecture Pattern**: Traditional shell pipeline
```
Input → Lexer → Parser → AST → Executor → Output
```

**Tech Stack**:
- reedline 0.26+ (line editing)
- nushell_protocol (command protocol)
- Rust 1.75+ (edition 2021)

**Documentation**: See [rush Shell Architecture](rush-shell-architecture.md)

---

### 2. rstn - Specification-Driven Development TUI

**Purpose**: Terminal UI for managing feature specs, plans, and implementation

**Status**: Active development (refactoring in progress)

**Key Modules**:
- `tui/` - Terminal UI layer (ratatui)
  - `app.rs` - Main application (3,404 lines, needs refactoring)
  - `views/` - View implementations (WorktreeView, Dashboard, Settings)
  - `widgets/` - Reusable UI components
- `domain/` - Business logic layer
  - `git/` - Git operations (commit, worktree, security scanning)
  - `specify/` - Spec generation
  - `clarify/` - Spec clarification
  - `plan/` - Plan generation
- `mcp_server.rs` - MCP server for Claude Code integration (882 lines)
- `runners/` - Command runners (cargo, bash, python, Claude CLI)

**Architecture Pattern**: Event-driven TUI with embedded HTTP server
```
User Input → TUI Event Loop → Domain Layer → External Tools
                    ↕                            ↕
              MCP Server ←─────── Claude Code (HTTP)
```

**Tech Stack**:
- ratatui 0.29+ (TUI framework)
- crossterm 0.28 (terminal I/O)
- axum (HTTP server for MCP)
- tokio (async runtime)
- tracing (logging)

**Documentation**: See [rstn TUI Architecture](rstn-tui-architecture.md)

---

### 3. MCP Integration Layer

**Purpose**: Communication bridge between rstn TUI and Claude Code

**Protocol**: Model Context Protocol (JSON-RPC 2.0 over HTTP)

**Architecture**:
```
┌─────────────────────────────────────────────────────┐
│                 rstn Process                        │
│  ┌──────────────┐           ┌──────────────┐       │
│  │  TUI Loop    │◄─────────►│ Axum Server  │       │
│  │              │  mpsc     │ (port 0)     │       │
│  └──────────────┘  channel  └──────────────┘       │
│                                   ▲                 │
└───────────────────────────────────┼─────────────────┘
                                    │ HTTP POST /mcp
                                    │
                       ┌────────────▼────────────┐
                       │  Claude Code (Subprocess)│
                       │  --mcp-config ~/.rstn/  │
                       └─────────────────────────┘
```

**MCP Tools**:
1. `rstn_report_status` - Report task status changes
2. `rstn_read_spec` - Read spec artifacts (spec, plan, tasks)
3. `rstn_get_context` - Get feature context metadata
4. `rstn_complete_task` - Mark task complete

**Configuration**: Automatically generated at `~/.rstn/mcp-session.json`

**Documentation**: See [MCP Integration](mcp-integration.md)

---

## Data Flow

### Specification-Driven Development Flow

```
1. User Request
   │
   ▼
2. rstn TUI (WorktreeView)
   │
   ├─► /speckit.specify → Generate spec.md
   │   └─► Claude Code (via MCP) → AI-generated spec
   │
   ├─► /speckit.plan → Generate plan.md
   │   └─► Claude Code (via MCP) → AI-generated plan
   │
   ├─► /speckit.tasks → Generate tasks.md
   │   └─► Claude Code (via MCP) → AI-generated tasks
   │
   └─► /speckit.implement → Execute tasks
       └─► rstn runners → cargo build/test, git commit
```

### MCP Communication Flow

```
1. Claude Code subprocess calls MCP tool
   │
   ▼
2. HTTP POST /mcp → rstn Axum server
   │
   ▼
3. Tool handler (e.g., handle_report_status)
   │
   ├─► If needs_input: Create oneshot channel
   │   ├─► Push event to TUI via mpsc
   │   ├─► TUI shows InputDialog
   │   ├─► User enters input
   │   ├─► TUI sends response via oneshot
   │   └─► HTTP response with user input
   │
   └─► If completed/error: Update TUI state
       └─► HTTP response with result
```

### Command Execution Flow

```
1. User selects command in TUI
   │
   ▼
2. CommandRunner spawns tokio task
   │
   ├─► cargo build/test → Stream output to TUI
   ├─► bash script → Capture stdout/stderr
   ├─► Claude Code CLI → Parse streaming JSON
   └─► git commit → Security scan + commit
   │
   ▼
3. Output displayed in WorktreeView
   │
   └─► LogBuffer + FileChangeTracker
```

---

## Technology Stack

### Core Languages & Frameworks
- **Rust 1.75+** (edition 2021)
- **tokio** - Async runtime
- **serde/serde_json** - Serialization

### TUI (rstn)
- **ratatui 0.29+** - Terminal UI framework
- **crossterm 0.28** - Terminal I/O and event handling
- **axum** - HTTP server for MCP
- **tracing** - Structured logging

### Shell (rush)
- **reedline 0.26+** - Line editing and history
- **nushell_protocol** - Command protocol

### Development Tools
- **Claude Code** - AI-powered development assistant
- **Model Context Protocol (MCP)** - Structured AI communication
- **cargo** - Build system and package manager

---

## Directory Structure

```
rustation/
├── crates/
│   ├── rush/
│   │   ├── src/
│   │   │   ├── parser/          - Lexer, parser, AST
│   │   │   ├── executor/        - Command execution
│   │   │   ├── builtins/        - Built-in commands
│   │   │   ├── completion/      - Tab completion
│   │   │   └── main.rs          - Shell entry point
│   │   ├── tests/               - Integration tests
│   │   └── Cargo.toml
│   └── rstn/
│       ├── src/
│       │   ├── tui/             - Terminal UI
│       │   │   ├── app.rs       - Main app (3,404 LOC)
│       │   │   ├── views/       - View implementations
│       │   │   │   ├── worktree/  - Worktree view (4,118 LOC)
│       │   │   │   ├── dashboard.rs
│       │   │   │   ├── settings.rs
│       │   │   │   └── spec.rs
│       │   │   └── widgets/     - Reusable widgets
│       │   ├── domain/          - Business logic
│       │   │   ├── git/         - Git operations
│       │   │   ├── specify/     - Spec generation
│       │   │   ├── clarify/     - Clarification
│       │   │   └── plan/        - Planning
│       │   ├── runners/         - Command runners
│       │   ├── mcp_server.rs    - MCP HTTP server (882 LOC)
│       │   └── main.rs          - TUI entry point
│       └── Cargo.toml
├── specs/                       - Feature specifications
│   ├── features.json            - Feature catalog
│   └── {NNN}-{name}/            - Individual features
│       ├── spec.md              - Requirements
│       ├── plan.md              - Architecture
│       └── tasks.md             - Task breakdown
├── docs/
│   ├── kb/                      - Knowledge base (this file)
│   └── mcp-tools.md             - MCP tool reference
├── .claude/
│   ├── agents/                  - Subagent definitions
│   ├── commands/                - Slash commands
│   └── docs/                    - Claude Code integration docs
├── CLAUDE.md                    - Development workflow
├── ROADMAP.md                   - Project roadmap
└── Cargo.toml                   - Workspace manifest
```

---

## Communication Protocols

### 1. Text-Based Protocol (Legacy, being phased out)

**Pattern**: Embedded status blocks in command output
```
@@@ RSCLI_PROTOCOL_START @@@
{"status": "needs_input", "prompt": "Describe the feature"}
@@@ RSCLI_PROTOCOL_END @@@
```

**Status**: Being removed in Feature 064

---

### 2. MCP Tools (Current, recommended)

**Protocol**: JSON-RPC 2.0 over HTTP

**Example Tool Call**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "rstn_report_status",
    "arguments": {
      "status": "needs_input",
      "prompt": "Describe the feature"
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{"type": "text", "text": "User response: Blue feature"}]
  }
}
```

---

### 3. Event Channel (Internal)

**Pattern**: mpsc channel between TUI and MCP server

**Event Types**:
```rust
pub enum Event {
    McpStatus { status: String, prompt: String },
    TaskCompleted { task_id: String },
    InputRequired { phase: String },
    OutputLine { category: LogCategory, message: String },
    // ... 8+ variants
}
```

---

## Deployment & Runtime

### Development Mode
```bash
# Build and run rstn TUI
cargo run -p rstn

# Build and run rush shell
cargo run -p rush
```

### Installation
```bash
# Install both binaries
cargo install --path crates/rstn
cargo install --path crates/rush

# Or use just (if configured)
just install-dev
```

### Configuration Files
- `~/.rustation/logs/` - Log files
- `~/.rstn/mcp-session.json` - MCP server config (auto-generated)
- `.specify/templates/` - Spec templates
- `.claude/` - Claude Code integration configs

---

## Key Design Decisions

### Why Rust?
- Memory safety without GC
- Excellent async support (tokio)
- Rich ecosystem (ratatui, axum, serde)
- Zero-cost abstractions

### Why ratatui?
- Immediate mode rendering (simple mental model)
- Cross-platform terminal support
- Active community, good documentation
- Widget-based composition

### Why MCP over Text Parsing?
- Structured communication (JSON-RPC)
- Tool discovery and validation
- Better error handling
- Forward-compatible protocol

### Why Embedded HTTP Server?
- Claude Code recommends HTTP over SSE
- Port 0 (auto-assign) avoids conflicts
- Async HTTP (Axum) integrates with tokio
- Easy to test and debug

---

## Performance Characteristics

### rstn TUI
- **Startup time**: <100ms
- **Event loop**: 60 Hz (crossterm polling)
- **MCP response time**: <50ms (local HTTP)
- **Memory usage**: ~20MB (idle), ~50MB (running command)

### rush Shell
- **Startup time**: <50ms
- **Command execution**: Varies by command
- **Memory usage**: ~10MB (idle)

---

## Known Issues & Technical Debt

See [Technical Debt Analysis](../03-complexity-analysis/technical-debt.md) for detailed breakdown.

**Summary**:
- God Classes: App (3,404 LOC), WorktreeView (4,118 LOC)
- State explosion: 54+ mutable fields in WorktreeView
- Tight coupling: Domain logic in TUI layer
- Error handling: 308 unwrap/panic sites
- Testing gaps: ~40% coverage

**Plan**: 5-6 month refactoring roadmap in progress

---

## Related Documents

- [rstn TUI Architecture](rstn-tui-architecture.md) - Detailed TUI design
- [rush Shell Architecture](rush-shell-architecture.md) - Shell internals
- [MCP Integration](mcp-integration.md) - MCP protocol details
- [Technical Debt](../03-complexity-analysis/technical-debt.md) - Current issues
- [SDD Workflow](../04-sdd-workflow/when-to-use-which.md) - Development process

---

## Changelog

- 2025-12-18: Initial system overview created
