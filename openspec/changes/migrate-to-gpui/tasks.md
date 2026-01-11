## 1. Foundation & Architecture
- [x] 1.1 Create new Rust crate `crates/rstn/` in workspace (changed from apps/ to crates/ per Zed pattern)
- [x] 1.2 Configure `crates/rstn-core/` for direct Rust usage (moved from packages/core, removed napi-rs)
- [x] 1.3 Initialize GPUI application entry point (`main.rs`)
- [ ] 1.4 Implement global state integration (`AppState` -> GPUI `Model`)
- [ ] 1.5 **BLOCKED**: GPUI build requires Metal Toolchain (Xcode 26 beta issue)

## 2. Shell & Layout
- [ ] 2.1 Implement Window/Shell layout (Sidebar, Main Content, Status Bar)
- [ ] 2.2 Implement `Sidebar` view with feature navigation
- [ ] 2.3 Implement `Theme` system using GPUI styling

## 3. Core Features Migration
- [ ] 3.1 Port `Tasks` view (List, details, running output)
- [ ] 3.2 Port `Dockers` view (Service list, logs, actions)
- [ ] 3.3 Port `Settings` view
- [ ] 3.4 Port `Terminal` view (PTY integration + Text rendering)
- [ ] 3.5 Port `Explorer` view (File tree, git status)
- [ ] 3.6 Port `Chat` view (Message rendering, input)
- [ ] 3.7 Port `Workflows` view (Constitution, Change Management)

## 4. MCP & Advanced Features
- [ ] 4.1 Port `MCP` inspector view
- [ ] 4.2 Port `A2UI` dynamic renderer
- [ ] 4.3 Port `Context Engine` visualizations

## 5. Cleanup & Polish
- [ ] 5.1 Verify feature parity with Electron app
- [ ] 5.2 Remove `desktop/` (Electron) directory
- [ ] 5.3 Update `justfile` and CI/CD pipelines
