//! rstn - A developer workbench powered by GPUI
//!
//! This is the main entry point for the rustation desktop application.

use gpui::*;

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

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .h(px(48.0))
                    .px(px(16.0))
                    .bg(rgb(0x2d2d2d))
                    .border_b_1()
                    .border_color(rgb(0x3d3d3d))
                    .child("rstn - Developer Workbench")
            )
            .child(
                // Main content area
                div()
                    .flex()
                    .flex_1()
                    .child(
                        // Sidebar
                        div()
                            .flex()
                            .flex_col()
                            .w(px(200.0))
                            .bg(rgb(0x252525))
                            .border_r_1()
                            .border_color(rgb(0x3d3d3d))
                            .p(px(8.0))
                            .gap(px(4.0))
                            .child(self.nav_item("Tasks", "tasks", app.active_tab == "tasks"))
                            .child(self.nav_item("Dockers", "dockers", app.active_tab == "dockers"))
                            .child(self.nav_item("Explorer", "explorer", app.active_tab == "explorer"))
                            .child(self.nav_item("Terminal", "terminal", app.active_tab == "terminal"))
                            .child(self.nav_item("Chat", "chat", app.active_tab == "chat"))
                            .child(self.nav_item("Settings", "settings", app.active_tab == "settings"))
                    )
                    .child(
                        // Content area
                        div()
                            .flex()
                            .flex_1()
                            .flex_col()
                            .p(px(16.0))
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child(format!("Welcome to rstn"))
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_color(rgb(0xaaaaaa))
                                    .child("GPUI-powered developer workbench")
                            )
                            .child(
                                div()
                                    .mt(px(16.0))
                                    .p(px(12.0))
                                    .bg(rgb(0x2d2d2d))
                                    .rounded(px(8.0))
                                    .child(format!("Active tab: {}", app.active_tab))
                            )
                    )
            )
            .child(
                // Status bar
                div()
                    .flex()
                    .items_center()
                    .h(px(24.0))
                    .px(px(16.0))
                    .bg(rgb(0x007acc))
                    .text_sm()
                    .child("Ready")
            )
    }
}

impl AppView {
    fn nav_item(&self, label: &str, _id: &str, active: bool) -> Div {
        let bg = if active { rgb(0x3d3d3d) } else { rgb(0x252525) };

        div()
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(4.0))
            .bg(bg)
            .hover(|s| s.bg(rgb(0x3d3d3d)))
            .cursor_pointer()
            .child(label)
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
