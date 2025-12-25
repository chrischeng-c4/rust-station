//! Git worktree parsing and management.
//!
//! Parses the output of `git worktree list` to get all worktrees for a project.

use crate::actions::WorktreeData;
use std::path::Path;
use std::process::Command;

/// Get the git repository root for a given path.
///
/// Uses `git rev-parse --show-toplevel` to find the root.
/// Returns None if the path is not inside a git repository.
///
/// # Arguments
/// * `path` - Any path, can be the repo root, a subdirectory, or a file
///
/// # Returns
/// * `Some(root_path)` - The absolute path to the git repo root
/// * `None` - If not inside a git repository
pub fn get_git_root(path: &str) -> Option<String> {
    // Determine the directory to check
    let check_path = if Path::new(path).is_file() {
        Path::new(path).parent()?.to_str()?
    } else {
        path
    };

    let output = Command::new("git")
        .arg("-C")
        .arg(check_path)
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let root = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if root.is_empty() {
        None
    } else {
        Some(root)
    }
}

/// Check if a path is inside any worktree of a project.
///
/// # Arguments
/// * `path` - The path to check
/// * `project_path` - The main worktree path of the project
///
/// # Returns
/// * `Some(worktree_index)` - The index of the matching worktree
/// * `None` - If the path is not inside any worktree
pub fn find_worktree_for_path(path: &str, worktrees: &[WorktreeData]) -> Option<usize> {
    let check_path = Path::new(path);

    for (idx, wt) in worktrees.iter().enumerate() {
        let wt_path = Path::new(&wt.path);
        // Check if path starts with worktree path (is inside or equal to)
        if check_path.starts_with(wt_path) {
            return Some(idx);
        }
    }
    None
}

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

    #[test]
    fn test_find_worktree_for_path_exact_match() {
        let worktrees = vec![
            WorktreeData {
                path: "/projects/main".to_string(),
                branch: "main".to_string(),
                is_main: true,
            },
            WorktreeData {
                path: "/projects/feature".to_string(),
                branch: "feature/test".to_string(),
                is_main: false,
            },
        ];

        assert_eq!(find_worktree_for_path("/projects/main", &worktrees), Some(0));
        assert_eq!(find_worktree_for_path("/projects/feature", &worktrees), Some(1));
    }

    #[test]
    fn test_find_worktree_for_path_subdirectory() {
        let worktrees = vec![
            WorktreeData {
                path: "/projects/main".to_string(),
                branch: "main".to_string(),
                is_main: true,
            },
        ];

        assert_eq!(find_worktree_for_path("/projects/main/src/lib.rs", &worktrees), Some(0));
        assert_eq!(find_worktree_for_path("/projects/main/packages/core", &worktrees), Some(0));
    }

    #[test]
    fn test_find_worktree_for_path_no_match() {
        let worktrees = vec![
            WorktreeData {
                path: "/projects/main".to_string(),
                branch: "main".to_string(),
                is_main: true,
            },
        ];

        assert_eq!(find_worktree_for_path("/other/project", &worktrees), None);
        assert_eq!(find_worktree_for_path("/projects/other", &worktrees), None);
    }
}
