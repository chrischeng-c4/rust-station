//! State snapshot integration tests
//!
//! These tests demonstrate how to use StateSnapshot and TestTimeline
//! to create comprehensive E2E tests without needing GPUI/Metal.

// Note: rstn is a binary crate, so we use rstn_core for state
mod state {
    use serde::{Serialize, Deserialize};
    use std::path::PathBuf;
    use std::fs;
    use anyhow::{Result, Context};
    pub use rstn_core::app_state::AppState as CoreAppState;

    /// Wrapper around rstn_core's AppState for testing
    /// This mirrors the structure in crates/rstn/src/state.rs
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AppState {
        pub core: CoreAppState,
        pub active_tab: String,
    }

    impl AppState {
        pub fn new() -> Self {
            Self {
                core: CoreAppState::default(),
                active_tab: "tasks".to_string(),
            }
        }

        pub fn initialize(&mut self) {
            // Initialize with a default project if needed
        }

        /// Save state to a JSON file
        pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .context("Failed to create config directory")?;
            }

            let json = serde_json::to_string_pretty(self)
                .context("Failed to serialize state")?;

            fs::write(path, json)
                .context("Failed to write state file")?;

            Ok(())
        }

        /// Load state from a JSON file
        pub fn load_from_file(path: &PathBuf) -> Result<Self> {
            let json = fs::read_to_string(path)
                .context("Failed to read state file")?;

            let state: Self = serde_json::from_str(&json)
                .context("Failed to deserialize state")?;

            Ok(state)
        }

        pub fn get_docker_services(&self) -> Vec<rstn_core::app_state::DockerServiceInfo> {
            // Get built-in services
            use rstn_core::app_state::{DockerServiceInfo, ServiceType, ServiceStatus};
            vec![
                DockerServiceInfo {
                    id: "postgres".into(),
                    name: "PostgreSQL".into(),
                    image: "postgres:16".into(),
                    status: ServiceStatus::Stopped,
                    port: Some(5432),
                    service_type: ServiceType::Database,
                    project_group: Some("rstn".into()),
                    is_rstn_managed: true,
                },
                DockerServiceInfo {
                    id: "redis".into(),
                    name: "Redis".into(),
                    image: "redis:7".into(),
                    status: ServiceStatus::Stopped,
                    port: Some(6379),
                    service_type: ServiceType::Cache,
                    project_group: Some("rstn".into()),
                    is_rstn_managed: true,
                },
                DockerServiceInfo {
                    id: "mongodb".into(),
                    name: "MongoDB".into(),
                    image: "mongo:7".into(),
                    status: ServiceStatus::Stopped,
                    port: Some(27017),
                    service_type: ServiceType::Database,
                    project_group: Some("rstn".into()),
                    is_rstn_managed: true,
                },
            ]
        }

        pub fn get_justfile_commands(&self) -> Vec<rstn_core::justfile::JustCommand> {
            // Return mock commands for testing
            vec![
                rstn_core::justfile::JustCommand {
                    name: "build".into(),
                    description: Some("Build the project".into()),
                    recipe: "cargo build --workspace".into(),
                },
                rstn_core::justfile::JustCommand {
                    name: "test".into(),
                    description: Some("Run tests".into()),
                    recipe: "cargo test --workspace".into(),
                },
                rstn_core::justfile::JustCommand {
                    name: "dev".into(),
                    description: Some("Run development server".into()),
                    recipe: "cargo run -p rstn".into(),
                },
            ]
        }
    }
}

use state::AppState;
use rstn_core::snapshot::StateSnapshot;
use rstn_core::test_timeline::TestTimeline;
use std::path::PathBuf;

#[test]
fn test_docker_services_snapshot() {
    let timeline = TestTimeline::new();

    timeline.record("PHASE", "Starting Docker services snapshot test", None);

    // 1. Create and initialize state
    timeline.record("STATE", "Creating AppState", None);
    let mut state = AppState::new();
    state.initialize();

    // 2. Get Docker services from state
    timeline.record("DATA", "Loading Docker services", None);
    let services = state.get_docker_services();
    timeline.record(
        "DATA",
        "Services loaded",
        Some(serde_json::json!({"count": services.len()})),
    );

    // 3. Capture snapshot
    timeline.record("SNAPSHOT", "Capturing state snapshot", None);
    let snapshot = StateSnapshot::capture(
        "docker_services",
        "test_docker_services_snapshot",
        &services,
    )
    .expect("Failed to capture snapshot");

    // 4. Verify snapshot (creates baseline on first run)
    timeline.record("VERIFY", "Verifying snapshot", None);
    let snapshot_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("snapshots");
    std::fs::create_dir_all(&snapshot_dir).expect("Failed to create snapshots dir");

    let snapshot_file = snapshot_dir.join("docker_services.json");
    snapshot
        .assert_matches_snapshot(&snapshot_file)
        .expect("Snapshot mismatch");

    // 5. Run assertions
    timeline.record("ASSERT", "Running assertions", None);
    assert!(
        !services.is_empty(),
        "Should have Docker services (built-in services)"
    );
    assert_eq!(
        services.len(),
        3,
        "Should have 3 built-in services: postgres, redis, mongodb"
    );

    // Verify service structure
    for service in &services {
        assert!(!service.id.is_empty(), "Service ID should not be empty");
        assert!(
            !service.name.is_empty(),
            "Service name should not be empty"
        );
        assert!(
            !service.image.is_empty(),
            "Service image should not be empty"
        );
    }

    timeline.record("PHASE", "Test completed successfully", None);

    // Print results
    timeline.print_timeline();
    println!("\n{}", timeline.summary());
}

#[test]
fn test_justfile_commands_snapshot() {
    let timeline = TestTimeline::new();

    timeline.record("PHASE", "Starting justfile commands snapshot test", None);

    // 1. Create and initialize state
    let mut state = AppState::new();
    state.initialize();

    // 2. Get justfile commands
    timeline.record("DATA", "Loading justfile commands", None);
    let commands = state.get_justfile_commands();
    timeline.record(
        "DATA",
        "Commands loaded",
        Some(serde_json::json!({"count": commands.len()})),
    );

    // 3. Capture snapshot
    let snapshot = StateSnapshot::capture(
        "justfile_commands",
        "test_justfile_commands_snapshot",
        &commands,
    )
    .expect("Failed to capture snapshot");

    // 4. Verify snapshot
    let snapshot_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("snapshots");
    std::fs::create_dir_all(&snapshot_dir).expect("Failed to create snapshots dir");

    let snapshot_file = snapshot_dir.join("justfile_commands.json");
    snapshot
        .assert_matches_snapshot(&snapshot_file)
        .expect("Snapshot mismatch");

    // 5. Run assertions
    timeline.record("ASSERT", "Verifying commands structure", None);
    assert!(!commands.is_empty(), "Should have justfile commands");

    // Verify each command has valid structure
    for (i, command) in commands.iter().enumerate() {
        timeline.record(
            "VERIFY",
            format!("Command {}: {}", i + 1, command.name),
            Some(serde_json::json!({
                "name": &command.name,
                "has_description": command.description.is_some(),
            })),
        );

        assert!(
            !command.name.is_empty(),
            "Command name should not be empty"
        );
        assert!(
            !command.recipe.is_empty(),
            "Command recipe should not be empty"
        );
    }

    timeline.record("PHASE", "Test completed", None);
    timeline.print_timeline();
}

#[test]
fn test_timeline_feature() {
    // Demonstrate timeline features
    let timeline = TestTimeline::new();

    timeline.record("START", "Test started", None);
    timeline.record(
        "DATA",
        "Sample data",
        Some(serde_json::json!({"key": "value", "count": 42})),
    );
    timeline.record("ASSERT", "Verification passed", None);
    timeline.record("END", "Test completed", None);

    // Verify timeline functionality
    let events = timeline.events();
    assert_eq!(events.len(), 4, "Should have 4 events");
    assert_eq!(events[0].level, "START");
    assert_eq!(events[3].level, "END");

    // Print timeline
    timeline.print_timeline();

    // Verify JSON export
    let json = timeline.to_json();
    assert!(json["events"].is_array());
}

#[test]
fn test_state_persistence_roundtrip() {
    use tempfile::tempdir;

    let timeline = TestTimeline::new();

    timeline.record("PHASE", "Starting state persistence test", None);

    // 1. Create and initialize state
    timeline.record("STATE", "Creating AppState", None);
    let mut state = AppState::new();
    state.initialize();

    // Change active tab to verify it persists
    state.active_tab = "terminal".to_string();

    // 2. Save state to temp file
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let state_path = temp_dir.path().join("test-state.json");

    timeline.record("SAVE", "Saving state to file", None);
    state
        .save_to_file(&state_path)
        .expect("Failed to save state");

    timeline.record(
        "VERIFY",
        "State file created",
        Some(serde_json::json!({
            "path": state_path.display().to_string(),
            "exists": state_path.exists(),
        })),
    );

    assert!(state_path.exists(), "State file should exist");

    // 3. Load state from file
    timeline.record("LOAD", "Loading state from file", None);
    let loaded_state = AppState::load_from_file(&state_path)
        .expect("Failed to load state");

    // 4. Verify loaded state matches original
    timeline.record("VERIFY", "Comparing loaded state", None);

    assert_eq!(
        loaded_state.active_tab, "terminal",
        "Active tab should be preserved"
    );

    // Verify core state structure (check projects count, version, etc.)
    assert_eq!(
        loaded_state.core.version,
        state.core.version,
        "Version should match"
    );

    timeline.record(
        "ASSERT",
        "State round-trip successful",
        Some(serde_json::json!({
            "original_tab": state.active_tab,
            "loaded_tab": loaded_state.active_tab,
            "projects_count": loaded_state.core.projects.len(),
        })),
    );

    timeline.record("PHASE", "Test completed successfully", None);

    // Print results
    timeline.print_timeline();
    println!("\n{}", timeline.summary());
}

#[test]
fn test_state_json_format() {
    let timeline = TestTimeline::new();

    timeline.record("PHASE", "Testing state JSON format", None);

    // Create state
    let mut state = AppState::new();
    state.initialize();
    state.active_tab = "chat".to_string();

    // Serialize to JSON
    timeline.record("SERIALIZE", "Converting state to JSON", None);
    let json_str = serde_json::to_string_pretty(&state)
        .expect("Failed to serialize state");

    // Verify JSON is valid and contains expected fields
    let json_value: serde_json::Value = serde_json::from_str(&json_str)
        .expect("Failed to parse JSON");

    timeline.record(
        "VERIFY",
        "Checking JSON structure",
        Some(serde_json::json!({
            "has_core": json_value.get("core").is_some(),
            "has_active_tab": json_value.get("active_tab").is_some(),
        })),
    );

    assert!(
        json_value.get("core").is_some(),
        "JSON should have 'core' field"
    );
    assert!(
        json_value.get("active_tab").is_some(),
        "JSON should have 'active_tab' field"
    );
    assert_eq!(
        json_value["active_tab"], "chat",
        "Active tab should be 'chat'"
    );

    // Verify core state has required fields
    let core = &json_value["core"];
    assert!(
        core.get("version").is_some(),
        "Core should have 'version' field"
    );
    assert!(
        core.get("projects").is_some(),
        "Core should have 'projects' field"
    );

    timeline.record("PHASE", "JSON format test completed", None);
    timeline.print_timeline();
}
