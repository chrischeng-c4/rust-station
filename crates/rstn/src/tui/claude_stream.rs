//! Parse Claude Code streaming JSON output (JSONL format)
//!
//! When Claude Code runs with `--output-format stream-json`, it outputs one JSON
//! object per line. This module parses that output for display in the TUI.

use serde::Deserialize;

/// A single line from Claude's stream-json output
#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeStreamMessage {
    #[serde(rename = "type")]
    pub msg_type: String, // "init", "assistant", "user", "result"

    #[serde(default)]
    pub message: Option<ClaudeMessage>,

    #[serde(default)]
    pub session_id: Option<String>,

    #[serde(default)]
    pub result: Option<String>,

    #[serde(default)]
    pub total_cost_usd: Option<f64>,

    #[serde(default)]
    pub is_error: Option<bool>,
}

/// Message content from Claude
#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: Vec<ClaudeContent>,
}

/// Content block within a message
#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeContent {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(default)]
    pub text: Option<String>,
}

impl ClaudeStreamMessage {
    /// Extract all text content from message
    pub fn get_text(&self) -> Option<String> {
        let msg = self.message.as_ref()?;
        let texts: Vec<&str> = msg
            .content
            .iter()
            .filter_map(|c| c.text.as_deref())
            .collect();
        if texts.is_empty() {
            None
        } else {
            Some(texts.join("\n"))
        }
    }

    /// Get text for display (MCP tools handle status, no stripping needed)
    pub fn get_display_text(&self) -> Option<String> {
        self.get_text()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_text_multiple_content() {
        let msg = ClaudeStreamMessage {
            msg_type: "assistant".to_string(),
            message: Some(ClaudeMessage {
                role: "assistant".to_string(),
                content: vec![
                    ClaudeContent {
                        content_type: "text".to_string(),
                        text: Some("First part".to_string()),
                    },
                    ClaudeContent {
                        content_type: "text".to_string(),
                        text: Some("Second part".to_string()),
                    },
                ],
            }),
            session_id: None,
            result: None,
            total_cost_usd: None,
            is_error: None,
        };

        let text = msg.get_text().unwrap();
        assert!(text.contains("First part"));
        assert!(text.contains("Second part"));
    }

    #[test]
    fn test_result_message() {
        let json = r#"{"type":"result","result":"Done","session_id":"abc123","total_cost_usd":0.05,"is_error":false}"#;
        let msg: ClaudeStreamMessage = serde_json::from_str(json).unwrap();

        assert_eq!(msg.msg_type, "result");
        assert_eq!(msg.session_id, Some("abc123".to_string()));
        assert_eq!(msg.total_cost_usd, Some(0.05));
        assert_eq!(msg.is_error, Some(false));
    }

    #[test]
    fn test_init_message() {
        let json = r#"{"type":"init","apiKeySource":"ANTHROPIC_API_KEY","model":"claude-sonnet-4-20250514"}"#;
        let msg: ClaudeStreamMessage = serde_json::from_str(json).unwrap();

        assert_eq!(msg.msg_type, "init");
    }
}
