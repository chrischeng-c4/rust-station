//! Living Context system for CESDD Phase 3.
//!
//! Manages the Living Context Layer (`.rstn/context/`) which stores
//! the system's current state and is auto-curated by rstn.

use std::path::Path;

use crate::app_state::{ContextFile, ContextType};

/// Default template for product.md
pub const PRODUCT_TEMPLATE: &str = r#"---
name: "Product"
type: product
last_updated: ""
token_estimate: 300
---

# Product Overview

> What this product does and who it's for.

## Description

[Describe the product's purpose and main functionality]

## Target Users

[Who uses this product and why]

## Key Features

- [Feature 1]
- [Feature 2]
- [Feature 3]
"#;

/// Default template for tech-stack.md
pub const TECH_STACK_TEMPLATE: &str = r#"---
name: "Tech Stack"
type: tech-stack
last_updated: ""
token_estimate: 400
---

# Technology Stack

> Current technology decisions and rationale.

## Languages

| Language | Version | Purpose |
|----------|---------|---------|
| | | |

## Frameworks & Libraries

| Name | Version | Purpose |
|------|---------|---------|
| | | |

## Key Decisions

| Decision | Rationale | Date |
|----------|-----------|------|
| | | |
"#;

/// Default template for system-architecture.md
pub const ARCHITECTURE_TEMPLATE: &str = r#"---
name: "System Architecture"
type: architecture
last_updated: ""
token_estimate: 500
---

# System Architecture

> High-level system design and component relationships.

## Overview

[High-level description of the system]

## Components

### Component 1

[Description]

### Component 2

[Description]

## Data Flow

[How data flows through the system]
"#;

/// Default template for recent-changes.md
pub const RECENT_CHANGES_TEMPLATE: &str = r#"---
name: "Recent Changes"
type: recent-changes
last_updated: ""
token_estimate: 300
---

# Recent Changes

> Summary of recent significant changes.

## Latest Changes

| Date | Change | Impact |
|------|--------|--------|
| | | |

## Pending Changes

[Any changes in progress]
"#;

/// Check if context directory exists and has files
pub fn context_exists(project_path: &Path) -> bool {
    let context_dir = project_path.join(".rstn").join("context");
    if !context_dir.exists() || !context_dir.is_dir() {
        return false;
    }

    // Check if there's at least one .md file
    if let Ok(entries) = std::fs::read_dir(&context_dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" {
                    return true;
                }
            }
        }
    }

    false
}

/// Read all context files from .rstn/context/
pub fn read_context(project_path: &Path) -> Vec<ContextFile> {
    let context_dir = project_path.join(".rstn").join("context");
    let mut files = Vec::new();

    if !context_dir.exists() || !context_dir.is_dir() {
        return files;
    }

    if let Ok(entries) = std::fs::read_dir(&context_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let filename = path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();

                    let name = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();

                    let context_type = ContextType::from_filename(&filename);
                    let last_updated = extract_last_updated(&content);
                    let token_estimate = extract_token_estimate(&content);

                    files.push(ContextFile {
                        name,
                        path: path.to_string_lossy().to_string(),
                        content,
                        context_type,
                        last_updated,
                        token_estimate,
                    });
                }
            }
        }
    }

    // Sort by context type for consistent ordering
    files.sort_by(|a, b| {
        let a_order = context_type_order(&a.context_type);
        let b_order = context_type_order(&b.context_type);
        a_order.cmp(&b_order)
    });

    files
}

/// Get ordering for context types (product first, recent-changes last)
fn context_type_order(ct: &ContextType) -> u8 {
    match ct {
        ContextType::Product => 0,
        ContextType::TechStack => 1,
        ContextType::Architecture => 2,
        ContextType::ApiContracts => 3,
        ContextType::DataModels => 4,
        ContextType::RecentChanges => 5,
        ContextType::Custom => 6,
    }
}

/// Read combined context content for prompts
pub fn read_context_combined(project_path: &Path) -> Option<String> {
    let files = read_context(project_path);
    if files.is_empty() {
        return None;
    }

    let combined = files
        .into_iter()
        .map(|f| f.content)
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    Some(combined)
}

/// Extract last_updated from YAML frontmatter
fn extract_last_updated(content: &str) -> String {
    if let Some(stripped) = content.strip_prefix("---") {
        if let Some(end_idx) = stripped.find("---") {
            let frontmatter = &stripped[..end_idx];
            for line in frontmatter.lines() {
                let line = line.trim();
                if let Some(value) = line.strip_prefix("last_updated:") {
                    return value.trim().trim_matches('"').to_string();
                }
            }
        }
    }
    String::new()
}

/// Extract token_estimate from YAML frontmatter
fn extract_token_estimate(content: &str) -> u32 {
    if let Some(stripped) = content.strip_prefix("---") {
        if let Some(end_idx) = stripped.find("---") {
            let frontmatter = &stripped[..end_idx];
            for line in frontmatter.lines() {
                let line = line.trim();
                if let Some(value) = line.strip_prefix("token_estimate:") {
                    if let Ok(estimate) = value.trim().parse::<u32>() {
                        return estimate;
                    }
                }
            }
        }
    }
    300 // Default estimate
}

/// Initialize context directory with default templates
pub async fn initialize_context(project_path: &Path) -> Result<(), String> {
    let rstn_dir = project_path.join(".rstn");
    let context_dir = rstn_dir.join("context");

    // Create directory
    tokio::fs::create_dir_all(&context_dir)
        .await
        .map_err(|e| format!("Failed to create context directory: {}", e))?;

    // Write default templates
    let timestamp = chrono::Utc::now().to_rfc3339();

    // product.md
    let product_content = PRODUCT_TEMPLATE.replace("last_updated: \"\"", &format!("last_updated: \"{}\"", timestamp));
    tokio::fs::write(context_dir.join("product.md"), product_content)
        .await
        .map_err(|e| format!("Failed to write product.md: {}", e))?;

    // tech-stack.md
    let tech_content = TECH_STACK_TEMPLATE.replace("last_updated: \"\"", &format!("last_updated: \"{}\"", timestamp));
    tokio::fs::write(context_dir.join("tech-stack.md"), tech_content)
        .await
        .map_err(|e| format!("Failed to write tech-stack.md: {}", e))?;

    // system-architecture.md
    let arch_content = ARCHITECTURE_TEMPLATE.replace("last_updated: \"\"", &format!("last_updated: \"{}\"", timestamp));
    tokio::fs::write(context_dir.join("system-architecture.md"), arch_content)
        .await
        .map_err(|e| format!("Failed to write system-architecture.md: {}", e))?;

    // recent-changes.md
    let changes_content = RECENT_CHANGES_TEMPLATE.replace("last_updated: \"\"", &format!("last_updated: \"{}\"", timestamp));
    tokio::fs::write(context_dir.join("recent-changes.md"), changes_content)
        .await
        .map_err(|e| format!("Failed to write recent-changes.md: {}", e))?;

    Ok(())
}

/// Update a single context file
pub async fn update_context_file(
    project_path: &Path,
    name: &str,
    content: &str,
) -> Result<(), String> {
    let context_dir = project_path.join(".rstn").join("context");

    if !context_dir.exists() {
        return Err("Context directory does not exist".to_string());
    }

    let filename = if name.ends_with(".md") {
        name.to_string()
    } else {
        format!("{}.md", name)
    };

    let file_path = context_dir.join(&filename);

    // Update last_updated timestamp in content
    let timestamp = chrono::Utc::now().to_rfc3339();
    let updated_content = update_frontmatter_timestamp(content, &timestamp);

    tokio::fs::write(&file_path, updated_content)
        .await
        .map_err(|e| format!("Failed to write {}: {}", filename, e))?;

    Ok(())
}

/// Update the last_updated field in frontmatter
fn update_frontmatter_timestamp(content: &str, timestamp: &str) -> String {
    if let Some(stripped) = content.strip_prefix("---") {
        if let Some(end_idx) = stripped.find("---") {
            let frontmatter = &stripped[..end_idx];
            let body = &stripped[end_idx..];

            // Update or add last_updated
            let mut new_frontmatter = String::new();
            let mut found = false;

            for line in frontmatter.lines() {
                if line.trim().starts_with("last_updated:") {
                    new_frontmatter.push_str(&format!("last_updated: \"{}\"\n", timestamp));
                    found = true;
                } else {
                    new_frontmatter.push_str(line);
                    new_frontmatter.push('\n');
                }
            }

            if !found {
                new_frontmatter.push_str(&format!("last_updated: \"{}\"\n", timestamp));
            }

            return format!("---{}---{}", new_frontmatter.trim_end(), body);
        }
    }

    // No frontmatter, return as-is
    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_context_exists_none() {
        let temp_dir = TempDir::new().unwrap();
        assert!(!context_exists(temp_dir.path()));
    }

    #[test]
    fn test_context_exists_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let context_dir = temp_dir.path().join(".rstn").join("context");
        std::fs::create_dir_all(&context_dir).unwrap();
        std::fs::write(context_dir.join("product.md"), PRODUCT_TEMPLATE).unwrap();

        assert!(context_exists(temp_dir.path()));
    }

    #[test]
    fn test_read_context_empty() {
        let temp_dir = TempDir::new().unwrap();
        let files = read_context(temp_dir.path());
        assert!(files.is_empty());
    }

    #[test]
    fn test_read_context_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let context_dir = temp_dir.path().join(".rstn").join("context");
        std::fs::create_dir_all(&context_dir).unwrap();

        std::fs::write(context_dir.join("product.md"), PRODUCT_TEMPLATE).unwrap();
        std::fs::write(context_dir.join("tech-stack.md"), TECH_STACK_TEMPLATE).unwrap();

        let files = read_context(temp_dir.path());
        assert_eq!(files.len(), 2);

        // Should be sorted: product first, then tech-stack
        assert_eq!(files[0].name, "product");
        assert_eq!(files[1].name, "tech-stack");
    }

    #[test]
    fn test_extract_token_estimate() {
        let content = "---\ntoken_estimate: 500\n---\ncontent";
        assert_eq!(extract_token_estimate(content), 500);

        let content_no_estimate = "---\nname: test\n---\ncontent";
        assert_eq!(extract_token_estimate(content_no_estimate), 300); // default

        let no_frontmatter = "just content";
        assert_eq!(extract_token_estimate(no_frontmatter), 300); // default
    }

    #[test]
    fn test_extract_last_updated() {
        let content = "---\nlast_updated: \"2025-01-01\"\n---\ncontent";
        assert_eq!(extract_last_updated(content), "2025-01-01");

        let content_no_date = "---\nname: test\n---\ncontent";
        assert_eq!(extract_last_updated(content_no_date), "");
    }

    #[tokio::test]
    async fn test_initialize_context() {
        let temp_dir = TempDir::new().unwrap();

        initialize_context(temp_dir.path()).await.unwrap();

        let context_dir = temp_dir.path().join(".rstn").join("context");
        assert!(context_dir.join("product.md").exists());
        assert!(context_dir.join("tech-stack.md").exists());
        assert!(context_dir.join("system-architecture.md").exists());
        assert!(context_dir.join("recent-changes.md").exists());
    }

    #[test]
    fn test_read_context_combined() {
        let temp_dir = TempDir::new().unwrap();
        let context_dir = temp_dir.path().join(".rstn").join("context");
        std::fs::create_dir_all(&context_dir).unwrap();

        std::fs::write(context_dir.join("product.md"), "# Product").unwrap();
        std::fs::write(context_dir.join("tech-stack.md"), "# Tech").unwrap();

        let combined = read_context_combined(temp_dir.path());
        assert!(combined.is_some());

        let content = combined.unwrap();
        assert!(content.contains("# Product"));
        assert!(content.contains("# Tech"));
        assert!(content.contains("---")); // separator
    }
}
