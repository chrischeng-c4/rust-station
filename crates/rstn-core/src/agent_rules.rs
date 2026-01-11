//! Agent rules file management
//!
//! Generates temporary system prompt files for Claude Code CLI.
//!
//! When custom agent rules are enabled, this module creates a temporary text file
//! that Claude Code CLI can use to override the default CLAUDE.md behavior
//! via the `--system-prompt-file` flag.

use std::fs;
use std::path::PathBuf;

/// Generate agent rules file for Claude Code CLI
///
/// Creates a temporary text file at `/tmp/rstn-agent-rules-{project_id}.txt` containing
/// the custom system prompt that will replace Claude Code's default CLAUDE.md behavior.
///
/// This allows different projects to have different Claude behavior and rules.
///
/// # Arguments
/// * `project_id` - Unique identifier for the project (used in filename)
/// * `prompt_content` - The custom system prompt text to write
///
/// # Returns
/// * `Ok(String)` - Absolute path to the generated rules file
/// * `Err(String)` - Error message if file creation fails
///
/// # Example
/// ```no_run
/// let rules_path = generate_agent_rules_file(
///     "my-project-123",
///     "You are a helpful Rust developer. Always use snake_case."
/// )?;
/// // Creates: /tmp/rstn-agent-rules-my-project-123.txt
/// ```
pub fn generate_agent_rules_file(
    project_id: &str,
    prompt_content: &str,
) -> Result<String, String> {
    // Sanitize project_id to prevent path traversal
    let safe_project_id = project_id
        .replace(['/', '\\'], "-")
        .replace("..", "-");

    // Build rules file path
    let temp_dir = std::env::temp_dir();
    let filename = format!("rstn-agent-rules-{}.txt", safe_project_id);
    let rules_path = temp_dir.join(filename);

    // Write prompt content to file
    fs::write(&rules_path, prompt_content)
        .map_err(|e| format!("Failed to write agent rules to {:?}: {}", rules_path, e))?;

    // Return absolute path as string
    rules_path
        .to_str()
        .ok_or_else(|| "Rules path contains invalid UTF-8".to_string())
        .map(|s| s.to_string())
}

/// Remove agent rules file
///
/// Deletes the temporary rules file. Idempotent - does not error if file doesn't exist.
///
/// # Arguments
/// * `rules_path` - Absolute path to the rules file
///
/// # Returns
/// * `Ok(())` - File deleted or didn't exist
/// * `Err(String)` - Error message if deletion fails (permissions, etc.)
///
/// # Example
/// ```no_run
/// cleanup_agent_rules_file("/tmp/rstn-agent-rules-my-project-123.txt")?;
/// // File is deleted, or already was missing (both OK)
/// ```
pub fn cleanup_agent_rules_file(rules_path: &str) -> Result<(), String> {
    let path = PathBuf::from(rules_path);

    // Check if file exists
    if !path.exists() {
        // Idempotent - not an error if file already gone
        return Ok(());
    }

    // Delete file
    fs::remove_file(&path)
        .map_err(|e| format!("Failed to delete agent rules at {:?}: {}", path, e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_agent_rules_file() {
        let project_id = "test-project-123";
        let prompt_content = "You are a helpful Rust developer.\nAlways use snake_case.";

        // Generate rules file
        let result = generate_agent_rules_file(project_id, prompt_content);
        assert!(result.is_ok(), "Rules generation should succeed");

        let rules_path = result.unwrap();
        assert!(
            rules_path.contains("rstn-agent-rules-test-project-123.txt"),
            "Rules path should contain project ID"
        );

        // Verify file exists
        let path = PathBuf::from(&rules_path);
        assert!(path.exists(), "Rules file should exist");

        // Read and verify content
        let contents = fs::read_to_string(&path).expect("Should read rules file");
        assert_eq!(
            contents, prompt_content,
            "File content should match input"
        );

        // Cleanup
        cleanup_agent_rules_file(&rules_path).ok();
    }

    #[test]
    fn test_cleanup_agent_rules_file() {
        // Create a temp file
        let project_id = "test-cleanup";
        let rules_path = generate_agent_rules_file(project_id, "test content")
            .expect("Should generate rules");

        let path = PathBuf::from(&rules_path);
        assert!(path.exists(), "Rules should exist before cleanup");

        // Clean up
        let result = cleanup_agent_rules_file(&rules_path);
        assert!(result.is_ok(), "Cleanup should succeed");
        assert!(!path.exists(), "Rules should not exist after cleanup");

        // Idempotent - cleanup again should not error
        let result2 = cleanup_agent_rules_file(&rules_path);
        assert!(result2.is_ok(), "Cleanup should be idempotent");
    }

    #[test]
    fn test_generate_rules_sanitizes_project_id() {
        // Test with project_id containing path traversal attempts
        let dangerous_id = "../../../etc/passwd";
        let result = generate_agent_rules_file(dangerous_id, "test");

        assert!(result.is_ok(), "Should handle dangerous IDs");

        let rules_path = result.unwrap();

        // Verify path doesn't contain traversal sequences
        assert!(
            !rules_path.contains(".."),
            "Rules path should not contain '..' after sanitization"
        );
        assert!(
            !rules_path.contains("/etc/"),
            "Rules path should not escape to /etc/"
        );

        // Cleanup
        cleanup_agent_rules_file(&rules_path).ok();
    }

    #[test]
    fn test_generate_rules_different_content() {
        let project_id = "test-content";

        // Test with various content types
        let long_content = "Very long content ".repeat(100);
        let test_cases = vec![
            "Simple one-liner",
            "Multi-line\nwith\nnewlines",
            "Unicode: 你好世界 🦀",
            "Empty content is OK",
            long_content.as_str(),
        ];

        for (i, content) in test_cases.iter().enumerate() {
            let test_id = format!("{}-{}", project_id, i);
            let rules_path = generate_agent_rules_file(&test_id, content)
                .expect("Should generate rules");

            let read_content = fs::read_to_string(&rules_path)
                .expect("Should read rules");
            assert_eq!(
                &read_content, content,
                "Content should match for test case {}",
                i
            );

            cleanup_agent_rules_file(&rules_path).ok();
        }
    }

    #[test]
    fn test_empty_prompt_handling() {
        let project_id = "test-empty";
        let empty_content = "";

        // Should still create file, even if empty
        let result = generate_agent_rules_file(project_id, empty_content);
        assert!(result.is_ok(), "Should handle empty content");

        let rules_path = result.unwrap();
        let path = PathBuf::from(&rules_path);
        assert!(path.exists(), "File should exist even with empty content");

        let contents = fs::read_to_string(&path).expect("Should read file");
        assert_eq!(contents, "", "Content should be empty");

        cleanup_agent_rules_file(&rules_path).ok();
    }
}
