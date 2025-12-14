//! Library root for rstn

pub mod cli;
pub mod commands;
pub mod runners;
pub mod session;
pub mod settings;
pub mod tui;
pub mod ui;
pub mod version;

use thiserror::Error;

/// Error types for rstn
#[derive(Error, Debug)]
pub enum RscliError {
    #[error("Cargo command failed: {0}")]
    CargoFailed(String),

    #[error("Test execution failed: {0}")]
    TestFailed(String),

    #[error("Build failed: {0}")]
    BuildFailed(String),

    #[error("Repository root not found. Are you inside the rustation project?")]
    RepoNotFound,

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result type alias for rstn
pub type Result<T> = std::result::Result<T, RscliError>;
