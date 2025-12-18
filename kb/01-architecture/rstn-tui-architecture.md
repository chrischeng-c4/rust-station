# rstn TUI Architecture

**Last updated**: 2025-12-18
**Status**: Current architecture (with known issues) + Proposed refactoring

This document describes the rstn terminal user interface architecture, including current design, known issues, and proposed refactoring.

---

## Table of Contents

1. [Current Architecture](#current-architecture)
2. [Known Issues](#known-issues)
3. [Target Architecture](#target-architecture)
4. [Migration Strategy](#migration-strategy)
5. [Design Patterns](#design-patterns)

---

## Current Architecture

### Overview

rstn uses ratatui (immediate mode TUI framework) with an event-driven architecture. The application consists of three main layers:

1. **TUI Layer**: User interface (App, Views, Widgets)
2. **Domain Layer**: Business logic (partially extracted)
3. **Infrastructure Layer**: MCP server, command runners

### Layer Diagram (Current)

```
┌─────────────────────────────────────────────────────────────────┐
│                        TUI Layer                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  App (3,404 lines)                                       │   │
│  │  - Event loop, view orchestration, domain logic mixed   │   │
│  └──────────────────────────────────────────────────────────┘   │
│          │                                                       │
│          ├─► WorktreeView (4,118 lines, 54 fields)             │
│          ├─► Dashboard                                           │
│          ├─► McpServerView                                       │
│          ├─► SettingsView                                        │
│          └─► Widgets (InputDialog, TextInput, OptionPicker)     │
├─────────────────────────────────────────────────────────────────┤
│                      Domain Layer (Partial)                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  domain/                                                 │   │
│  │  ├─ git/       - Git operations                          │   │
│  │  ├─ specify/   - Spec generation (Feature 052)          │   │
│  │  ├─ clarify/   - Clarification (Feature 053)            │   │
│  │  ├─ plan/      - Planning (Feature 054)                 │   │
│  │  └─ prompts/   - Prompt templates                       │   │
│  └──────────────────────────────────────────────────────────┘   │
│          ▲                                                       │
│          │ ❌ BUT: Much domain logic still in app.rs/views    │
├─────────────────────────────────────────────────────────────────┤
│                   Infrastructure Layer                           │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  MCP Server (Axum HTTP)                                  │   │
│  │  Command Runners (cargo, bash, Claude CLI)              │   │
│  │  Protocol Parser (text-based, being phased out)         │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Breakdown

### 1. App (Main Application)

**File**: `crates/rstn/src/tui/app.rs` (3,404 lines)

**Responsibilities** (too many):
- Main event loop (`run()`)
- View lifecycle management
- Event routing:
  - Keyboard events (`handle_key_event()`)
  - Mouse events (`handle_mouse_event()`)
  - Protocol messages (`handle_protocol_message()`)
  - MCP events (`poll_mcp_events()`)
- Domain logic orchestration:
  - Spec generation (`run_specify_phase()`)
  - Clarify workflow (`run_clarify_phase()`)
  - Plan generation (`run_plan_phase()`)
  - Task generation (`run_tasks_phase()`)
- Input/dialog management
- Widget coordination
- Session management

**Key Methods** (64 total):
```rust
impl App {
    // Lifecycle
    pub fn new(mcp_state: Arc<Mutex<McpState>>) -> Self
    pub async fn run(&mut self, terminal: &mut Terminal) -> AppResult<()>

    // Event handling
    fn handle_key_event(&mut self, key: KeyEvent) -> AppResult<()>
    fn handle_mouse_event(&mut self, mouse: MouseEvent)
    fn handle_protocol_message(&mut self, msg: ProtocolMessage)
    fn poll_mcp_events(&mut self)

    // Domain operations (SHOULD BE IN DOMAIN LAYER)
    async fn run_specify_phase(&mut self) -> AppResult<()>
    async fn run_clarify_phase(&mut self) -> AppResult<()>
    async fn run_plan_phase(&mut self) -> AppResult<()>
    async fn run_tasks_phase(&mut self) -> AppResult<()>
    async fn run_analysis_phase(&mut self) -> AppResult<()>
    async fn run_implement_phase(&mut self) -> AppResult<()>

    // View switching
    fn switch_to_worktree(&mut self)
    fn switch_to_dashboard(&mut self)
    fn switch_to_settings(&mut self)

    // Input handling
    fn submit_user_input(&mut self, value: String)
    fn handle_input_dialog_submit(&mut self, value: String)

    // ... and 40+ more methods
}
```

**Problem**: God Class violating Single Responsibility Principle

---

### 2. WorktreeView (Primary Development View)

**File**: `crates/rstn/src/tui/views/worktree/view.rs` (4,118 lines)

**Responsibilities** (too many):
- Feature context detection
- Spec/plan/tasks content loading and caching
- Phase status tracking
- Command list management
- UI rendering (3 panes: commands, content, output)
- Input handling
- Auto-flow orchestration
- Command execution
- Commit workflow (10+ fields)
- Specify workflow state
- Protocol parsing
- Mouse click handling

**State Fields** (54+ public mutable):
```rust
pub struct WorktreeView {
    // Feature context
    pub feature_info: Option<FeatureInfo>,
    pub worktree_type: WorktreeType,

    // Content cache
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

    // Commands
    pub commands: Vec<Command>,
    pub pending_git_command: Option<GitCommand>,

    // Auto-flow
    pub auto_flow: AutoFlowState,

    // Logging/output
    pub log_buffer: LogBuffer,
    pub file_tracker: FileChangeTracker,
    pub output_scroll: usize,
    pub is_running: bool,
    pub running_phase: Option<String>,
    pub spinner_frame: usize,

    // Input
    pub pending_input_phase: Option<SpecPhase>,
    pub active_session_id: Option<String>,
    pub pending_follow_up: bool,

    // Progress
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

    // Layout (for mouse clicks)
    pub commands_pane_rect: Option<Rect>,
    pub content_pane_rect: Option<Rect>,
    pub output_pane_rect: Option<Rect>,

    // Inline input (Feature 076)
    pub inline_input: Option<InlineInput>,

    // ... and more
}
```

**Problem**: State explosion with 54 mutable fields, impossible to reason about

---

### 3. Other Views

**Dashboard** (`crates/rstn/src/tui/views/dashboard.rs`, 588 lines):
- Overview of all features
- Recent activity
- Quick stats

**McpServerView** (`crates/rstn/src/tui/views/mcp_server.rs`):
- MCP server metrics
- Tool call history
- Connection status

**SettingsView** (`crates/rstn/src/tui/views/settings.rs`):
- Configuration options
- Preferences

**SpecView** (`crates/rstn/src/tui/views/spec.rs`, 1,169 lines):
- Alternative spec viewing mode
- Phase navigation

---

### 4. Widgets

Reusable UI components:

**InputDialog** (`crates/rstn/src/tui/widgets/input_dialog.rs`):
- Modal dialog for user input
- Centered popup with title and prompt

**TextInput** (`crates/rstn/src/tui/widgets/text_input.rs`, 717 lines):
- Multi-line text editing
- Cursor movement, selection, copy/paste

**OptionPicker** (`crates/rstn/src/tui/widgets/option_picker.rs`):
- Select from multiple options
- Keyboard navigation

---

### 5. Event Flow

#### Main Event Loop

```rust
// Simplified from app.rs
pub async fn run(&mut self, terminal: &mut Terminal) -> AppResult<()> {
    loop {
        // 1. Render
        terminal.draw(|f| self.ui(f))?;

        // 2. Poll events
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => self.handle_key_event(key)?,
                Event::Mouse(mouse) => self.handle_mouse_event(mouse),
                _ => {}
            }
        }

        // 3. Handle async events
        self.poll_mcp_events();
        self.handle_command_output();

        // 4. Check exit condition
        if !self.running { break; }
    }
    Ok(())
}
```

#### Event Types

**External Events**:
- Keyboard input (crossterm)
- Mouse input (crossterm)
- MCP tool calls (HTTP)
- Command output (tokio channels)

**Internal Events**:
```rust
pub enum Event {
    McpStatus { status: String, prompt: String, message: String },
    TaskCompleted { task_id: String },
    InputRequired { phase: String },
    OutputLine { category: LogCategory, message: String },
    CommandFinished { exit_code: i32 },
    FileChanged { path: PathBuf },
    ProtocolMessage(ProtocolMessage),
    // ... 8+ variants
}
```

---

## Known Issues

### 1. God Classes

**App (3,404 lines)**:
- Handles everything: events, views, domain logic
- No abstraction between layers
- Cannot test in isolation

**WorktreeView (4,118 lines)**:
- 54 mutable fields
- Multiple responsibilities in one struct
- High cognitive load

**Impact**: Slow development, frequent bugs, hard to onboard

---

### 2. Tight Coupling

**App ↔ Views**:
```rust
// App directly mutates view state
self.worktree_view.pending_commit_message = Some(msg);
self.worktree_view.commit_warnings = warnings;
self.worktree_view.is_running = true;
```

**TUI ↔ Domain**:
```rust
// Domain logic in app.rs (should be in domain layer)
async fn run_specify_phase(&mut self) -> AppResult<()> {
    let feature_desc = self.worktree_view.inline_input.take(); // TUI state
    let spec = generate_spec(feature_desc).await?; // Domain logic
    self.worktree_view.spec_content = Some(spec); // Back to TUI state
}
```

**Impact**: Cannot swap implementations, cannot test independently

---

### 3. State Management Complexity

**Multiple Layers of Shared State**:
1. `Arc<Mutex<McpState>>` - shared between App, Axum, views
2. `mpsc::channel` - events from runners to TUI
3. `oneshot::channel` - MCP tool responses
4. Public mutable fields everywhere

**Concurrency Issues**:
- Race conditions possible
- Deadlock risk when locking multiple mutexes
- Unclear ownership

**Impact**: Hard to reason about state, debugging concurrency issues

---

### 4. Testing Gaps

**No Unit Tests for**:
- View state transitions
- Event handling logic
- Domain operations in app.rs

**Only Integration Tests**:
- TUI e2e tests (via external tui-tester agent)
- Requires full TUI setup

**Impact**: Cannot refactor safely, regressions frequent

---

## Target Architecture

### Proposed Refactoring

```
┌─────────────────────────────────────────────────────────────────┐
│                        TUI Layer                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  App (<500 lines) - Event routing ONLY                  │   │
│  └──────────────────────────────────────────────────────────┘   │
│          │                                                       │
│          ▼                                                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  ViewCoordinator - View lifecycle management            │   │
│  └──────────────────────────────────────────────────────────┘   │
│          │                                                       │
│          ├─► WorktreeView (<500 lines, coordinator only)        │
│          │   ├─ WorktreeState (<200 lines, immutable)          │
│          │   ├─ WorktreeCommands (<300 lines)                   │
│          │   ├─ WorktreeRenderer (<300 lines)                   │
│          │   ├─ WorktreeInput (<200 lines)                      │
│          │   └─ WorktreeProtocol (<200 lines)                   │
│          │                                                       │
│          ├─► Dashboard                                           │
│          ├─► McpServerView                                       │
│          ├─► SettingsView                                        │
│          └─► Widgets                                             │
├─────────────────────────────────────────────────────────────────┤
│                      Domain Layer (Clean)                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  domain/                                                 │   │
│  │  ├─ spec_workflow    - ALL spec/clarify/plan logic      │   │
│  │  ├─ command_executor - Command execution service        │   │
│  │  ├─ git_service      - Git operations service           │   │
│  │  └─ (existing modules)                                   │   │
│  └──────────────────────────────────────────────────────────┘   │
│          ▲                                                       │
│          │ ✅ Clean interface, fully testable                  │
├─────────────────────────────────────────────────────────────────┤
│                   Infrastructure Layer                           │
│  (No changes - already clean)                                    │
└─────────────────────────────────────────────────────────────────┘
```

### Key Changes

**1. App Simplification** (3,404 → <500 lines):
```rust
pub struct App {
    running: bool,
    view_coordinator: ViewCoordinator,
    input_manager: InputManager,
    event_sender: mpsc::Sender<Event>,
    mcp_state: Arc<Mutex<McpState>>,
}

impl App {
    pub async fn run(&mut self, terminal: &mut Terminal) -> AppResult<()> {
        // Event loop only - delegate everything else
    }

    fn route_event(&mut self, event: Event) {
        self.view_coordinator.handle_event(event);
    }
}
```

**2. ViewCoordinator (New)**:
```rust
pub struct ViewCoordinator {
    views: HashMap<ViewType, Box<dyn View>>,
    current_view: ViewType,
    domain_services: DomainServices,
}

impl ViewCoordinator {
    pub fn switch_view(&mut self, view: ViewType) { /* ... */ }
    pub fn current_view_mut(&mut self) -> &mut dyn View { /* ... */ }
    pub fn handle_event(&mut self, event: Event) { /* ... */ }
    pub fn handle_view_action(&mut self, action: ViewAction) { /* ... */ }
}
```

**3. WorktreeView Refactoring** (4,118 → <500 lines):
```rust
// Main coordinator
pub struct WorktreeView {
    state: WorktreeState,
    commands: WorktreeCommands,
    layout: WorktreeLayout,
}

// Immutable state (15 fields max)
pub struct WorktreeState {
    feature_info: Option<FeatureInfo>,
    phases: Vec<(SpecPhase, PhaseStatus)>,
    content: ContentCache,
    focus: WorktreeFocus,
    // ... <15 total fields
}

// Command handling
pub struct WorktreeCommands {
    available_commands: Vec<Command>,
    selected_index: usize,
    executor: Arc<CommandExecutor>, // from domain layer
}

// Layout state
pub struct WorktreeLayout {
    commands_rect: Option<Rect>,
    content_rect: Option<Rect>,
    output_rect: Option<Rect>,
}
```

**4. Domain Layer Extraction**:
```rust
// domain/spec_workflow.rs
pub struct SpecWorkflow {
    claude_cli: ClaudeCliRunner,
    prompt_manager: PromptManager,
}

impl SpecWorkflow {
    pub async fn generate_spec(&self, desc: String) -> Result<String, SpecError> {
        // All spec generation logic here (not in app.rs)
    }

    pub async fn generate_plan(&self, spec_path: PathBuf) -> Result<String, SpecError> {
        // All plan generation logic here
    }
}
```

---

## Migration Strategy

### Phase 1: Domain Extraction (4 weeks)

**Goal**: Move all domain logic out of app.rs and views

**Steps**:
1. Create `domain/spec_workflow.rs` - Move spec/clarify/plan/tasks logic (373 lines from app.rs)
2. Create `domain/command_executor.rs` - Move command execution logic
3. Create `domain/git_service.rs` - Move git commit/review logic
4. Update App to delegate to services (not inline logic)

**Validation**:
- [ ] Cargo build passes
- [ ] All tests pass
- [ ] No domain logic in app.rs (grep check)
- [ ] Domain services have 80%+ test coverage

---

### Phase 2: WorktreeView Refactoring (6 weeks)

**Goal**: Break down 4,118-line God Class into modules

**Steps**:
1. **Extract state** (2 weeks):
   - Create `worktree/state.rs` with immutable WorktreeState
   - Create `worktree/commands.rs` with command handling
   - Create `worktree/layout.rs` with layout state
   - Update view.rs to use these types

2. **Split rendering** (2 weeks):
   - Create `worktree/render.rs` for UI rendering logic
   - Move all `render_*()` functions from view.rs
   - Keep view.rs as thin coordinator

3. **Extract input handling** (1 week):
   - Create `worktree/input.rs` for key/mouse input
   - Move input handlers from view.rs

4. **Clean up protocol** (1 week):
   - Create `worktree/protocol.rs` for protocol parsing
   - Remove legacy text-based parsing

**Validation**:
- [ ] view.rs <500 lines
- [ ] Each module <300 lines
- [ ] WorktreeState <15 fields
- [ ] Unit tests for each module

---

### Phase 3: App Refactoring (3 weeks)

**Goal**: Reduce App from 3,404 → <500 lines

**Steps**:
1. **Extract ViewCoordinator** (1.5 weeks):
   - Create `view_coordinator.rs`
   - Move view lifecycle logic from App
   - Update App to delegate to coordinator

2. **Simplify event routing** (1 week):
   - Remove direct view mutations from App
   - Use message passing instead
   - Clean up event handlers

3. **Extract input management** (0.5 week):
   - Create `input_manager.rs` for widgets
   - Move input/dialog logic from App

**Validation**:
- [ ] App.rs <500 lines
- [ ] No business logic in App
- [ ] All tests passing
- [ ] ViewCoordinator tested

---

## Design Patterns

### 1. Message Passing Over Shared State

**Current (problematic)**:
```rust
// App directly mutates view
self.worktree_view.is_running = true;
self.worktree_view.progress_step = Some(1);
```

**Target (clean)**:
```rust
// App sends messages
self.event_sender.send(Event::CommandStarted { phase: "specify" })?;
self.event_sender.send(Event::ProgressUpdate { step: 1, total: 10 })?;

// View handles messages
match event {
    Event::CommandStarted { phase } => {
        self.state = self.state.with_running_phase(phase);
    }
    Event::ProgressUpdate { step, total } => {
        self.state = self.state.with_progress(step, total);
    }
}
```

---

### 2. Immutable State Updates

**Current (problematic)**:
```rust
// Direct mutation
self.state.spec_content = Some(content);
self.state.is_running = true;
self.state.progress_step = Some(1);
```

**Target (clean)**:
```rust
// Builder pattern for immutability
self.state = self.state
    .with_spec_content(content)
    .with_running(true)
    .with_progress(1, 10);
```

---

### 3. Dependency Injection

**Current (problematic)**:
```rust
// Hardcoded dependencies
impl App {
    async fn run_specify_phase(&mut self) {
        let output = Command::new("claude") // Hardcoded
            .args(["-p", "Generate spec"])
            .output()
            .await?;
    }
}
```

**Target (clean)**:
```rust
// Injected dependencies
impl App {
    pub fn new(domain_services: DomainServices) -> Self { /* ... */ }
}

impl ViewCoordinator {
    async fn run_specify_phase(&mut self) {
        let spec = self.domain_services
            .spec_workflow
            .generate_spec(description)
            .await?;
    }
}
```

---

### 4. Trait-Based Abstraction

**Target**:
```rust
pub trait View {
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: Event) -> ViewAction;
    fn is_focused(&self) -> bool;
}

pub enum ViewAction {
    None,
    SwitchView(ViewType),
    Execute(Command),
    ShowDialog(String),
}
```

---

## Testing Strategy

### Unit Tests

**Domain Layer** (new):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_spec() {
        let workflow = SpecWorkflow::new_mock();
        let spec = workflow.generate_spec("Test feature".to_string()).await.unwrap();
        assert!(spec.contains("# Spec:"));
    }
}
```

**View State** (new):
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_state_with_running() {
        let state = WorktreeState::default();
        let new_state = state.with_running(true);
        assert!(new_state.is_running);
        assert!(!state.is_running); // Original unchanged
    }
}
```

### Integration Tests

**View Isolation**:
```rust
#[test]
fn test_worktree_view_command_selection() {
    let mut view = WorktreeView::new_for_test();
    view.handle_event(Event::Key(KeyCode::Down));
    assert_eq!(view.selected_command_index(), 1);
}
```

### TUI E2E Tests

**Via tui-tester agent** (existing):
- Mouse click handling
- Keyboard navigation
- Widget interactions

---

## Performance Considerations

**Current Performance**:
- Event loop: 60 Hz (16ms poll interval)
- Render time: <5ms (typical)
- Memory: ~50MB (with active command)

**Expected After Refactoring**:
- Event loop: Same (60 Hz)
- Render time: <3ms (less state to process)
- Memory: ~40MB (less state copying)

**Optimization Opportunities**:
- Reduce unnecessary state copies
- Cache rendered widgets
- Optimize protocol parsing
- Use Arc<str> instead of String for immutable text

---

## Related Documents

- [System Overview](overview.md) - High-level architecture
- [Technical Debt](../03-complexity-analysis/technical-debt.md) - Detailed issues
- [Plan File](/Users/chris.cheng/.claude/plans/purrfect-weaving-star.md) - Full refactoring plan

---

## Changelog

- 2025-12-18: Initial architecture documentation created
