//! rstn - A developer workbench powered by GPUI
//!
//! This is the main entry point for the rustation desktop application.

use gpui::*;
use rstn_ui::{MaterialTheme, NavItem, PageHeader, ShellLayout, Sidebar};
// Note: Uncomment when Metal Toolchain is fixed
// use rstn_views::{TasksView, DockersView};

/// Application state wrapper for GPUI
struct RstnApp {
    /// Current active tab
    active_tab: &'static str,
}

impl RstnApp {
    fn new() -> Self {
        Self {
            active_tab: "tasks",
        }
    }
}

/// Main application view
struct AppView {
    app: Model<RstnApp>,
}

impl AppView {
    fn new(cx: &mut WindowContext) -> Self {
        let app = cx.new_model(|_| RstnApp::new());
        Self { app }
    }
}

impl Render for AppView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let app = self.app.read(cx);
        let theme = MaterialTheme::dark();

        // Create navigation items based on OLD_UI_ANALYSIS.md sidebar structure
        let nav_items = vec![
            NavItem::new("explorer", "Explorer", "📁"),
            NavItem::new("workflows", "Flows", "⚡"),
            NavItem::new("claude-code", "Claude", "🤖"),
            NavItem::new("tasks", "Tasks", "📋"),
            NavItem::new("mcp", "rstn", "🔌"),
            NavItem::new("chat", "Chat", "💬"),
            NavItem::new("a2ui", "A2UI", "🎨"),
            NavItem::new("terminal", "Term", "⌨️"),
        ];

        let sidebar = Sidebar::new(nav_items, app.active_tab.to_string(), theme.clone());
        let shell = ShellLayout::new("rstn - Developer Workbench", sidebar, theme.clone());

        // Render content based on active tab
        let content = self.render_content(app.active_tab, &theme, cx);

        shell.render(content, cx)
    }
}

impl AppView {
    /// Render content area based on active tab
    fn render_content(&self, active_tab: &str, theme: &MaterialTheme, cx: &WindowContext) -> Div {
        match active_tab {
            // TODO: Uncomment when Metal Toolchain is fixed
            // "tasks" => {
            //     let commands = vec![]; // Load from rstn-core::justfile
            //     TasksView::new(commands, theme.clone()).render(cx)
            // }
            // "dockers" => {
            //     let services = vec![]; // Load from rstn-core::docker
            //     DockersView::new(services, theme.clone()).render(cx)
            // }
            _ => {
                // Fallback: Welcome screen
                let page_header = PageHeader::new(
                    "Welcome to rstn",
                    Some("GPUI-powered developer workbench"),
                    theme.clone(),
                );

                div()
                    .flex()
                    .flex_col()
                    .child(page_header.render(None::<Div>))
                    .child(
                        div()
                            .mt(theme.spacing(2.0))
                            .p(theme.spacing(1.5))
                            .bg(theme.background.paper)
                            .rounded(theme.shape.border_radius_sm)
                            .child(format!("Active tab: {}", active_tab)),
                    )
            }
        }
    }
}


fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting rstn...");

    // Initialize GPUI application
    App::new().run(|cx: &mut AppContext| {
        // Create window options
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.0), px(100.0)),
                size: size(px(1200.0), px(800.0)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("rstn".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        // Open main window
        cx.open_window(options, |cx| {
            cx.new_view(|cx| AppView::new(cx))
        })
        .expect("Failed to open window");
    });
}
