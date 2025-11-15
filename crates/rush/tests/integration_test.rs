//! Integration tests for rush shell
//!
//! These tests verify end-to-end functionality of the shell components.

use rush::{Config, Repl};

#[test]
fn test_repl_initialization() {
    // Test that REPL can be created with default config
    let repl = Repl::new();
    assert!(repl.is_ok(), "REPL should initialize successfully");
}

#[test]
fn test_repl_with_custom_config() {
    // Test that REPL accepts custom configuration
    let mut config = Config::default();
    config.history_size = 5000;
    config.prompt = ">> ".to_string();

    let repl = Repl::with_config(config);
    assert!(repl.is_ok(), "REPL should initialize with custom config");
}

#[test]
fn test_config_default_values() {
    let config = Config::default();

    assert_eq!(config.history_size, 10_000);
    assert_eq!(config.prompt, "$ ");
    assert_eq!(config.completion_timeout_ms, 100);
    assert_eq!(config.suggestion_delay_ms, 50);
}

#[test]
fn test_config_custom_values() {
    let mut config = Config::default();

    config.history_size = 50_000;
    config.prompt = "λ ".to_string();
    config.completion_timeout_ms = 200;
    config.suggestion_delay_ms = 100;

    assert_eq!(config.history_size, 50_000);
    assert_eq!(config.prompt, "λ ");
    assert_eq!(config.completion_timeout_ms, 200);
    assert_eq!(config.suggestion_delay_ms, 100);
}

#[test]
fn test_config_load_creates_directories() {
    // This should not panic even if directories don't exist
    let config = Config::load();
    assert_eq!(config.history_size, 10_000); // Should use defaults
}
