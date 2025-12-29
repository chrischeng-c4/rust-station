//! Archive system for CESDD Phase 4.
//!
//! Manages the archive layer (`.rstn/archive/`) which stores
//! completed changes for historical reference.

use std::path::Path;

/// Check if archive directory exists
pub fn archive_exists(project_path: &Path) -> bool {
    let archive_dir = project_path.join(".rstn").join("archive");
    archive_dir.exists() && archive_dir.is_dir()
}

/// List all archived changes
pub fn list_archived_changes(project_path: &Path) -> Vec<String> {
    let archive_dir = project_path.join(".rstn").join("archive");
    let mut changes = Vec::new();

    if !archive_dir.exists() || !archive_dir.is_dir() {
        return changes;
    }

    if let Ok(entries) = std::fs::read_dir(&archive_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    changes.push(name.to_string());
                }
            }
        }
    }

    changes.sort();
    changes
}

/// Archive a change (move from .rstn/changes/<name>/ to .rstn/archive/<name>/)
pub async fn archive_change(project_path: &Path, change_name: &str) -> Result<(), String> {
    let rstn_dir = project_path.join(".rstn");
    let changes_dir = rstn_dir.join("changes");
    let archive_dir = rstn_dir.join("archive");

    let source = changes_dir.join(change_name);
    let destination = archive_dir.join(change_name);

    // Verify source exists
    if !source.exists() {
        return Err(format!(
            "Change directory does not exist: {}",
            source.display()
        ));
    }

    // Create archive directory if it doesn't exist
    tokio::fs::create_dir_all(&archive_dir)
        .await
        .map_err(|e| format!("Failed to create archive directory: {}", e))?;

    // Check if destination already exists (shouldn't happen, but handle it)
    if destination.exists() {
        // Add timestamp suffix to avoid collision
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let new_name = format!("{}-{}", change_name, timestamp);
        let new_destination = archive_dir.join(&new_name);

        tokio::fs::rename(&source, &new_destination)
            .await
            .map_err(|e| format!("Failed to archive change: {}", e))?;
    } else {
        tokio::fs::rename(&source, &destination)
            .await
            .map_err(|e| format!("Failed to archive change: {}", e))?;
    }

    Ok(())
}

/// Read archived change's proposal
pub fn read_archived_proposal(project_path: &Path, change_name: &str) -> Option<String> {
    let proposal_path = project_path
        .join(".rstn")
        .join("archive")
        .join(change_name)
        .join("proposal.md");

    std::fs::read_to_string(proposal_path).ok()
}

/// Read archived change's plan
pub fn read_archived_plan(project_path: &Path, change_name: &str) -> Option<String> {
    let plan_path = project_path
        .join(".rstn")
        .join("archive")
        .join(change_name)
        .join("plan.md");

    std::fs::read_to_string(plan_path).ok()
}

/// Read change's proposal (from active changes directory)
pub fn read_change_proposal(project_path: &Path, change_name: &str) -> Option<String> {
    let proposal_path = project_path
        .join(".rstn")
        .join("changes")
        .join(change_name)
        .join("proposal.md");

    std::fs::read_to_string(proposal_path).ok()
}

/// Read change's plan (from active changes directory)
pub fn read_change_plan(project_path: &Path, change_name: &str) -> Option<String> {
    let plan_path = project_path
        .join(".rstn")
        .join("changes")
        .join(change_name)
        .join("plan.md");

    std::fs::read_to_string(plan_path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_archive_exists_none() {
        let temp_dir = TempDir::new().unwrap();
        assert!(!archive_exists(temp_dir.path()));
    }

    #[test]
    fn test_archive_exists_with_dir() {
        let temp_dir = TempDir::new().unwrap();
        let archive_dir = temp_dir.path().join(".rstn").join("archive");
        std::fs::create_dir_all(&archive_dir).unwrap();

        assert!(archive_exists(temp_dir.path()));
    }

    #[test]
    fn test_list_archived_changes_empty() {
        let temp_dir = TempDir::new().unwrap();
        let changes = list_archived_changes(temp_dir.path());
        assert!(changes.is_empty());
    }

    #[test]
    fn test_list_archived_changes_with_items() {
        let temp_dir = TempDir::new().unwrap();
        let archive_dir = temp_dir.path().join(".rstn").join("archive");

        // Create some archived changes
        std::fs::create_dir_all(archive_dir.join("feature-auth")).unwrap();
        std::fs::create_dir_all(archive_dir.join("fix-bug-123")).unwrap();
        std::fs::create_dir_all(archive_dir.join("add-api")).unwrap();

        let changes = list_archived_changes(temp_dir.path());
        assert_eq!(changes.len(), 3);
        // Should be sorted
        assert_eq!(changes[0], "add-api");
        assert_eq!(changes[1], "feature-auth");
        assert_eq!(changes[2], "fix-bug-123");
    }

    #[tokio::test]
    async fn test_archive_change() {
        let temp_dir = TempDir::new().unwrap();
        let changes_dir = temp_dir.path().join(".rstn").join("changes");
        let change_dir = changes_dir.join("feature-auth");

        // Create a change with proposal and plan
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "# Proposal\n\nAdd auth").unwrap();
        std::fs::write(change_dir.join("plan.md"), "# Plan\n\n1. Do stuff").unwrap();

        // Archive it
        archive_change(temp_dir.path(), "feature-auth")
            .await
            .unwrap();

        // Verify source is gone
        assert!(!change_dir.exists());

        // Verify destination exists
        let archived_dir = temp_dir.path().join(".rstn").join("archive").join("feature-auth");
        assert!(archived_dir.exists());
        assert!(archived_dir.join("proposal.md").exists());
        assert!(archived_dir.join("plan.md").exists());
    }

    #[tokio::test]
    async fn test_archive_change_not_found() {
        let temp_dir = TempDir::new().unwrap();

        let result = archive_change(temp_dir.path(), "nonexistent").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_read_change_proposal() {
        let temp_dir = TempDir::new().unwrap();
        let change_dir = temp_dir.path().join(".rstn").join("changes").join("feature-x");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "# My Proposal").unwrap();

        let content = read_change_proposal(temp_dir.path(), "feature-x");
        assert!(content.is_some());
        assert_eq!(content.unwrap(), "# My Proposal");
    }

    #[test]
    fn test_read_change_plan() {
        let temp_dir = TempDir::new().unwrap();
        let change_dir = temp_dir.path().join(".rstn").join("changes").join("feature-x");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(change_dir.join("plan.md"), "# My Plan").unwrap();

        let content = read_change_plan(temp_dir.path(), "feature-x");
        assert!(content.is_some());
        assert_eq!(content.unwrap(), "# My Plan");
    }

    #[test]
    fn test_read_archived_proposal() {
        let temp_dir = TempDir::new().unwrap();
        let archive_dir = temp_dir.path().join(".rstn").join("archive").join("old-feature");
        std::fs::create_dir_all(&archive_dir).unwrap();
        std::fs::write(archive_dir.join("proposal.md"), "# Old Proposal").unwrap();

        let content = read_archived_proposal(temp_dir.path(), "old-feature");
        assert!(content.is_some());
        assert_eq!(content.unwrap(), "# Old Proposal");
    }
}
