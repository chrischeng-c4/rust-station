# Change: Migrate to GPUI

## Why
The current Electron-based frontend suffers from high memory usage, large bundle size, and sluggish performance typical of web-based desktop apps. Migrating to a native Rust UI using GPUI (Zed's UI framework) will provide a high-performance, native-feeling application with significantly lower resource footprint and tighter integration with the Rust backend. This unifies the tech stack into a single language (Rust), simplifying build tooling and type sharing.

## What Changes
- **Architecture**: Replace Electron (Node.js/Chromium) with a native Rust binary using GPUI.
- **Frontend**: Rewrite all React components as GPUI Views in Rust.
- **Backend**: Integrate `packages/core` directly into the application binary, removing `napi-rs` bridge overhead.
- **Terminal**: Replace `xterm.js` with a GPUI-native terminal renderer (or integrated `alacritty` logic).
- **Build**: Switch from `pnpm` + `vite` to standard `cargo build`.

## Impact
- **Affected Specs**: 
  - `shared-ui` (Architecture requirements)
  - `terminal-pty` (Rendering technology)
  - `file-explorer` (UI implementation details)
  - `docker-management`, `tasks-justfile`, `mcp-server`, `context-engine` (UI layer replacement)
- **Affected Code**: 
  - `desktop/` (Entirely replaced/deprecated)
  - `packages/core/` (Refactored to support direct Rust usage)
- **Breaking Changes**: **BREAKING** - The entire UI codebase is being rewritten. Existing React components will be discarded.
