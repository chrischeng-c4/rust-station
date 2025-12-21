# rstn v2 UX Redesign: State Machine-Based Workflow Architecture

**Status**: 📋 Vision Document (未實作)
**Date**: 2025-12-20
**Complexity**: 🔴 High (5 weeks estimated)
**Priority**: 💭 Future Consideration

## 背景

在 2025-12-20 的使用者回饋中，發現了三個核心的 UX/DX 問題，促使我們重新思考 rstn TUI 的架構設計。

## 使用者反饋 (2025-12-20)

### 問題 1: 強制顯示 tabs 體驗差

**現象**:
- Content 區域固定顯示 "Spec | Plan | Tasks | Commit Review" tabs
- 即使不在相關工作流程也會顯示
- 使用者感覺 UI 雜亂，不知道該關注什麼

**原話**: "先完全移除 tab，我覺得我們現在工作流的規劃跟 UX/DX 還沒有調整好"

### 問題 2: Log column 重複

**現象**:
- 既然已有 log 檔案 (`~/.rstn/logs/`)，UI 中的 Log column 是多餘的
- Log column 佔用 40% 螢幕空間但價值有限
- 純文字輸出缺乏視覺層次

**原話**: "整合到 content 區塊，類似 Claude Code 的顯示方式，content 區塊要顯示的是開發者高度關注的內容，甚至我們可以調整樣式而非都是純文字，而 log 檔案是全部內容"

**關鍵洞察**:
- UI 應該顯示 "開發者高度關注的內容" (重點資訊)
- Log 檔案保存 "全部內容" (完整記錄)
- 需要樣式化顯示 (顏色、格式)，不是純文字

### 問題 3: 缺乏明確的工作流狀態機

**現象**:
- 目前 LLM 互動缺乏明確流程控制
- 不清楚何時需要 LLM、何時不需要
- 狀態轉換隱式，容易出錯

**原話**: "一個 command 是一項流程，你應該知道 n8n - 當這項流程走到需要跟 LLM 合作的節點才需要，所以我們要有的，是這個狀態機"

**關鍵洞察**:
- 每個 command 應該是一個完整的 workflow (像 n8n)
- Workflow 由多個 node 組成
- 只有特定 node 需要 LLM (不是所有操作都需要)
- 需要明確的狀態機架構

## 目前架構的問題

### 1. 散亂的狀態管理 (20+ State Fields)

**File**: `crates/rstn/src/tui/views/worktree/view.rs`

```rust
pub struct WorktreeView {
    // 36+ fields spread across subsystems:
    pub feature_info: Option<FeatureInfo>,
    pub content_type: ContentType,      // 8 variants - 無驗證
    pub focus: WorktreeFocus,           // 3 variants
    pub specify_state: SpecifyState,    // 12 fields
    pub auto_flow: AutoFlowState,       // 5 fields
    pub commit_groups: Option<Vec<CommitGroup>>,
    pub prompt_input: Option<TextInput>,
    // ... 20+ more fields
}
```

**問題**:
- 狀態不變量未強制執行 (例如 `ContentType::PromptInput` 假設 `prompt_input.is_some()`)
- 無驗證有效的狀態轉換
- LLM 可從多個入口點觸發，無防護措施

### 2. Ad-hoc UI Layout (3-column 固定 tabs)

**Current Layout** (`view.rs:3321-3328`):
```
┌─────────────┬─────────────────────┬─────────────────────┐
│  Commands   │      Content        │        Log          │
│    (20%)    │       (40%)         │       (40%)         │
│             │                     │                     │
│  - Workflow │  ┌─────────────┐   │  Command output     │
│  - SDD      │  │Spec│Plan│...│   │  (LogBuffer)        │
│  - GIT      │  └─────────────┘   │                     │
└─────────────┴─────────────────────┴─────────────────────┘
```

**問題**:
- Log column 重複 log 檔案
- Tabs 總是顯示，即使無關
- 無視覺化的工作流進度
- 純文字輸出 (無樣式，像 Claude Code)

### 3. 無明確狀態機

**Current Flow** (Explore agent analysis):
```
ContentType 狀態: Spec, Plan, Tasks, CommitReview, SpecifyInput,
                  SpecifyReview, PromptInput, PromptRunning

轉換發生在多個地方:
- handle_key() 可以改變 ContentType
- Event handlers 可以改變 is_running
- ViewAction handlers 可以改變多個欄位
- 無驗證有效的下一個狀態
```

**LLM 觸發點** (5 個地方):
1. `RunPromptClaude { prompt }` - 直接 Claude 互動
2. `RunSpecPhase { phase, command, options }` - SDD phases
3. `GenerateSpec { phase, description }` - 內容生成
4. `ExecuteTask { task_id, ... }` - 實作執行
5. `RunIntelligentCommit` - AI commit grouping

**問題**: 這些可從多個入口點觸發，無工作流上下文驗證。

## 提議的架構: State Machine-Based UX

### 設計原則 (n8n-style)

**核心洞察**: 一個 command 是一項 workflow，只在特定 node 需要 LLM

#### Workflow Example 1: Specify Phase (需要 LLM)

```
Commands → Select "Specify"
    ↓
InputNode: Ask user for feature description
    ↓
LLMNode: Generate spec.md (invoke Claude)  ← 只有這個 node 需要 LLM
    ↓
ReviewNode: Display spec, allow edit/approve
    ↓
SaveNode: Write to specs/{NNN}-{name}/spec.md
    ↓
Commands (workflow complete)
```

#### Workflow Example 2: Git Commit (不需要 LLM)

```
Commands → Select "Commit"
    ↓
ValidateNode: Check for staged changes
    ↓
InputNode: Ask commit message
    ↓
ExecuteNode: Run `git commit`  ← 純 Git 操作，無 LLM
    ↓
Commands (workflow complete)
```

**關鍵差異**:
- Specify workflow 有 LLMNode (需要 Claude)
- Commit workflow 無 LLMNode (純 Git 操作)
- 狀態機清楚知道何時需要 LLM

### New UI Layout (2-column, no tabs)

**Proposed Layout**:
```
┌─────────────┬───────────────────────────────────────────┐
│  Commands   │               Content                     │
│    (20%)    │               (80%)                       │
│             │                                           │
│  - Workflow │  ┌─────────────────────────────────────┐ │
│  - SDD      │  │ Dynamic content based on state:     │ │
│  - GIT      │  │                                     │ │
│             │  │ • Input prompts (styled TextInput)  │ │
│             │  │ • Streaming output (Claude Code)    │ │
│             │  │ • Review panels (spec/plan/tasks)   │ │
│             │  │ • Commit review (diff + message)    │ │
│             │  │ • Task execution progress           │ │
│             │  └─────────────────────────────────────┘ │
│             │                                           │
│  [Feature]  │  [Workflow Progress Bar]                 │
│  #123       │  ▓▓▓░░░░░░░ 3/10 steps                   │
└─────────────┴───────────────────────────────────────────┘
```

**Key Changes**:
1. ❌ 移除 Log column → 使用 log 檔案
2. ❌ 移除固定 tabs → 根據 workflow state 動態內容
3. ✅ 新增 workflow progress indicator
4. ✅ 樣式化輸出 (顏色、格式，非純文字)
5. ✅ Content 顯示 "開發者高度關注的內容" (使用者回饋)

**Future Extension** (目前範圍外):
- 右側可能顯示檔案內容 (使用者: "我們要移除 tab 右側可能可以擺檔案內容")

### State Machine Architecture

#### WorkflowState Enum

```rust
/// Workflow state machine - each command is a workflow
pub enum WorkflowState {
    /// Idle state - showing command list
    Commands {
        selected_index: usize,
    },

    /// Prompt Claude workflow (3 nodes)
    PromptInput {
        input_buffer: String,
    },
    PromptRunning {
        session_id: String,
        streaming_output: Vec<String>,
    },
    PromptComplete {
        output: String,
    },

    /// Specify workflow (4 nodes)
    SpecifyInput {
        description_buffer: String,
    },
    SpecifyGenerating {
        session_id: String,
    },
    SpecifyReview {
        generated_spec: String,
        feature_number: String,
        feature_name: String,
    },
    SpecifyEdit {
        spec_buffer: String,
        cursor: usize,
    },

    /// Commit workflow (NO LLM - pure Git)
    CommitInput {
        message_buffer: String,
    },
    CommitValidating {
        staged_files: Vec<String>,
    },
    CommitExecuting {
        commit_hash: Option<String>,
    },

    // ... Plan, Tasks, IntelligentCommit workflows
}
```

#### State Transition Validation

```rust
impl WorkflowState {
    /// Check if transition is valid
    pub fn can_transition_to(&self, next: &WorkflowState) -> bool {
        match (self, next) {
            // Commands can start any workflow
            (Commands { .. }, PromptInput { .. }) => true,
            (Commands { .. }, SpecifyInput { .. }) => true,
            (Commands { .. }, CommitInput { .. }) => true,

            // Prompt workflow transitions
            (PromptInput { .. }, PromptRunning { .. }) => true,
            (PromptRunning { .. }, PromptComplete { .. }) => true,
            (PromptComplete { .. }, Commands { .. }) => true,

            // Invalid: cannot skip PromptRunning
            (PromptInput { .. }, PromptComplete { .. }) => false,

            _ => false,
        }
    }

    /// Does this state need LLM?
    pub fn requires_llm(&self) -> bool {
        matches!(
            self,
            WorkflowState::PromptRunning { .. }
                | WorkflowState::SpecifyGenerating { .. }
                | WorkflowState::PlanGenerating { .. }
                | WorkflowState::TasksGenerating { .. }
                | WorkflowState::IntelligentCommitGrouping { .. }
        )
    }

    /// Get LLM command for this state
    pub fn llm_command(&self) -> Option<String> {
        match self {
            WorkflowState::PromptRunning { .. } => Some("claude prompt ...".to_string()),
            WorkflowState::SpecifyGenerating { .. } => Some("/speckit.specify".to_string()),
            WorkflowState::PlanGenerating { .. } => Some("/speckit.plan".to_string()),
            _ => None,
        }
    }
}
```

#### State Machine Controller

```rust
pub struct WorkflowStateMachine {
    current: WorkflowState,
}

impl WorkflowStateMachine {
    pub fn transition(&mut self, event: WorkflowEvent) -> Result<(), String> {
        let next_state = self.compute_next_state(&event)?;

        // Validate transition
        if !self.current.can_transition_to(&next_state) {
            return Err(format!(
                "Invalid transition from {:?} to {:?}",
                self.current, next_state
            ));
        }

        // If next state needs LLM, invoke it
        if next_state.requires_llm() {
            if let Some(cmd) = next_state.llm_command() {
                self.invoke_llm(&cmd)?;
            }
        }

        self.current = next_state;
        Ok(())
    }

    fn invoke_llm(&self, command: &str) -> Result<(), String> {
        // Single entry point for all LLM invocations
        // Ensures workflow context is preserved
        Ok(())
    }
}
```

### n8n-style Workflow Definitions

#### Prompt Workflow (with LLM)

```rust
pub struct PromptWorkflow;

impl Workflow for PromptWorkflow {
    fn nodes(&self) -> Vec<WorkflowNode> {
        vec![
            WorkflowNode::Input {
                name: "User Prompt",
                validation: |input| !input.is_empty(),
            },
            WorkflowNode::LLM {  // ← LLM node
                name: "Run Claude",
                command: |input| format!("claude prompt '{}'", input),
            },
            WorkflowNode::Display {
                name: "Show Output",
                style: DisplayStyle::Streaming,
            },
        ]
    }

    fn start_state(&self) -> WorkflowState {
        WorkflowState::PromptInput {
            input_buffer: String::new(),
        }
    }

    fn end_state(&self) -> WorkflowState {
        WorkflowState::Commands { selected_index: 0 }
    }
}
```

#### Git Commit Workflow (NO LLM)

```rust
pub struct GitCommitWorkflow;

impl Workflow for GitCommitWorkflow {
    fn nodes(&self) -> Vec<WorkflowNode> {
        vec![
            WorkflowNode::Validate {
                name: "Check Staged Files",
                check: |ctx| has_staged_changes(ctx),
            },
            WorkflowNode::Input {
                name: "Commit Message",
                validation: |input| !input.is_empty(),
            },
            WorkflowNode::Execute {  // ← No LLM node
                name: "Git Commit",
                command: |msg| format!("git commit -m '{}'", msg),
            },
        ]
    }

    // NO LLM node - pure Git workflow
}
```

## Implementation Plan (5 Phases)

### Phase 1: State Machine Core (Week 1)

**Objective**: Build state machine foundation without breaking existing UI

**New Files**:
1. `crates/rstn/src/tui/state_machine.rs` (300 lines)
   - `WorkflowState` enum
   - `WorkflowEvent` enum
   - `WorkflowStateMachine` struct
   - Validation functions

2. `crates/rstn/src/tui/workflows/mod.rs` (50 lines)
   - `Workflow` trait
   - `WorkflowNode` enum
   - Workflow registry

3. `crates/rstn/src/tui/workflows/prompt.rs` (100 lines)
   - PromptWorkflow implementation
   - 3 nodes: Input → LLM → Display

**Modified Files**:
- `crates/rstn/src/tui/views/worktree/view.rs`
  - Add `state_machine: WorkflowStateMachine` field
  - Keep `content_type` for backward compatibility

**Testing**:
- Unit tests: State transition validation
- Integration tests: Prompt workflow end-to-end
- Backward compatibility maintained

### Phase 2: Remove Log Column + Merge Content (Week 2)

**Objective**: Simplify layout, remove redundant Log column

**Changes**:
1. Layout change: 3-column → 2-column
   ```rust
   // OLD: Commands (20%) | Content (40%) | Log (40%)
   // NEW: Commands (20%) | Content (80%)
   ```

2. Merge output rendering into content area
3. Add styled output (Claude Code-style)

**New Component**:
- `crates/rstn/src/tui/widgets/styled_output.rs` (200 lines)
  - StyledOutput widget
  - Syntax highlighting
  - Streaming animation
  - Progress indicators

### Phase 3: Remove Tabs + Dynamic Content (Week 3)

**Objective**: Remove fixed tabs, render content based on workflow state

**Changes**:
1. Remove tab bar (lines 973-999 in `render_content`)
2. Dynamic content dispatch:
   ```rust
   match self.state_machine.current {
       WorkflowState::PromptInput { .. } => render_prompt_input(),
       WorkflowState::PromptRunning { .. } => render_streaming_output(),
       WorkflowState::SpecifyReview { .. } => render_spec_review(),
       // ...
   }
   ```

3. Migrate `content_type` → `state_machine.current`

### Phase 4: Workflow Progress + Visual Enhancements (Week 4)

**Objective**: Add workflow progress indicators and polish

**New Components**:
1. `crates/rstn/src/tui/widgets/workflow_progress.rs` (150 lines)
   - Progress bar for multi-step workflows
   - Step indicators (✓ Done, ▶ Current, ○ Pending)
   - Duration tracking

2. Enhanced styling:
   - Color-coded workflow states
   - Icons for node types (🎤 Input, 🤖 LLM, 📝 Review)
   - Animations for long-running operations

### Phase 5: Cleanup + Migration (Week 5)

**Objective**: Remove deprecated code, full migration to state machine

**Removals**:
- ContentType enum (replaced by WorkflowState)
- render_output() method (merged into render_content)
- Tab-related code

**Updates**:
- All event handlers use state machine transitions
- All ViewAction handlers dispatch through state machine
- Documentation updates

## Migration Strategy

### Backward Compatibility

During Phases 1-3, maintain dual mode:

```rust
pub struct WorktreeView {
    // New (state machine)
    state_machine: WorkflowStateMachine,

    // Old (deprecated but functional)
    content_type: ContentType,

    // Compatibility flag
    use_state_machine: bool,  // Default: false
}
```

**Experimental Setting**:
```
Settings → Experimental → Use State Machine: [ ] (default off)
```

After Phase 5, remove compatibility mode.

## Expected Outcomes

### User Experience Improvements

**Before** (Current):
```
1. Press 'p' → Input dialog
2. Type prompt → Submit
3. [Black box - no visibility]
4. Result in Log column (plain text)
5. Tabs always visible
```

**After** (State Machine):
```
1. Select "Prompt Claude" from Commands
2. Content shows styled input editor
3. Submit → Streaming output (Claude Code-style)
   - Real-time responses
   - Progress: "Running... (Turn 1/10)"
   - Syntax highlighting
4. Complete → Result with summary
   - Duration: 33s, Turns: 1/10, Cost: $0.05
5. No unnecessary tabs
```

### Developer Benefits

1. **Type-Safe Workflows**: Invalid transitions caught at compile time
2. **Single LLM Entry Point**: All Claude invocations through state machine
3. **Testable State Logic**: State transitions isolated from UI
4. **Clear Workflow Structure**: n8n-style node definitions
5. **Easier Debugging**: State machine state is serializable, loggable
6. **No Redundant UI**: Log column removed, focused content

### Technical Debt Reduction

**Before**:
- 20+ scattered state fields
- 8 ContentType variants with implicit transitions
- 5 LLM invocation points
- Ad-hoc validation

**After**:
- 1 state machine with explicit states
- Type-safe transitions with validation
- 1 LLM invocation point
- Declarative workflow definitions

## Files Summary

**New Files** (7 files, ~1000 LOC):
1. `crates/rstn/src/tui/state_machine.rs` (300 lines)
2. `crates/rstn/src/tui/workflows/mod.rs` (50 lines)
3. `crates/rstn/src/tui/workflows/prompt.rs` (100 lines)
4. `crates/rstn/src/tui/workflows/specify.rs` (150 lines)
5. `crates/rstn/src/tui/workflows/git_commit.rs` (80 lines)
6. `crates/rstn/src/tui/widgets/styled_output.rs` (200 lines)
7. `crates/rstn/src/tui/widgets/workflow_progress.rs` (150 lines)

**Modified Files** (5 files):
1. `crates/rstn/src/tui/views/worktree/view.rs` (major refactor)
2. `crates/rstn/src/tui/views/mod.rs`
3. `crates/rstn/src/tui/app.rs`
4. `crates/rstn/src/tui/event.rs`
5. `crates/rstn/src/settings.rs`

## Challenges & Risks

### Technical Challenges

1. **State Migration Complexity**
   - 20+ existing state fields need mapping to new state machine
   - Risk: Breaking existing workflows during migration
   - Mitigation: Dual mode with experimental flag

2. **UI Rendering Performance**
   - Dynamic content dispatch may have overhead
   - Risk: Lag during state transitions
   - Mitigation: Benchmark early, optimize hot paths

3. **Backward Compatibility**
   - Need to support old ContentType during migration
   - Risk: Code duplication, maintenance burden
   - Mitigation: Time-boxed compatibility (remove in Phase 5)

### Design Challenges

1. **Workflow Definition Complexity**
   - n8n-style nodes are powerful but may be overkill
   - Risk: Over-engineering simple workflows
   - Consideration: Start simple, add complexity only when needed

2. **State Serialization**
   - WorkflowState needs to be serializable for session management
   - Risk: Complex states (e.g., closures) can't be serialized
   - Mitigation: Keep state data-only, no function pointers

3. **Error Handling**
   - State machine errors need to be user-friendly
   - Risk: Cryptic error messages on invalid transitions
   - Mitigation: Rich error types with context

### UX Challenges

1. **Learning Curve**
   - Users familiar with tabs may be confused by dynamic content
   - Risk: User resistance to change
   - Mitigation: Gradual rollout with experimental flag, good documentation

2. **Progress Visibility**
   - Multi-step workflows need clear progress indicators
   - Risk: Users confused about workflow state
   - Mitigation: Prominent progress bar, step labels

## Why This Is Hard

### 1. Large Refactor

- 3000+ lines in `worktree/view.rs` need restructuring
- Multiple subsystems affected (UI, state, events, commands)
- High risk of regressions

### 2. Conceptual Shift

- From "tabs + panels" to "workflow-driven UI"
- From "scattered state" to "state machine"
- From "implicit transitions" to "explicit validation"

### 3. Preserving Functionality

- Existing workflows must continue working during migration
- Can't break user experience mid-implementation
- Need comprehensive testing at each phase

### 4. Unknown Unknowns

- Edge cases not covered in current analysis
- State machine corner cases discovered during implementation
- UI rendering issues only visible in production

## Recommendation

**Status**: 📋 Vision Document

**建議**:
1. **先記錄** - 這個設計值得保留，但不急於實作
2. **分階段評估** - 可以先做 Phase 2 (移除 Log column)，看效果
3. **漸進式改進** - 不一定要完整實作狀態機，可以先改善 UI
4. **收集更多回饋** - 先用目前版本一段時間，看是否真的需要這麼大的改動

**使用者原話**: "先寫在 kb 吧，這個專案並不好設計也不太好做，先記錄"

## References

- **User Feedback**: 2025-12-20 conversation
- **Current State Analysis**: Explore agent report (aca1764)
- **State Machine Pattern**: Rust state machine libraries (e.g., `sm` crate)
- **UI Inspiration**: Claude Code streaming display
- **Workflow Pattern**: n8n node-based workflows
- **Related Docs**:
  - `kb/02-architecture/state-first.md` - State-first architecture principles
  - `kb/04-development/testing-guide.md` - State testing approach
  - `kb/99-archive/v1-designs/worktree-view-redesign.md` - v1 redesign attempts

## Appendix: Explore Agent Report

**Agent ID**: aca1764
**Date**: 2025-12-20

**Key Findings**:
1. ContentType has 8 variants controlling UI (no validation)
2. LLM invocation happens in 5 places (scattered)
3. State transitions happen in multiple places (handle_key, event handlers, ViewAction handlers)
4. No explicit state machine - all transitions are ad-hoc
5. 36+ fields in WorktreeView spread across subsystems
6. State invariants not enforced (e.g., PromptInput assumes prompt_input.is_some())

**Files Analyzed**:
- `/crates/rstn/src/tui/views/worktree/view.rs` (3300+ lines)
- `/crates/rstn/src/tui/views/mod.rs` (ViewAction enum)
- `/crates/rstn/src/tui/event.rs` (Event types)
- `/crates/rstn/src/tui/app.rs` (Event handling)
- `/crates/rstn/src/tui/state/worktree.rs` (Serializable state)
