# Design: GPUI Migration

## Context
The current application uses Electron for the frontend and Rust (via `napi-rs`) for the backend. While functional, Electron imposes significant resource overhead. GPUI is a high-performance, GPU-accelerated UI framework for Rust (used by Zed Editor) that offers superior performance and a native feel.

## Goals / Non-Goals
- **Goals**:
    - Migrate full feature set to native Rust/GPUI.
    - Reduce memory footprint (< 100MB idle).
    - Instant startup (< 200ms).
    - Unify codebase (100% Rust).
- **Non-Goals**:
    - Web support (wasm) is not a priority for this migration.
    - Windows/Linux support is secondary (dependent on GPUI maturity).

## Decisions
- **Dependency**: Use `gpui` crate (git dependency from zed repo or crates.io if available).
- **Core Integration**: 
    - Reuse `packages/core` logic.
    - Wrap `rstn_core::app_state::AppState` in a GPUI `Model<AppState>`.
    - Expose `dispatch` method on the Model that calls `rstn_core::reducer::reduce`.
    - This ensures business logic remains identical.
- **Terminal Rendering**: 
    - Use `alacritty_terminal` crate for PTY state management and ANSI parsing.
    - Implement a custom GPUI View to render the terminal grid.
- **Styling**:
    - Implement a minimal implementation of Material Design 3 using GPUI's styling primitives (rounded corners, elevation, color tokens).
    - Port the existing theme colors.

## Risks / Trade-offs
- **Platform Support**: GPUI is primarily optimized for macOS (Metal). Linux (Vulkan) is experimental. Windows is WIP. This restricts the app to macOS initially.
- **Ecosystem**: GPUI has fewer ready-made components than React/MUI. Custom implementation of complex widgets (Tables, Tabs) is required.
- **Migration Effort**: Rewrite of all UI code is significant.

## Migration Plan
1. **Parallel Development**: Build the GPUI app alongside Electron.
2. **Core Refactor**: Ensure `rstn-core` is cleanly usable as a library.
3. **Feature-by-Feature Port**: Port one tab at a time.
4. **Switchover**: When feature parity is reached, deprecate Electron.
