use crate::actions::Action;
use crate::app_state::{AppState, Notification};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::AddNotification {
            message,
            notification_type,
        } => {
            state.notifications.push(Notification::new(
                message,
                notification_type.into(),
            ));
        }

        Action::DismissNotification { id } => {
            state.notifications.retain(|n| n.id != id);
        }

        Action::MarkNotificationRead { id } => {
            if let Some(notification) = state.notifications.iter_mut().find(|n| n.id == id) {
                notification.read = true;
            }
        }

        Action::MarkAllNotificationsRead => {
            for notification in &mut state.notifications {
                notification.read = true;
            }
        }

        Action::ClearNotifications => {
            state.notifications.clear();
        }
        _ => {}
    }
}
