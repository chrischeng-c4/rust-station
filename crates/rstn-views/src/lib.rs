//! rstn-views - Feature views for rustation
//!
//! This crate contains all feature page implementations using GPUI.
//! Each view corresponds to a tab in the main application.

pub mod dockers;
pub mod explorer;
pub mod tasks;
pub mod terminal;

// Re-export key types
pub use dockers::DockersView;
pub use explorer::ExplorerView;
pub use tasks::TasksView;
pub use terminal::TerminalView;
