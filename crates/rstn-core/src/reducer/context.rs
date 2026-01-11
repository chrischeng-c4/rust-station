use crate::actions::Action;
use crate::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::LoadContext | Action::InitializeContext | Action::RefreshContext | Action::CheckContextExists => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_loading = true;
                }
            }
        }

        Action::SetContext { files } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.files = files.into_iter().map(|f| f.into()).collect();
                    worktree.context.is_loading = false;
                    worktree.context.is_initialized = true;
                    worktree.context.last_refreshed = Some(chrono::Utc::now().to_rfc3339());
                }
            }
        }

        Action::SetContextLoading { is_loading } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_loading = is_loading;
                }
            }
        }

        Action::UpdateContextFile { name, content } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(file) = worktree.context.files.iter_mut().find(|f| f.name == name) {
                        file.content = content;
                        file.last_updated = chrono::Utc::now().to_rfc3339();
                    }
                }
            }
        }

        Action::SetContextInitialized { initialized } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_initialized = initialized;
                    worktree.context.is_loading = false;
                }
            }
        }

        Action::GenerateContext => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_generating = true;
                    worktree.context.generation_output.clear();
                    worktree.context.generation_error = None;
                }
            }
        }

        Action::AppendGenerateContextOutput { content } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.generation_output.push_str(&content);
                }
            }
        }

        Action::CompleteGenerateContext => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_generating = false;
                    worktree.context.is_initialized = true;
                    worktree.context.generation_output.clear();
                }
            }
        }

        Action::FailGenerateContext { error } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_generating = false;
                    worktree.context.generation_error = Some(error);
                }
            }
        }

        Action::SyncContext { .. } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_loading = true;
                }
            }
        }

        Action::AppendContextSyncOutput { .. } => {
            // Handled in monolithic reducer, but I'll add it here too
        }

        Action::CompleteContextSync { .. } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.context.is_syncing = false;
                    worktree.context.sync_output.clear();
                    worktree.context.sync_error = None;
                }
            }
        }

        Action::SetChangeArchived { .. } => {
            // Already handled in changes.rs? No, it's context-related transition
        }
        _ => {}
    }
}
