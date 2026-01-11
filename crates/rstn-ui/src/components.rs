//! Reusable UI components for rstn
//!
//! This module contains all shared UI components following Material Design 3 principles.

use crate::theme::{MaterialTheme, Themed};
use gpui::*;

/// Navigation item for the sidebar
#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    /// Unique identifier (e.g., "tasks", "dockers")
    pub id: String,
    /// Display label
    pub label: String,
    /// Icon element (SVG or icon component)
    pub icon: SharedString,
}

impl NavItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, icon: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: icon.into(),
        }
    }
}

/// Sidebar navigation component
///
/// Renders a vertical list of navigation items with pill-shaped selection indicator.
/// Based on the old MUI Sidebar component from desktop/src/renderer/src/components/layout/Sidebar.tsx
pub struct Sidebar {
    /// List of navigation items
    items: Vec<NavItem>,
    /// Currently selected item ID
    active_item: String,
    /// Theme configuration
    theme: MaterialTheme,
}

impl Sidebar {
    pub fn new(items: Vec<NavItem>, active_item: String, theme: MaterialTheme) -> Self {
        Self {
            items,
            active_item,
            theme,
        }
    }

    /// Render a single navigation item
    fn render_nav_item(&self, item: &NavItem, _cx: &WindowContext) -> Div {
        let is_active = item.id == self.active_item;

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(self.theme.spacing(0.5))
            .mb(self.theme.spacing(1.5))
            .pill(&self.theme, is_active)
            .child(
                // Icon placeholder (in real implementation, would render SVG)
                div()
                    .w(px(24.0))
                    .h(px(24.0))
                    .child(item.icon.clone()),
            )
            .child(
                // Label
                div()
                    .text_xs() // 12px (caption size)
                    .child(item.label.clone()),
            )
    }

    pub fn render(&self, cx: &WindowContext) -> Div {
        div()
            .flex()
            .flex_col()
            .w(px(200.0))
            .bg(self.theme.background.paper)
            .border_r_1()
            .border_color(self.theme.border.divider)
            .p(self.theme.spacing(1.0))
            .gap(self.theme.spacing(0.5))
            .children(
                self.items
                    .iter()
                    .map(|item| self.render_nav_item(item, cx)),
            )
    }
}

/// Shell layout component
///
/// Provides the main application shell with:
/// - Header (title bar)
/// - Sidebar (navigation)
/// - Content area (main content)
/// - Status bar (bottom status)
///
/// Based on the old Electron App.tsx layout
pub struct ShellLayout {
    /// Application title
    title: SharedString,
    /// Sidebar component
    sidebar: Sidebar,
    /// Theme configuration
    theme: MaterialTheme,
}

impl ShellLayout {
    pub fn new(title: impl Into<SharedString>, sidebar: Sidebar, theme: MaterialTheme) -> Self {
        Self {
            title: title.into(),
            sidebar,
            theme,
        }
    }

    /// Render header bar
    fn render_header(&self) -> Div {
        div()
            .flex()
            .items_center()
            .h(px(48.0))
            .px(self.theme.spacing(2.0))
            .bg(self.theme.background.paper)
            .border_b_1()
            .border_color(self.theme.border.divider)
            .child(self.title.clone())
    }

    /// Render status bar
    fn render_status_bar(&self) -> Div {
        div()
            .flex()
            .items_center()
            .h(px(24.0))
            .px(self.theme.spacing(2.0))
            .bg(self.theme.primary.main)
            .text_sm()
            .text_color(self.theme.primary.on_primary)
            .child("Ready")
    }

    pub fn render(&self, content: impl IntoElement, cx: &WindowContext) -> Div {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(self.theme.background.default)
            .text_color(self.theme.text.primary)
            .child(self.render_header())
            .child(
                div()
                    .flex()
                    .flex_1()
                    .child(self.sidebar.render(cx))
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .flex_col()
                            .p(self.theme.spacing(2.0))
                            .child(content),
                    ),
            )
            .child(self.render_status_bar())
    }
}

/// Page header component
///
/// Displays a page title with optional description and action buttons.
/// Based on desktop/src/renderer/src/components/shared/PageHeader.tsx
pub struct PageHeader {
    /// Page title
    title: SharedString,
    /// Optional description
    description: Option<SharedString>,
    /// Theme configuration
    theme: MaterialTheme,
}

impl PageHeader {
    pub fn new(
        title: impl Into<SharedString>,
        description: Option<impl Into<SharedString>>,
        theme: MaterialTheme,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.map(|d| d.into()),
            theme,
        }
    }

    pub fn render(&self, actions: Option<impl IntoElement>) -> Div {
        let mut container = div()
            .flex()
            .items_center()
            .justify_between()
            .mb(self.theme.spacing(2.0))
            .pb(self.theme.spacing(2.0))
            .border_b_1()
            .border_color(self.theme.border.divider);

        // Title section
        let mut title_section = div().flex().flex_col().gap(self.theme.spacing(0.5));

        title_section = title_section.child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .child(self.title.clone()),
        );

        if let Some(desc) = &self.description {
            title_section = title_section.child(
                div()
                    .text_sm()
                    .text_color(self.theme.text.secondary)
                    .child(desc.clone()),
            );
        }

        container = container.child(title_section);

        // Actions section
        if let Some(actions_el) = actions {
            container = container.child(div().flex().gap(self.theme.spacing(1.0)).child(actions_el));
        }

        container
    }
}

/// Empty state component
///
/// Displays a placeholder when no data is available.
/// Based on desktop/src/renderer/src/components/shared/EmptyState.tsx
pub struct EmptyState {
    /// Icon placeholder
    icon: SharedString,
    /// Title text
    title: SharedString,
    /// Description text
    description: SharedString,
    /// Theme configuration
    theme: MaterialTheme,
}

impl EmptyState {
    pub fn new(
        icon: impl Into<SharedString>,
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
        theme: MaterialTheme,
    ) -> Self {
        Self {
            icon: icon.into(),
            title: title.into(),
            description: description.into(),
            theme,
        }
    }

    pub fn render(&self, action: Option<impl IntoElement>) -> Div {
        let mut container = div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .gap(self.theme.spacing(2.0))
            .p(self.theme.spacing(4.0));

        // Icon
        container = container.child(
            div()
                .w(px(64.0))
                .h(px(64.0))
                .text_color(self.theme.text.disabled)
                .child(self.icon.clone()),
        );

        // Title
        container = container.child(
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .child(self.title.clone()),
        );

        // Description
        container = container.child(
            div()
                .text_sm()
                .text_color(self.theme.text.secondary)
                .child(self.description.clone()),
        );

        // Optional action button
        if let Some(action_el) = action {
            container = container.child(
                div()
                    .mt(self.theme.spacing(2.0))
                    .child(action_el),
            );
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nav_item_creation() {
        let item = NavItem::new("tasks", "Tasks", "📋");
        assert_eq!(item.id, "tasks");
        assert_eq!(item.label, "Tasks");
    }

    #[test]
    fn test_sidebar_creation() {
        let theme = MaterialTheme::dark();
        let items = vec![
            NavItem::new("tasks", "Tasks", "📋"),
            NavItem::new("dockers", "Dockers", "🐳"),
        ];
        let sidebar = Sidebar::new(items, "tasks".to_string(), theme);
        assert_eq!(sidebar.active_item, "tasks");
        assert_eq!(sidebar.items.len(), 2);
    }
}
