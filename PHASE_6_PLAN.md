# Phase 6 Plan: Polish & Integration

## Overview

**Goal**: Complete data integration and feature implementation to achieve feature parity with the old Electron version.

**Timeline**: TBD
**Dependencies**: Phase 5 complete ✅
**Status**: Planning

---

## Current State

### ✅ Completed (Phase 1-5)
- UI Framework: GPUI integrated
- Theme System: Material Design 3 implemented
- Components: 8 feature views with placeholder data
- Application: Compiles and runs successfully
- Code Quality: All warnings resolved

### ⏸️ Pending (Phase 6)
- Backend data integration (0%)
- Interactive functionality (0%)
- State management system (0%)
- Testing infrastructure (0%)
- Documentation (0%)
- Performance optimization (0%)

---

## Phase 6 Objectives

### 1. Backend Data Integration (Priority: HIGH)

#### TasksView - Justfile Integration
**Current State**: Uses empty Vec<> placeholder
**Target**: Load actual commands from `rstn-core::justfile`

**Tasks**:
- [ ] Implement `JustfileCommand` → `TaskCard` data mapping
- [ ] Add command execution via `rstn-core::justfile::run_command()`
- [ ] Add real-time output streaming to LogPanel
- [ ] Implement command status updates (Idle/Running/Success/Failed)
- [ ] Add error handling for command failures

**Files to modify**:
- `crates/rstn-views/src/tasks.rs`
- `crates/rstn/src/main.rs` (remove placeholder Vec)

---

#### DockersView - Docker API Integration
**Current State**: Uses empty Vec<> placeholder
**Target**: Load actual services from `rstn-core::docker`

**Tasks**:
- [ ] Implement `DockerService` → `ServiceCard` data mapping
- [ ] Add real-time status polling (docker ps)
- [ ] Implement start/stop/restart actions
- [ ] Add container logs streaming
- [ ] Add resource usage metrics (CPU, memory)

**Files to modify**:
- `crates/rstn-views/src/dockers.rs`
- `crates/rstn/src/main.rs` (remove placeholder Vec)

---

#### ExplorerView - File Tree Integration
**Current State**: Uses empty TreeNode placeholder
**Target**: Load actual file tree from `rstn-core::worktree`

**Tasks**:
- [ ] Implement `Worktree` → `TreeNode` data mapping
- [ ] Add Git status integration (modified, untracked, staged)
- [ ] Implement directory expansion/collapse
- [ ] Add file selection and navigation
- [ ] Add file metadata display (size, modified date)
- [ ] Implement .gitignore filtering

**Files to modify**:
- `crates/rstn-views/src/explorer.rs`
- `crates/rstn/src/main.rs` (remove placeholder TreeNode)

---

#### TerminalView - PTY Integration
**Current State**: Uses empty sessions Vec<>
**Target**: Load actual terminal sessions from `rstn-core::terminal`

**Tasks**:
- [ ] Integrate `alacritty_terminal` crate
- [ ] Implement PTY session management
- [ ] Add terminal rendering with GPUI
- [ ] Add keyboard input handling
- [ ] Implement ANSI escape sequence parsing
- [ ] Add tab switching for multiple sessions
- [ ] Add scrollback buffer

**Files to modify**:
- `crates/rstn-views/src/terminal.rs`
- `crates/rstn/src/main.rs` (remove placeholder Vec)

**Research needed**:
- GPUI text rendering capabilities
- Terminal cell rendering strategy
- Performance optimization for large outputs

---

#### ChatView - Claude API Integration
**Current State**: Uses empty messages Vec<>
**Target**: Integrate with Claude API and persist chat history

**Tasks**:
- [ ] Implement Claude API client
- [ ] Add message streaming support
- [ ] Implement chat history persistence (JSON/SQLite)
- [ ] Add message formatting (code blocks, markdown)
- [ ] Add input validation and error handling
- [ ] Implement conversation context management

**Files to modify**:
- `crates/rstn-views/src/chat.rs`
- `crates/rstn-core/src/` (new `claude_client.rs`)
- `crates/rstn/src/main.rs` (remove placeholder Vec)

---

#### McpView - MCP Server Integration
**Current State**: Uses ServerStatus::Stopped placeholder
**Target**: Connect to real MCP server and fetch tools

**Tasks**:
- [ ] Implement MCP server health check
- [ ] Add tools list fetching from HTTP endpoint
- [ ] Add real-time status monitoring
- [ ] Implement tool parameter inspection
- [ ] Add server configuration management

**Files to modify**:
- `crates/rstn-views/src/mcp.rs`
- `crates/rstn/src/main.rs` (remove placeholder data)

---

#### SettingsView - Configuration Management
**Current State**: Static category display
**Target**: Implement read/write configuration persistence

**Tasks**:
- [ ] Implement config file management (TOML/JSON)
- [ ] Add settings read/write functionality
- [ ] Implement form input handling
- [ ] Add validation for settings values
- [ ] Add config hot-reload support

**Files to modify**:
- `crates/rstn-views/src/settings.rs`
- `crates/rstn-core/src/` (new `config.rs`)

---

### 2. Interactive Functionality (Priority: HIGH)

#### Event Handling System
**Tasks**:
- [ ] Implement button click handlers
- [ ] Add keyboard shortcut system
- [ ] Implement input field focus and editing
- [ ] Add scroll handling for lists
- [ ] Implement drag-and-drop (if needed)

**Files to modify**:
- All view files in `crates/rstn-views/src/`

---

#### Navigation System
**Tasks**:
- [ ] Implement tab switching logic
- [ ] Add browser-style history (back/forward)
- [ ] Add breadcrumb navigation
- [ ] Implement keyboard navigation (Ctrl+1-8 for tabs)

**Files to modify**:
- `crates/rstn/src/main.rs`
- `crates/rstn-ui/src/components.rs` (Sidebar)

---

### 3. State Management (Priority: MEDIUM)

#### Application State Architecture
**Current**: Each view is stateless, data passed from main.rs
**Target**: Redux-like state management with event loop

**Tasks**:
- [ ] Design `AppState` struct with all view states
- [ ] Implement `Action` enum for state updates
- [ ] Create `reducer` function for state transitions
- [ ] Add event channel for async updates
- [ ] Implement state persistence (save/restore)

**Files to create**:
- `crates/rstn/src/state.rs`
- `crates/rstn/src/actions.rs`
- `crates/rstn/src/reducer.rs`

---

### 4. Testing Infrastructure (Priority: MEDIUM)

#### Unit Tests
**Current**: Basic view creation tests exist
**Target**: Comprehensive test coverage

**Tasks**:
- [ ] Fix SIGBUS test execution error
- [ ] Add unit tests for all data transformations
- [ ] Add tests for event handlers
- [ ] Add tests for state transitions
- [ ] Achieve >80% code coverage

---

#### Integration Tests
**Tasks**:
- [ ] Test backend → view data flow
- [ ] Test user interactions end-to-end
- [ ] Test error handling scenarios
- [ ] Test performance under load

**Files to create**:
- `crates/rstn/tests/integration_tests.rs`

---

### 5. Performance Optimization (Priority: LOW)

**Tasks**:
- [ ] Profile GPU rendering performance
- [ ] Optimize state update frequency
- [ ] Add virtualization for large lists
- [ ] Implement lazy loading for file tree
- [ ] Benchmark startup time (<1 second target)
- [ ] Memory usage profiling

**Tools**:
- `cargo flamegraph`
- `cargo instruments` (macOS)
- `perf` (Linux)

---

### 6. Documentation (Priority: LOW)

#### User Documentation
**Tasks**:
- [ ] Update README.md with GPUI info
- [ ] Write user guide for each feature
- [ ] Create keyboard shortcuts reference
- [ ] Add screenshots/GIFs

**Files to create/update**:
- `README.md`
- `docs/user-guide.md`
- `docs/shortcuts.md`

---

#### Developer Documentation
**Tasks**:
- [ ] Document GPUI architecture decisions
- [ ] Write contribution guide
- [ ] Create component development guide
- [ ] Document state management patterns

**Files to create/update**:
- `dev-docs/architecture/gpui-overview.md`
- `dev-docs/workflow/contribution-guide.md`

---

## Implementation Strategy

### Stage 1: Critical Path (Weeks 1-2)
Focus on making the app functional with real data.

1. **TasksView + DockersView** (Week 1)
   - Highest user value (command runner + container manager)
   - Simplest data integration
   - Good for early validation

2. **ExplorerView** (Week 2)
   - Core navigation feature
   - Complex tree structure
   - Tests file system integration

### Stage 2: Interactive Features (Weeks 3-4)
Add interactivity and state management.

3. **Event System** (Week 3)
   - Button clicks, keyboard shortcuts
   - Input handling

4. **State Management** (Week 4)
   - Centralized AppState
   - Event-driven updates

### Stage 3: Advanced Features (Weeks 5-6)
Complete remaining views and polish.

5. **TerminalView + ChatView** (Week 5)
   - PTY integration
   - Claude API client

6. **McpView + SettingsView** (Week 6)
   - MCP server communication
   - Config persistence

### Stage 4: Polish (Week 7)
Final touches and optimization.

7. **Testing & Documentation**
   - Fix test infrastructure
   - Write comprehensive docs

8. **Performance Optimization**
   - Profile and optimize hot paths
   - GPU rendering tuning

---

## Success Criteria

### Must Have (MVP)
- [ ] All 8 views display real data from backend
- [ ] Application compiles and runs without errors
- [ ] Basic interactivity (button clicks, tab switching)
- [ ] TasksView can execute commands
- [ ] DockersView can start/stop containers

### Should Have
- [ ] State management system working
- [ ] All views fully interactive
- [ ] Terminal renders correctly
- [ ] Chat integrates with Claude API
- [ ] Unit tests pass

### Nice to Have
- [ ] Performance benchmarks met
- [ ] Comprehensive documentation
- [ ] Advanced features (MCP inspector, A2UI)
- [ ] Keyboard shortcuts for all actions

---

## Risks & Mitigations

### Risk 1: Terminal Rendering Complexity
**Likelihood**: High
**Impact**: High
**Mitigation**:
- Use `alacritty_terminal` as reference
- Start with basic ANSI support, iterate
- Consider embedding web-based terminal as fallback

### Risk 2: State Management Performance
**Likelihood**: Medium
**Impact**: Medium
**Mitigation**:
- Profile early and often
- Use GPUI's built-in state management patterns
- Optimize hot paths first

### Risk 3: Test Infrastructure Issues
**Likelihood**: High (SIGBUS error)
**Impact**: Low (tests not blocking development)
**Mitigation**:
- Focus on integration tests first
- Use manual testing as validation
- Investigate SIGBUS after MVP complete

---

## Next Actions

1. **Immediate (Today)**:
   - Review this plan with team
   - Start TasksView integration (highest value)

2. **This Week**:
   - Complete TasksView + DockersView integration
   - Add basic event handling
   - Verify real data displays correctly

3. **Next Week**:
   - Begin ExplorerView integration
   - Start designing state management system

---

## Notes

- All backend integration uses existing `rstn-core` code (no new backend work needed)
- Focus on vertical slices (one feature end-to-end) rather than horizontal layers
- Prioritize user-visible functionality over perfect architecture
- GPUI is still evolving - expect API changes, plan for refactoring

---

**Last Updated**: 2026-01-12
**Next Review**: TBD
