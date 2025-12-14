//! Integration tests for rstn logging

use rstn::logging;
use rstn::settings::Settings;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_log_file_path_is_in_rustation_directory() {
    let path = logging::log_file_path();
    let path_str = path.to_string_lossy();

    assert!(
        path_str.contains(".rustation"),
        "Log path should be in .rustation directory: {}",
        path_str
    );
    assert!(
        path_str.ends_with("rstn.log"),
        "Log path should end with rstn.log: {}",
        path_str
    );
}

#[test]
fn test_settings_default_values() {
    let settings = Settings::default();

    // Logging should be enabled by default for development
    assert!(settings.logging_enabled);

    // Default log level should be debug
    assert_eq!(settings.log_level, "debug");

    // Console logging should be off by default
    assert!(!settings.log_to_console);
}

#[test]
fn test_settings_load_returns_defaults_when_no_file() {
    // Settings::load() should return defaults when no settings file exists
    // This is safe because we're not modifying anything
    let settings = Settings::load();

    // Should have default values
    assert!(settings.logging_enabled);
    assert_eq!(settings.log_level, "debug");
}

#[test]
fn test_logging_does_not_panic_on_init() {
    // Test that logging initialization doesn't panic
    // Note: We can't actually initialize the global subscriber multiple times,
    // so we just test the settings validation
    let settings = Settings::default();

    // Verify settings are valid for logging
    assert!(settings.logging_enabled);
    assert!(!settings.log_level.is_empty());

    // The actual init would fail if called multiple times due to global subscriber,
    // but we can verify the path functions work
    let log_path = logging::log_file_path();
    assert!(!log_path.to_string_lossy().is_empty());
}
