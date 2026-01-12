//! State snapshot testing utilities
//!
//! Provides tools for capturing and comparing application state snapshots.
//! This enables regression testing and debugging by comparing state changes over time.

use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// A captured snapshot of application state
///
/// # Examples
///
/// ```
/// use rstn_core::snapshot::StateSnapshot;
/// use std::path::PathBuf;
///
/// #[derive(serde::Serialize)]
/// struct MyState {
///     count: usize,
///     name: String,
/// }
///
/// let state = MyState { count: 42, name: "test".into() };
/// let snapshot = StateSnapshot::capture(
///     "my_state",
///     "test_example",
///     &state,
/// ).unwrap();
///
/// snapshot.pretty_print();
/// ```
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    /// Name of this snapshot
    pub name: String,
    /// Captured state as JSON
    pub state_json: Value,
    /// ISO 8601 timestamp when captured
    pub timestamp: String,
    /// Name of the test that created this snapshot
    pub test_name: String,
}

impl StateSnapshot {
    /// Capture a snapshot of any serializable value
    ///
    /// # Arguments
    ///
    /// * `name` - Descriptive name for this snapshot
    /// * `test_name` - Name of the test creating this snapshot
    /// * `value` - Value to snapshot (must be serializable)
    ///
    /// # Returns
    ///
    /// A `StateSnapshot` containing the serialized state
    pub fn capture<T: serde::Serialize>(
        name: &str,
        test_name: &str,
        value: &T,
    ) -> Result<Self> {
        let state_json = serde_json::to_value(value)?;

        Ok(Self {
            name: name.to_string(),
            state_json,
            timestamp: chrono::Utc::now().to_rfc3339(),
            test_name: test_name.to_string(),
        })
    }

    /// Compare this snapshot with a saved baseline
    ///
    /// On first run, saves the snapshot to `snapshot_path`.
    /// On subsequent runs, compares current state with saved baseline.
    ///
    /// # Arguments
    ///
    /// * `snapshot_path` - Path to snapshot file
    ///
    /// # Returns
    ///
    /// `Ok(())` if snapshot matches (or was created), `Err` if mismatch
    pub fn assert_matches_snapshot(&self, snapshot_path: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = snapshot_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if !snapshot_path.exists() {
            // First run: save snapshot as baseline
            let json_str = serde_json::to_string_pretty(&self.state_json)?;
            fs::write(snapshot_path, json_str)?;
            println!("📸 Snapshot created: {}", snapshot_path.display());
            return Ok(());
        }

        // Subsequent runs: compare with baseline
        let expected = fs::read_to_string(snapshot_path)?;
        let expected_json: Value = serde_json::from_str(&expected)?;

        if self.state_json == expected_json {
            println!("✅ Snapshot matches: {}", self.name);
            Ok(())
        } else {
            println!("❌ Snapshot mismatch: {}", self.name);
            println!("\nExpected:");
            println!("{}", serde_json::to_string_pretty(&expected_json)?);
            println!("\nActual:");
            println!("{}", serde_json::to_string_pretty(&self.state_json)?);
            Err(anyhow::anyhow!(
                "Snapshot mismatch for '{}' at {}",
                self.name,
                snapshot_path.display()
            ))
        }
    }

    /// Pretty print this snapshot for debugging
    ///
    /// Outputs snapshot metadata and formatted JSON to stdout
    pub fn pretty_print(&self) {
        println!("\n📸 State Snapshot: {}", self.name);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Test: {}", self.test_name);
        println!("Time: {}", self.timestamp);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        if let Ok(pretty) = serde_json::to_string_pretty(&self.state_json) {
            println!("{}", pretty);
        } else {
            println!("{:?}", self.state_json);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    /// Get a summary of this snapshot
    pub fn summary(&self) -> String {
        format!(
            "Snapshot '{}' from test '{}' at {}",
            self.name, self.test_name, self.timestamp
        )
    }

    /// Export snapshot to a JSON string
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.state_json).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[derive(Debug, Serialize, PartialEq)]
    struct TestState {
        count: usize,
        items: Vec<String>,
    }

    #[test]
    fn test_snapshot_capture() {
        let state = TestState {
            count: 3,
            items: vec!["a".into(), "b".into(), "c".into()],
        };

        let snapshot = StateSnapshot::capture("test_state", "test_snapshot_capture", &state)
            .expect("Failed to capture snapshot");

        assert_eq!(snapshot.name, "test_state");
        assert_eq!(snapshot.test_name, "test_snapshot_capture");
        assert!(snapshot.state_json.is_object());
    }

    #[test]
    fn test_snapshot_roundtrip() {
        let temp_dir = tempdir().unwrap();
        let snapshot_path = temp_dir.path().join("test_snapshot.json");

        let state = TestState {
            count: 5,
            items: vec!["x".into(), "y".into()],
        };

        let snapshot = StateSnapshot::capture("test_state", "test_snapshot_roundtrip", &state)
            .expect("Failed to capture");

        // First run: creates snapshot
        snapshot
            .assert_matches_snapshot(&snapshot_path)
            .expect("Failed to save snapshot");
        assert!(snapshot_path.exists());

        // Second run: matches saved snapshot
        let snapshot2 = StateSnapshot::capture("test_state", "test_snapshot_roundtrip", &state)
            .expect("Failed to capture");
        snapshot2
            .assert_matches_snapshot(&snapshot_path)
            .expect("Snapshot should match");
    }

    #[test]
    fn test_snapshot_mismatch() {
        let temp_dir = tempdir().unwrap();
        let snapshot_path = temp_dir.path().join("test_mismatch.json");

        let state1 = TestState {
            count: 1,
            items: vec!["first".into()],
        };

        let snapshot1 = StateSnapshot::capture("test_state", "test_snapshot_mismatch", &state1)
            .expect("Failed to capture");
        snapshot1
            .assert_matches_snapshot(&snapshot_path)
            .expect("Failed to save");

        // Different state should cause mismatch
        let state2 = TestState {
            count: 2,
            items: vec!["second".into()],
        };

        let snapshot2 = StateSnapshot::capture("test_state", "test_snapshot_mismatch", &state2)
            .expect("Failed to capture");

        let result = snapshot2.assert_matches_snapshot(&snapshot_path);
        assert!(result.is_err(), "Should detect mismatch");
    }

    #[test]
    fn test_pretty_print() {
        let state = TestState {
            count: 42,
            items: vec!["test".into()],
        };

        let snapshot = StateSnapshot::capture("test_state", "test_pretty_print", &state)
            .expect("Failed to capture");

        // Should not panic
        snapshot.pretty_print();
    }

    #[test]
    fn test_summary() {
        let state = TestState {
            count: 1,
            items: vec![],
        };

        let snapshot = StateSnapshot::capture("my_snapshot", "test_summary", &state)
            .expect("Failed to capture");

        let summary = snapshot.summary();
        assert!(summary.contains("my_snapshot"));
        assert!(summary.contains("test_summary"));
    }
}
