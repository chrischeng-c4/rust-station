use crate::actions::Action;
use crate::app_state::{AppState, WorktreeState};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::SwitchWorktree { index } => {
            if let Some(project) = state.active_project_mut() {
                if index < project.worktrees.len() {
                    project.active_worktree_index = index;
                }
            }
        }

        Action::RefreshWorktrees => {
            // Async trigger
        }

        Action::SetWorktrees { worktrees } => {
            if let Some(project) = state.active_project_mut() {
                let new_worktrees: Vec<WorktreeState> = worktrees
                    .into_iter()
                    .map(|w| WorktreeState::new(w.path, w.branch, w.is_main))
                    .collect();

                project.worktrees = new_worktrees;
                if project.active_worktree_index >= project.worktrees.len() {
                    project.active_worktree_index = 0;
                }
            }
        }

        Action::AddWorktree { .. }
        | Action::AddWorktreeNewBranch { .. }
        | Action::RemoveWorktree { .. } => {
            // Async triggers
        }

        Action::FetchBranches => {
            if let Some(project) = state.active_project_mut() {
                project.is_loading_branches = true;
            }
        }

        Action::SetBranches { branches } => {
            if let Some(project) = state.active_project_mut() {
                project.available_branches = branches
                    .into_iter()
                    .map(|b| crate::app_state::BranchInfo {
                        name: b.name,
                        has_worktree: b.has_worktree,
                        is_current: b.is_current,
                    })
                    .collect();
                project.is_loading_branches = false;
            }
        }

        Action::SetBranchesLoading { is_loading } => {
            if let Some(project) = state.active_project_mut() {
                project.is_loading_branches = is_loading;
            }
        }
        _ => {}
    }
}
