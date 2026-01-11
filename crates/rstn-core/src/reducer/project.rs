use crate::actions::Action;
use crate::app_state::{AppState, ProjectState};
use crate::persistence;
use crate::worktree;
use crate::reducer::update_recent_projects;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::OpenProject { path } => {
            // Normalize to git root if inside a git repository
            let project_path = if std::path::Path::new(&path).exists() {
                worktree::get_git_root(&path).unwrap_or_else(|| path.clone())
            } else {
                path.clone()
            };

            // Check if this project (by git root) is already open
            if let Some(idx) = state.projects.iter().position(|p| p.path == project_path) {
                state.active_project_index = idx;

                // If the original path is a subdirectory, try to find matching worktree
                if path != project_path {
                    if let Some(project) = state.active_project_mut() {
                        let worktree_data: Vec<_> = project
                            .worktrees
                            .iter()
                            .map(|w| crate::actions::WorktreeData {
                                path: w.path.clone(),
                                branch: w.branch.clone(),
                                is_main: w.is_main,
                            })
                            .collect();

                        if let Some(wt_idx) = worktree::find_worktree_for_path(&path, &worktree_data) {
                            project.active_worktree_index = wt_idx;
                        }
                    }
                }
                return;
            }

            // Check if the path is inside any worktree of an existing project
            for (proj_idx, project) in state.projects.iter().enumerate() {
                let worktree_data: Vec<_> = project
                    .worktrees
                    .iter()
                    .map(|w| crate::actions::WorktreeData {
                        path: w.path.clone(),
                        branch: w.branch.clone(),
                        is_main: w.is_main,
                    })
                    .collect();

                if let Some(wt_idx) = worktree::find_worktree_for_path(&path, &worktree_data) {
                    state.active_project_index = proj_idx;
                    if let Some(proj) = state.active_project_mut() {
                        proj.active_worktree_index = wt_idx;
                    }
                    return;
                }
            }

            // Create new project with the normalized git root path
            let mut project = ProjectState::new(project_path.clone());

            // Load and apply persisted project state (only if path exists on disk)
            if std::path::Path::new(&project_path).exists() {
                if let Ok(Some(persisted)) = persistence::load_project(&project_path) {
                    persisted.apply_to(&mut project);
                }
            }

            state.projects.push(project);
            state.active_project_index = state.projects.len() - 1;

            // Update recent_projects (only for real paths)
            if std::path::Path::new(&project_path).exists() {
                update_recent_projects(state, &project_path);
            }
        }

        Action::CloseProject { index } => {
            if index < state.projects.len() {
                // Save project state before closing (only for real paths)
                let project = &state.projects[index];
                if std::path::Path::new(&project.path).exists() {
                    let _ = persistence::save_project(project);
                }

                state.projects.remove(index);

                // Adjust active index
                if state.projects.is_empty() {
                    state.active_project_index = 0;
                } else if state.active_project_index >= state.projects.len() {
                    state.active_project_index = state.projects.len() - 1;
                } else if index < state.active_project_index {
                    state.active_project_index -= 1;
                }
            }
        }

        Action::SwitchProject { index } => {
            if index < state.projects.len() {
                state.active_project_index = index;
            }
        }

        Action::SetFeatureTab { tab } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.active_tab = tab;
                }
                // Save project state when tab changes (only for real paths)
                if std::path::Path::new(&project.path).exists() {
                    let _ = persistence::save_project(project);
                }
            }
        }
        _ => {}
    }
}
