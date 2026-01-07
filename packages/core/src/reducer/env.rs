use crate::actions::Action;
use crate::app_state::{AppState, EnvCopyResult};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::SetEnvCopyResult { result } => {
            if let Some(project) = state.active_project_mut() {
                project.env_config.last_copy_result = Some(EnvCopyResult {
                    copied_files: result.copied_files,
                    failed_files: result.failed_files,
                    timestamp: result.timestamp,
                });
            }
        }

        Action::SetEnvTrackedPatterns { patterns } => {
            if let Some(project) = state.active_project_mut() {
                project.env_config.tracked_patterns = patterns;
            }
        }

        Action::SetEnvAutoCopy { enabled } => {
            if let Some(project) = state.active_project_mut() {
                project.env_config.auto_copy_enabled = enabled;
            }
        }

        Action::SetEnvSourceWorktree { worktree_path } => {
            if let Some(project) = state.active_project_mut() {
                project.env_config.source_worktree = worktree_path;
            }
        }

        Action::SetAgentRulesEnabled { enabled } => {
            if let Some(project) = state.active_project_mut() {
                project.agent_rules_config.enabled = enabled;
                if !enabled {
                    project.agent_rules_config.active_profile_id = None;
                }
            }
        }

        Action::SetAgentRulesPrompt { prompt } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(profile) = project.agent_rules_config.profiles.iter_mut().find(|p| !p.is_builtin) {
                    profile.prompt = prompt;
                    profile.updated_at = chrono::Utc::now().to_rfc3339();
                } else {
                    let now = chrono::Utc::now().to_rfc3339();
                    let new_profile = crate::app_state::AgentProfile {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: "Custom".to_string(),
                        prompt,
                        is_builtin: false,
                        created_at: now.clone(),
                        updated_at: now,
                    };
                    project.agent_rules_config.profiles.push(new_profile.clone());
                    project.agent_rules_config.active_profile_id = Some(new_profile.id);
                }
            }
        }

        Action::SetAgentRulesTempFile { path } => {
            if let Some(project) = state.active_project_mut() {
                project.agent_rules_config.temp_file_path = path;
            }
        }

        Action::CreateAgentProfile { name, prompt } => {
            if let Some(project) = state.active_project_mut() {
                let now = chrono::Utc::now().to_rfc3339();
                let profile = crate::app_state::AgentProfile {
                    id: uuid::Uuid::new_v4().to_string(),
                    name,
                    prompt,
                    is_builtin: false,
                    created_at: now.clone(),
                    updated_at: now,
                };
                project.agent_rules_config.profiles.push(profile);
            }
        }

        Action::UpdateAgentProfile { id, name, prompt } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(profile) = project.agent_rules_config.profiles.iter_mut().find(|p| p.id == id && !p.is_builtin) {
                    profile.name = name;
                    profile.prompt = prompt;
                    profile.updated_at = chrono::Utc::now().to_rfc3339();
                }
            }
        }

        Action::DeleteAgentProfile { id } => {
            if let Some(project) = state.active_project_mut() {
                project.agent_rules_config.profiles.retain(|p| p.id != id || p.is_builtin);
                if project.agent_rules_config.active_profile_id.as_ref() == Some(&id) {
                    project.agent_rules_config.active_profile_id = None;
                }
            }
        }

        Action::SelectAgentProfile { profile_id } => {
            if let Some(project) = state.active_project_mut() {
                project.agent_rules_config.active_profile_id = profile_id.clone();
                if profile_id.is_some() {
                    project.agent_rules_config.enabled = true;
                }
            }
        }
        _ => {}
    }
}
