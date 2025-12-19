# Plan: MCP Resource Tools

**Feature**: 062-mcp-resource-tools
**Created**: 2024-12-17
**Depends On**: 061-mcp-status-tool

## Architecture Overview

```
Claude Code                    rstn MCP Server
    │                               │
    │ tools/call rstn_read_spec     │
    │ {"artifact": "spec"}          │
    │ ─────────────────────────────>│
    │                               │ Read specs/NNN-name/spec.md
    │         {"content": "..."}    │
    │ <─────────────────────────────│
    │                               │
    │ tools/call rstn_get_context   │
    │ ─────────────────────────────>│
    │                               │ Detect current feature
    │         {"feature_num":...}   │
    │ <─────────────────────────────│
```

## Implementation Approach

### Phase 1: rstn_read_spec Tool
- Map artifact name to filename
- Read from current feature's spec directory
- Return content or error if not found

### Phase 2: rstn_get_context Tool
- Reuse existing detect_current_feature()
- Return structured context JSON

## Key Components

### Artifact Mapping

```rust
fn artifact_to_filename(artifact: &str) -> &'static str {
    match artifact {
        "spec" => "spec.md",
        "plan" => "plan.md",
        "tasks" => "tasks.md",
        "checklist" => "checklist.md",
        "analysis" => "analysis.md",
        _ => "spec.md",
    }
}
```

### Context Response

```rust
#[derive(Serialize)]
struct FeatureContext {
    feature_num: String,
    feature_name: String,
    branch: String,
    phase: String,
    spec_dir: String,
}
```

## Files to Modify

| File | Changes |
|------|---------|
| `mcp_server.rs` | Add two tool handlers |

## Estimated Complexity

~150-200 lines
