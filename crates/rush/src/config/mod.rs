//! Configuration management module
//!
//! Provides:
//! - Configuration loading from TOML
//! - Default configuration values
//! - Zero-config operation

pub mod defaults;

pub use defaults::{Config, Theme};
