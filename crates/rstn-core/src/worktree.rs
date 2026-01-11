//! Git worktree parsing and management.
//!
//! Provides functions to:
//! - List worktrees for a project
//! - List available branches
//! - Create new worktrees (from existing or new branch)
//! - Remove worktrees

use crate::actions::WorktreeData;
use std::path::Path;
use std::process::Command;

/// Branch information for UI display
#[derive(Debug, Clone)]
pub struct BranchInfo {
    /// Branch name (e.g., "main", "feature/auth")
    pub name: String,
    /// Whether this branch already has a worktree
    pub has_worktree: bool,
    /// Whether this is the current branch in main worktree
    pub is_current: bool,
}

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
// Branch Management
// ============================================================================

/// List all branches in a repository.
///
/// Returns branches that can be used to create worktrees.
/// Branches that already have worktrees are marked.
pub fn list_branches(repo_path: &str) -> Result<Vec<BranchInfo>, String> {
    // Get all branches
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("branch")
        .arg("-a")
        .arg("--format=%(refname:short)%(if)%(HEAD)%(then)*%(end)")
        .output()
        .map_err(|e| format!("Failed to run git branch: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git branch failed: {}", stderr));
    }

    // Get existing worktrees to mark branches that already have one
    let worktrees = list_worktrees(repo_path).unwrap_or_default();
    let worktree_branches: std::collections::HashSet<_> =
        worktrees.iter().map(|w| w.branch.as_str()).collect();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut branches = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Skip remote tracking branches (origin/xxx)
        if line.contains("origin/") || line.contains("HEAD") {
            continue;
        }

        let is_current = line.ends_with('*');
        let name = line.trim_end_matches('*').to_string();
        let has_worktree = worktree_branches.contains(name.as_str());

        branches.push(BranchInfo {
            name,
            has_worktree,
            is_current,
        });
    }

    Ok(branches)
}

/// Generate the worktree path for a new worktree.
///
/// Uses Option B: sibling directory with project-branch naming.
/// Example: /Users/chris/projects/rustation-feature-auth
fn generate_worktree_path(repo_path: &str, branch: &str) -> String {
    let repo_path = Path::new(repo_path);
    let parent = repo_path.parent().unwrap_or(Path::new("/"));
    let repo_name = repo_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project");

    // Sanitize branch name for filesystem (replace / with -)
    let safe_branch = branch.replace('/', "-");

    let worktree_name = format!("{}-{}", repo_name, safe_branch);
    parent.join(worktree_name).to_string_lossy().to_string()
}

/// Create a new worktree from an existing branch.
///
/// The worktree will be created as a sibling directory.
/// Example: /projects/rustation -> /projects/rustation-feature-auth
pub fn add_worktree(repo_path: &str, branch: &str) -> Result<WorktreeData, String> {
    let worktree_path = generate_worktree_path(repo_path, branch);

    // Check if path already exists
    if Path::new(&worktree_path).exists() {
        return Err(format!("Path already exists: {}", worktree_path));
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("worktree")
        .arg("add")
        .arg(&worktree_path)
        .arg(branch)
        .output()
        .map_err(|e| format!("Failed to run git worktree add: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git worktree add failed: {}", stderr));
    }

    Ok(WorktreeData {
        path: worktree_path,
        branch: branch.to_string(),
        is_main: false,
    })
}

/// Create a new worktree with a new branch.
///
/// Creates a new branch from the current HEAD and checks it out in a new worktree.
pub fn add_worktree_new_branch(repo_path: &str, branch: &str) -> Result<WorktreeData, String> {
    let worktree_path = generate_worktree_path(repo_path, branch);

    // Check if path already exists
    if Path::new(&worktree_path).exists() {
        return Err(format!("Path already exists: {}", worktree_path));
    }

    // Check if branch already exists
    let branch_check = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("rev-parse")
        .arg("--verify")
        .arg(format!("refs/heads/{}", branch))
        .output()
        .map_err(|e| format!("Failed to check branch: {}", e))?;

    if branch_check.status.success() {
        return Err(format!("Branch '{}' already exists", branch));
    }

    // Create worktree with new branch (-b flag)
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("worktree")
        .arg("add")
        .arg("-b")
        .arg(branch)
        .arg(&worktree_path)
        .output()
        .map_err(|e| format!("Failed to run git worktree add: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git worktree add failed: {}", stderr));
    }

    Ok(WorktreeData {
        path: worktree_path,
        branch: branch.to_string(),
        is_main: false,
    })
}

/// Remove a worktree.
///
/// This removes the worktree directory and its git metadata.
/// Cannot remove the main worktree.
pub fn remove_worktree(repo_path: &str, worktree_path: &str) -> Result<(), String> {
    // Safety check: don't remove main worktree
    let worktrees = list_worktrees(repo_path)?;
    let worktree = worktrees
        .iter()
        .find(|w| w.path == worktree_path)
        .ok_or_else(|| format!("Worktree not found: {}", worktree_path))?;

    if worktree.is_main {
        return Err("Cannot remove the main worktree".to_string());
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("worktree")
        .arg("remove")
        .arg(worktree_path)
        .arg("--force")
        .output()
        .map_err(|e| format!("Failed to run git worktree remove: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git worktree remove failed: {}", stderr));
    }

    Ok(())
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

    #[test]
    fn test_generate_worktree_path() {
        // Simple branch
        let path = generate_worktree_path("/Users/chris/projects/rustation", "feature-auth");
        assert_eq!(path, "/Users/chris/projects/rustation-feature-auth");

        // Branch with slash
        let path = generate_worktree_path("/Users/chris/projects/rustation", "feature/auth");
        assert_eq!(path, "/Users/chris/projects/rustation-feature-auth");

        // Nested branch
        let path = generate_worktree_path("/Users/chris/projects/rustation", "fix/bug/123");
        assert_eq!(path, "/Users/chris/projects/rustation-fix-bug-123");
    }

    #[test]
    fn test_generate_worktree_path_preserves_parent() {
        let path = generate_worktree_path("/home/user/code/myproject", "develop");
        assert_eq!(path, "/home/user/code/myproject-develop");
    }
}
