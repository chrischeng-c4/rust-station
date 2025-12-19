# Technical Debt Analysis

**Last updated**: 2025-12-18
**Analysis date**: 2025-12-18
**Codebase version**: commit 8a4217e

This document catalogs the current technical debt in the rustation project, with focus on the rstn TUI codebase.

---

## Executive Summary

The rstn TUI has accumulated significant technical debt due to rapid feature development without architectural refactoring. The primary issues are:

1. **God Classes**: App (3,404 lines) and WorktreeView (4,118 lines) violate Single Responsibility Principle
2. **State Explosion**: WorktreeView has 54+ mutable public fields
3. **Tight Coupling**: TUI layer directly contains domain logic
4. **Fragile Error Handling**: 308 unwrap/panic sites
5. **Testing Gaps**: ~40% test coverage, no view isolation tests

**Impact**: High cognitive load, difficult to maintain, slow feature development, frequent bugs

**Priority**: CRITICAL - Refactoring required before major new features

---

## Quantified Issues

### A. God Class: WorktreeView (4,118 lines)

**Location**: `crates/rstn/src/tui/views/worktree/view.rs`

**Lines of Code**: 4,118 lines (single file)

**Public Fields**: 54+ mutable fields

**Responsibilities**:
- Feature context management
- Spec/plan/tasks content caching
- Phase tracking
- UI state management (focus, scroll, selection)
- Command list management
- Auto-flow orchestration
- Logging and file tracking
- Command execution
- Input handling (multiple forms)
- Progress tracking
- Commit workflow (10+ fields)
- Specify workflow state
- Layout rect storage for mouse clicks
- Inline input management

**Field Breakdown**:
```rust
pub struct WorktreeView {
    // Feature context
    pub feature_info: Option<FeatureInfo>,
    pub worktree_type: WorktreeType,

    // Spec content (cached)
    pub spec_content: Option<String>,
    pub plan_content: Option<String>,
    pub tasks_content: Option<String>,

    // Phase tracking
    pub phases: Vec<(SpecPhase, PhaseStatus)>,
    pub current_phase: Option<SpecPhase>,

    // UI state
    pub focus: WorktreeFocus,
    pub phase_state: ListState,
    pub command_state: ListState,
    pub content_scroll: usize,
    pub content_type: ContentType,

    // Unified command list
    pub commands: Vec<Command>,
    pub pending_git_command: Option<GitCommand>,

    // Auto-flow state
    pub auto_flow: AutoFlowState,

    // Logging/output (using LogBuffer)
    pub log_buffer: LogBuffer,
    pub file_tracker: FileChangeTracker,
    pub output_scroll: usize,
    pub is_running: bool,
    pub running_phase: Option<String>,
    pub spinner_frame: usize,

    // Input handling
    pub pending_input_phase: Option<SpecPhase>,
    pub active_session_id: Option<String>,
    pub pending_follow_up: bool,

    // Progress tracking
    pub progress_step: Option<u32>,
    pub progress_total: Option<u32>,
    pub progress_message: Option<String>,

    // Commit workflow (10 fields)
    pub pending_commit_message: Option<String>,
    pub commit_warnings: Vec<SecurityWarning>,
    pub commit_groups: Option<Vec<CommitGroup>>,
    pub current_commit_index: usize,
    pub commit_message_input: String,
    pub commit_message_cursor: usize,
    pub commit_sensitive_files: Vec<String>,
    pub commit_validation_error: Option<String>,

    // Specify workflow
    pub specify_state: SpecifyState,

    // Layout rects (for mouse clicks)
    pub commands_pane_rect: Option<Rect>,
    pub content_pane_rect: Option<Rect>,
    pub output_pane_rect: Option<Rect>,

    // Inline input (Feature 076)
    pub inline_input: Option<InlineInput>,

    // ... and more
}
```

**Problem**: No way to understand which fields affect which behavior. State consistency issues when mutating 54 fields. Testing requires mocking all state.

**Target**: Refactor into 4-5 smaller modules, each <300 lines, with <15 fields per struct

---

### B. God Class: App (3,404 lines)

**Location**: `crates/rstn/src/tui/app.rs`

**Lines of Code**: 3,404 lines

**Public Methods**: 64 methods

**Responsibilities**:
- Main event loop
- View lifecycle management
- Event routing (keyboard, mouse, protocol, MCP)
- Command execution orchestration
- State synchronization across views
- Domain logic (spec generation, clarify, plan, tasks)
- Git operations
- Input/dialog management
- Widget coordination
- Copy/paste operations
- Session management

**Public Fields** (95 total):
```rust
pub struct App {
    pub running: bool,
    pub current_view: CurrentView,

    // All views owned directly
    pub worktree_view: WorktreeView,
    pub mcp_server_view: McpServerView,
    pub dashboard: Dashboard,
    pub settings_view: SettingsView,
    pub spec_view: SpecView,

    // Command execution
    pub command_runner: CommandRunner,

    // Status & events
    pub status_message: Option<String>,
    pub event_sender: Option<mpsc::Sender<Event>>,
    pub running_spec_phase: Option<String>,

    // UI state
    pub copy_visual_view: bool,
    pub protocol_parser: OutputParser,

    // Widgets
    pub text_input: Option<TextInput>,
    pub input_dialog: Option<InputDialog>,
    pub option_picker: Option<OptionPicker>,

    // Input modes
    pub input_mode: bool,
    pub picker_mode: bool,

    // Pending state
    pub pending_auto_continue: Option<(String, u64)>,
    pub pending_commit_groups: Option<Vec<CommitGroup>>,
    pub current_group_index: usize,

    // Session
    pub session_id: Option<String>,

    // Layout rects
    pub tab_bar_rect: Option<Rect>,
    pub shortcuts_bar_rect: Option<Rect>,

    // MCP state
    pub mcp_state: Arc<Mutex<McpState>>,
}
```

**Problem**: App orchestrates everything. No abstraction between app and views. Business logic mixed with UI logic. Impossible to change view behavior without touching App.

**Target**: Refactor to <500 lines, extract ViewCoordinator, move domain logic to domain layer

---

### C. Domain Logic in TUI Layer

**Issue**: Business logic embedded in `app.rs` and `worktree/view.rs`

**Examples**:
1. **Spec generation** (app.rs:886-1010): Claude CLI invocation, prompt building, output parsing
2. **Clarify workflow** (app.rs:1118-1143): Question generation, user interaction
3. **Plan generation** (app.rs:1144-1169): Design artifact creation
4. **Task generation** (app.rs:1170-1197): Task breakdown logic
5. **Commit workflow** (worktree/view.rs): Security scanning, grouping, message generation

**Problem**: Cannot test domain logic without TUI. Cannot reuse logic in CLI mode. Violates separation of concerns.

**Impact**:
- Domain changes require TUI changes
- Cannot unit test business rules
- Duplicated logic across views

**Target**: Extract to `crates/rstn/src/domain/` layer with clean interfaces

---

### D. Error Handling Issues

**Unwrap/Panic Count**: 308 sites across codebase

**Common Patterns**:
```rust
// Pattern 1: Unwrap on Arc<Mutex<T>>
let state = self.mcp_state.lock().unwrap();

// Pattern 2: Unwrap on channels
self.event_sender.as_ref().unwrap().send(event).unwrap();

// Pattern 3: Expect on file operations
fs::read_to_string(path).expect("Failed to read spec");

// Pattern 4: Unwrap on JSON parsing
serde_json::from_str(&content).unwrap()
```

**Problem**: Panics can crash TUI in production. Async context makes panics harder to debug. No graceful error recovery.

**Impact**: User-facing crashes, data loss, poor UX

**Target**: Reduce to <50 unwrap sites, use proper error handling (Result<T, E>), add error recovery paths

---

### E. State Management Complexity

**Issue**: Multiple layers of mutable shared state

**Patterns**:
1. `Arc<Mutex<McpState>>` shared between App, Axum server, and views
2. `mpsc::channel` for events between TUI and command runners
3. `oneshot::channel` for MCP tool responses
4. Public mutable fields on all view structs
5. Protocol parser state machine in App

**Concurrency Issues**:
- 35+ instances of `Arc<Mutex<T>>` or `Arc<RwLock<T>>`
- 35+ instances of `tokio::spawn`, `mpsc`, `oneshot`
- Mixed sync/async code paths
- Potential deadlocks when locking multiple mutexes

**Problem**: State updates can happen from multiple places. Race conditions possible. Hard to reason about "who modified what when".

**Target**: Immutable state where possible, clear ownership, message-passing over shared memory

---

### F. Testing Gaps

**Current Coverage**: ~40% (estimated)

**Missing Tests**:
- View isolation tests (WorktreeView, Dashboard, Settings)
- State transition tests (phases, auto-flow)
- Domain logic unit tests (spec generation, git operations)
- Integration tests (end-to-end workflows)
- Error path tests (rollback, recovery)

**Existing Tests**:
- Some unit tests in `rstn-core` (Feature 052: 29 tests)
- TUI e2e tests via external tui-tester agent (requires manual dispatch)
- No automated view tests

**Problem**: Cannot refactor confidently without breaking tests. Cannot verify bug fixes. Regressions frequent.

**Target**: 70%+ coverage for domain layer, 60%+ for TUI layer

---

### G. Tight Coupling Issues

**App ↔ Views Coupling**:
- App directly mutates all view state (no encapsulation)
- Views cannot be tested in isolation
- Cannot swap view implementations
- Cannot reuse views in different apps

**TUI ↔ Domain Coupling**:
- Domain logic embedded in TUI event handlers
- Cannot run domain logic without TUI
- Cannot test business rules independently

**View ↔ View Coupling**:
- WorktreeView directly accesses SpecView state
- Dashboard reads WorktreeView state
- No clear boundaries between views

**Problem**: Change in one area ripples through entire codebase. Cannot modify views without touching App. Cannot extract domain logic without TUI changes.

**Target**: Clean interfaces, dependency injection, trait-based abstraction

---

## Impact Assessment

### High Impact (Blocking)

**H1. Feature Development Velocity**
- Current: 2-4 weeks for medium feature
- With refactoring: 1-2 weeks for medium feature
- **Cause**: High cognitive load, fear of breaking things, testing gaps

**H2. Bug Fix Time**
- Current: 1-3 days for simple bug (+ regression risk)
- With refactoring: <1 day for simple bug
- **Cause**: Tight coupling, unclear ownership, state complexity

**H3. Onboarding Time**
- Current: 2-3 weeks for new contributor to be productive
- With refactoring: <1 week for new contributor
- **Cause**: God Classes, lack of documentation, unclear architecture

### Medium Impact (Slowing)

**M1. Code Review Time**
- Current: 1-2 hours per PR (high scrutiny required)
- With refactoring: 20-30 minutes per PR
- **Cause**: Large files, state complexity, untestable code

**M2. Refactoring Risk**
- Current: HIGH (308 panic sites, no tests)
- With refactoring: LOW (tests, smaller modules)
- **Cause**: Fragile error handling, testing gaps

**M3. Performance**
- Current: Acceptable but not optimized
- Potential: 2-3x improvement with proper architecture
- **Cause**: Unnecessary state copies, inefficient updates

---

## Prioritized Fix List

### Critical (Fix First)

1. **Extract Domain Logic** (Phase 3A, 4 weeks)
   - Create `domain/spec_workflow.rs`
   - Create `domain/command_executor.rs`
   - Create `domain/git_service.rs`
   - **Impact**: Enables testing, unblocks parallel work

2. **Refactor WorktreeView** (Phase 3B, 6 weeks)
   - Split into 4-5 modules (<300 lines each)
   - Reduce fields to <15 per struct
   - **Impact**: Reduces cognitive load, improves maintainability

3. **Refactor App** (Phase 3C, 3 weeks)
   - Extract ViewCoordinator
   - Reduce to <500 lines
   - **Impact**: Clean architecture, easier to reason about

### Important (Fix Next)

4. **Add Characterization Tests** (2 weeks)
   - Tests for current behavior before refactoring
   - **Impact**: Safe refactoring

5. **Fix Error Handling** (ongoing)
   - Replace unwrap with proper error handling
   - Target: <50 unwrap sites
   - **Impact**: Stability, better UX

6. **Add Unit Tests** (ongoing)
   - Domain layer: 80%+ coverage
   - View state: 70%+ coverage
   - **Impact**: Confidence in changes

### Nice to Have (Later)

7. **Performance Optimization** (4-6 weeks)
   - Profile hot paths
   - Optimize state updates
   - **Impact**: Faster TUI, better UX

8. **Documentation** (ongoing)
   - Inline rustdoc for all public APIs
   - Architecture diagrams
   - **Impact**: Easier onboarding

---

## Key Metrics (Baseline)

| Metric | Current (2025-12-18) | Target | Timeline |
|--------|----------------------|--------|----------|
| App.rs LOC | 3,404 | <500 | 3 weeks (Phase 3C) |
| WorktreeView LOC | 4,118 | <500 | 6 weeks (Phase 3B) |
| WorktreeState fields | 54+ | <15 | 6 weeks (Phase 3B) |
| Unwrap/panic sites | 308 | <50 | 6 months (ongoing) |
| Test coverage | ~40% | 70%+ | 6 months (ongoing) |
| Domain logic in TUI | 373 lines | 0 | 4 weeks (Phase 3A) |
| Public mutable fields | 149 | <50 | 6 months (ongoing) |

---

## Risks of Not Addressing

**Short-term** (1-3 months):
- Feature development slows down
- More bugs introduced
- Contributor frustration

**Mid-term** (3-6 months):
- New features become impossible to add
- Maintenance burden unsustainable
- Contributors leave project

**Long-term** (6+ months):
- Codebase becomes unmaintainable
- Requires full rewrite
- Project abandonment risk

---

## Success Criteria

**Phase 1 Success** (Domain Extraction, 4 weeks):
- [ ] All spec/plan/tasks logic in domain layer
- [ ] App.rs has no business logic
- [ ] Domain layer has 80%+ test coverage
- [ ] Cargo build/test pass

**Phase 2 Success** (WorktreeView Refactor, 6 weeks):
- [ ] WorktreeView main file <500 lines
- [ ] Each module <300 lines
- [ ] State struct <15 fields
- [ ] Unit tests for each module

**Phase 3 Success** (App Refactor, 3 weeks):
- [ ] App.rs <500 lines
- [ ] ViewCoordinator handles view lifecycle
- [ ] No business logic in App
- [ ] All tests passing

**Overall Success** (5-6 months):
- [ ] All metrics at target levels
- [ ] Feature development velocity 2x faster
- [ ] Bug fix time <1 day
- [ ] New contributor onboarding <1 week
- [ ] Zero "too complex to modify" areas

---

## Related Documents

- [System Overview](../01-architecture/overview.md) - High-level architecture
- [rstn TUI Architecture](../01-architecture/rstn-tui-architecture.md) - Current TUI design
- [Refactoring Roadmap](refactoring-roadmap.md) - Detailed refactoring plan
- [Plan File](/Users/chris.cheng/.claude/plans/purrfect-weaving-star.md) - Full retrospective plan

---

## Changelog

- 2025-12-18: Initial technical debt analysis created from exploration findings
