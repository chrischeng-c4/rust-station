//! Environment variable management
//!
//! Manages variables for the shell, tracking which ones are exported
//! (visible to subshells) and which are local to the shell session.

use crate::error::{Result, RushError};
use std::collections::{HashMap, HashSet};

/// Manages shell environment variables
#[derive(Debug, Clone)]
pub struct VariableManager {
    /// All variables (name -> value)
    variables: HashMap<String, String>,
    /// Which variables are exported (for subshells)
    exported: HashSet<String>,
}

impl VariableManager {
    /// Create a new variable manager
    pub fn new() -> Self {
        Self { variables: HashMap::new(), exported: HashSet::new() }
    }

    /// Set a variable
    ///
    /// # Arguments
    /// * `name` - Variable name (must be valid identifier)
    /// * `value` - Variable value
    ///
    /// # Returns
    /// * `Ok(())` - Variable set successfully
    /// * `Err(_)` - Invalid variable name
    pub fn set(&mut self, name: String, value: String) -> Result<()> {
        if !Self::is_valid_name(&name) {
            return Err(RushError::Execution(format!("set: {}: invalid identifier", name)));
        }
        self.variables.insert(name, value);
        Ok(())
    }

    /// Get a variable value
    pub fn get(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    /// Remove a variable
    ///
    /// # Returns
    /// * `true` - Variable was removed
    /// * `false` - Variable didn't exist
    pub fn remove(&mut self, name: &str) -> bool {
        self.exported.remove(name);
        self.variables.remove(name).is_some()
    }

    /// Mark a variable as exported (visible to subshells)
    ///
    /// # Returns
    /// * `Ok(())` - Variable marked as exported
    /// * `Err(_)` - Variable doesn't exist
    pub fn export(&mut self, name: &str) -> Result<()> {
        if !self.variables.contains_key(name) {
            return Err(RushError::Execution(format!("export: {}: not set", name)));
        }
        self.exported.insert(name.to_string());
        Ok(())
    }

    /// Check if a variable is exported
    pub fn is_exported(&self, name: &str) -> bool {
        self.exported.contains(name)
    }

    /// List all variables (sorted by name)
    pub fn list(&self) -> Vec<(&str, &str)> {
        let mut vars: Vec<_> = self
            .variables
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        vars.sort_by(|a, b| a.0.cmp(b.0));
        vars
    }

    /// List only exported variables (sorted by name)
    pub fn list_exported(&self) -> Vec<(&str, &str)> {
        let mut vars: Vec<_> = self
            .variables
            .iter()
            .filter(|(k, _)| self.exported.contains(k.as_str()))
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        vars.sort_by(|a, b| a.0.cmp(b.0));
        vars
    }

    /// Get number of variables
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// Check if a variable name is valid
    ///
    /// Valid names are: alphanumeric + underscore, must start with letter or underscore
    fn is_valid_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let first = name.chars().next().unwrap();
        if !first.is_alphabetic() && first != '_' {
            return false;
        }

        name.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
}

impl Default for VariableManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut vm = VariableManager::new();
        vm.set("name".to_string(), "value".to_string()).unwrap();
        assert_eq!(vm.get("name"), Some("value"));
    }

    #[test]
    fn test_get_nonexistent() {
        let vm = VariableManager::new();
        assert_eq!(vm.get("nonexistent"), None);
    }

    #[test]
    fn test_remove() {
        let mut vm = VariableManager::new();
        vm.set("name".to_string(), "value".to_string()).unwrap();
        assert!(vm.remove("name"));
        assert_eq!(vm.get("name"), None);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut vm = VariableManager::new();
        assert!(!vm.remove("nonexistent"));
    }

    #[test]
    fn test_export() {
        let mut vm = VariableManager::new();
        vm.set("name".to_string(), "value".to_string()).unwrap();
        vm.export("name").unwrap();
        assert!(vm.is_exported("name"));
    }

    #[test]
    fn test_export_nonexistent() {
        let mut vm = VariableManager::new();
        assert!(vm.export("nonexistent").is_err());
    }

    #[test]
    fn test_list() {
        let mut vm = VariableManager::new();
        vm.set("alpha".to_string(), "1".to_string()).unwrap();
        vm.set("beta".to_string(), "2".to_string()).unwrap();
        vm.set("gamma".to_string(), "3".to_string()).unwrap();

        let list = vm.list();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].0, "alpha");
        assert_eq!(list[1].0, "beta");
        assert_eq!(list[2].0, "gamma");
    }

    #[test]
    fn test_list_exported() {
        let mut vm = VariableManager::new();
        vm.set("local".to_string(), "1".to_string()).unwrap();
        vm.set("exported".to_string(), "2".to_string()).unwrap();
        vm.export("exported").unwrap();

        let exported = vm.list_exported();
        assert_eq!(exported.len(), 1);
        assert_eq!(exported[0].0, "exported");
    }

    #[test]
    fn test_invalid_names() {
        let mut vm = VariableManager::new();

        // Starting with number
        assert!(vm.set("1name".to_string(), "value".to_string()).is_err());

        // With hyphen
        assert!(vm.set("my-var".to_string(), "value".to_string()).is_err());

        // With space
        assert!(vm.set("my var".to_string(), "value".to_string()).is_err());
    }

    #[test]
    fn test_valid_names() {
        let mut vm = VariableManager::new();

        // Starting with letter
        assert!(vm.set("name".to_string(), "value".to_string()).is_ok());

        // Starting with underscore
        assert!(vm.set("_private".to_string(), "value".to_string()).is_ok());

        // With numbers
        assert!(vm.set("var123".to_string(), "value".to_string()).is_ok());

        // Uppercase
        assert!(vm.set("MYVAR".to_string(), "value".to_string()).is_ok());
    }

    #[test]
    fn test_update_variable() {
        let mut vm = VariableManager::new();
        vm.set("name".to_string(), "value1".to_string()).unwrap();
        assert_eq!(vm.get("name"), Some("value1"));

        vm.set("name".to_string(), "value2".to_string()).unwrap();
        assert_eq!(vm.get("name"), Some("value2"));
    }

    #[test]
    fn test_remove_exported_variable() {
        let mut vm = VariableManager::new();
        vm.set("name".to_string(), "value".to_string()).unwrap();
        vm.export("name").unwrap();
        assert!(vm.is_exported("name"));

        vm.remove("name");
        assert!(!vm.is_exported("name"));
        assert_eq!(vm.get("name"), None);
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut vm = VariableManager::new();
        assert!(vm.is_empty());
        assert_eq!(vm.len(), 0);

        vm.set("name".to_string(), "value".to_string()).unwrap();
        assert!(!vm.is_empty());
        assert_eq!(vm.len(), 1);

        vm.remove("name");
        assert!(vm.is_empty());
        assert_eq!(vm.len(), 0);
    }
}
