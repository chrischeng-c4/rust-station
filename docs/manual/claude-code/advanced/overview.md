---
title: "Claude Code Advanced Features - Overview"
description: "Feature matrix and navigation for advanced Claude Code integration"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
tags: [claude-code, advanced, overview]
weight: 1
aliases: ["/03-api-reference/claude-code-advanced-features.md"]
---

# Claude Code Advanced Features - Overview

This directory contains advanced Claude Code CLI features for rstn integration beyond basic `claude -p` usage.

## Feature Matrix

| Feature | Status | Document | Use Case |
|---------|--------|----------|----------|
| Permission Modes | 🟢 Implemented | [permission-modes.md](permission-modes.md) | Control tool execution (plan/auto/ask) |
| MCP Interaction | 🟢 Implemented | [mcp-interaction.md](mcp-interaction.md) | Mini TUI mode for CLI prompts |
| Multi-File Context | 🟢 Implemented | [multi-file-context.md](multi-file-context.md) | --context flag with JSONL |
| Stream Message | 🟢 Implemented | [stream-message.md](stream-message.md) | Extended ClaudeStreamMessage struct |
| Session Management | 🟢 Implemented | [session-management.md](session-management.md) | Dual-layer session tracking |
| Edit Approval | 🟢 Implemented | [edit-approval.md](edit-approval.md) | Diff preview before edits |
| Cancellation | 🟢 Implemented | [cancellation.md](cancellation.md) | Ctrl+C / Esc handling |
| Cost Tracking | 🟢 Implemented | [cost-tracking.md](cost-tracking.md) | Real-time cumulative cost |
| Error Handling | 🟢 Implemented | [error-handling.md](error-handling.md) | MCP error suggestions |
| UI Improvements | 🟢 Implemented | [ui-improvements.md](ui-improvements.md) | Progress indicators, history |

## Implementation Status

**✅ Phase 1 (High Priority) - COMPLETE**:
- ✅ Extended tool metadata parsing
- ✅ Permission Modes integration
- ✅ Dual-layer session management

**✅ Phase 2 (Medium Priority) - COMPLETE**:
- ✅ Multi-file context via `--context` flag
- ✅ Cancellation support

**✅ Phase 3 (Nice to Have) - COMPLETE**:
- ✅ Real-time cost tracking
- ✅ MCP interaction in CLI mode
- ✅ Edit approval with diff preview
- ✅ MCP error handling
- ✅ Additional UI improvements

## Quick Start

**New to advanced features?** Start with:
1. [Permission Modes](permission-modes.md) - Control tool execution
2. [Multi-File Context](multi-file-context.md) - Add context files
3. [Session Management](session-management.md) - Track LLM sessions

**Debugging?** See:
- [Error Handling](error-handling.md) - MCP error recovery
- [Cancellation](cancellation.md) - Stop running commands

## Related Documentation

- [Communication Channels](../communication.md) - rstn ↔ Claude Code channels
- [CLI Reference](../cli-reference.md) - Complete CLI flags
- [Headless Mode](../headless-mode.md) - Headless patterns
- [MCP Tools](../../mcp/tools.md) - MCP tool schemas
