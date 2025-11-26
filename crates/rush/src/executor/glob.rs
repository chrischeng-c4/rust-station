//! Glob pattern expansion module
//!
//! Expands wildcard patterns in command arguments to matching file paths.
//! Supports standard shell glob patterns: `*`, `?`, `[...]`
//!
//! # Examples
//!
//! ```ignore
//! use rush::executor::glob::expand_globs;
//!
//! // Expand *.rs to all Rust files
//! let args = vec!["ls".to_string(), "*.rs".to_string()];
//! let expanded = expand_globs(&args[1..]);
//! // expanded might be: ["main.rs", "lib.rs", "mod.rs"]
//! ```

use glob::glob as glob_match;
use std::path::Path;

/// Characters that indicate a glob pattern
const GLOB_CHARS: &[char] = &['*', '?', '['];

/// Check if a string contains unescaped glob pattern characters
///
/// Returns `true` if the string contains `*`, `?`, or `[` that are not
/// preceded by a backslash escape.
pub fn contains_glob_chars(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            // Skip escaped character
            i += 2;
            continue;
        }
        if GLOB_CHARS.contains(&chars[i]) {
            return true;
        }
        i += 1;
    }
    false
}

/// Expand glob patterns in a list of arguments
///
/// Each argument is checked for glob patterns. If a pattern is found and matches
/// files, it's replaced with the matching file paths. If no matches are found,
/// the literal pattern is preserved (POSIX behavior).
///
/// Arguments that are quoted (start and end with quotes) are not expanded.
pub fn expand_globs(args: &[String]) -> Vec<String> {
    args.iter()
        .flat_map(|arg| expand_single_glob(arg))
        .collect()
}

/// Expand a single argument if it contains glob patterns
///
/// Returns a vector of matched paths, or the original argument if:
/// - It doesn't contain glob characters
/// - It's quoted
/// - The pattern matches no files (POSIX behavior)
fn expand_single_glob(arg: &str) -> Vec<String> {
    // Check if argument is quoted (don't expand)
    if is_quoted(arg) {
        return vec![unquote(arg)];
    }

    // Check if argument contains glob characters
    if !contains_glob_chars(arg) {
        return vec![arg.to_string()];
    }

    // Handle escaped glob characters
    let pattern = unescape_non_glob(arg);

    // Perform glob expansion
    match glob_match(&pattern) {
        Ok(paths) => {
            let mut matches: Vec<String> = paths
                .filter_map(|entry| entry.ok())
                .filter(|path| !is_hidden_unless_explicit(&pattern, path))
                .map(|path| path.to_string_lossy().to_string())
                .collect();

            if matches.is_empty() {
                // No matches: return literal pattern (POSIX behavior)
                vec![arg.to_string()]
            } else {
                // Sort matches alphabetically
                matches.sort_by_key(|a| a.to_lowercase());
                matches
            }
        }
        Err(_) => {
            // Invalid pattern: return literal
            vec![arg.to_string()]
        }
    }
}

/// Check if an argument is quoted (single or double quotes)
fn is_quoted(s: &str) -> bool {
    (s.starts_with('"') && s.ends_with('"') && s.len() >= 2)
        || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2)
}

/// Remove surrounding quotes from a string
fn unquote(s: &str) -> String {
    if is_quoted(s) && s.len() >= 2 {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Unescape non-glob backslash sequences
///
/// Converts `\*` to literal `*` by using glob's escape mechanism
fn unescape_non_glob(s: &str) -> String {
    // The glob crate handles escaping via [*] syntax
    // Convert \* to [*], \? to [?], \[ to [[]
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            let next = chars[i + 1];
            if GLOB_CHARS.contains(&next) {
                // Escape glob char using character class
                result.push('[');
                result.push(next);
                result.push(']');
                i += 2;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Filter hidden files unless the pattern explicitly requests them
///
/// Hidden files (starting with `.`) are excluded unless the pattern
/// explicitly starts with `.` or `*/.*` etc.
fn is_hidden_unless_explicit(pattern: &str, path: &Path) -> bool {
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    // If filename starts with '.', it's hidden
    if !filename.starts_with('.') {
        return false; // Not hidden, include it
    }

    // Check if pattern explicitly matches hidden files
    // Look at the last component of the pattern (the filename part)
    let pattern_filename = Path::new(pattern)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(pattern);

    // Pattern explicitly requests hidden if its filename part starts with '.'
    let pattern_requests_hidden = pattern_filename.starts_with('.');

    // Exclude hidden files unless explicitly requested
    !pattern_requests_hidden
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_contains_glob_chars_star() {
        assert!(contains_glob_chars("*.rs"));
        assert!(contains_glob_chars("file*"));
        assert!(contains_glob_chars("*"));
    }

    #[test]
    fn test_contains_glob_chars_question() {
        assert!(contains_glob_chars("file?.txt"));
        assert!(contains_glob_chars("?"));
        assert!(contains_glob_chars("a?b"));
    }

    #[test]
    fn test_contains_glob_chars_bracket() {
        assert!(contains_glob_chars("[abc]"));
        assert!(contains_glob_chars("file[0-9].txt"));
        assert!(contains_glob_chars("[!a]"));
    }

    #[test]
    fn test_contains_glob_chars_none() {
        assert!(!contains_glob_chars("file.txt"));
        assert!(!contains_glob_chars("hello"));
        assert!(!contains_glob_chars("path/to/file"));
    }

    #[test]
    fn test_contains_glob_chars_escaped() {
        assert!(!contains_glob_chars(r"\*"));
        assert!(!contains_glob_chars(r"\?"));
        assert!(!contains_glob_chars(r"\[abc\]"));
        // \*.rs has NO unescaped glob chars - the * is escaped, .rs has no globs
        assert!(!contains_glob_chars(r"\*.rs"));
        // But *.rs\? has an unescaped * at the start
        assert!(contains_glob_chars(r"*.rs\?"));
    }

    #[test]
    fn test_is_quoted() {
        assert!(is_quoted("\"hello\""));
        assert!(is_quoted("'hello'"));
        assert!(!is_quoted("hello"));
        assert!(!is_quoted("\"hello"));
        assert!(!is_quoted("hello\""));
    }

    #[test]
    fn test_unquote() {
        assert_eq!(unquote("\"hello\""), "hello");
        assert_eq!(unquote("'world'"), "world");
        assert_eq!(unquote("noquotes"), "noquotes");
    }

    #[test]
    fn test_expand_globs_no_pattern() {
        let args = vec!["file.txt".to_string(), "other.rs".to_string()];
        let result = expand_globs(&args);
        assert_eq!(result, args);
    }

    #[test]
    fn test_expand_globs_quoted() {
        let args = vec!["\"*.rs\"".to_string()];
        let result = expand_globs(&args);
        assert_eq!(result, vec!["*.rs"]);
    }

    #[test]
    fn test_expand_star_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test files
        File::create(dir_path.join("file1.rs")).unwrap();
        File::create(dir_path.join("file2.rs")).unwrap();
        File::create(dir_path.join("other.txt")).unwrap();

        let pattern = format!("{}/*.rs", dir_path.display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.ends_with("file1.rs")));
        assert!(result.iter().any(|p| p.ends_with("file2.rs")));
    }

    #[test]
    fn test_expand_question_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test files
        File::create(dir_path.join("a.txt")).unwrap();
        File::create(dir_path.join("ab.txt")).unwrap();
        File::create(dir_path.join("abc.txt")).unwrap();

        let pattern = format!("{}/?.txt", dir_path.display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 1);
        assert!(result[0].ends_with("a.txt"));
    }

    #[test]
    fn test_expand_character_class() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test files
        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("file2.txt")).unwrap();
        File::create(dir_path.join("file3.txt")).unwrap();
        File::create(dir_path.join("filea.txt")).unwrap();

        let pattern = format!("{}/file[12].txt", dir_path.display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.ends_with("file1.txt")));
        assert!(result.iter().any(|p| p.ends_with("file2.txt")));
    }

    #[test]
    fn test_expand_range_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test files
        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("file2.txt")).unwrap();
        File::create(dir_path.join("file3.txt")).unwrap();

        let pattern = format!("{}/file[1-3].txt", dir_path.display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_no_match_returns_literal() {
        let temp_dir = TempDir::new().unwrap();
        let pattern = format!("{}/nonexistent*.xyz", temp_dir.path().display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], pattern);
    }

    #[test]
    fn test_hidden_files_excluded() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create regular and hidden files
        File::create(dir_path.join("visible.txt")).unwrap();
        File::create(dir_path.join(".hidden.txt")).unwrap();

        let pattern = format!("{}/*.txt", dir_path.display());
        let result = expand_single_glob(&pattern);

        // Should only match visible.txt, not .hidden.txt
        assert_eq!(result.len(), 1);
        assert!(result[0].ends_with("visible.txt"));
    }

    #[test]
    fn test_hidden_files_explicit() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create hidden files
        File::create(dir_path.join(".hidden1")).unwrap();
        File::create(dir_path.join(".hidden2")).unwrap();
        File::create(dir_path.join("visible")).unwrap();

        let pattern = format!("{}/.*", dir_path.display());
        let result = expand_single_glob(&pattern);

        // Should match hidden files (may include . and ..)
        assert!(result.iter().any(|p| p.ends_with(".hidden1")));
        assert!(result.iter().any(|p| p.ends_with(".hidden2")));
    }

    #[test]
    fn test_directory_glob() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create subdirectory with files
        let sub_dir = dir_path.join("src");
        fs::create_dir(&sub_dir).unwrap();
        File::create(sub_dir.join("main.rs")).unwrap();
        File::create(sub_dir.join("lib.rs")).unwrap();

        let pattern = format!("{}/src/*.rs", dir_path.display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_results_sorted() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create files in non-alphabetical order
        File::create(dir_path.join("zebra.txt")).unwrap();
        File::create(dir_path.join("apple.txt")).unwrap();
        File::create(dir_path.join("mango.txt")).unwrap();

        let pattern = format!("{}/*.txt", dir_path.display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 3);
        // Check alphabetical order (case-insensitive)
        assert!(result[0].ends_with("apple.txt"));
        assert!(result[1].ends_with("mango.txt"));
        assert!(result[2].ends_with("zebra.txt"));
    }

    #[test]
    fn test_escaped_glob_chars() {
        // Test that escaped glob chars don't trigger expansion
        let result = expand_single_glob(r"\*.rs");
        // Since there's no file literally named "*.rs", should return literal
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_negated_character_class() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("file2.txt")).unwrap();
        File::create(dir_path.join("filea.txt")).unwrap();

        // [!12] matches anything except 1 or 2
        let pattern = format!("{}/file[!12].txt", dir_path.display());
        let result = expand_single_glob(&pattern);

        assert_eq!(result.len(), 1);
        assert!(result[0].ends_with("filea.txt"));
    }

    #[test]
    fn test_multiple_globs_in_args() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        File::create(dir_path.join("a.rs")).unwrap();
        File::create(dir_path.join("b.txt")).unwrap();

        let args = vec![
            format!("{}/*.rs", dir_path.display()),
            format!("{}/*.txt", dir_path.display()),
        ];
        let result = expand_globs(&args);

        assert_eq!(result.len(), 2);
    }
}
