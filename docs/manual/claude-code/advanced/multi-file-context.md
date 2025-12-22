---
title: "Multi-File Context via --context"
description: "Add context files to Claude prompts using --context flag"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "062"
tags: [claude-code, context, jsonl]
weight: 4
---

## 3. Multi-File Context via stream-json

**Status**: ✅ IMPLEMENTED (Phase 2)

### Problem Statement

How to provide multiple files to Claude simultaneously (like `--context` flag proposal).

**User Question**: "attach 檔案 應該使用input-form stream-json來達成？"

### Solution: --input-format stream-json

**Reference**: [kb/03-api-reference/claude-code-communication.md:120-240](claude-code-communication.md#L120-L240)

#### JSONL Input Format

Claude Code accepts JSONL input via stdin when `--input-format stream-json` is specified:

```bash
cat input.jsonl | claude --input-format stream-json
```

**input.jsonl**:
```jsonl
{"role":"user","content":[{"type":"text","text":"User message"}]}
```

#### Multi-File Context Pattern

```jsonl
{
  "role": "user",
  "content": [
    {
      "type": "text",
      "text": "Add dark mode support based on these files:"
    },
    {
      "type": "text",
      "text": "=== src/theme.rs ===\n\npub struct Theme {\n    pub bg: Color,\n    pub fg: Color,\n}\n"
    },
    {
      "type": "text",
      "text": "=== src/settings.rs ===\n\npub struct Settings {\n    pub theme_name: String,\n}\n"
    }
  ]
}
```

### Implementation Details

**File**: [crates/rstn/src/cli.rs:60-65](../../crates/rstn/src/cli.rs#L60-L65)

**CLI Args**:
```rust
#[derive(clap::Parser)]
pub enum Commands {
    Prompt {
        message: String,
        max_turns: u32,
        skip_permissions: bool,
        continue_session: bool,
        session_id: Option<String>,
        allowed_tools: Vec<String>,
        /// Additional files for context (comma-separated paths)
        #[arg(long, value_delimiter = ',')]
        context: Vec<std::path::PathBuf>, // ✅ IMPLEMENTED
    },
    // ...
}
```

**JSONL Builder** ([cargo.rs:110-140](../../crates/rstn/src/runners/cargo.rs#L110-L140)):
```rust
async fn build_jsonl_with_context(
    prompt: &str,
    context_files: &[std::path::PathBuf],
) -> Result<String> {
    use serde_json::json;

    let mut content_blocks = vec![json!({
        "type": "text",
        "text": prompt
    })];

    // Add context files
    for path in context_files {
        let file_content = tokio::fs::read_to_string(path).await?;
        let header = format!("=== {} ===\n\n", path.display());
        content_blocks.push(json!({
            "type": "text",
            "text": format!("{}{}", header, file_content)
        }));
    }

    let message = json!({
        "role": "user",
        "content": content_blocks
    });

    let jsonl = format!("{}\n", serde_json::to_string(&message)?);
    Ok(jsonl)
}
```

**Command Building** ([cargo.rs:450-480](../../crates/rstn/src/runners/cargo.rs#L450-L480)):
```rust
// Use stream-json input if context files provided
let (message_arg, use_stdin) = if !options.context_files.is_empty() {
    let jsonl = build_jsonl_with_context(message, &options.context_files).await?;
    (jsonl, true)
} else {
    (message.to_string(), false)
};

if use_stdin {
    command.arg("--input-format").arg("stream-json");
    command.stdin(std::process::Stdio::piped());
}
```

**Usage in CLI**:
```bash
rstn prompt "Add dark mode" --context src/theme.rs,src/settings.rs
rstn prompt "Fix bug" --context src/main.rs,tests/test.rs --max-turns 3
```

**Usage in TUI**:
```rust
let options = ClaudeCliOptions {
    context_files: vec![
        PathBuf::from("spec.md"),
        PathBuf::from("plan.md"),
    ],
    max_turns: Some(5),
    // ...
};
```

#### Advanced: Image Context

For image support (mentioned in current implementation):

```rust
// File content detection
for path in &args.context {
    let content = if is_image(&path) {
        // Base64 encode image
        let bytes = tokio::fs::read(path).await?;
        let base64 = base64::encode(&bytes);
        let media_type = detect_mime_type(path);

        serde_json::json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": media_type,
                "data": base64
            }
        })
    } else {
        // Text file
        let text = tokio::fs::read_to_string(path).await?;
        serde_json::json!({
            "type": "text",
            "text": format!("=== {} ===\n\n{}", path.display(), text)
        })
    };

    content_blocks.push(content);
}
```

**Current implementation** already uses `--add-dir` for image access:
```rust
// File: crates/rstn/src/runners/cargo.rs:480-485
for dir in &options.add_dirs {
    if dir.exists() {
        cmd.arg("--add-dir").arg(dir);
    }
}
```

---

