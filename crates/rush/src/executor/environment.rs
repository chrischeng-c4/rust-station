//! Environment variable management for rush shell
//!
//! This module provides the `EnvironmentManager` struct which handles
//! all environment variable operations including inheritance, expansion,
//! and passing to child processes.

use crate::error::RushError;
use std::collections::HashMap;

/// Manages environment variables for a shell session
///
/// # Example
///
/// ```rust
/// use rush::executor::environment::EnvironmentManager;
///
/// let mut env = EnvironmentManager::new();
/// assert!(env.get("HOME").is_some()); // Inherited from system
///
/// env.set("MY_VAR".to_string(), "my_value".to_string()).unwrap();
/// assert_eq!(env.get("MY_VAR"), Some("my_value"));
/// ```
pub struct EnvironmentManager {
    variables: HashMap<String, String>,
}

impl EnvironmentManager {
    /// Create a new EnvironmentManager, inheriting all variables from parent process
    pub fn new() -> Self {
        Self { variables: std::env::vars().collect() }
    }

    /// Get the value of an environment variable
    ///
    /// Returns `Some(&str)` if the variable is defined, `None` otherwise
    pub fn get(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    /// Set or update an environment variable
    ///
    /// # Errors
    ///
    /// Returns an error if the variable name is invalid (doesn't match `[a-zA-Z_][a-zA-Z0-9_]*`)
    pub fn set(&mut self, name: String, value: String) -> Result<(), RushError> {
        if name.is_empty() {
            return Err(RushError::Execution("Variable name cannot be empty".to_string()));
        }
        if !is_valid_variable_name(&name) {
            return Err(RushError::Execution(format!(
                "Invalid variable name '{}': must start with letter or underscore, contain only alphanumeric and underscore",
                name
            )));
        }
        self.variables.insert(name, value);
        Ok(())
    }

    /// Remove an environment variable
    ///
    /// Returns the old value if it existed
    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.variables.remove(name)
    }

    /// Iterate over all environment variables
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.variables.iter()
    }

    /// Get environment as HashMap for passing to child processes
    ///
    /// Suitable for use with `Command::envs()`
    pub fn as_env_map(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Get the number of environment variables
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Check if environment is empty
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

impl Default for EnvironmentManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate that a variable name follows POSIX conventions
///
/// Valid names match `[a-zA-Z_][a-zA-Z0-9_]*`
pub fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();

    // First character: letter or underscore
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    // Remaining: letters, digits, or underscores
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_manager_new() {
        let env = EnvironmentManager::new();
        // Should inherit system environment
        assert!(env.get("PATH").is_some() || env.get("HOME").is_some());
    }

    #[test]
    fn test_environment_manager_default() {
        let env = EnvironmentManager::default();
        assert!(!env.is_empty());
    }

    #[test]
    fn test_get_existing_variable() {
        let env = EnvironmentManager::new();
        // PATH should exist on all systems
        if let Some(path) = env.get("PATH") {
            assert!(!path.is_empty());
        }
    }

    #[test]
    fn test_get_nonexistent_variable() {
        let env = EnvironmentManager::new();
        assert!(env.get("THIS_VAR_DOES_NOT_EXIST_12345").is_none());
    }

    #[test]
    fn test_set_valid_variable() {
        let mut env = EnvironmentManager::new();
        let result = env.set("MY_TEST_VAR".to_string(), "test_value".to_string());
        assert!(result.is_ok());
        assert_eq!(env.get("MY_TEST_VAR"), Some("test_value"));
    }

    #[test]
    fn test_set_empty_name() {
        let mut env = EnvironmentManager::new();
        let result = env.set("".to_string(), "value".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_set_invalid_name_starts_with_number() {
        let mut env = EnvironmentManager::new();
        let result = env.set("123VAR".to_string(), "value".to_string());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid variable name"));
    }

    #[test]
    fn test_set_invalid_name_with_dash() {
        let mut env = EnvironmentManager::new();
        let result = env.set("MY-VAR".to_string(), "value".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_set_valid_name_with_underscore() {
        let mut env = EnvironmentManager::new();
        let result = env.set("_MY_VAR".to_string(), "value".to_string());
        assert!(result.is_ok());
        assert_eq!(env.get("_MY_VAR"), Some("value"));
    }

    #[test]
    fn test_set_valid_name_with_numbers() {
        let mut env = EnvironmentManager::new();
        let result = env.set("VAR123".to_string(), "value".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_overwrites_existing() {
        let mut env = EnvironmentManager::new();
        env.set("TEST_VAR".to_string(), "old".to_string()).unwrap();
        env.set("TEST_VAR".to_string(), "new".to_string()).unwrap();
        assert_eq!(env.get("TEST_VAR"), Some("new"));
    }

    #[test]
    fn test_remove_existing() {
        let mut env = EnvironmentManager::new();
        env.set("TO_REMOVE".to_string(), "value".to_string())
            .unwrap();
        let removed = env.remove("TO_REMOVE");
        assert_eq!(removed, Some("value".to_string()));
        assert!(env.get("TO_REMOVE").is_none());
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut env = EnvironmentManager::new();
        let removed = env.remove("NONEXISTENT_VAR");
        assert!(removed.is_none());
    }

    #[test]
    fn test_iter() {
        let mut env = EnvironmentManager::new();
        env.set("ITER_TEST".to_string(), "value".to_string())
            .unwrap();

        let found = env.iter().any(|(k, v)| k == "ITER_TEST" && v == "value");
        assert!(found);
    }

    #[test]
    fn test_as_env_map() {
        let mut env = EnvironmentManager::new();
        env.set("MAP_TEST".to_string(), "value".to_string())
            .unwrap();

        let map = env.as_env_map();
        assert_eq!(map.get("MAP_TEST"), Some(&"value".to_string()));
    }

    #[test]
    fn test_len() {
        let mut env = EnvironmentManager::new();
        let initial_len = env.len();
        env.set("NEW_VAR".to_string(), "value".to_string()).unwrap();
        assert_eq!(env.len(), initial_len + 1);
    }

    #[test]
    fn test_is_empty() {
        let env = EnvironmentManager::new();
        // System environment should not be empty
        assert!(!env.is_empty());
    }

    // Tests for is_valid_variable_name function
    #[test]
    fn test_valid_names() {
        assert!(is_valid_variable_name("HOME"));
        assert!(is_valid_variable_name("PATH"));
        assert!(is_valid_variable_name("_private"));
        assert!(is_valid_variable_name("VAR_123"));
        assert!(is_valid_variable_name("a"));
        assert!(is_valid_variable_name("A"));
        assert!(is_valid_variable_name("_"));
        assert!(is_valid_variable_name("__"));
        assert!(is_valid_variable_name("var123"));
    }

    #[test]
    fn test_invalid_names() {
        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name("123"));
        assert!(!is_valid_variable_name("123var"));
        assert!(!is_valid_variable_name("-foo"));
        assert!(!is_valid_variable_name("var-name"));
        assert!(!is_valid_variable_name("foo.bar"));
        assert!(!is_valid_variable_name("foo bar"));
        assert!(!is_valid_variable_name("$VAR"));
    }
}
