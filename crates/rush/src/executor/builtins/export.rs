//! 'export' built-in command
//!
//! Sets environment variables that will be passed to child processes.
//! Supports `VAR=value` syntax with variable expansion in values.

use crate::error::{Result, RushError};
use crate::executor::execute::CommandExecutor;
use crate::executor::parser::expand_variables_in_string;

/// Execute the 'export' builtin command
///
/// # Arguments
///
/// * `executor` - Mutable reference to command executor
/// * `args` - Command arguments (VAR=value pairs)
///
/// # Returns
///
/// * `Ok(0)` on success
/// * `Err(RushError)` on invalid variable name or syntax
///
/// # Syntax
///
/// ```text
/// export VAR=value           # Set VAR to "value"
/// export VAR="value"         # Set VAR to "value"
/// export VAR=$OTHER          # Set VAR to value of OTHER
/// export VAR=                # Set VAR to empty string
/// export                     # List all variables (like set)
/// ```
///
/// # Errors
///
/// * Invalid variable name (doesn't match `[a-zA-Z_][a-zA-Z0-9_]*`)
/// * Missing `=` in argument
pub fn execute(executor: &mut CommandExecutor, args: &[String]) -> Result<i32> {
    // No args: list all variables (like set)
    if args.is_empty() {
        let env = executor.env_manager();
        let mut vars: Vec<(&String, &String)> = env.iter().collect();
        vars.sort_by_key(|(k, _)| k.as_str());
        for (name, value) in vars {
            println!("{}={}", name, value);
        }
        return Ok(0);
    }

    // Process each VAR=value argument
    for arg in args {
        // Parse VAR=value syntax (split on first '=')
        let (name, value) = match arg.split_once('=') {
            Some((n, v)) => (n.to_string(), v.to_string()),
            None => {
                return Err(RushError::Execution(format!(
                    "export: invalid syntax '{}', expected VAR=value",
                    arg
                )));
            }
        };

        // Expand variables in value before setting
        let expanded_value = expand_variables_in_string(&value, executor.env_manager());

        // Set the variable (validation happens in env_manager.set())
        executor.env_manager_mut().set(name, expanded_value)?;
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // T028: Test export VAR=value parsing
    #[test]
    fn test_export_var_value_parsing() {
        let mut executor = CommandExecutor::new();

        let result = execute(&mut executor, &["FOO=bar".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        assert_eq!(executor.env_manager().get("FOO"), Some("bar"));
    }

    // T029: Test export with no args (list variables)
    #[test]
    fn test_export_no_args_lists_variables() {
        let mut executor = CommandExecutor::new();

        // Set a variable first
        executor
            .env_manager_mut()
            .set("EXPORT_TEST".to_string(), "value".to_string())
            .unwrap();

        // export with no args should succeed
        let result = execute(&mut executor, &[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    // T030: Test export with invalid variable name
    #[test]
    fn test_export_invalid_variable_name() {
        let mut executor = CommandExecutor::new();

        // Variable name starting with number
        let result = execute(&mut executor, &["123VAR=value".to_string()]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid variable name"));
    }

    // T031: Test export VAR=$OTHER (expansion at assignment)
    #[test]
    fn test_export_var_with_expansion() {
        let mut executor = CommandExecutor::new();

        // Set source variable
        executor
            .env_manager_mut()
            .set("SOURCE".to_string(), "source_value".to_string())
            .unwrap();

        // Export with expansion
        let result = execute(&mut executor, &["TARGET=$SOURCE".to_string()]);
        assert!(result.is_ok());

        // TARGET should have expanded value
        assert_eq!(executor.env_manager().get("TARGET"), Some("source_value"));
    }

    #[test]
    fn test_export_empty_value() {
        let mut executor = CommandExecutor::new();

        let result = execute(&mut executor, &["EMPTY=".to_string()]);
        assert!(result.is_ok());
        assert_eq!(executor.env_manager().get("EMPTY"), Some(""));
    }

    #[test]
    fn test_export_multiple_vars() {
        let mut executor = CommandExecutor::new();

        let result = execute(&mut executor, &["A=1".to_string(), "B=2".to_string()]);
        assert!(result.is_ok());

        assert_eq!(executor.env_manager().get("A"), Some("1"));
        assert_eq!(executor.env_manager().get("B"), Some("2"));
    }

    #[test]
    fn test_export_missing_equals() {
        let mut executor = CommandExecutor::new();

        let result = execute(&mut executor, &["NOEQUALS".to_string()]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("expected VAR=value"));
    }

    #[test]
    fn test_export_value_with_equals() {
        let mut executor = CommandExecutor::new();

        // Value containing = should work (split on first =)
        let result = execute(&mut executor, &["KEY=a=b=c".to_string()]);
        assert!(result.is_ok());
        assert_eq!(executor.env_manager().get("KEY"), Some("a=b=c"));
    }

    #[test]
    fn test_export_invalid_name_with_dash() {
        let mut executor = CommandExecutor::new();

        let result = execute(&mut executor, &["MY-VAR=value".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_export_underscore_prefix() {
        let mut executor = CommandExecutor::new();

        let result = execute(&mut executor, &["_PRIVATE=secret".to_string()]);
        assert!(result.is_ok());
        assert_eq!(executor.env_manager().get("_PRIVATE"), Some("secret"));
    }

    #[test]
    fn test_export_path_append() {
        let mut executor = CommandExecutor::new();

        // Set initial PATH
        executor
            .env_manager_mut()
            .set("PATH".to_string(), "/usr/bin".to_string())
            .unwrap();

        // Append to PATH
        let result = execute(&mut executor, &["PATH=$PATH:/custom/bin".to_string()]);
        assert!(result.is_ok());
        assert_eq!(executor.env_manager().get("PATH"), Some("/usr/bin:/custom/bin"));
    }

    #[test]
    fn test_export_braced_expansion() {
        let mut executor = CommandExecutor::new();

        executor
            .env_manager_mut()
            .set("BASE".to_string(), "/home".to_string())
            .unwrap();

        let result = execute(&mut executor, &["MYDIR=${BASE}/mydir".to_string()]);
        assert!(result.is_ok());
        assert_eq!(executor.env_manager().get("MYDIR"), Some("/home/mydir"));
    }
}
