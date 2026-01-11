use crate::actions::Action;
use crate::app_state::{AppState, TaskStatus};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::LoadJustfileCommands | Action::RefreshJustfile => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.is_loading = true;
                    worktree.tasks.error = None;
                }
            }
        }

        Action::SetJustfileCommands { commands } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.commands = commands.into_iter().map(|c| c.into()).collect();
                    worktree.tasks.is_loading = false;
                }
            }
        }

        Action::RunJustCommand { name, .. } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.active_command = Some(name.clone());
                    worktree.tasks.task_statuses.insert(name, TaskStatus::Running);
                    worktree.tasks.output.clear();
                    worktree.is_modified = true;
                }
            }
        }

        Action::SetTaskStatus { name, status } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.task_statuses.insert(name, status.into());
                    if matches!(status, crate::actions::TaskStatusData::Success | crate::actions::TaskStatusData::Error) {
                        worktree.is_modified = false;
                    }
                }
            }
        }

        Action::SetActiveCommand { name } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.active_command = name;
                }
            }
        }

        Action::AppendTaskOutput { line } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.output.push(line);
                }
            }
        }

        Action::ClearTaskOutput => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.output.clear();
                }
            }
        }

        Action::SetTasksLoading { is_loading } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.is_loading = is_loading;
                }
            }
        }

        Action::SetTasksError { error } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.error = error;
                    worktree.tasks.is_loading = false;
                }
            }
        }
        _ => {}
    }
}
