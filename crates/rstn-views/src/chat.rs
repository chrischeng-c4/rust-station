//! Chat view - AI conversation interface
//!
//! Provides UI for chatting with Claude AI:
//! - Message history display
//! - Input field for user messages
//! - Model selection
//! - Conversation controls

use gpui::*;
use rstn_ui::MaterialTheme;

/// Chat message role
#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Chat message
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: String,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: impl Into<String>, timestamp: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            timestamp: timestamp.into(),
        }
    }

    pub fn render(&self, theme: &MaterialTheme, _window: &mut Window, _cx: &mut App) -> Div {
        let is_user = self.role == MessageRole::User;
        let bg_color = if is_user {
            theme.primary.container
        } else {
            theme.secondary.container
        };
        let text_color = if is_user {
            theme.primary.on_primary_container
        } else {
            theme.secondary.on_secondary_container
        };

        div()
            .flex()
            .flex_col()
            .mb(theme.spacing(1.5))
            .p(theme.spacing(1.5))
            .bg(bg_color)
            .rounded(theme.shape.border_radius)
            .child(
                // Header: role + timestamp
                div()
                    .flex()
                    .justify_between()
                    .mb(theme.spacing(0.5))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(text_color)
                            .child(match self.role {
                                MessageRole::User => "You",
                                MessageRole::Assistant => "Claude",
                                MessageRole::System => "System",
                            }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_color)
                            .child(self.timestamp.clone()),
                    ),
            )
            .child(
                // Message content
                div()
                    .text_sm()
                    .text_color(text_color)
                    .child(self.content.clone()),
            )
    }
}

/// Main Chat view
pub struct ChatView {
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
    pub theme: MaterialTheme,
}

impl ChatView {
    pub fn new(messages: Vec<ChatMessage>, theme: MaterialTheme) -> Self {
        Self {
            messages,
            input_text: String::new(),
            theme,
        }
    }

    pub fn render(&self, window: &mut Window, cx: &mut App) -> Div {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // Messages area (scrollable)
                div()
                    .flex_1()
                    .overflow_hidden()
                    .p(self.theme.spacing(2.0))
                    .children(
                        self.messages
                            .iter()
                            .map(|msg| msg.render(&self.theme, window, cx)),
                    ),
            )
            .child(
                // Input area (fixed bottom)
                div()
                    .flex()
                    .items_center()
                    .p(self.theme.spacing(2.0))
                    .bg(self.theme.background.paper)
                    .border_t_1()
                    .border_color(self.theme.border.divider)
                    .child(
                        // Input field placeholder
                        div()
                            .flex_1()
                            .px(self.theme.spacing(1.5))
                            .py(self.theme.spacing(1.0))
                            .bg(self.theme.background.default)
                            .border_1()
                            .border_color(self.theme.border.divider)
                            .rounded(self.theme.shape.border_radius_sm)
                            .text_sm()
                            .text_color(self.theme.text.secondary)
                            .child("Type a message..."),
                    )
                    .child(
                        // Send button placeholder
                        div()
                            .ml(self.theme.spacing(1.0))
                            .px(self.theme.spacing(2.0))
                            .py(self.theme.spacing(1.0))
                            .bg(self.theme.primary.main)
                            .rounded(self.theme.shape.border_radius_sm)
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(self.theme.primary.on_primary)
                            .child("Send"),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_creation() {
        let msg = ChatMessage::new(MessageRole::User, "Hello", "12:34");
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello");
        assert_eq!(msg.timestamp, "12:34");
    }

    #[test]
    fn test_chat_view_creation() {
        let theme = MaterialTheme::dark();
        let messages = vec![
            ChatMessage::new(MessageRole::User, "Hi", "12:00"),
            ChatMessage::new(MessageRole::Assistant, "Hello!", "12:01"),
        ];
        let view = ChatView::new(messages, theme);
        assert_eq!(view.messages.len(), 2);
    }
}
