// suggest.rs - History-based autosuggestions for rush shell
//
// This module implements the Hinter trait from reedline to provide fish-like
// autosuggestions based on command history. Suggestions are displayed as
// grayed-out text after the cursor and can be accepted with Right Arrow.

use nu_ansi_term::{Color, Style};
use reedline::{Hinter, History, SearchQuery};

/// Provides history-based autosuggestions for the rush shell.
///
/// `RushHinter` implements reedline's `Hinter` trait to suggest commands from
/// history as the user types. Suggestions are displayed as dimmed text and can
/// be accepted with Right Arrow (full suggestion) or Alt+Right Arrow (word-by-word).
///
/// # Behavior
///
/// - Only suggests when cursor is at end of line
/// - Searches history in reverse chronological order (most recent first)
/// - Returns the most recent command that starts with current input
/// - Handles empty history and no-match scenarios gracefully
///
/// # Example
///
/// ```no_run
/// use rush::repl::suggest::RushHinter;
/// use reedline::Reedline;
///
/// let hinter = Box::new(RushHinter::new());
/// let editor = Reedline::create()
///     .with_hinter(hinter);
/// ```
pub struct RushHinter {
    /// Current hint text (stored for complete_hint and next_hint_token)
    current_hint: String,
    /// Style for rendering hints
    style: Style,
}

impl RushHinter {
    /// Creates a new `RushHinter` instance.
    ///
    /// The hinter uses a light gray style for suggestions.
    ///
    /// # Returns
    ///
    /// A new `RushHinter` ready to provide suggestions.
    ///
    /// # Example
    ///
    /// ```
    /// use rush::repl::suggest::RushHinter;
    ///
    /// let hinter = RushHinter::new();
    /// ```
    pub fn new() -> Self {
        Self { current_hint: String::new(), style: Style::new().fg(Color::DarkGray).dimmed() }
    }
}

impl Default for RushHinter {
    fn default() -> Self {
        Self::new()
    }
}

impl Hinter for RushHinter {
    /// Provides a suggestion for the current line and cursor position.
    ///
    /// This is called by reedline on every keystroke to update suggestions.
    ///
    /// # Arguments
    ///
    /// * `line` - The current input buffer content
    /// * `pos` - The current cursor position (0-indexed)
    /// * `history` - Access to command history
    /// * `use_ansi_coloring` - Whether to apply color styling
    /// * `_cwd` - Current working directory (unused)
    ///
    /// # Returns
    ///
    /// Formatted suggestion string (empty if no suggestion)
    fn handle(
        &mut self,
        line: &str,
        pos: usize,
        history: &dyn History,
        use_ansi_coloring: bool,
        _cwd: &str,
    ) -> String {
        // Only suggest when cursor is at end of line
        if pos != line.len() {
            self.current_hint.clear();
            return String::new();
        }

        // Don't suggest for empty input
        if line.is_empty() {
            self.current_hint.clear();
            return String::new();
        }

        // Search history for most recent match
        self.current_hint = history
            .search(SearchQuery::last_with_prefix(line.to_string(), history.session()))
            .unwrap_or_default()
            .first()
            .and_then(|entry| {
                let cmd = &entry.command_line;
                // Skip exact matches
                if cmd == line {
                    None
                } else {
                    // Return suffix (everything after the input)
                    cmd.get(line.len()..).map(|s| s.to_string())
                }
            })
            .unwrap_or_default();

        // Apply styling if requested and hint is not empty
        if use_ansi_coloring && !self.current_hint.is_empty() {
            self.style.paint(&self.current_hint).to_string()
        } else {
            self.current_hint.clone()
        }
    }

    /// Return the current hint unformatted for full completion
    fn complete_hint(&self) -> String {
        self.current_hint.clone()
    }

    /// Return the first token of the hint for incremental completion
    fn next_hint_token(&self) -> String {
        // Return the first whitespace-delimited token
        self.current_hint
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hinter = RushHinter::new();
        assert_eq!(hinter.current_hint, "");
    }

    #[test]
    fn test_default() {
        let hinter = RushHinter::default();
        assert_eq!(hinter.current_hint, "");
    }

    #[test]
    fn test_complete_hint() {
        use reedline::Hinter;

        let mut hinter = RushHinter::new();
        hinter.current_hint = "full hint text".to_string();

        // Test complete_hint() returns the full hint
        let complete = hinter.complete_hint();
        assert_eq!(complete, "full hint text");
    }

    #[test]
    fn test_next_hint_token() {
        use reedline::Hinter;

        let mut hinter = RushHinter::new();
        hinter.current_hint = "first second third".to_string();

        // Test next_hint_token() returns only the first token
        let token = hinter.next_hint_token();
        assert_eq!(token, "first");
    }

    #[test]
    fn test_next_hint_token_empty() {
        use reedline::Hinter;

        let mut hinter = RushHinter::new();
        hinter.current_hint = "".to_string();

        // Test next_hint_token() with empty hint
        let token = hinter.next_hint_token();
        assert_eq!(token, "");
    }

    #[test]
    fn test_hint_with_ansi_styling() {
        use reedline::{FileBackedHistory, Hinter, History, HistoryItem};

        let mut hinter = RushHinter::new();

        // Create a temporary history file
        let temp_dir = std::env::temp_dir();
        let history_file = temp_dir.join(format!("rush_test_history_{}.txt", std::process::id()));

        // Create history with a command
        let mut history = FileBackedHistory::with_file(100, history_file.clone()).unwrap();
        let item = HistoryItem::from_command_line("echo hello");
        let _ = history.save(item);

        // Test handle() with ansi coloring enabled (line 122)
        let styled_hint = hinter.handle("echo", 4, &history, true, "/tmp");

        // Should return styled hint (contains ANSI codes from line 122)
        assert!(!styled_hint.is_empty());
        assert!(styled_hint.contains("hello"));

        // Clean up
        let _ = std::fs::remove_file(history_file);
    }

    #[test]
    fn test_hint_cursor_not_at_end() {
        use reedline::{FileBackedHistory, Hinter};

        let mut hinter = RushHinter::new();

        // Create a temporary history file
        let temp_dir = std::env::temp_dir();
        let history_file =
            temp_dir.join(format!("rush_test_hint_cursor_{}.txt", std::process::id()));

        let history = FileBackedHistory::with_file(100, history_file.clone()).unwrap();

        // Set a hint first
        hinter.current_hint = "some hint".to_string();

        // Test handle() with cursor NOT at end of line (lines 92-94)
        // Line is "echo hello" (10 chars) but cursor is at position 4
        let hint = hinter.handle("echo hello", 4, &history, true, "/tmp");

        // Should return empty and clear current_hint
        assert!(hint.is_empty());
        assert!(hinter.current_hint.is_empty());

        // Clean up
        let _ = std::fs::remove_file(history_file);
    }

    #[test]
    fn test_hint_empty_input() {
        use reedline::{FileBackedHistory, Hinter};

        let mut hinter = RushHinter::new();

        // Create a temporary history file
        let temp_dir = std::env::temp_dir();
        let history_file =
            temp_dir.join(format!("rush_test_hint_empty_{}.txt", std::process::id()));

        let history = FileBackedHistory::with_file(100, history_file.clone()).unwrap();

        // Set a hint first
        hinter.current_hint = "some hint".to_string();

        // Test handle() with empty input (lines 98-100)
        let hint = hinter.handle("", 0, &history, true, "/tmp");

        // Should return empty and clear current_hint
        assert!(hint.is_empty());
        assert!(hinter.current_hint.is_empty());

        // Clean up
        let _ = std::fs::remove_file(history_file);
    }

    #[test]
    fn test_hint_exact_match_skipped() {
        use reedline::{FileBackedHistory, Hinter, History, HistoryItem};

        let mut hinter = RushHinter::new();

        // Create a temporary history file
        let temp_dir = std::env::temp_dir();
        let history_file =
            temp_dir.join(format!("rush_test_hint_exact_{}.txt", std::process::id()));

        // Create history with exact command
        let mut history = FileBackedHistory::with_file(100, history_file.clone()).unwrap();
        let item = HistoryItem::from_command_line("echo");
        let _ = history.save(item);

        // Test handle() with exact match (lines 111-112)
        let hint = hinter.handle("echo", 4, &history, true, "/tmp");

        // Should return empty because exact match is skipped
        assert!(hint.is_empty());

        // Clean up
        let _ = std::fs::remove_file(history_file);
    }

    #[test]
    fn test_hint_without_ansi_styling() {
        use reedline::{FileBackedHistory, Hinter, History, HistoryItem};

        let mut hinter = RushHinter::new();

        // Create a temporary history file
        let temp_dir = std::env::temp_dir();
        let history_file =
            temp_dir.join(format!("rush_test_hint_nostyle_{}.txt", std::process::id()));

        // Create history with a command
        let mut history = FileBackedHistory::with_file(100, history_file.clone()).unwrap();
        let item = HistoryItem::from_command_line("echo hello");
        let _ = history.save(item);

        // Test handle() with ansi coloring disabled (line 124)
        let hint = hinter.handle("echo", 4, &history, false, "/tmp");

        // Should return plain hint without ANSI codes
        assert_eq!(hint, " hello");

        // Clean up
        let _ = std::fs::remove_file(history_file);
    }
}
