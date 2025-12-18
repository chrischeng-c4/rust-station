//! Worktree view state
//!
//! This module defines the serializable state for WorktreeView.
//! Phase 1: P1 fields (10 fields) - feature context, content cache, phase tracking
//! Phase 3A: P1+P2 fields (19 fields) - added commands and logging/output state
//! Phase 3B: P1+P2+P3+P4+P5 fields (36 fields) - complete state-first architecture

use crate::domain::git::{CommitGroup, SecurityWarning};
use crate::tui::event::WorktreeType;
use crate::tui::logging::LogEntry;
use crate::tui::views::{Command, ContentType, FeatureInfo, GitCommand, InlineInput, PhaseStatus, SpecPhase, SpecifyState, WorktreeFocus};
use crate::tui::widgets::TextInput;
use serde::{Deserialize, Serialize};

use super::StateInvariants;

/// Worktree view state (Phase 3B: P1+P2+P3+P4+P5 fields)
///
/// This struct contains 36 core serializable fields:
/// - Feature context (2 fields) - P1
/// - Content cache (3 fields) - P1
/// - Phase tracking (2 fields) - P1
/// - UI state (3 fields) - P1
/// - Commands subsystem (2 fields) - P2
/// - Logging/Output subsystem (7 fields) - P2
/// - Input subsystem (3 fields) - P3
/// - Progress subsystem (3 fields) - P3
/// - Commit workflow (8 fields) - P4
/// - Specify workflow (1 field) - P5
/// - Prompt workflow (2 fields) - P5
///
/// This represents the complete state-first architecture implementation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorktreeViewState {
    // ========================================
    // Feature Context (2 fields)
    // ========================================

    /// Current feature information (if in feature worktree)
    pub feature_info: Option<FeatureInfo>,

    /// Worktree type (main repo vs feature worktree)
    pub worktree_type: WorktreeType,

    // ========================================
    // Content Cache (3 fields)
    // ========================================

    /// Cached spec.md content
    pub spec_content: Option<String>,

    /// Cached plan.md content
    pub plan_content: Option<String>,

    /// Cached tasks.md content
    pub tasks_content: Option<String>,

    // ========================================
    // Phase Tracking (2 fields)
    // ========================================

    /// SDD workflow phases with status
    pub phases: Vec<(SpecPhase, PhaseStatus)>,

    /// Currently active phase
    pub current_phase: Option<SpecPhase>,

    // ========================================
    // UI State (3 fields)
    // ========================================

    /// Current focus area
    pub focus: WorktreeFocus,

    /// Content type being displayed
    pub content_type: ContentType,

    /// Content scroll position (line number)
    pub content_scroll: usize,

    // ========================================
    // Commands Subsystem - P2 (2 fields)
    // ========================================

    /// Unified command list (SDD phases + Git actions)
    pub commands: Vec<Command>,

    /// Selected command index (derived from ListState)
    pub command_state_index: Option<usize>,

    // ========================================
    // Logging/Output Subsystem - P2 (7 fields)
    // ========================================

    /// Log entries (serializable form of LogBuffer)
    pub log_entries: Vec<LogEntry>,

    /// Output scroll position
    pub output_scroll: usize,

    /// Whether a command is currently running
    pub is_running: bool,

    /// Which phase is currently running (e.g., "Specify", "Plan")
    pub running_phase: Option<String>,

    /// Pending git command to execute
    pub pending_git_command: Option<GitCommand>,

    /// Active Claude session ID (for session continuation)
    pub active_session_id: Option<String>,

    /// Whether pending follow-up input from Claude
    pub pending_follow_up: bool,

    // ========================================
    // Input Subsystem - P3 (3 fields)
    // ========================================

    /// Pending input request for a specific phase
    pub pending_input_phase: Option<SpecPhase>,

    /// Multi-line prompt input widget
    pub prompt_input: Option<TextInput>,

    /// Inline input widget for Claude follow-up questions
    pub inline_input: Option<InlineInput>,

    // ========================================
    // Progress Subsystem - P3 (3 fields)
    // ========================================

    /// Current progress step
    pub progress_step: Option<u32>,

    /// Total progress steps
    pub progress_total: Option<u32>,

    /// Progress status message
    pub progress_message: Option<String>,

    // ========================================
    // Commit Workflow Subsystem - P4 (8 fields)
    // ========================================

    /// Pending commit message from intelligent commit workflow
    pub pending_commit_message: Option<String>,

    /// Security warnings found during commit scanning
    pub commit_warnings: Vec<SecurityWarning>,

    /// Grouped changes for staged commit workflow
    pub commit_groups: Option<Vec<CommitGroup>>,

    /// Current group index in commit review
    pub current_commit_index: usize,

    /// User input for commit message
    pub commit_message_input: String,

    /// Cursor position in commit message input
    pub commit_message_cursor: usize,

    /// Sensitive files found during commit
    pub commit_sensitive_files: Vec<String>,

    /// Commit validation error message
    pub commit_validation_error: Option<String>,

    // ========================================
    // Specify Workflow Subsystem - P5 (1 field)
    // ========================================

    /// SDD workflow state (Specify/Plan/Tasks phases)
    pub specify_state: SpecifyState,

    // ========================================
    // Prompt Workflow Subsystem - P5 (2 fields)
    // ========================================

    /// Prompt edit mode flag (toggle with i/Esc)
    pub prompt_edit_mode: bool,

    /// Accumulated streaming output from prompt
    pub prompt_output: String,
}

impl Default for WorktreeViewState {
    fn default() -> Self {
        let phases = SpecPhase::all()
            .iter()
            .map(|&p| (p, PhaseStatus::NotStarted))
            .collect::<Vec<_>>();

        // Build unified command list (Workflow + SDD phases + Git commands)
        let mut commands = Vec::new();
        // WORKFLOW section
        commands.push(Command::PromptClaude);
        // SDD section
        for (phase, status) in &phases {
            commands.push(Command::SddPhase(*phase, *status));
        }
        // GIT section
        for git_cmd in GitCommand::all() {
            commands.push(Command::GitAction(*git_cmd));
        }

        Self {
            // Feature context
            feature_info: None,
            worktree_type: WorktreeType::NotGit,

            // Content cache
            spec_content: None,
            plan_content: None,
            tasks_content: None,

            // Phase tracking
            phases,
            current_phase: None,

            // UI state
            focus: WorktreeFocus::Commands,
            content_type: ContentType::Spec, // Default to Spec view
            content_scroll: 0,

            // Commands subsystem (P2)
            commands,
            command_state_index: Some(1), // Start on "Prompt Claude"

            // Logging/Output subsystem (P2)
            log_entries: Vec::new(),
            output_scroll: 0,
            is_running: false,
            running_phase: None,
            pending_git_command: None,
            active_session_id: None,
            pending_follow_up: false,

            // Input subsystem (P3)
            pending_input_phase: None,
            prompt_input: None,
            inline_input: None,

            // Progress subsystem (P3)
            progress_step: None,
            progress_total: None,
            progress_message: None,

            // Commit workflow (P4)
            pending_commit_message: None,
            commit_warnings: Vec::new(),
            commit_groups: None,
            current_commit_index: 0,
            commit_message_input: String::new(),
            commit_message_cursor: 0,
            commit_sensitive_files: Vec::new(),
            commit_validation_error: None,

            // Specify workflow (P5)
            specify_state: SpecifyState::default(),

            // Prompt workflow (P5)
            prompt_edit_mode: false,
            prompt_output: String::new(),
        }
    }
}

impl StateInvariants for WorktreeViewState {
    fn assert_invariants(&self) {
        // Invariant 1: Feature info present for feature worktrees
        if matches!(self.worktree_type, WorktreeType::FeatureWorktree { .. }) {
            assert!(
                self.feature_info.is_some(),
                "Feature worktree requires feature_info"
            );
        }

        // Invariant 2: Content scroll within reasonable bounds (0..100000)
        assert!(
            self.content_scroll < 100000,
            "Content scroll position unreasonably large: {}",
            self.content_scroll
        );

        // Invariant 3: Output scroll within reasonable bounds (0..100000)
        assert!(
            self.output_scroll < 100000,
            "Output scroll position unreasonably large: {}",
            self.output_scroll
        );

        // Invariant 4: Command state index within bounds
        if let Some(idx) = self.command_state_index {
            assert!(
                idx < self.commands.len(),
                "Command state index {} out of bounds (commands.len = {})",
                idx,
                self.commands.len()
            );
        }

        // Invariant 5: Running phase implies is_running
        if self.running_phase.is_some() {
            assert!(
                self.is_running,
                "running_phase is set but is_running is false"
            );
        }

        // Phase 3A: Added P2 invariants
        // More invariants will be added in Phase 3B as more fields are added
    }
}

impl WorktreeViewState {
    /// Get phase status for a specific phase
    pub fn get_phase_status(&self, phase: SpecPhase) -> PhaseStatus {
        self.phases
            .iter()
            .find(|(p, _)| *p == phase)
            .map(|(_, status)| *status)
            .unwrap_or(PhaseStatus::NotStarted)
    }

    /// Set phase status for a specific phase
    pub fn set_phase_status(&mut self, phase: SpecPhase, status: PhaseStatus) {
        for (p, s) in &mut self.phases {
            if *p == phase {
                *s = status;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_is_valid() {
        let state = WorktreeViewState::default();
        state.assert_invariants();
    }

    #[test]
    fn test_get_phase_status() {
        let state = WorktreeViewState::default();
        assert_eq!(
            state.get_phase_status(SpecPhase::Specify),
            PhaseStatus::NotStarted
        );
    }

    #[test]
    fn test_set_phase_status() {
        let mut state = WorktreeViewState::default();
        state.set_phase_status(SpecPhase::Specify, PhaseStatus::Completed);
        assert_eq!(
            state.get_phase_status(SpecPhase::Specify),
            PhaseStatus::Completed
        );
    }
}
