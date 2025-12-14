//! Version information for rstn
//!
//! Provides version, git hash, and build date captured at compile time.

/// Package version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Git commit hash (short form)
pub const GIT_HASH: &str = env!("GIT_HASH");

/// Build date (YYYY-MM-DD)
pub const BUILD_DATE: &str = env!("BUILD_DATE");

/// Full version string for --version output (compile-time concatenated)
/// Example: "0.1.0 (35c8b0d, 2024-12-13)"
pub const FULL_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("GIT_HASH"),
    ", ",
    env!("BUILD_DATE"),
    ")"
);

/// Full version string for --version output
/// Example: "0.1.0 (35c8b0d, 2024-12-13)"
pub fn full_version() -> &'static str {
    FULL_VERSION
}

/// Short version string for TUI title
/// Example: "v0.1.0 (35c8b0d)"
pub fn short_version() -> String {
    format!("v{} ({})", VERSION, GIT_HASH)
}
