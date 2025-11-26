//! Path completion for files and directories
//!
//! Provides tab completion for file and directory paths when typing arguments.
//! Supports relative paths, absolute paths, tilde expansion, hidden files, and
//! proper handling of paths with spaces.

use nu_ansi_term::{Color, Style};
use reedline::{Completer, Span, Suggestion};
use std::fs;

/// Completes file and directory paths for command arguments
///
/// Features:
/// - Relative and absolute paths
/// - Tilde expansion (~/ → home directory)
/// - Hidden files (shown only when prefix starts with '.')
/// - Directory markers (appends '/' to directories)
/// - Quoted paths with spaces
/// - Platform-specific case sensitivity
pub struct PathCompleter {
    /// Whether to perform case-sensitive matching (platform-dependent)
    #[cfg(target_os = "macos")]
    case_sensitive: bool,

    #[cfg(not(target_os = "macos"))]
    case_sensitive: bool,
}

impl PathCompleter {
    /// Create a new PathCompleter with platform-specific case sensitivity (T023)
    ///
    /// # Platform behavior
    /// - macOS: Case-insensitive matching (HFS+/APFS default)
    /// - Linux: Case-sensitive matching
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "macos")]
            case_sensitive: false,
            #[cfg(not(target_os = "macos"))]
            case_sensitive: true,
        }
    }

    /// Extract the partial path from the line at cursor position (T024)
    ///
    /// Extracts the path fragment being completed, handling:
    /// - Relative paths: `src/m` → `src/m`
    /// - Absolute paths: `/usr/l` → `/usr/l`
    /// - Tilde paths: `~/D` → `~/D`
    /// - Multiple arguments: `ls src/m other` (cursor at 8) → `src/m`
    fn extract_partial_path(&self, line: &str, pos: usize) -> Option<String> {
        let before_cursor = &line[..pos];

        // Find the start of the current argument (after last space)
        let start = before_cursor.rfind(' ').map(|i| i + 1).unwrap_or(0);

        // Extract from start to cursor
        let partial = before_cursor[start..].to_string();

        // Only complete if we're not in the first word (that's command completion)
        if start == 0 {
            return None;
        }

        Some(partial)
    }

    /// Split path into parent directory and filename prefix (T025)
    ///
    /// Examples:
    /// - `src/main.rs` → (`src/`, `main.rs`)
    /// - `main.rs` → (`./`, `main.rs`)
    /// - `/usr/bin/git` → (`/usr/bin/`, `git`)
    /// - `~/Documents/test` → (`~/Documents/`, `test`)
    fn split_path_and_prefix(&self, path: &str) -> (String, String) {
        // Handle empty path
        if path.is_empty() {
            return ("./".to_string(), String::new());
        }

        // Find the last path separator
        let sep_idx = path.rfind('/');

        match sep_idx {
            Some(idx) => {
                // Has a directory component
                let parent = &path[..=idx]; // Include the /
                let prefix = &path[idx + 1..];
                (parent.to_string(), prefix.to_string())
            }
            None => {
                // No directory component, use current directory
                ("./".to_string(), path.to_string())
            }
        }
    }

    /// Expand tilde (~) to home directory (T030)
    ///
    /// Examples:
    /// - `~/Documents` → `/Users/username/Documents`
    /// - `~` → `/Users/username`
    /// - `./test` → `./test` (no change)
    fn expand_tilde(&self, path: &str) -> String {
        if path.starts_with("~/") || path == "~" {
            if let Some(home) = dirs::home_dir() {
                if path == "~" {
                    return home.to_string_lossy().to_string();
                } else {
                    return path.replacen("~", &home.to_string_lossy(), 1);
                }
            }
        }
        path.to_string()
    }

    /// Check if filename matches prefix (case-sensitive or insensitive)
    fn matches_prefix(&self, name: &str, prefix: &str) -> bool {
        if self.case_sensitive {
            name.starts_with(prefix)
        } else {
            name.to_lowercase().starts_with(&prefix.to_lowercase())
        }
    }

    /// List directory entries matching the given prefix (T026)
    ///
    /// Returns matching files and directories with appropriate formatting:
    /// - Directories have '/' appended
    /// - Paths with spaces are quoted
    /// - Hidden files only shown if prefix starts with '.'
    fn list_directory_entries(
        &self,
        parent: &str,
        prefix: &str,
    ) -> Result<Vec<String>, std::io::Error> {
        // Expand tilde in parent directory
        let parent_expanded = self.expand_tilde(parent);

        // Read directory
        let entries = fs::read_dir(&parent_expanded)?;

        let mut matches = Vec::new();

        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                // T027: Skip hidden files unless prefix starts with '.'
                if name.starts_with('.') && !prefix.starts_with('.') {
                    continue;
                }

                // Check if name matches prefix
                if self.matches_prefix(&name, prefix) {
                    // T028: Append '/' to directories
                    let mut display_name = name.clone();
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            display_name.push('/');
                        }
                    }

                    // T028: Quote paths with spaces
                    if display_name.contains(' ') {
                        display_name = format!("\"{}\"", display_name);
                    }

                    matches.push(display_name);
                }
            }
        }

        // Sort alphabetically
        matches.sort();

        Ok(matches)
    }
}

impl Default for PathCompleter {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement reedline's Completer trait for PathCompleter (T029)
impl Completer for PathCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        use std::time::Instant;
        let start = Instant::now();

        tracing::debug!(
            line = %line,
            pos = pos,
            "Path completion triggered"
        );

        // Extract partial path
        let partial = match self.extract_partial_path(line, pos) {
            Some(p) => p,
            None => {
                tracing::debug!("Not completing path (in first word - command position)");
                return vec![];
            }
        };

        tracing::debug!(
            partial = %partial,
            "Completing partial path"
        );

        // Split into parent directory and filename prefix
        let (parent, prefix) = self.split_path_and_prefix(&partial);

        tracing::debug!(
            parent = %parent,
            prefix = %prefix,
            "Split path into parent and prefix"
        );

        // List matching entries
        let matches = match self.list_directory_entries(&parent, &prefix) {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    parent = %parent,
                    "Failed to read directory for path completion"
                );
                return vec![];
            }
        };

        tracing::debug!(
            match_count = matches.len(),
            sample = ?matches.iter().take(5).collect::<Vec<_>>(),
            "Found path matches"
        );

        // T032: Limit to 50 matches
        if matches.len() > 50 {
            let elapsed = start.elapsed();
            tracing::warn!(
                count = matches.len(),
                partial = %partial,
                duration_ms = elapsed.as_millis(),
                "Too many path matches - returning empty vec"
            );
            return vec![];
        }

        if matches.is_empty() {
            let elapsed = start.elapsed();
            tracing::info!(
                partial = %partial,
                duration_ms = elapsed.as_millis(),
                "No matching paths found"
            );
            return vec![];
        }

        let elapsed = start.elapsed();
        tracing::info!(
            count = matches.len(),
            partial = %partial,
            duration_ms = elapsed.as_millis(),
            "Path completion successful"
        );

        // Convert to Suggestion objects
        // Calculate the span to replace (entire partial path)
        let start_pos = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);

        // Style for path completions (green to distinguish from commands)
        let path_style = Style::new().fg(Color::Green);

        matches
            .into_iter()
            .map(|path| {
                // Reconstruct full path for display
                let mut value = parent.clone();
                value.push_str(&path);

                Suggestion {
                    value,
                    description: None,
                    extra: None,
                    span: Span { start: start_pos, end: pos },
                    append_whitespace: false, // Don't append space after paths (user might add more path)
                    style: Some(path_style),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_completer_new() {
        let completer = PathCompleter::new();

        #[cfg(target_os = "macos")]
        assert!(!completer.case_sensitive);

        #[cfg(not(target_os = "macos"))]
        assert!(completer.case_sensitive);
    }

    #[test]
    fn test_split_path_and_prefix() {
        let completer = PathCompleter::new();

        assert_eq!(
            completer.split_path_and_prefix("src/main.rs"),
            ("src/".to_string(), "main.rs".to_string())
        );

        assert_eq!(
            completer.split_path_and_prefix("main.rs"),
            ("./".to_string(), "main.rs".to_string())
        );

        assert_eq!(
            completer.split_path_and_prefix("/usr/bin/git"),
            ("/usr/bin/".to_string(), "git".to_string())
        );

        assert_eq!(completer.split_path_and_prefix(""), ("./".to_string(), String::new()));
    }

    #[test]
    fn test_expand_tilde() {
        let completer = PathCompleter::new();

        // Tilde should expand to home directory
        let expanded = completer.expand_tilde("~/Documents");
        assert!(expanded.contains("Documents"));
        assert!(!expanded.starts_with("~"));

        // Non-tilde paths should remain unchanged
        assert_eq!(completer.expand_tilde("./test"), "./test");
        assert_eq!(completer.expand_tilde("/usr/local"), "/usr/local");
    }

    #[test]
    fn test_extract_partial_path() {
        let completer = PathCompleter::new();

        // Should extract path from argument position
        assert_eq!(completer.extract_partial_path("ls src/m", 8), Some("src/m".to_string()));

        // Should return None for first word (command position)
        assert_eq!(completer.extract_partial_path("ls", 2), None);

        // Should handle cursor in middle of argument
        assert_eq!(
            completer.extract_partial_path("ls src/main.rs other", 14),
            Some("src/main.rs".to_string())
        );
    }

    #[test]
    fn test_expand_tilde_exact() {
        let completer = PathCompleter::new();

        // Just "~" should expand to home directory (line 108)
        let expanded = completer.expand_tilde("~");
        assert!(!expanded.starts_with("~"));
        assert!(!expanded.is_empty());
    }

    #[test]
    fn test_path_completer_default() {
        let completer = PathCompleter::default();

        #[cfg(target_os = "macos")]
        assert!(!completer.case_sensitive);

        #[cfg(not(target_os = "macos"))]
        assert!(completer.case_sensitive);
    }

    #[test]
    fn test_matches_prefix() {
        let completer = PathCompleter::new();

        #[cfg(target_os = "macos")]
        {
            // Case-insensitive on macOS
            assert!(completer.matches_prefix("README.md", "readme"));
            assert!(completer.matches_prefix("README.md", "README"));
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Case-sensitive on Linux
            assert!(completer.matches_prefix("README.md", "README"));
            assert!(!completer.matches_prefix("README.md", "readme"));
        }
    }

    #[test]
    fn test_list_directory_entries() {
        let completer = PathCompleter::new();

        // List entries from /tmp which should exist
        let result = completer.list_directory_entries("/tmp/", "");
        assert!(result.is_ok());
        // /tmp might have entries
    }

    #[test]
    fn test_list_directory_entries_nonexistent() {
        let completer = PathCompleter::new();

        // Nonexistent directory should return error
        let result = completer.list_directory_entries("/nonexistent_dir_12345/", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_complete_returns_empty_for_first_word() {
        let mut completer = PathCompleter::new();

        // First word is command, not path
        let suggestions = completer.complete("ls", 2);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_complete_with_valid_path() {
        let mut completer = PathCompleter::new();

        // Complete in /tmp directory
        let suggestions = completer.complete("ls /tmp/", 8);
        // May or may not have entries, but should not panic
        assert!(suggestions.len() <= 50);
    }

    #[test]
    fn test_complete_with_tilde_path() {
        let mut completer = PathCompleter::new();

        // Complete in home directory
        let suggestions = completer.complete("ls ~/", 5);
        // Home directory should exist and potentially have entries
        // Just verify it doesn't panic
        let _ = suggestions.len();
    }

    #[test]
    fn test_list_directory_entries_with_spaces() {
        use std::fs;

        // Create temp directory with file containing space
        let test_dir = "/tmp/rush_path_test_spaces";
        let _ = fs::remove_dir_all(test_dir);
        fs::create_dir_all(test_dir).unwrap();
        fs::write(format!("{}/file with space.txt", test_dir), "test").unwrap();

        let completer = PathCompleter::new();
        let result = completer.list_directory_entries(&format!("{}/", test_dir), "");

        fs::remove_dir_all(test_dir).unwrap();

        assert!(result.is_ok());
        let entries = result.unwrap();
        // Should have the file with space, quoted (line 164)
        assert!(entries
            .iter()
            .any(|e| e.contains("\"") && e.contains("space")));
    }

    #[test]
    fn test_list_directory_entries_with_directory() {
        use std::fs;

        // Create temp directory with a subdirectory
        let test_dir = "/tmp/rush_path_test_dir";
        let _ = fs::remove_dir_all(test_dir);
        fs::create_dir_all(format!("{}/subdir", test_dir)).unwrap();

        let completer = PathCompleter::new();
        let result = completer.list_directory_entries(&format!("{}/", test_dir), "");

        fs::remove_dir_all(test_dir).unwrap();

        assert!(result.is_ok());
        let entries = result.unwrap();
        // Subdirectory should have / appended (line 158)
        assert!(entries.iter().any(|e| e == "subdir/"));
    }

    #[test]
    fn test_case_sensitive_path_matching() {
        let completer = PathCompleter::new();

        #[cfg(target_os = "macos")]
        {
            // Case-insensitive on macOS (line 122)
            assert!(completer.matches_prefix("README.md", "readme"));
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Case-sensitive on Linux (line 120)
            assert!(completer.matches_prefix("readme.md", "readme"));
            assert!(!completer.matches_prefix("README.md", "readme"));
        }
    }

    #[test]
    fn test_complete_nonexistent_directory() {
        let mut completer = PathCompleter::new();

        // Complete in a nonexistent directory (lines 223-229)
        let suggestions = completer.complete("ls /nonexistent_dir_xyz123/", 27);
        // Should return empty vec when directory doesn't exist
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_complete_no_matches() {
        use std::fs;

        let mut completer = PathCompleter::new();

        // Create a temp directory with specific files
        let test_dir = "/tmp/rush_path_nomatch_test";
        let _ = fs::remove_dir_all(test_dir);
        fs::create_dir_all(test_dir).unwrap();
        fs::write(format!("{}/apple.txt", test_dir), "test").unwrap();

        // Complete with a prefix that doesn't match anything (lines 251-258)
        let cmd = format!("ls {}/xyz", test_dir);
        let suggestions = completer.complete(&cmd, cmd.len());

        fs::remove_dir_all(test_dir).unwrap();

        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_complete_too_many_matches() {
        use std::fs;

        let mut completer = PathCompleter::new();

        // Create a temp directory with >50 files (lines 240-248)
        let test_dir = "/tmp/rush_path_toomany_test";
        let _ = fs::remove_dir_all(test_dir);
        fs::create_dir_all(test_dir).unwrap();

        // Create 55 files
        for i in 0..55 {
            fs::write(format!("{}/file_{:03}.txt", test_dir, i), "test").unwrap();
        }

        // Complete with empty prefix - should trigger "too many matches"
        let cmd = format!("ls {}/", test_dir);
        let suggestions = completer.complete(&cmd, cmd.len());

        fs::remove_dir_all(test_dir).unwrap();

        // Should return empty when >50 matches
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_complete_hidden_files() {
        use std::fs;

        let mut completer = PathCompleter::new();

        // Create temp directory with hidden and non-hidden files
        let test_dir = "/tmp/rush_path_hidden_test";
        let _ = fs::remove_dir_all(test_dir);
        fs::create_dir_all(test_dir).unwrap();
        fs::write(format!("{}/.hidden", test_dir), "test").unwrap();
        fs::write(format!("{}/visible", test_dir), "test").unwrap();

        // Complete without . prefix - should not show hidden file
        let cmd1 = format!("ls {}/v", test_dir);
        let suggestions1 = completer.complete(&cmd1, cmd1.len());

        // Complete with . prefix - should show hidden file (T027)
        let cmd2 = format!("ls {}/.", test_dir);
        let suggestions2 = completer.complete(&cmd2, cmd2.len());

        fs::remove_dir_all(test_dir).unwrap();

        // Visible file should be found
        assert!(suggestions1.iter().any(|s| s.value.contains("visible")));
        // Hidden should only show with . prefix
        assert!(suggestions2.iter().any(|s| s.value.contains(".hidden")));
    }

    #[test]
    fn test_expand_tilde_just_tilde() {
        let completer = PathCompleter::new();

        // Test "~" expansion (line 107-108)
        let expanded = completer.expand_tilde("~");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(expanded, home.to_string_lossy().to_string());
        }
    }
}
