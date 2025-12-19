# Plan: MCP Testing & Documentation

**Feature**: 065-mcp-testing-docs
**Created**: 2024-12-17
**Depends On**: 064-mcp-migration-cleanup

## Architecture Overview

```
Testing Layers:
┌─────────────────────────────────────────┐
│         Integration Tests               │
│  (full tool → event → state flow)       │
├─────────────────────────────────────────┤
│           Unit Tests                    │
│  (server, tools, handlers)              │
└─────────────────────────────────────────┘

Documentation:
┌─────────────────────────────────────────┐
│          CLAUDE.md                      │
│  (MCP architecture overview)            │
├─────────────────────────────────────────┤
│        docs/mcp-tools.md                │
│  (tool schema reference)                │
└─────────────────────────────────────────┘
```

## Implementation Approach

### Phase 1: Unit Tests
- Server startup/shutdown tests
- Tool registration tests
- Individual tool handler tests
- Event dispatch tests

### Phase 2: Integration Tests
- Full status flow test
- Task completion flow test
- Error handling tests

### Phase 3: Documentation
- Update CLAUDE.md with MCP section
- Create tool schema reference
- Add troubleshooting guide

## Key Components

### Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_startup() { ... }

    #[tokio::test]
    async fn test_tool_registration() { ... }

    #[tokio::test]
    async fn test_report_status_handler() { ... }

    #[tokio::test]
    async fn test_read_spec_handler() { ... }

    #[tokio::test]
    async fn test_get_context_handler() { ... }

    #[tokio::test]
    async fn test_complete_task_handler() { ... }
}
```

### Integration Test Structure

```rust
// tests/mcp_integration_test.rs
#[tokio::test]
async fn test_full_status_flow() { ... }

#[tokio::test]
async fn test_task_completion_flow() { ... }

#[tokio::test]
async fn test_error_recovery() { ... }
```

### Documentation Structure

```
CLAUDE.md
├── MCP Architecture section
│   ├── Dual-channel overview
│   ├── Display vs Control channels
│   └── Tool usage examples

docs/mcp-tools.md
├── rstn_report_status schema
├── rstn_read_spec schema
├── rstn_get_context schema
└── rstn_complete_task schema
```

## Files to Create/Modify

| File | Changes |
|------|---------|
| `mcp_server.rs` | Add #[cfg(test)] module |
| `tests/mcp_integration_test.rs` | New - integration tests |
| `CLAUDE.md` | Add MCP architecture section |
| `docs/mcp-tools.md` | New - tool reference |

## Estimated Complexity

~300 lines tests, ~200 lines docs
