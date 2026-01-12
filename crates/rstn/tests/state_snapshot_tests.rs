//! State snapshot integration tests
//!
//! These tests demonstrate how to use StateSnapshot and TestTimeline
//! to create comprehensive E2E tests without needing GPUI/Metal.

// Note: rstn is a binary crate, so we use rstn_core for state
mod state {
    pub use rstn_core::app_state::AppState as CoreAppState;

    /// Wrapper around rstn_core's AppState for testing
    pub struct AppState {
        pub core: CoreAppState,
    }

    impl AppState {
        pub fn new() -> Self {
            Self {
                core: CoreAppState::default(),
            }
        }

        pub fn initialize(&mut self) {
            // Initialize with a default project if needed
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
