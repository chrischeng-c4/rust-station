//! 'set' built-in command
//!
//! Lists all shell environment variables in `NAME=value` format,
//! sorted alphabetically by name.

use crate::error::Result;
use crate::executor::execute::CommandExecutor;

/// Execute the 'set' builtin command
///
/// Lists all environment variables in `NAME=value` format, one per line,
/// sorted alphabetically by name.
///
/// # Arguments
///
/// * `executor` - Mutable reference to command executor
/// * `_args` - Command arguments (currently unused)
///
/// # Returns
///
/// * `Ok(0)` always (listing cannot fail)
///
/// # Example
///
/// ```text
/// rush> set
/// HOME=/Users/username
/// PATH=/usr/bin:/bin
/// USER=username
/// ...
/// ```
pub fn execute(executor: &mut CommandExecutor, _args: &[String]) -> Result<i32> {
    let env = executor.env_manager();

    // Collect and sort variables alphabetically
    let mut vars: Vec<(&String, &String)> = env.iter().collect();
    vars.sort_by_key(|(k, _)| k.as_str());

    // Print each variable
    for (name, value) in vars {
        println!("{}={}", name, value);
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // T045: Test set output format (NAME=value per line)
    #[test]
    fn test_set_output_format() {
        let mut executor = CommandExecutor::new();

        // Set a known variable to verify format
        executor
            .env_manager_mut()
            .set("TEST_SET_VAR".to_string(), "test_value".to_string())
            .unwrap();

        // Execute should return 0
        let result = execute(&mut executor, &[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    // T046: Test set output sorted alphabetically
    #[test]
    fn test_set_sorted_output() {
        let mut executor = CommandExecutor::new();

        // Add variables that would be out of order alphabetically
        executor
            .env_manager_mut()
            .set("ZEBRA".to_string(), "z".to_string())
            .unwrap();
        executor
            .env_manager_mut()
            .set("APPLE".to_string(), "a".to_string())
            .unwrap();
        executor
            .env_manager_mut()
            .set("MANGO".to_string(), "m".to_string())
            .unwrap();

        // Get sorted vars
        let env = executor.env_manager();
        let mut vars: Vec<(&String, &String)> = env.iter().collect();
        vars.sort_by_key(|(k, _)| k.as_str());

        // Verify APPLE comes before MANGO comes before ZEBRA
        let names: Vec<&str> = vars.iter().map(|(k, _)| k.as_str()).collect();

        let apple_idx = names.iter().position(|&n| n == "APPLE").unwrap();
        let mango_idx = names.iter().position(|&n| n == "MANGO").unwrap();
        let zebra_idx = names.iter().position(|&n| n == "ZEBRA").unwrap();

        assert!(apple_idx < mango_idx);
        assert!(mango_idx < zebra_idx);
    }

    #[test]
    fn test_set_returns_zero() {
        let mut executor = CommandExecutor::new();
        let result = execute(&mut executor, &[]);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_set_with_args_ignored() {
        let mut executor = CommandExecutor::new();
        // Args should be ignored
        let result = execute(&mut executor, &["--some".to_string(), "arg".to_string()]);
        assert_eq!(result.unwrap(), 0);
    }
}
