use crate::actions::Action;
use crate::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    if let Action::SetA2UIPayload { payload } = action {
        state.a2ui.payload = payload;
    }
}
