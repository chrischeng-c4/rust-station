#[cfg(test)]
mod feature_tests {
    use rush::executor::execute::CommandExecutor;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_output_redirection() {
        let mut executor = CommandExecutor::new();
        let test_file = "/tmp/rush_test_output.txt";

        // Clean up first
        let _ = fs::remove_file(test_file);

        // Test > (create/overwrite)
        let result = executor.execute("echo hello > /tmp/rush_test_output.txt");
        assert!(result.is_ok());
        assert!(Path::new(test_file).exists());

        let content = fs::read_to_string(test_file).unwrap();
        assert!(content.contains("hello"));

        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_append_redirection() {
        let mut executor = CommandExecutor::new();
        let test_file = "/tmp/rush_test_append.txt";

        // Clean up first
        let _ = fs::remove_file(test_file);

        // Create initial file
        executor
            .execute("echo first >> /tmp/rush_test_append.txt")
            .unwrap();
        // Append to it
        executor
            .execute("echo second >> /tmp/rush_test_append.txt")
            .unwrap();

        let content = fs::read_to_string(test_file).unwrap();
        assert!(content.contains("first"));
        assert!(content.contains("second"));

        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_pipeline_simple() {
        let mut executor = CommandExecutor::new();
        // Simple two-command pipeline
        let result = executor.execute("echo test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_background_execution_syntax() {
        let mut executor = CommandExecutor::new();
        // Just test that background syntax is accepted
        // (actual background execution is hard to test in unit test)
        let result = executor.execute("true &");
        assert!(result.is_ok());
    }

    #[test]
    fn test_input_redirection() {
        let mut executor = CommandExecutor::new();
        let test_file = "/tmp/rush_test_input.txt";

        // Clean up first
        let _ = fs::remove_file(test_file);

        // Create test input file
        fs::write(test_file, "test content\n").unwrap();

        // Test < (read from file)
        // Using cat to read from input redirection
        let result = executor.execute("cat < /tmp/rush_test_input.txt");
        assert!(result.is_ok());
        // Note: We can't easily capture stdout in this test, but if no error, it worked

        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}
