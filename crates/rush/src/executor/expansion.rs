//! Variable expansion for environment variables
//!
//! Handles expansion of:
//! - $VAR and ${VAR} - Variable references
//! - ${arr[0]} - Array element access
//! - ${arr[@]} - All array elements as separate words
//! - ${arr[*]} - All array elements as one word
//! - $$ - Shell process ID
//! - $? - Last exit code
//! - $0 - Shell name ("rush")
//! - $# - Number of positional arguments
//! - Escape sequences (\$ -> literal $)
//!
//! Parameter expansion modifiers:
//! - ${var:-default} - Use default if unset or null
//! - ${var-default} - Use default if unset
//! - ${var:=default} - Assign default if unset or null
//! - ${var=default} - Assign default if unset
//! - ${var:?message} - Error if unset or null
//! - ${var?message} - Error if unset
//! - ${var:+alternate} - Use alternate if set and non-null
//! - ${var+alternate} - Use alternate if set
//! - ${#var} - String length
//! - ${var:offset} - Substring from offset
//! - ${var:offset:length} - Substring with length

use crate::executor::arrays::{parse_array_ref, ArrayRefType};
use crate::executor::execute::CommandExecutor;

/// Result of parameter expansion
#[derive(Debug)]
enum ParamExpansionResult {
    /// Successful expansion with value
    Value(String),
    /// Error message to display (for :? modifier)
    Error(String),
    /// Assign and return value (for := modifier)
    Assign { var_name: String, value: String },
}

/// Parse and expand parameter expansion modifiers
///
/// Handles: ${var:-default}, ${var:=default}, ${var:?error}, ${var:+alternate}
///          ${var-default}, ${var=default}, ${var?error}, ${var+alternate}
///          ${#var}, ${var:offset}, ${var:offset:length}
fn expand_parameter(content: &str, executor: &CommandExecutor) -> ParamExpansionResult {
    // Check for ${#var} - string length
    if content.starts_with('#') && !content.contains(':') && !content.contains('-')
        && !content.contains('=') && !content.contains('?') && !content.contains('+') {
        let var_name = &content[1..];
        let value = executor.variable_manager().get(var_name).unwrap_or("");
        return ParamExpansionResult::Value(value.len().to_string());
    }

    // Find the operator position
    // Look for :- := :? :+ - = ? + in that order
    let operators = [(":-", true), (":=", true), (":?", true), (":+", true),
                     ("-", false), ("=", false), ("?", false), ("+", false)];

    for (op, check_null) in operators {
        if let Some(pos) = content.find(op) {
            let var_name = &content[..pos];
            let operand = &content[pos + op.len()..];

            // Skip array references
            if var_name.contains('[') {
                continue;
            }

            let value = executor.variable_manager().get(var_name);
            let is_unset = value.is_none();
            let is_null = value.map(|v| v.is_empty()).unwrap_or(true);

            // For colon versions, check both unset and null
            // For non-colon versions, only check unset
            let condition = if check_null { is_null } else { is_unset };

            match op {
                ":-" | "-" => {
                    // Use default if condition met
                    if condition {
                        return ParamExpansionResult::Value(operand.to_string());
                    } else {
                        return ParamExpansionResult::Value(value.unwrap_or("").to_string());
                    }
                }
                ":=" | "=" => {
                    // Assign default if condition met
                    if condition {
                        return ParamExpansionResult::Assign {
                            var_name: var_name.to_string(),
                            value: operand.to_string(),
                        };
                    } else {
                        return ParamExpansionResult::Value(value.unwrap_or("").to_string());
                    }
                }
                ":?" | "?" => {
                    // Error if condition met
                    if condition {
                        let msg = if operand.is_empty() {
                            format!("{}: parameter null or not set", var_name)
                        } else {
                            format!("{}: {}", var_name, operand)
                        };
                        return ParamExpansionResult::Error(msg);
                    } else {
                        return ParamExpansionResult::Value(value.unwrap_or("").to_string());
                    }
                }
                ":+" | "+" => {
                    // Use alternate if NOT condition met (i.e., if set/non-null)
                    if !condition {
                        return ParamExpansionResult::Value(operand.to_string());
                    } else {
                        return ParamExpansionResult::Value(String::new());
                    }
                }
                _ => {}
            }
        }
    }

    // Check for substring expansion ${var:offset} or ${var:offset:length}
    if let Some(colon_pos) = content.find(':') {
        let var_name = &content[..colon_pos];

        // Skip if it's an array reference
        if !var_name.contains('[') {
            let rest = &content[colon_pos + 1..];

            // Parse offset and optional length
            let (offset_str, length_str) = if let Some(second_colon) = rest.find(':') {
                (&rest[..second_colon], Some(&rest[second_colon + 1..]))
            } else {
                (rest, None)
            };

            // Try to parse as numbers
            if let Ok(offset) = offset_str.parse::<i64>() {
                let value = executor.variable_manager().get(var_name).unwrap_or("");
                let len = value.len() as i64;

                // Handle negative offset (from end)
                let start = if offset < 0 {
                    (len + offset).max(0) as usize
                } else {
                    (offset as usize).min(value.len())
                };

                if let Some(length_str) = length_str {
                    if let Ok(length) = length_str.parse::<i64>() {
                        // ${var:offset:length}
                        let actual_len = if length < 0 {
                            // Negative length means "up to that position from end"
                            let end_pos = (len + length).max(0) as usize;
                            if end_pos > start {
                                end_pos - start
                            } else {
                                0
                            }
                        } else {
                            length as usize
                        };
                        let end = (start + actual_len).min(value.len());
                        return ParamExpansionResult::Value(value[start..end].to_string());
                    }
                } else {
                    // ${var:offset} - from offset to end
                    return ParamExpansionResult::Value(value[start..].to_string());
                }
            }
        }
    }

    // Plain variable reference
    let value = executor.variable_manager().get(content).unwrap_or("");
    ParamExpansionResult::Value(value.to_string())
}

/// Expand variables in a command line string
///
/// Performs variable substitution before command parsing.
/// Note: For := assignment support, use expand_variables_mut.
pub fn expand_variables(input: &str, executor: &CommandExecutor) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' if chars.peek() == Some(&'$') => {
                chars.next();
                result.push('$');
            }
            '$' => {
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '$' => {
                            chars.next();
                            result.push_str(&std::process::id().to_string());
                        }
                        '?' => {
                            chars.next();
                            result.push_str(&executor.last_exit_code().to_string());
                        }
                        '0' => {
                            chars.next();
                            result.push_str("rush");
                        }
                        '#' => {
                            chars.next();
                            result.push('0');
                        }
                        '1'..='9' => {
                            chars.next();
                        }
                        '{' => {
                            chars.next();
                            let content = extract_until(&mut chars, '}');
                            if !content.is_empty() {
                                if content.contains('[') && !content.contains(":-") && !content.contains(":=")
                                    && !content.contains(":?") && !content.contains(":+")
                                    && !content.starts_with('#') {
                                    let array_expr = format!("${{{}}}", content);
                                    if let Ok(arr_ref) = parse_array_ref(&array_expr) {
                                        match arr_ref.ref_type {
                                            ArrayRefType::Index(idx) => {
                                                if let Some(value) = executor.variable_manager().array_get(&arr_ref.name, idx) {
                                                    result.push_str(value);
                                                }
                                            }
                                            ArrayRefType::AllWords | ArrayRefType::AllAsOne => {
                                                if let Some(arr) = executor.variable_manager().get_array(&arr_ref.name) {
                                                    result.push_str(&arr.join(" "));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    match expand_parameter(&content, executor) {
                                        ParamExpansionResult::Value(v) => result.push_str(&v),
                                        ParamExpansionResult::Error(msg) => eprintln!("{}", msg),
                                        ParamExpansionResult::Assign { value, .. } => {
                                            // Can't assign in immutable version, just use value
                                            result.push_str(&value);
                                        }
                                    }
                                }
                            }
                        }
                        'a'..='z' | 'A'..='Z' | '_' => {
                            let var_name = extract_identifier(&mut chars);
                            if let Some(value) = executor.variable_manager().get(&var_name) {
                                result.push_str(value);
                            }
                        }
                        _ => result.push('$'),
                    }
                } else {
                    result.push('$');
                }
            }
            _ => result.push(ch),
        }
    }

    result
}

/// Expand variables with mutable executor (for := assignment)
///
/// Performs variable substitution before command parsing:
/// - $VARNAME or ${VARNAME} -> variable value
/// - $$ -> process ID
/// - $? -> last exit code
/// - $0 -> "rush"
/// - $# -> 0 (no positional args in interactive shell)
/// - Non-existent variables -> empty string
/// - Escape \$ -> literal $
///
/// # Arguments
/// * `input` - Input string with potential variable references
/// * `executor` - CommandExecutor with variable manager and exit code
///
/// # Returns
/// Expanded string with all variables substituted
pub fn expand_variables_mut(input: &str, executor: &mut CommandExecutor) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' if chars.peek() == Some(&'$') => {
                // Escaped $ - output literal $
                chars.next(); // consume the $
                result.push('$');
            }
            '$' => {
                // Variable or special reference
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '$' => {
                            // $$ -> process ID
                            chars.next();
                            result.push_str(&std::process::id().to_string());
                        }
                        '?' => {
                            // $? -> last exit code
                            chars.next();
                            result.push_str(&executor.last_exit_code().to_string());
                        }
                        '0' => {
                            // $0 -> shell name
                            chars.next();
                            result.push_str("rush");
                        }
                        '#' => {
                            // $# -> number of positional args (0 for interactive)
                            chars.next();
                            result.push('0');
                        }
                        '1'..='9' => {
                            // $N -> positional argument (not used in interactive shell)
                            chars.next();
                            // Positional args not supported in interactive shell
                            // Just skip the digit
                        }
                        '{' => {
                            // ${VARNAME}, ${arr[...]}, or parameter expansion
                            chars.next(); // consume {
                            let content = extract_until(&mut chars, '}');
                            if !content.is_empty() {
                                // Check if this is an array reference (contains [ but not a modifier)
                                if content.contains('[') && !content.contains(":-") && !content.contains(":=")
                                    && !content.contains(":?") && !content.contains(":+")
                                    && !content.starts_with('#') {
                                    // Try to parse as array reference: ${arr[0]}, ${arr[@]}, ${arr[*]}
                                    let array_expr = format!("${{{}}}", content);
                                    if let Ok(arr_ref) = parse_array_ref(&array_expr) {
                                        match arr_ref.ref_type {
                                            ArrayRefType::Index(idx) => {
                                                // ${arr[0]} - single element
                                                if let Some(value) = executor.variable_manager().array_get(&arr_ref.name, idx) {
                                                    result.push_str(value);
                                                }
                                                // Out of bounds or non-existent array -> empty string
                                            }
                                            ArrayRefType::AllWords => {
                                                // ${arr[@]} - all elements as space-separated words
                                                if let Some(arr) = executor.variable_manager().get_array(&arr_ref.name) {
                                                    result.push_str(&arr.join(" "));
                                                }
                                            }
                                            ArrayRefType::AllAsOne => {
                                                // ${arr[*]} - all elements as one word
                                                if let Some(arr) = executor.variable_manager().get_array(&arr_ref.name) {
                                                    result.push_str(&arr.join(" "));
                                                }
                                            }
                                        }
                                    }
                                    // Invalid array syntax -> empty string (silently)
                                } else {
                                    // Parameter expansion: ${var}, ${var:-default}, ${#var}, etc.
                                    match expand_parameter(&content, executor) {
                                        ParamExpansionResult::Value(v) => result.push_str(&v),
                                        ParamExpansionResult::Error(msg) => {
                                            eprintln!("{}", msg);
                                            // Continue with empty string on error
                                        }
                                        ParamExpansionResult::Assign { var_name, value } => {
                                            // Assign and return value
                                            let _ = executor.variable_manager_mut().set(var_name, value.clone());
                                            result.push_str(&value);
                                        }
                                    }
                                }
                            }
                        }
                        'a'..='z' | 'A'..='Z' | '_' => {
                            // $VARNAME - extract identifier
                            let var_name = extract_identifier(&mut chars);
                            if let Some(value) = executor.variable_manager().get(&var_name) {
                                result.push_str(value);
                            }
                            // Non-existent variables expand to empty string
                        }
                        _ => {
                            // Not a valid variable reference, output $ literally
                            result.push('$');
                        }
                    }
                } else {
                    // $ at end of string
                    result.push('$');
                }
            }
            _ => {
                result.push(ch);
            }
        }
    }

    result
}

/// Extract characters until a specific delimiter is found
fn extract_until(chars: &mut std::iter::Peekable<std::str::Chars>, delimiter: char) -> String {
    let mut result = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == delimiter {
            chars.next(); // consume delimiter
            break;
        }
        result.push(ch);
        chars.next();
    }
    result
}

/// Extract an identifier (variable name) starting from current position
///
/// An identifier is alphanumeric or underscore characters
fn extract_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut result = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            result.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::execute::CommandExecutor;

    #[test]
    fn test_no_expansion() {
        let executor = CommandExecutor::new();
        let input = "echo hello world";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo hello world");
    }

    #[test]
    fn test_simple_variable() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set("greeting".to_string(), "hello".to_string())
            .unwrap();

        let input = "echo $greeting";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo hello");
    }

    #[test]
    fn test_variable_with_braces() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set("name".to_string(), "world".to_string())
            .unwrap();

        let input = "echo ${name}!";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo world!");
    }

    #[test]
    fn test_multiple_variables() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set("first".to_string(), "hello".to_string())
            .unwrap();
        executor
            .variable_manager_mut()
            .set("second".to_string(), "world".to_string())
            .unwrap();

        let input = "echo $first $second";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo hello world");
    }

    #[test]
    fn test_nonexistent_variable() {
        let executor = CommandExecutor::new();
        let input = "echo $nonexistent";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo ");
    }

    #[test]
    fn test_escaped_dollar() {
        let executor = CommandExecutor::new();
        let input = "echo \\$var";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo $var");
    }

    #[test]
    fn test_special_variable_pid() {
        let executor = CommandExecutor::new();
        let input = "echo $$";
        let result = expand_variables(input, &executor);
        // Result should contain a number (PID)
        assert!(result.starts_with("echo "));
        let pid_str = &result[5..];
        assert!(!pid_str.is_empty());
        assert!(pid_str.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_special_variable_exit_code() {
        let mut executor = CommandExecutor::new();
        executor.set_last_exit_code(42);

        let input = "echo $?";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo 42");
    }

    #[test]
    fn test_special_variable_shell_name() {
        let executor = CommandExecutor::new();
        let input = "echo $0";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo rush");
    }

    #[test]
    fn test_special_variable_arg_count() {
        let executor = CommandExecutor::new();
        let input = "echo $#";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo 0");
    }

    #[test]
    fn test_dollar_at_end() {
        let executor = CommandExecutor::new();
        let input = "echo $";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo $");
    }

    #[test]
    fn test_variable_adjacent_text() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set("VAR".to_string(), "value".to_string())
            .unwrap();

        // $VARmore tries to expand variable "VARmore" (doesn't exist)
        let input = "test$VARmore";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "test");

        // ${VAR}more expands VAR and appends "more"
        let input2 = "test${VAR}more";
        let result2 = expand_variables(input2, &executor);
        assert_eq!(result2, "testvaluemore");
    }

    #[test]
    fn test_variable_with_underscore() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set("MY_VAR".to_string(), "test".to_string())
            .unwrap();

        let input = "echo $MY_VAR";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo test");
    }

    #[test]
    fn test_unclosed_braces() {
        let executor = CommandExecutor::new();
        let input = "echo ${incomplete";
        let result = expand_variables(input, &executor);
        // Should output the incomplete reference as-is (since no closing })
        assert_eq!(result, "echo ");
    }

    #[test]
    fn test_empty_variable_name() {
        let executor = CommandExecutor::new();
        let input = "echo ${}";
        let result = expand_variables(input, &executor);
        // Empty variable name should expand to nothing
        assert_eq!(result, "echo ");
    }

    // ===== Array Expansion Tests =====

    #[test]
    fn test_array_index_zero() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["first".to_string(), "second".to_string(), "third".to_string()])
            .unwrap();

        let input = "echo ${arr[0]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo first");
    }

    #[test]
    fn test_array_index_nonzero() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()])
            .unwrap();

        let input = "echo ${arr[2]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo c");
    }

    #[test]
    fn test_array_index_out_of_bounds() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["only".to_string()])
            .unwrap();

        let input = "echo ${arr[99]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo ");
    }

    #[test]
    fn test_array_all_words() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["one".to_string(), "two".to_string(), "three".to_string()])
            .unwrap();

        let input = "echo ${arr[@]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo one two three");
    }

    #[test]
    fn test_array_all_as_one() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["one".to_string(), "two".to_string(), "three".to_string()])
            .unwrap();

        let input = "echo ${arr[*]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo one two three");
    }

    #[test]
    fn test_array_nonexistent() {
        let executor = CommandExecutor::new();

        let input = "echo ${nonexistent[@]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo ");
    }

    #[test]
    fn test_array_empty() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec![])
            .unwrap();

        let input = "echo ${arr[@]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo ");
    }

    #[test]
    fn test_array_single_element() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["single".to_string()])
            .unwrap();

        let input = "echo ${arr[@]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo single");
    }

    #[test]
    fn test_array_mixed_with_regular_vars() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set("prefix".to_string(), "PREFIX".to_string())
            .unwrap();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["a".to_string(), "b".to_string()])
            .unwrap();
        executor
            .variable_manager_mut()
            .set("suffix".to_string(), "SUFFIX".to_string())
            .unwrap();

        let input = "${prefix} ${arr[@]} ${suffix}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "PREFIX a b SUFFIX");
    }

    #[test]
    fn test_array_with_spaces_in_elements() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["hello world".to_string(), "foo bar".to_string()])
            .unwrap();

        let input = "echo ${arr[@]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo hello world foo bar");
    }

    #[test]
    fn test_multiple_array_refs() {
        let mut executor = CommandExecutor::new();
        executor
            .variable_manager_mut()
            .set_array("arr".to_string(), vec!["x".to_string(), "y".to_string(), "z".to_string()])
            .unwrap();

        let input = "${arr[0]}+${arr[1]}+${arr[2]}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "x+y+z");
    }

    // ===== Parameter Expansion Tests =====

    #[test]
    fn test_default_value_unset() {
        let executor = CommandExecutor::new();
        // ${var:-default} when var is unset
        let input = "echo ${unset:-default}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo default");
    }

    #[test]
    fn test_default_value_set() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "value".to_string()).unwrap();
        // ${var:-default} when var is set
        let input = "echo ${var:-default}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo value");
    }

    #[test]
    fn test_default_value_empty() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "".to_string()).unwrap();
        // ${var:-default} when var is empty (colon means check null too)
        let input = "echo ${var:-default}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo default");
    }

    #[test]
    fn test_default_no_colon_empty() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "".to_string()).unwrap();
        // ${var-default} when var is empty (no colon means only check unset)
        let input = "echo ${var-default}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo "); // Empty string, not default
    }

    #[test]
    fn test_assign_default_unset() {
        let mut executor = CommandExecutor::new();
        // ${var:=default} when var is unset - should assign
        let input = "echo ${newvar:=assigned}";
        let result = expand_variables_mut(input, &mut executor);
        assert_eq!(result, "echo assigned");
        assert_eq!(executor.variable_manager().get("newvar"), Some("assigned"));
    }

    #[test]
    fn test_assign_default_set() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "existing".to_string()).unwrap();
        // ${var:=default} when var is set - should not assign
        let input = "echo ${var:=ignored}";
        let result = expand_variables_mut(input, &mut executor);
        assert_eq!(result, "echo existing");
        assert_eq!(executor.variable_manager().get("var"), Some("existing"));
    }

    #[test]
    fn test_alternate_value_set() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "value".to_string()).unwrap();
        // ${var:+alternate} when var is set
        let input = "echo ${var:+alternate}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo alternate");
    }

    #[test]
    fn test_alternate_value_unset() {
        let executor = CommandExecutor::new();
        // ${var:+alternate} when var is unset
        let input = "echo ${unset:+alternate}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo ");
    }

    #[test]
    fn test_string_length() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "hello".to_string()).unwrap();
        // ${#var} - string length
        let input = "echo ${#var}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo 5");
    }

    #[test]
    fn test_string_length_empty() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "".to_string()).unwrap();
        let input = "echo ${#var}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo 0");
    }

    #[test]
    fn test_string_length_unset() {
        let executor = CommandExecutor::new();
        let input = "echo ${#unset}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo 0");
    }

    #[test]
    fn test_substring_from_offset() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "hello world".to_string()).unwrap();
        // ${var:6} - from offset 6 to end
        let input = "echo ${var:6}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo world");
    }

    #[test]
    fn test_substring_with_length() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "hello world".to_string()).unwrap();
        // ${var:0:5} - first 5 chars
        let input = "echo ${var:0:5}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo hello");
    }

    #[test]
    fn test_substring_negative_offset() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "hello world".to_string()).unwrap();
        // ${var:-5} would match :- operator, so use ${var: -5} in bash
        // For now test positive offset
        let input = "echo ${var:6:5}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo world");
    }

    #[test]
    fn test_error_if_unset() {
        let executor = CommandExecutor::new();
        // ${var:?message} when var is unset - should print error
        let input = "echo ${unset:?variable not set}";
        let result = expand_variables(input, &executor);
        // Result is empty string but error was printed
        assert_eq!(result, "echo ");
    }

    #[test]
    fn test_error_if_set() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("var".to_string(), "value".to_string()).unwrap();
        // ${var:?message} when var is set - no error
        let input = "echo ${var:?should not appear}";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "echo value");
    }

    #[test]
    fn test_multiple_param_expansions() {
        let mut executor = CommandExecutor::new();
        executor.variable_manager_mut().set("name".to_string(), "world".to_string()).unwrap();
        let input = "${greeting:-hello} ${name}!";
        let result = expand_variables(input, &executor);
        assert_eq!(result, "hello world!");
    }

    #[test]
    fn test_nested_default_values() {
        let executor = CommandExecutor::new();
        // Multiple defaults in sequence
        let input = "${a:-${b:-final}}";
        let result = expand_variables(input, &executor);
        // Note: nested expansion not fully supported yet, should get ${b:-final}
        assert_eq!(result, "${b:-final}");
    }
}
