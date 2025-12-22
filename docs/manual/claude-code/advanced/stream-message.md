---
title: "Extended ClaudeStreamMessage Structure"
description: "Extended message struct with tool metadata for enhanced display"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "061"
tags: [claude-code, streaming, jsonl, tools]
weight: 5
---

## 4. Extended ClaudeStreamMessage Structure

**Status**: ✅ IMPLEMENTED (Phase 1)

### Implementation Details

**File**: [crates/rstn/src/tui/claude_stream.rs:38-65](../../crates/rstn/src/tui/claude_stream.rs#L38-L65)

**Code**:
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeContent {
    #[serde(rename = "type")]
    pub content_type: String,

    // For "text" type
    #[serde(default)]
    pub text: Option<String>,

    // For "tool_use" type - ✅ IMPLEMENTED
    #[serde(default)]
    pub id: Option<String>,           // "toolu_01ABC..."
    #[serde(default)]
    pub name: Option<String>,         // "Read", "Edit", etc.
    #[serde(default)]
    pub input: Option<serde_json::Value>,  // Tool parameters

    // For "tool_result" type - ✅ IMPLEMENTED
    #[serde(default)]
    pub tool_use_id: Option<String>,  // Reference to tool_use
    #[serde(default)]
    pub content: Option<String>,       // Result content (alt to text)

    #[serde(default)]
    pub is_error: Option<bool>,
}
```

**Enhanced Display** ([session_output.rs:95-125](../../crates/rstn/src/tui/views/session_output.rs#L95-L125)):
```rust
"tool_use" => {
    if let Some(msg) = &message.message {
        for content in &msg.content {
            if content.content_type == "tool_use" {
                let tool_name = content.name.as_deref().unwrap_or("Unknown");
                let tool_id = content
                    .id
                    .as_deref()
                    .and_then(|id| id.get(..8))
                    .unwrap_or("????????");

                // Format input parameters
                let params = if let Some(input) = &content.input {
                    let json_str = serde_json::to_string(input).unwrap_or_default();
                    if json_str.len() > 60 {
                        format!("{}...", &json_str[..60])
                    } else {
                        json_str
                    }
                } else {
                    "{}".to_string()
                };

                // ✅ NOW SHOWS: 🔧 Tool: Read [toolu_01A] {"file_path":"/test.rs"}
                self.output_lines.push(format!(
                    "🔧 Tool: {} [{}] {}",
                    tool_name, tool_id, params
                ));
            }
        }
    }
}
```

**Tests** ([claude_stream.rs:152-186](../../crates/rstn/src/tui/claude_stream.rs#L152-L186)):
```rust
#[test]
fn test_tool_use_with_metadata() {
    let json = r#"{"type":"assistant","message":{"role":"assistant","content":[
        {"type":"tool_use","id":"toolu_01ABC123","name":"Read","input":{"file_path":"/test.rs"}}
    ]}}\"#;

    let msg: ClaudeStreamMessage = serde_json::from_str(json).unwrap();
    let content = &msg.message.unwrap().content[0];

    assert_eq!(content.name, Some("Read".to_string()));
    assert_eq!(content.id, Some("toolu_01ABC123".to_string()));
    assert!(content.input.is_some());
}
```

### Enhanced Display

**File**: `crates/rstn/src/tui/views/session_output.rs` (updated)

```rust
"tool_use" => {
    if let Some(msg) = &message.message {
        for content in &msg.content {
            if content.content_type == "tool_use" {
                let tool_name = content.name.as_deref().unwrap_or("Unknown");
                let tool_id = content.id.as_deref().unwrap_or("").get(..8).unwrap_or("");

                // Format input parameters nicely
                let params = if let Some(input) = &content.input {
                    format!("{}", input)
                } else {
                    "{}".to_string()
                };

                self.output_lines.push(format!(
                    "🔧 Tool: {} [{}] {}",
                    tool_name,
                    tool_id,
                    truncate(&params, 60)
                ));
            }
        }
    }
}
```

**Example Output**:
```
🔧 Tool: Read [toolu_01A] {"file_path": "/path/to/file.rs"}
  ✓ Result: Read 150 lines from file.rs
🔧 Tool: Edit [toolu_01B] {"file_path": "/path/to/file.rs", "old_string": "..."}
  ✓ Result: Edited file successfully
```

### Backward Compatibility

All new fields use `#[serde(default)]`, so existing code continues to work with incomplete data.

---

