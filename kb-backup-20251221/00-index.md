# Rustation Knowledge Base

**Last updated**: 2025-12-20

**🎯 rustation v2 - Fresh Start**: All v1 specs (001-065) archived. Starting clean with state-first architecture.

Welcome to the rustation knowledge base! This is your central hub for understanding the rustation project architecture, APIs, workflows, and development practices.

---

## 🆕 What's New in v2

**Project Restart** (2025-12-19): Rustation has been restarted with a clean slate approach:
- **State-first architecture**: All state must be JSON/YAML serializable (Feature 079)
- **Simplified design**: Focus on core workflows, avoid complexity
- **Clean separation**: CLI/TUI share business logic, different interfaces
- **Testability**: State-based testing, not UI testing

**v1 archived**: All previous specs and design docs moved to [`99-archive/`](99-archive/) for historical reference.

---

## 🚀 New to rustation?

**Get started in 10 minutes**:

1. **[Installation Guide](01-getting-started/installation.md)** - Install rstn on your system
2. **[Quick Start](01-getting-started/quick-start.md)** - Run your first session
3. **[Core Concepts](01-getting-started/concepts.md)** - Understand state-first architecture

**Already installed?** Jump to [Quick Start](01-getting-started/quick-start.md)

---

## Quick Links

**For Contributors**:
- [🎯 State-First Architecture](02-architecture/state-first.md) - **Core principle**: State as JSON/YAML + State Persistence guide
- [🎯 Prompt Claude Architecture](04-development/prompt-command-architecture.md) - **Workflow with state diagram, flow chart, sequence**
- [Core Principles](02-architecture/core-principles.md) - v2 architectural pillars
- [SDD Workflow Guide](04-development/sdd-workflow.md) - When to use full vs lightweight SDD
- [MCP Tools Reference](03-api-reference/mcp-tools.md) - Available MCP tools

**For Users**:
- [Installation Guide](01-getting-started/installation.md) - Install rstn
- [Quick Start](01-getting-started/quick-start.md) - First steps
- [Core Concepts](01-getting-started/concepts.md) - Understand v2
- [MCP Tools Reference](03-api-reference/mcp-tools.md) - Available MCP tools

**For Claude Code Integration**:
- [Communication Channels](03-api-reference/claude-code-communication.md) - **🎯 How rstn ↔ Claude Code communicate** (stream-json, hooks, MCP)
- [Advanced Features](03-api-reference/claude-code-advanced-features.md) - **🎯 Advanced CLI integration** (permission modes, multi-file context, session management)
- [MCP Tools Reference](03-api-reference/mcp-tools.md) - MCP tool schemas
- [Claude CLI Reference](03-api-reference/claude-cli.md) - Condensed CLI flags
- [Claude CLI (Full)](03-api-reference/claude-cli-full.md) - Complete reference
- [Claude Headless Mode](03-api-reference/claude-headless.md) - Headless patterns

**v1 Archive** (historical reference only):
- [Archive README](99-archive/) - What's archived and why
- [v2 UX Redesign Vision](99-archive/v2-ux-redesign-state-machine.md) - State machine-based workflow architecture (未實作)
- v1 complexity analysis, design docs (not for implementation)

---

## Documentation Structure

```
kb/
├── 00-index.md (this file)           - START HERE
│
├── 01-getting-started/               - User Onboarding (NEW)
│   ├── installation.md               - Install rstn
│   ├── quick-start.md                - First 10 minutes
│   └── concepts.md                   - Core concepts
│
├── 02-architecture/                  - Core Principles (v2)
│   ├── state-first.md                - **🎯 Core principle**: State as JSON/YAML + State Persistence (7 sections)
│   └── core-principles.md            - v2 architectural pillars
│
├── 03-api-reference/                 - API Documentation
│   ├── claude-code-communication.md  - **🎯 rstn ↔ Claude Code channels** (stream-json, hooks, MCP)
│   ├── mcp-tools.md                  - MCP tool schemas
│   ├── claude-cli.md                 - Condensed CLI reference
│   ├── claude-cli-full.md            - Complete CLI reference
│   └── claude-headless.md            - Headless mode patterns
│
├── 04-development/                   - Development Workflow (v2)
│   ├── prompt-command-architecture.md - **🎯 Prompt Claude workflow** (state diagram, flow, sequence)
│   ├── sdd-workflow.md               - Full vs lightweight SDD + state testing
│   ├── contribution-guide.md         - Setup, workflow, PR requirements
│   ├── testing-guide.md              - State-first testing patterns
│   └── debugging.md                  - State inspection, logs, issues
│
└── 99-archive/                       - v1 Archive (historical)
    ├── README.md                     - What's archived and why
    ├── v2-ux-redesign-state-machine.md - **🎯 v2 UX Redesign Vision** (未實作)
    ├── v1-analysis/                  - v1 complexity analysis
    │   ├── technical-debt.md         - v1 God Classes, state explosion
    │   └── architecture-overview.md  - v1 system architecture
    └── v1-designs/                   - Unimplemented v1 designs
        ├── worktree-view-redesign.md
        ├── worktree-state-machine.md
        ├── rstn-integration-flow.md
        └── logging-specification.md
```

**Note**: v2 focuses on core principles. Additional docs created only when needed.

---

## Getting Started

### I'm a new contributor (v2)

1. Read [State-First Architecture](02-architecture/state-first.md) - **The v2 core principle**
2. Read [Core Principles](02-architecture/core-principles.md) - v2 architectural pillars
3. Read [SDD Workflow Guide](04-development/sdd-workflow.md) - Development process
4. Look at recent commits to see v2 patterns

### I'm debugging an issue (v2)

1. Check logs at `~/.rustation/logs/rstn.log` or `~/.rstn/logs/`
2. Use `--save-state` / `--load-state` to reproduce issues (see [State Persistence](02-architecture/state-first.md#state-persistence))
3. Review [State Persistence guide](02-architecture/state-first.md#state-persistence) for state + logs debugging techniques
4. Review [MCP Tools Reference](03-api-reference/mcp-tools.md) if MCP-related
5. Review recent code in `crates/rstn/src/`

### I'm implementing a feature (v2)

1. **State-first**: Define state structs FIRST (must be serializable)
2. Write state tests BEFORE implementation (MANDATORY)
3. Use [SDD Workflow](04-development/sdd-workflow.md) for planning
4. For SDD: Run `/speckit.specify` → `/speckit.plan` → `/speckit.tasks`
5. Ensure all state tests pass (enforced in code review)

### I want to understand the codebase (v2)

1. Start with [State-First Architecture](02-architecture/state-first.md) - Core principle
2. Read [Core Principles](02-architecture/core-principles.md) - Full architectural overview
3. Read `CLAUDE.md` for CLI/TUI architecture pattern
4. Explore `crates/rstn/src/tui/state/` for state management
5. Check `crates/rstn/tests/` for testing patterns
6. (Optional) Read [v1 Archive](99-archive/) for historical context

---

## Key Technologies

- **Rust 1.75+** (edition 2021)
- **ratatui 0.29+** - TUI framework
- **crossterm 0.28** - Terminal I/O
- **tokio** - Async runtime
- **axum** - HTTP server (MCP)
- **serde/serde_json** - Serialization
- **tracing** - Logging

---

## Project Status (as of 2025-12-19)

**🆕 rustation v2 - Fresh Start**:
- Status: **Active restart** (v1 archived 2025-12-19)
- Core principle: **State-first architecture** (Feature 079)
- Approach: Clean slate, avoid v1 complexity
- All v1 specs (001-065) archived to `specs/archive/`

**rush shell**:
- Status: Maintenance mode (unchanged)
- Completed: Phase 1-6 (35 features)
- Remaining: Phase 7-8 (6 features, then suspend)

**rstn TUI** (v2):
- Status: Rebuilding from scratch
- Current focus: State management, testing infrastructure
- Philosophy: Simple, observable, testable
- No v1 technical debt to carry forward

---

## Key Metrics (v2)

**Codebase** (post-restart):
- v1 archived, v2 metrics TBD
- Target: Keep modules small (<500 lines)
- Target: State structs simple (<15 fields)

**Documentation**:
- KB docs: 7 active files (~3,700 lines)
- v1 archive: 6 files (~3,556 lines)
- Specs: v1 archived (001-065), v2 TBD

**Process**:
- State-first testing: **Mandatory** for all features
- Test coverage target: 70%+ (state + integration)
- SDD workflow: Continued from v1

---

## Related Documentation

- [CLAUDE.md](../CLAUDE.md) - Development workflow instructions
- [ROADMAP.md](../ROADMAP.md) - rush shell roadmap
- [specs/](../specs/) - Feature specifications
- [.claude/agents/](../.claude/agents/) - Subagent definitions
- [.claude/commands/](../.claude/commands/) - Slash commands

---

## Contributing

Questions or suggestions for the knowledge base?
- Open an issue in the GitHub repository
- Update this documentation and submit a PR
- Ask in the team chat

---

## Changelog

- 2025-12-21: **Claude Code Advanced Features Implementation** - Completed Phase 1-3 implementation
  - **Phase 1 (High Priority) - COMPLETE**:
    - ✅ Extended ClaudeStreamMessage Structure ([claude_stream.rs](../crates/rstn/src/tui/claude_stream.rs#L38-L65))
      - Added tool metadata fields: `id`, `name`, `input`, `tool_use_id`, `content`, `is_error`
      - Enhanced tool display to show actual tool names and parameters
      - Tests for tool_use and tool_result parsing
    - ✅ Permission Modes Integration ([cargo.rs](../crates/rstn/src/runners/cargo.rs#L40-L80))
      - `PermissionMode` enum: Plan, Auto, Ask
      - Integrated into `ClaudeCliOptions` and command building
      - Ready for TUI/CLI usage
    - ✅ Dual-Layer Session Management ([session_manager.rs](../crates/rstn/src/session_manager.rs))
      - `RstnSession` (workflow-level): Prompt, Specify, Plan, Tasks, Implement
      - `ClaudeSession` (LLM-level): UUID, purpose, turns, cost, status
      - SQLite persistence with foreign key relationships
      - Session history and aggregation
  - **Phase 2 (Medium Priority) - COMPLETE**:
    - ✅ Multi-File Context via --context flag ([cli.rs](../crates/rstn/src/cli.rs#L60-L65), [cargo.rs](../crates/rstn/src/runners/cargo.rs#L110-L140))
      - `--context file1.rs,file2.rs` CLI argument
      - JSONL builder for stream-json input format
      - Stdin piping to Claude subprocess
    - ✅ Cancellation Support ([session_output.rs](../crates/rstn/src/tui/views/session_output.rs#L338-L342))
      - CLI: Ctrl+C → SIGINT propagation (OS-level, automatic)
      - TUI: Esc → Close view (subprocess continues in background)
      - Verified existing implementation
  - **Phase 3 (Nice to Have) - PARTIAL**:
    - ✅ Real-time Cost Tracking ([session_output.rs](../crates/rstn/src/tui/views/session_output.rs#L28-L31))
      - `cumulative_cost_usd` field with real-time updates
      - Budget warning threshold (default $0.50)
      - Status line display with ⚠️ indicator
      - Comprehensive tests (4 test cases)
  - **Build Status**: ✅ 287 tests (286 passed), ✅ Clippy clean, ✅ Build success (4.75s)
  - **Next Steps**: Remaining Phase 3 features (MCP interaction, Edit approval, Error handling, UI improvements)
- 2025-12-21: **Claude Code Advanced Features** - Created comprehensive integration guide
  - Created claude-code-advanced-features.md - 10 advanced features and integration patterns
  - **Permission Modes**: `--permission-mode plan|auto|ask` with rstn integration strategy
  - **MCP Interaction in CLI**: 3 solutions (Mini TUI mode, permission-prompt-tool, Hybrid mode)
  - **Multi-File Context**: `--input-format stream-json` with `--context` flag implementation
  - **Extended ClaudeStreamMessage**: Added tool metadata fields (id, name, input, tool_use_id)
  - **Dual-Layer Sessions**: Claude session (UUID) + rstn session (workflow) management
  - **Edit Approval**: Diff preview via stream-json tool_use interception
  - **Cancellation Support**: Ctrl+C (CLI) and Esc (TUI) with graceful shutdown
  - **Real-time Cost Tracking**: Token usage display, cumulative cost, budget warnings
  - **MCP Error Handling**: Structured errors with suggestions
  - **Additional Improvements**: Tool progress, session history UI, diff preview widget
  - **Status**: Design specification, ready for phased implementation
- 2025-12-20: **v2 UX Redesign Vision** - Documented state machine-based workflow architecture (未實作)
  - Created v2-ux-redesign-state-machine.md - Comprehensive UX redesign vision
  - **User feedback**: 3 core problems (fixed tabs, redundant Log column, missing state machine)
  - **Proposed solution**: n8n-style workflows (command = workflow, LLM only at specific nodes)
  - **New layout**: 2-column (Commands 20% | Content 80%), remove tabs, styled output
  - **State machine**: WorkflowState enum, transition validation, single LLM entry point
  - **Implementation plan**: 5 phases (state machine core → remove Log → remove tabs → progress → cleanup)
  - **Challenges**: Large refactor (3000+ lines), conceptual shift, preserving functionality
  - **Status**: Vision document only, not implemented (記錄供未來參考)
- 2025-12-20: **Prompt Claude Workflow Architecture** - Created comprehensive workflow documentation
  - Created prompt-command-architecture.md - Complete workflow with diagrams
  - **State diagram**: 9 states (Idle → InputPending → Validating → Spawning → Streaming → WaitingForInput → Completing → Completed/Error)
  - **Flow chart**: Complete execution flow with decision points (CLI vs TUI, validation, MCP interaction)
  - **Sequence diagrams**: TUI mode (with MCP) + CLI mode (direct stdout)
  - **State management**: State fields, transitions table, invariants
  - **Implementation details**: ClaudeCliOptions, JSONL parsing, MCP needs_input flow, image paste support
  - **Testing strategy**: CLI (simple) vs TUI (complex) approaches
  - **Common issues**: Debugging guide with solutions
  - Updated index with Prompt Claude Architecture in Quick Links
- 2025-12-20: **Communication Channels Documentation** - Created comprehensive reference for rstn ↔ Claude Code communication
  - Created claude-code-communication.md - Complete guide to 3 communication channels
  - **stream-json I/O**: Output (always used) + Input (future: task resumption)
  - **Hooks**: Permission control (deprecated in favor of --allowedTools)
  - **MCP**: Bidirectional state access (rstn_* tools)
  - **Role field specification**: Only "user" | "assistant" (no "tool" role)
  - **Client/Server model**: rstn + user = client, Claude Code = server
  - Includes code examples, message formats, use cases, future enhancements
  - All references updated in index
- 2025-12-19: **State Persistence Documentation** - Added comprehensive State Persistence section to state-first.md
  - 7 subsections (608 lines): file locations, schema, initialization, error handling, validation, state-to-UI flow, logging management
  - **Key addition**: Logging Management section explaining State + Logs = Observability principle
  - Two-tier logging architecture: file logging (tracing) + TUI event buffer (circular buffer, 8 categories)
  - Real debugging scenarios showing state snapshots + event logs enable complete observability
  - All code examples include file references for easy navigation
- 2025-12-19: **Phase 4 (Developer Documentation)** - Created comprehensive developer guides
  - Created contribution-guide.md - Setup, workflow, state testing MANDATORY, PR requirements
  - Created testing-guide.md - State-first testing (round-trip, transitions, invariants)
  - Created debugging.md - State inspection, logs, common issues, advanced techniques
  - Updated CLAUDE.md - Complete knowledge-base section rewrite for v2
  - **KB REORGANIZATION COMPLETE** - All 4 phases finished
- 2025-12-19: **Phase 3 (User Documentation)** - Created getting-started guides for new users
  - Created installation.md - Complete installation guide with prerequisites, troubleshooting
  - Created quick-start.md - First 10 minutes walkthrough with navigation, common actions
  - Created concepts.md - Core concepts (state-first, CLI/TUI, SDD, MCP, terminology)
  - Updated index with "New to rustation?" section
  - Updated Quick Links to include all getting-started guides
  - Updated Documentation Structure to include 01-getting-started/
- 2025-12-19: **Phase 2 (Core Foundation)** - Established state-first as north star
  - **CRITICAL FIX**: Corrected MCP protocol in mcp-tools.md (SSE→HTTP, dynamic port)
  - Enhanced state-serializability.md → state-first.md with TL;DR, Mermaid diagram, common pitfalls
  - Created core-principles.md - v2 architectural overview
  - Reorganized API reference (02→03, consistent naming)
  - Updated SDD workflow with mandatory state testing requirements
  - All cross-references updated
- 2025-12-19: **Phase 1 (Clean Up)** - Archived all v1 content (6 docs, ~3,556 lines) to `99-archive/`
  - Removed broken references from index
  - Updated Quick Links to reflect v2 focus
  - Simplified documentation structure
  - Added v2 guidance throughout
- 2025-12-18: Added state-serializability.md - Core architecture principle (Feature 079)
- 2025-12-18: Initial KB structure created
