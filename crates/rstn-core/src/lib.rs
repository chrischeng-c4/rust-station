//! Core domain logic for rstn development toolkit
//!
//! This crate contains the pure business logic for rstn, separated from
//! UI concerns. It provides functionality for:
//! - Build and test operations
//! - Git worktree management
//! - MCP configuration
//! - Service orchestration
//! - Health checking
//! - Spec-driven development workflow
//!
//! Version: 0.1.0

pub mod build;
pub mod errors;
pub mod git;
pub mod mcp;
pub mod paths;
pub mod service;
pub mod test;

// Modules to be added during migration
// pub mod config;
// pub mod spec;
// pub mod doctor;
// pub mod health;
// pub mod process;

// Re-export main types
pub use build::CommandOutput;
pub use errors::{CoreError, Result};
pub use git::{
    CommitGroup, CommitResult, FeatureInfo, SecurityScanResult, SecurityWarning, SensitiveFile,
    Severity, WorktreeInfo,
};
pub use mcp::{McpConfig, McpRegistry, McpServer};
pub use service::{ServiceInfo, ServiceState};
pub use test::TestResults;
