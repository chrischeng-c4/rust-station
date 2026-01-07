use crate::actions::Action;
use crate::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::SpawnTerminal { cols, rows } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.terminal.cols = cols;
                    worktree.terminal.rows = rows;
                }
            }
        }

        Action::SetTerminalSession { session_id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.terminal.session_id = session_id;
                }
            }
        }

        Action::SetTerminalSize { cols, rows } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.terminal.cols = cols;
                    worktree.terminal.rows = rows;
                }
            }
        }
        _ => {}
    }
}
