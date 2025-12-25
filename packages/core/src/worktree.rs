//! Git worktree parsing and management.
//!
//! Parses the output of `git worktree list` to get all worktrees for a project.

use crate::actions::WorktreeData;
use std::process::Command;

/// Parse the output of `git worktree list` for a given repo path.
///
/// Returns a list of worktrees with their paths, branches, and whether they're the main worktree.
///
/// Example output from `git worktree list`:
/// ```text
/// /Users/chris/projects/rustation           abc1234 [main]
/// /Users/chris/projects/rustation-feature   def5678 [feature/auth]
/// ```
pub fn list_worktrees(repo_path: &str) -> Result<Vec<WorktreeData>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("worktree")
        .arg("list")
        .output()
        .map_err(|e| format!("Failed to run git worktree list: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git worktree list failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_worktree_list(&stdout, repo_path)
}

/// Parse the output of `git worktree list`.
fn parse_worktree_list(output: &str, main_worktree_path: &str) -> Result<Vec<WorktreeData>, String> {
    let mut worktrees = Vec::new();

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some(worktree) = parse_worktree_line(line, main_worktree_path) {
            worktrees.push(worktree);
        }
    }

    // If no worktrees found, create a default main worktree
    if worktrees.is_empty() {
        worktrees.push(WorktreeData {
            path: main_worktree_path.to_string(),
            branch: "main".to_string(),
            is_main: true,
        });
    }

    Ok(worktrees)
}

/// Parse a single line from `git worktree list` output.
///
/// Format: `/path/to/worktree  abc1234 [branch-name]`
fn parse_worktree_line(line: &str, main_worktree_path: &str) -> Option<WorktreeData> {
    // Split by whitespace, but be careful with paths that might have spaces
    // The format is: <path> <commit> [<branch>] or <path> <commit> (bare/detached)

    // Find the branch name in square brackets
    let branch = if let Some(start) = line.rfind('[') {
        if let Some(end) = line.rfind(']') {
            if start < end {
                Some(line[start + 1..end].to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        // Check for (detached HEAD) or (bare)
        if line.contains("(detached HEAD)") {
            Some("HEAD (detached)".to_string())
        } else if line.contains("(bare)") {
            return None; // Skip bare repos
        } else {
            None
        }
    };

    // Extract the path (everything before the first space after the path)
    // The path is at the beginning, followed by whitespace and commit hash
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    // The path is the first part
    let path = parts[0].to_string();

    // Determine if this is the main worktree
    // Main worktree is the one that matches the repo_path
    let is_main = path == main_worktree_path
        || path.trim_end_matches('/') == main_worktree_path.trim_end_matches('/');

    Some(WorktreeData {
        path,
        branch: branch.unwrap_or_else(|| "unknown".to_string()),
        is_main,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_worktree() {
        let output = "/Users/chris/projects/rustation  abc1234 [main]";
        let result = parse_worktree_list(output, "/Users/chris/projects/rustation").unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "/Users/chris/projects/rustation");
        assert_eq!(result[0].branch, "main");
        assert!(result[0].is_main);
    }

    #[test]
    fn test_parse_multiple_worktrees() {
        let output = r#"/Users/chris/projects/rustation         abc1234 [main]
/Users/chris/projects/rustation-feature  def5678 [feature/auth]
/Users/chris/projects/rustation-fix      ghi9012 [fix/bug-123]"#;

        let result = parse_worktree_list(output, "/Users/chris/projects/rustation").unwrap();

        assert_eq!(result.len(), 3);

        assert_eq!(result[0].path, "/Users/chris/projects/rustation");
        assert_eq!(result[0].branch, "main");
        assert!(result[0].is_main);

        assert_eq!(result[1].path, "/Users/chris/projects/rustation-feature");
        assert_eq!(result[1].branch, "feature/auth");
        assert!(!result[1].is_main);

        assert_eq!(result[2].path, "/Users/chris/projects/rustation-fix");
        assert_eq!(result[2].branch, "fix/bug-123");
        assert!(!result[2].is_main);
    }

    #[test]
    fn test_parse_detached_head() {
        let output = "/Users/chris/projects/rustation  abc1234 (detached HEAD)";
        let result = parse_worktree_list(output, "/Users/chris/projects/rustation").unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].branch, "HEAD (detached)");
    }

    #[test]
    fn test_empty_output_creates_default() {
        let result = parse_worktree_list("", "/Users/chris/projects/rustation").unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "/Users/chris/projects/rustation");
        assert_eq!(result[0].branch, "main");
        assert!(result[0].is_main);
    }

    #[test]
    fn test_parse_worktree_line() {
        let line = "/path/to/worktree  abc1234 [feature/test]";
        let result = parse_worktree_line(line, "/path/main").unwrap();

        assert_eq!(result.path, "/path/to/worktree");
        assert_eq!(result.branch, "feature/test");
        assert!(!result.is_main);
    }

    #[test]
    fn test_parse_worktree_line_main() {
        let line = "/path/main  abc1234 [main]";
        let result = parse_worktree_line(line, "/path/main").unwrap();

        assert!(result.is_main);
    }
}
