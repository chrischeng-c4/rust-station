use ratatui::style::Color;
use std::time::SystemTime;

/// Category of log entry for styling and filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogCategory {
    /// User actions (command selection, input, focus changes)
    User,
    /// Command execution (e.g., Prompt Claude, /speckit.specify)
    Command,
    /// Claude Code streaming output
    ClaudeStream,
    /// MCP tool calls and responses
    Mcp,
    /// Hook execution and results
    Hook,
    /// File operations (read, write, errors)
    FileChange,
    /// Error conditions (validation, system errors)
    Error,
    /// System/TUI internal messages
    System,
}

impl LogCategory {
    /// Get emoji icon for this category
    pub fn icon(&self) -> &'static str {
        match self {
            Self::User => "🧑",
            Self::Command => "⚡",
            Self::ClaudeStream => "🤖",
            Self::Mcp => "🔌",
            Self::Hook => "🔧",
            Self::FileChange => "📝",
            Self::Error => "❌",
            Self::System => "ℹ️",
        }
    }

    /// Get ratatui Color for this category
    pub fn color(&self) -> Color {
        match self {
            Self::User => Color::Blue,
            Self::Command => Color::Cyan,
            Self::ClaudeStream => Color::White,
            Self::Mcp => Color::Magenta,
            Self::Hook => Color::Yellow,
            Self::FileChange => Color::Green,
            Self::Error => Color::Red,
            Self::System => Color::DarkGray,
        }
    }
}

/// A single timestamped log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub category: LogCategory,
    pub content: String,
}

impl LogEntry {
    /// Create a new log entry with current timestamp
    pub fn new(category: LogCategory, content: String) -> Self {
        Self {
            timestamp: SystemTime::now(),
            category,
            content,
        }
    }

    /// Format timestamp as HH:MM:SS
    pub fn format_timestamp(&self) -> String {
        use chrono::prelude::*;

        if let Ok(duration) = self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
            let datetime = Local.timestamp_opt(duration.as_secs() as i64, 0);
            if let Some(dt) = datetime.single() {
                return dt.format("%H:%M:%S").to_string();
            }
        }

        // Fallback if timestamp conversion fails
        "00:00:00".to_string()
    }

    /// Get emoji icon for this entry's category
    pub fn category_icon(&self) -> &'static str {
        self.category.icon()
    }
}
