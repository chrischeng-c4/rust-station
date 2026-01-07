use crate::actions::Action;
use crate::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::SetTheme { theme } => {
            state.global_settings.theme = theme;
        }

        Action::SetProjectPath { path } => {
            state.global_settings.default_project_path = path;
        }
        _ => {}
    }
}
