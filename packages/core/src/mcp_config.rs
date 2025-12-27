//! MCP configuration file management
//!
//! Generates temporary config files for Claude Code CLI MCP integration.
//!
//! When rstn's MCP server is running, this module creates a configuration file
//! that Claude Code CLI can use to connect to the server via the `--mcp-config` flag.

use serde_json::json;
use std::fs;
use std::path::PathBuf;

/// Generate MCP config file for Claude Code CLI
///
/// Creates a temporary JSON file at `/tmp/rstn-mcp-{worktree_id}.json` with format:
/// ```json
/// {
///   "mcpServers": {
///     "rstn": {
///       "type": "http",
///       "url": "http://localhost:PORT"
///     }
///   }
/// }
/// ```
///
/// This config file enables Claude Code to access rstn's MCP tools:
/// - `rstn::read_file`: Read files within the worktree
/// - `rstn::list_directory`: List directory contents
/// - `rstn::get_project_context`: Get project metadata
/// - `rstn::run_just_task`: Execute Just tasks
///
/// # Arguments
/// * `worktree_id` - Unique identifier for the worktree (used in filename)
/// * `port` - Port where MCP server is running
///
/// # Returns
/// * `Ok(String)` - Absolute path to the generated config file
/// * `Err(String)` - Error message if file creation fails
///
/// # Example
/// ```no_run
/// let config_path = generate_mcp_config_file("my-worktree-123", 3000)?;
/// // Creates: /tmp/rstn-mcp-my-worktree-123.json
/// ```
pub fn generate_mcp_config_file(worktree_id: &str, port: u16) -> Result<String, String> {
    // Sanitize worktree_id to prevent path traversal
    let safe_worktree_id = worktree_id
        .replace(['/', '\\'], "-")
        .replace("..", "-");

    // Build config file path
    let temp_dir = std::env::temp_dir();
    let filename = format!("rstn-mcp-{}.json", safe_worktree_id);
    let config_path = temp_dir.join(filename);

    // Create MCP config JSON structure
    // CRITICAL: Use "type": "http" (NOT "transport") per MCP schema
    let config = json!({
        "mcpServers": {
            "rstn": {
                "type": "http",
                "url": format!("http://localhost:{}", port)
            }
        }
    });

    // Serialize to pretty JSON
    let json_string = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize MCP config: {}", e))?;

    // Write to file
    fs::write(&config_path, json_string)
        .map_err(|e| format!("Failed to write MCP config to {:?}: {}", config_path, e))?;

    // Return absolute path as string
    config_path
        .to_str()
        .ok_or_else(|| "Config path contains invalid UTF-8".to_string())
        .map(|s| s.to_string())
}

/// Remove MCP config file
///
/// Deletes the temporary config file. Idempotent - does not error if file doesn't exist.
///
/// # Arguments
/// * `config_path` - Absolute path to the config file
///
/// # Returns
/// * `Ok(())` - File deleted or didn't exist
/// * `Err(String)` - Error message if deletion fails (permissions, etc.)
///
/// # Example
/// ```no_run
/// cleanup_mcp_config_file("/tmp/rstn-mcp-my-worktree-123.json")?;
/// // File is deleted, or already was missing (both OK)
/// ```
pub fn cleanup_mcp_config_file(config_path: &str) -> Result<(), String> {
    let path = PathBuf::from(config_path);

    // Check if file exists
    if !path.exists() {
        // Idempotent - not an error if file already gone
        return Ok(());
    }

    // Delete file
    fs::remove_file(&path)
        .map_err(|e| format!("Failed to delete MCP config at {:?}: {}", path, e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mcp_config_file() {
        let worktree_id = "test-worktree-123";
        let port = 3000;

        // Generate config
        let result = generate_mcp_config_file(worktree_id, port);
        assert!(result.is_ok(), "Config generation should succeed");

        let config_path = result.unwrap();
        assert!(
            config_path.contains("rstn-mcp-test-worktree-123.json"),
            "Config path should contain worktree ID"
        );

        // Verify file exists
        let path = PathBuf::from(&config_path);
        assert!(path.exists(), "Config file should exist");

        // Read and parse JSON
        let contents = fs::read_to_string(&path).expect("Should read config file");
        let json: serde_json::Value = serde_json::from_str(&contents)
            .expect("Should parse JSON");

        // Verify schema
        assert_eq!(
            json["mcpServers"]["rstn"]["type"],
            "http",
            "Should use 'type' field with 'http' value"
        );
        assert_eq!(
            json["mcpServers"]["rstn"]["url"],
            "http://localhost:3000",
            "Should have correct URL"
        );

        // Cleanup
        cleanup_mcp_config_file(&config_path).ok();
    }

    #[test]
    fn test_cleanup_mcp_config_file() {
        // Create a temp file
        let worktree_id = "test-cleanup";
        let config_path = generate_mcp_config_file(worktree_id, 3000)
            .expect("Should generate config");

        let path = PathBuf::from(&config_path);
        assert!(path.exists(), "Config should exist before cleanup");

        // Clean up
        let result = cleanup_mcp_config_file(&config_path);
        assert!(result.is_ok(), "Cleanup should succeed");
        assert!(!path.exists(), "Config should not exist after cleanup");

        // Idempotent - cleanup again should not error
        let result2 = cleanup_mcp_config_file(&config_path);
        assert!(result2.is_ok(), "Cleanup should be idempotent");
    }

    #[test]
    fn test_generate_config_sanitizes_worktree_id() {
        // Test with worktree_id containing path traversal attempts
        let dangerous_id = "../../../etc/passwd";
        let result = generate_mcp_config_file(dangerous_id, 3000);

        assert!(result.is_ok(), "Should handle dangerous IDs");

        let config_path = result.unwrap();

        // Verify path doesn't contain traversal sequences
        assert!(
            !config_path.contains(".."),
            "Config path should not contain '..' after sanitization"
        );
        assert!(
            !config_path.contains("/etc/"),
            "Config path should not escape to /etc/"
        );

        // Cleanup
        cleanup_mcp_config_file(&config_path).ok();
    }

    #[test]
    fn test_generate_config_different_ports() {
        let worktree_id = "test-ports";

        // Test with different ports
        for port in [3000, 3001, 8080, 9000] {
            let config_path = generate_mcp_config_file(worktree_id, port)
                .expect("Should generate config");

            let contents = fs::read_to_string(&config_path)
                .expect("Should read config");
            let json: serde_json::Value = serde_json::from_str(&contents)
                .expect("Should parse JSON");

            assert_eq!(
                json["mcpServers"]["rstn"]["url"],
                format!("http://localhost:{}", port),
                "Should use correct port"
            );

            cleanup_mcp_config_file(&config_path).ok();
        }
    }
}
