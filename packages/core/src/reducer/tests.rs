use crate::actions::Action;
use crate::app_state::{AppState, Theme};
use crate::reducer::reduce;

/// Helper to create a state with one project for testing
fn state_with_project() -> AppState {
    let mut state = AppState::default();
    reduce(
        &mut state,
        Action::OpenProject {
            path: "/test/project".to_string(),
        },
    );
    state
}

/// Helper to get the active worktree from state (for tests)
fn active_worktree(state: &AppState) -> &crate::app_state::WorktreeState {
    state
        .active_project()
        .unwrap()
        .active_worktree()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_open_project() {
        let mut state = AppState::default();
        reduce(&mut state, Action::OpenProject { path: "/test/project".to_string() });
        assert_eq!(state.projects.len(), 1);
        assert_eq!(state.projects[0].name, "project");
    }

    // ========================================================================
    // MCP Tests
    // ========================================================================
    #[test]
    fn test_mcp_actions() {
        let mut state = state_with_project();
        
        reduce(&mut state, Action::StartMcpServer);
        assert_eq!(active_worktree(&state).mcp.status, crate::app_state::McpStatus::Starting);

        reduce(&mut state, Action::SetMcpPort { port: 8080 });
        assert_eq!(active_worktree(&state).mcp.port, Some(8080));
        assert_eq!(active_worktree(&state).mcp.status, crate::app_state::McpStatus::Running);

        reduce(&mut state, Action::StopMcpServer);
        assert_eq!(active_worktree(&state).mcp.status, crate::app_state::McpStatus::Stopped);
        assert!(active_worktree(&state).mcp.port.is_none());
    }

    // ========================================================================
    // Notification Tests
    // ========================================================================
    #[test]
    fn test_notification_actions() {
        let mut state = AppState::default();
        
        reduce(&mut state, Action::AddNotification { 
            message: "Test".to_string(), 
            notification_type: crate::actions::NotificationTypeData::Success 
        });
        assert_eq!(state.notifications.len(), 1);
        let id = state.notifications[0].id.clone();

        reduce(&mut state, Action::MarkNotificationRead { id: id.clone() });
        assert!(state.notifications[0].read);

        reduce(&mut state, Action::DismissNotification { id });
        assert_eq!(state.notifications.len(), 0);
    }

    // ========================================================================
    // Terminal Tests
    // ========================================================================
    #[test]
    fn test_terminal_actions() {
        let mut state = state_with_project();
        
        reduce(&mut state, Action::SpawnTerminal { cols: 80, rows: 24 });
        assert_eq!(active_worktree(&state).terminal.cols, 80);

        reduce(&mut state, Action::SetTerminalSession { session_id: Some("test-session".to_string()) });
        assert_eq!(active_worktree(&state).terminal.session_id, Some("test-session".to_string()));
    }

    // ========================================================================
    // Change Management Full Flow Tests
    // ========================================================================
    #[test]
    fn test_change_management_flow() {
        let mut state = state_with_project();

        // 1. Create Change
        reduce(&mut state, Action::CreateChange { intent: "Add Auth".to_string() });
        assert!(active_worktree(&state).changes.is_loading);

        // Mock change created (async part)
        {
            let mut state_write = state.clone();
            if let Some(project) = state_write.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.changes.is_loading = false;
                    worktree.changes.changes.push(crate::app_state::Change {
                        id: "ch-1".to_string(),
                        name: "Add Auth".to_string(),
                        status: crate::app_state::ChangeStatus::Proposed,
                        intent: "Add Auth".to_string(),
                        proposal: None,
                        plan: None,
                        streaming_output: String::new(),
                        created_at: "now".to_string(),
                        updated_at: "now".to_string(),
                        proposal_review_session_id: None,
                        plan_review_session_id: None,
                        context_files: vec![],
                    });
                }
            }
            state = state_write;
        }

        // 2. Generate Proposal
        reduce(&mut state, Action::GenerateProposal { change_id: "ch-1".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Planning);

        reduce(&mut state, Action::AppendProposalOutput { change_id: "ch-1".to_string(), content: "Proposal Content".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].streaming_output, "Proposal Content");

        reduce(&mut state, Action::CompleteProposal { change_id: "ch-1".to_string() });
        let change = &active_worktree(&state).changes.changes[0];
        assert_eq!(change.status, crate::app_state::ChangeStatus::Proposed);
        assert_eq!(change.proposal, Some("Proposal Content".to_string()));
        assert!(change.streaming_output.is_empty());
        assert!(change.proposal_review_session_id.is_some());

        // 3. Generate Plan
        reduce(&mut state, Action::GeneratePlan { change_id: "ch-1".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Planning);

        reduce(&mut state, Action::AppendPlanOutput { change_id: "ch-1".to_string(), content: "Plan Content".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].streaming_output, "Plan Content");

        reduce(&mut state, Action::CompletePlan { change_id: "ch-1".to_string() });
        let change = &active_worktree(&state).changes.changes[0];
        assert_eq!(change.status, crate::app_state::ChangeStatus::Planned);
        assert_eq!(change.plan, Some("Plan Content".to_string()));
        assert!(change.streaming_output.is_empty());
        assert!(change.plan_review_session_id.is_some());

        // 4. Approve Plan (Explicit approval step)
        reduce(&mut state, Action::ApprovePlan { change_id: "ch-1".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Planned);

        // 5. Execute Plan
        reduce(&mut state, Action::ExecutePlan { change_id: "ch-1".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Implementing);

        reduce(&mut state, Action::AppendImplementationOutput { change_id: "ch-1".to_string(), content: "Executing...".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].streaming_output, "Executing...");

        reduce(&mut state, Action::CompleteImplementation { change_id: "ch-1".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Done);
    }

    // ========================================================================
    // Context Tests
    // ========================================================================
    #[test]
    fn test_context_actions() {
        let mut state = state_with_project();
        
        reduce(&mut state, Action::GenerateContext);
        assert!(active_worktree(&state).context.is_generating);

        reduce(&mut state, Action::AppendGenerateContextOutput { content: "test".to_string() });
        assert_eq!(active_worktree(&state).context.generation_output, "test");

        reduce(&mut state, Action::CompleteGenerateContext);
        assert!(!active_worktree(&state).context.is_generating);
        assert!(active_worktree(&state).context.is_initialized);
    }

    // ========================================================================
    // Change Tests
    // ========================================================================
    #[test]
    fn test_change_transitions() {
        let mut state = state_with_project();
        
        // Mock a change
        {
            let mut state_write = state.clone();
            if let Some(project) = state_write.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.changes.changes.push(crate::app_state::Change {
                        id: "test-change".to_string(),
                        name: "Test Change".to_string(),
                        status: crate::app_state::ChangeStatus::Proposed,
                        intent: "test".to_string(),
                        proposal: None,
                        plan: None,
                        streaming_output: String::new(),
                        created_at: "now".to_string(),
                        updated_at: "now".to_string(),
                        proposal_review_session_id: None,
                        plan_review_session_id: None,
                        context_files: vec![],
                    });
                }
            }
            state = state_write;
        }

        reduce(&mut state, Action::CancelChange { change_id: "test-change".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Cancelled);

        reduce(&mut state, Action::FailImplementation { change_id: "test-change".to_string(), error: "failed".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Failed);

        reduce(&mut state, Action::SetChangeArchived { change_id: "test-change".to_string() });
        assert_eq!(active_worktree(&state).changes.changes[0].status, crate::app_state::ChangeStatus::Archived);
    }

    // ========================================================================
    // Serialization Tests
    // ========================================================================
    #[test]
    fn test_serialization_roundtrip() {
        let state = AppState::default();
        let json = serde_json::to_string(&state).unwrap();
        let loaded: AppState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, loaded);
    }

    // ========================================================================
    // Chat Tests
    // ========================================================================
    #[test]
    fn test_chat_actions() {
        let mut state = state_with_project();

        // Send message (sets typing)
        reduce(&mut state, Action::SendChatMessage { text: "Hello".to_string() });
        assert!(active_worktree(&state).chat.is_typing);

        // Add message
        let msg = crate::actions::ChatMessageData {
            id: "msg-1".to_string(),
            role: crate::actions::ChatRoleData::User,
            content: "Hello".to_string(),
            timestamp: "now".to_string(),
            is_streaming: false,
        };
        reduce(&mut state, Action::AddChatMessage { message: msg });
        assert_eq!(active_worktree(&state).chat.messages.len(), 1);

        // Streaming response
        let asst_msg = crate::actions::ChatMessageData {
            id: "msg-2".to_string(),
            role: crate::actions::ChatRoleData::Assistant,
            content: "".to_string(),
            timestamp: "now".to_string(),
            is_streaming: true,
        };
        reduce(&mut state, Action::AddChatMessage { message: asst_msg });
        reduce(&mut state, Action::AppendChatContent { content: "Hi".to_string() });
        assert_eq!(active_worktree(&state).chat.messages[1].content, "Hi");

        // Stop typing (finishes streaming)
        reduce(&mut state, Action::SetChatTyping { is_typing: false });
        assert!(!active_worktree(&state).chat.is_typing);
        assert!(!active_worktree(&state).chat.messages[1].is_streaming);

        // Clear chat
        reduce(&mut state, Action::ClearChat);
        assert!(active_worktree(&state).chat.messages.is_empty());
    }

    // ========================================================================
    // Docker Tests
    // ========================================================================
    #[test]
    fn test_docker_actions() {
        let mut state = state_with_project();

        reduce(&mut state, Action::CheckDockerAvailability);
        assert!(state.docker.is_loading);

        reduce(&mut state, Action::SetDockerAvailable { available: true });
        assert_eq!(state.docker.docker_available, Some(true));
        assert!(!state.docker.is_loading);

        // Add a service
        let service = crate::actions::DockerServiceData {
            id: "s1".to_string(),
            name: "Service 1".to_string(),
            image: "img".to_string(),
            status: "stopped".to_string(),
            port: None,
            service_type: "Other".to_string(),
            project_group: None,
            is_rstn_managed: true,
        };
        reduce(&mut state, Action::SetDockerServices { services: vec![service] });
        assert_eq!(state.docker.services.len(), 1);

        // Start service
        reduce(&mut state, Action::StartDockerService { service_id: "s1".to_string() });
        assert_eq!(state.docker.services[0].status, crate::app_state::ServiceStatus::Starting);

        // Connection string
        reduce(&mut state, Action::SetDockerConnectionString { connection_string: Some("conn".to_string()) });
        assert_eq!(state.docker.last_connection_string, Some("conn".to_string()));
    }

    // ========================================================================
    // Settings Tests
    // ========================================================================
    #[test]
    fn test_settings_actions() {
        let mut state = AppState::default();

        reduce(&mut state, Action::SetTheme { theme: Theme::Dark });
        assert_eq!(state.global_settings.theme, Theme::Dark);

        reduce(&mut state, Action::SetProjectPath { path: Some("/new/path".to_string()) });
        assert_eq!(state.global_settings.default_project_path, Some("/new/path".to_string()));
    }

    // ========================================================================
    // File Explorer Tests
    // ========================================================================
    #[test]
    fn test_explorer_actions() {
        let mut state = state_with_project();

        // Explore (loading)
        reduce(&mut state, Action::ExploreDir { path: "/test/dir".to_string() });
        assert!(active_worktree(&state).explorer.is_loading);

        // Set entries
        let entries = vec![crate::actions::FileEntryData {
            name: "file.txt".to_string(),
            path: "/test/dir/file.txt".to_string(),
            kind: crate::actions::FileKindData::File,
            size: 100,
            permissions: "rw-".to_string(),
            updated_at: "now".to_string(),
            comment_count: 0,
            git_status: None,
        }];
        reduce(&mut state, Action::SetExplorerEntries { path: "/test/dir".to_string(), entries });
        assert!(!active_worktree(&state).explorer.is_loading);
        assert_eq!(active_worktree(&state).explorer.entries.len(), 1);

        // Select file
        reduce(&mut state, Action::SelectFile { path: Some("/test/dir/file.txt".to_string()) });
        assert_eq!(active_worktree(&state).explorer.selected_path, Some("/test/dir/file.txt".to_string()));

        // Sort/Filter
        reduce(&mut state, Action::SetExplorerSort { 
            field: crate::actions::SortFieldData::Size, 
            direction: crate::actions::SortDirectionData::Desc 
        });
        assert_eq!(active_worktree(&state).explorer.sort_config.field, crate::app_state::SortField::Size);
        assert_eq!(active_worktree(&state).explorer.sort_config.direction, crate::app_state::SortDirection::Desc);

        reduce(&mut state, Action::SetExplorerFilter { query: "foo".to_string() });
        assert_eq!(active_worktree(&state).explorer.filter_query, "foo");
    }

    // ========================================================================
    // File Viewer Tests
    // ========================================================================
    #[test]
    fn test_file_viewer_actions() {
        let mut state = AppState::default();

        // Read file (loading)
        reduce(&mut state, Action::ReadFile { path: "/path/to/file".to_string() });
        assert!(state.file_viewer.is_loading);
        assert_eq!(state.file_viewer.path, Some("/path/to/file".to_string()));

        // Set content
        reduce(&mut state, Action::SetFileContent { 
            path: "/path/to/file".to_string(), 
            content: Some("content".to_string()),
            error: None
        });
        assert!(!state.file_viewer.is_loading);
        assert_eq!(state.file_viewer.content, Some("content".to_string()));

        // Set content with error
        reduce(&mut state, Action::SetFileContent { 
            path: "/path/to/file".to_string(), 
            content: None,
            error: Some("Failed".to_string())
        });
        assert_eq!(state.file_viewer.error, Some("Failed".to_string()));
    }

    // ========================================================================
    // ReviewGate Tests
    // ========================================================================
    #[test]
    fn test_review_gate_actions() {
        let mut state = state_with_project();

        // Start Review
        let content = crate::actions::ReviewContentData {
            content_type: crate::actions::ReviewContentTypeData::Proposal,
            content: "# Proposal".to_string(),
            file_changes: vec![],
        };
        reduce(&mut state, Action::StartReview {
            workflow_node_id: "node-1".to_string(),
            content,
            policy: crate::actions::ReviewPolicyData::AlwaysReview,
        });

        let session_id = active_worktree(&state).tasks.review_gate.active_session_id.clone().unwrap();
        assert!(active_worktree(&state).tasks.review_gate.sessions.contains_key(&session_id));

        // Add Comment
        reduce(&mut state, Action::AddReviewComment {
            session_id: session_id.clone(),
            target: crate::actions::CommentTargetData::Document,
            content: "LGTM".to_string(),
        });
        assert_eq!(active_worktree(&state).tasks.review_gate.sessions[&session_id].comments.len(), 1);

        // Resolve Comment
        let comment_id = active_worktree(&state).tasks.review_gate.sessions[&session_id].comments[0].id.clone();
        reduce(&mut state, Action::ResolveReviewComment { session_id: session_id.clone(), comment_id });
        assert!(active_worktree(&state).tasks.review_gate.sessions[&session_id].comments[0].resolved);

        // Submit Feedback
        reduce(&mut state, Action::SubmitReviewFeedback { session_id: session_id.clone() });
        assert_eq!(active_worktree(&state).tasks.review_gate.sessions[&session_id].status, crate::app_state::ReviewStatus::Iterating);

        // Approve
        reduce(&mut state, Action::ApproveReview { session_id: session_id.clone() });
        assert_eq!(active_worktree(&state).tasks.review_gate.sessions[&session_id].status, crate::app_state::ReviewStatus::Approved);
    }

    // ========================================================================
    // Constitution Tests
    // ========================================================================
    #[test]
    fn test_constitution_actions() {
        let mut state = state_with_project();

        reduce(&mut state, Action::StartConstitutionWorkflow);
        assert!(active_worktree(&state).tasks.constitution_workflow.is_some());

        reduce(&mut state, Action::AnswerConstitutionQuestion { answer: "Rust".to_string() });
        let workflow = active_worktree(&state).tasks.constitution_workflow.as_ref().unwrap();
        assert_eq!(workflow.current_question, 1);
        assert_eq!(workflow.answers.get("tech_stack").unwrap(), "Rust");

        reduce(&mut state, Action::GenerateConstitution);
        assert_eq!(active_worktree(&state).tasks.constitution_workflow.as_ref().unwrap().status, crate::app_state::WorkflowStatus::Generating);

        reduce(&mut state, Action::AppendConstitutionOutput { content: "Rules".to_string() });
        assert_eq!(active_worktree(&state).tasks.constitution_workflow.as_ref().unwrap().output, "Rules");

        reduce(&mut state, Action::SaveConstitution);
        assert_eq!(active_worktree(&state).tasks.constitution_workflow.as_ref().unwrap().status, crate::app_state::WorkflowStatus::Complete);
    }

    // ========================================================================
    // Env Tests
    // ========================================================================
    #[test]
    fn test_env_actions() {
        let mut state = state_with_project();

        reduce(&mut state, Action::SetEnvTrackedPatterns { patterns: vec![".env".to_string()] });
        assert_eq!(state.active_project().unwrap().env_config.tracked_patterns, vec![".env"]);

        reduce(&mut state, Action::SetEnvAutoCopy { enabled: false });
        assert!(!state.active_project().unwrap().env_config.auto_copy_enabled);

        // Agent Rules
        reduce(&mut state, Action::CreateAgentProfile { name: "Test".to_string(), prompt: "You are a test".to_string() });
        assert_eq!(state.active_project().unwrap().agent_rules_config.profiles.len(), 1); // 1 custom (builtins not auto-populated in legacy config)

        let profile_id = state.active_project().unwrap().agent_rules_config.profiles.last().unwrap().id.clone();
        reduce(&mut state, Action::SelectAgentProfile { profile_id: Some(profile_id.clone()) });
        assert_eq!(state.active_project().unwrap().agent_rules_config.active_profile_id, Some(profile_id));
        assert!(state.active_project().unwrap().agent_rules_config.enabled);
    }

    // ========================================================================
    // Tasks Tests
    // ========================================================================
    #[test]
    fn test_tasks_actions() {
        let mut state = state_with_project();

        // Load commands
        let cmd = crate::actions::JustCommandData {
            name: "build".to_string(),
            description: None,
            recipe: "cargo build".to_string(),
        };
        reduce(&mut state, Action::SetJustfileCommands { commands: vec![cmd] });
        assert_eq!(active_worktree(&state).tasks.commands.len(), 1);

        // Run command
        reduce(&mut state, Action::RunJustCommand { name: "build".to_string(), cwd: ".".to_string() });
        assert_eq!(active_worktree(&state).tasks.active_command, Some("build".to_string()));
        assert_eq!(active_worktree(&state).tasks.task_statuses.get("build"), Some(&crate::app_state::TaskStatus::Running));
        assert!(active_worktree(&state).is_modified);

        // Append output
        reduce(&mut state, Action::AppendTaskOutput { line: "Compiling...".to_string() });
        assert_eq!(active_worktree(&state).tasks.output[0], "Compiling...");

        // Success (clears modified)
        reduce(&mut state, Action::SetTaskStatus { name: "build".to_string(), status: crate::actions::TaskStatusData::Success });
        assert_eq!(active_worktree(&state).tasks.task_statuses.get("build"), Some(&crate::app_state::TaskStatus::Success));
        assert!(!active_worktree(&state).is_modified);
    }
}