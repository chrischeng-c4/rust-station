//! Tab completion engine module
//!
//! Provides completions for:
//! - Commands from PATH
//! - Filesystem paths
//! - Command flags

pub mod command;
pub mod flag;
pub mod path;

/// A potential completion match for user input
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionResult {
    /// Completion text to insert
    pub text: String,

    /// What kind of completion this is
    pub completion_type: CompletionType,

    /// Optional helper text
    pub description: Option<String>,

    /// Relevance score for ranking (0.0 to 1.0, higher is better)
    pub score: f32,
}

impl CompletionResult {
    /// Create a new completion result
    pub fn new(
        text: String,
        completion_type: CompletionType,
        description: Option<String>,
        score: f32,
    ) -> Self {
        Self {
            text,
            completion_type,
            description,
            score: score.clamp(0.0, 1.0),
        }
    }
}

/// Type of completion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionType {
    /// Executable from PATH
    Command,

    /// File or directory
    Path,

    /// Command-line flag
    Flag,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_result_new() {
        let result = CompletionResult::new(
            "ls".to_string(),
            CompletionType::Command,
            Some("list files".to_string()),
            1.0,
        );

        assert_eq!(result.text, "ls");
        assert_eq!(result.completion_type, CompletionType::Command);
        assert_eq!(result.description, Some("list files".to_string()));
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn test_completion_score_clamping() {
        let result1 = CompletionResult::new(
            "test".to_string(),
            CompletionType::Path,
            None,
            1.5, // Above max
        );
        assert_eq!(result1.score, 1.0);

        let result2 = CompletionResult::new(
            "test".to_string(),
            CompletionType::Flag,
            None,
            -0.5, // Below min
        );
        assert_eq!(result2.score, 0.0);
    }

    #[test]
    fn test_completion_types() {
        let types = vec![
            CompletionType::Command,
            CompletionType::Path,
            CompletionType::Flag,
        ];
        assert_eq!(types.len(), 3);
    }

    #[test]
    fn test_completion_clone() {
        let result1 = CompletionResult::new(
            "echo".to_string(),
            CompletionType::Command,
            None,
            0.8,
        );
        let result2 = result1.clone();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_completion_with_description() {
        let result = CompletionResult::new(
            "git".to_string(),
            CompletionType::Command,
            Some("version control system".to_string()),
            0.95,
        );

        assert_eq!(result.text, "git");
        assert_eq!(result.description, Some("version control system".to_string()));
        assert_eq!(result.score, 0.95);
    }

    #[test]
    fn test_completion_types_equality() {
        assert_eq!(CompletionType::Command, CompletionType::Command);
        assert_ne!(CompletionType::Command, CompletionType::Path);
        assert_ne!(CompletionType::Path, CompletionType::Flag);
    }

    #[test]
    fn test_completion_score_boundaries() {
        // Test exact boundaries
        let result1 = CompletionResult::new(
            "test".to_string(),
            CompletionType::Command,
            None,
            0.0,
        );
        assert_eq!(result1.score, 0.0);

        let result2 = CompletionResult::new(
            "test".to_string(),
            CompletionType::Command,
            None,
            1.0,
        );
        assert_eq!(result2.score, 1.0);
    }

    #[test]
    fn test_completion_extreme_scores() {
        // Test very large positive score
        let result1 = CompletionResult::new(
            "test".to_string(),
            CompletionType::Command,
            None,
            1000.0,
        );
        assert_eq!(result1.score, 1.0); // Should clamp to 1.0

        // Test very large negative score
        let result2 = CompletionResult::new(
            "test".to_string(),
            CompletionType::Command,
            None,
            -1000.0,
        );
        assert_eq!(result2.score, 0.0); // Should clamp to 0.0
    }

    #[test]
    fn test_completion_result_equality() {
        let result1 = CompletionResult::new(
            "ls".to_string(),
            CompletionType::Command,
            Some("list".to_string()),
            0.9,
        );

        let result2 = CompletionResult::new(
            "ls".to_string(),
            CompletionType::Command,
            Some("list".to_string()),
            0.9,
        );

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_completion_different_types() {
        let cmd = CompletionResult::new(
            "test".to_string(),
            CompletionType::Command,
            None,
            0.5,
        );

        let path = CompletionResult::new(
            "test".to_string(),
            CompletionType::Path,
            None,
            0.5,
        );

        let flag = CompletionResult::new(
            "test".to_string(),
            CompletionType::Flag,
            None,
            0.5,
        );

        assert_ne!(cmd.completion_type, path.completion_type);
        assert_ne!(path.completion_type, flag.completion_type);
        assert_ne!(cmd.completion_type, flag.completion_type);
    }
}
