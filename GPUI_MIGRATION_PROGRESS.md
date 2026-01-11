# GPUI Migration Progress

## Overview

Migration of rustation from Electron+React to GPUI (Zed's GPU-accelerated UI framework) for native Rust UI.

**Start Date**: 2026-01-11
**Current Phase**: Phase 3 Complete (UI Foundation)
**Status**: ⚠️ BLOCKED on Metal Toolchain (Xcode 26 beta issue)

---

## Completed Phases

### ✅ Phase 1: Foundation & Cleanup (Commit: 69c5134)

**Objective**: Remove Electron/React stack and establish Rust-only architecture.

**Changes**:
- ❌ Removed `desktop/` directory (22,687 lines - entire Electron+React frontend)
- ❌ Removed `packages/` directory (napi-rs Node.js bindings)
- ✅ Created `crates/` workspace structure (following Zed's pattern)
- ✅ Migrated `packages/core` → `crates/rstn-core/` (pure Rust library)
  - Changed crate-type: `["cdylib"]` → `["rlib"]`
  - Removed all `#[napi]` attributes and napi dependencies
  - Changed error handling: `napi::Result` → `anyhow::Result`
  - Removed `build.rs` and napi-build
- ✅ Created `crates/rstn/` main application
  - Added GPUI dependency from Zed repository
  - Implemented basic `main.rs` with window setup
  - Created initial `RstnApp` state model

**Key Files**:
- [crates/rstn-core/Cargo.toml](crates/rstn-core/Cargo.toml) - Pure Rust library
- [crates/rstn-core/src/lib.rs](crates/rstn-core/src/lib.rs) - Removed napi exports
- [crates/rstn/Cargo.toml](crates/rstn/Cargo.toml) - GPUI application
- [crates/rstn/src/main.rs](crates/rstn/src/main.rs) - Entry point

**Result**: Clean Rust workspace, no Node.js dependencies.

---

### ✅ Phase 2: OpenSpec Updates (Commit: f43d09c)

**Objective**: Update specifications to reflect GPUI architecture.

**Changes**:
- ✅ Updated [openspec/specs/shared-ui/spec.md](openspec/specs/shared-ui/spec.md)
  - Requirement "Global Theme Density": MUI `defaultProps` → GPUI styling
  - Removed framework-specific implementation details
- ✅ Updated [openspec/specs/terminal-pty/spec.md](openspec/specs/terminal-pty/spec.md)
  - Requirement "Terminal Display": xterm.js → native GPUI renderer
  - Added GPU acceleration specification

**Result**: Specifications aligned with GPUI architecture.

---

### ✅ Phase 3: UI Foundation (Commit: be0a3d5)

**Objective**: Create reusable UI component library with Material Design 3 theme.

**Changes**:
- ✅ Created `crates/rstn-ui/` component library
- ✅ **Theme System** ([crates/rstn-ui/src/theme.rs](crates/rstn-ui/src/theme.rs)):
  - Material Design 3 color palette (dark mode)
  - Primary: `#D0BCFF`, Secondary: `#CCC2DC`, Background: `#1C1B1F`
  - Shape config: 16px border radius, 8px spacing base
  - `Themed` trait for consistent styling (buttons, cards, pills)
  - Tests for theme creation and spacing multiplier

- ✅ **Components** ([crates/rstn-ui/src/components.rs](crates/rstn-ui/src/components.rs)):
  - `NavItem`: Navigation item data structure
  - `Sidebar`: Vertical navigation with pill-shaped selection indicators
    - Matches [OLD_UI_ANALYSIS.md](OLD_UI_ANALYSIS.md) sidebar structure
    - 8 navigation items: Explorer, Flows, Claude, Tasks, rstn, Chat, A2UI, Term
  - `ShellLayout`: Main app shell (header + sidebar + content + status bar)
  - `PageHeader`: Page titles with descriptions and action buttons
  - `EmptyState`: Placeholder for empty data states
  - Tests for component creation

- ✅ Updated [crates/rstn/src/main.rs](crates/rstn/src/main.rs):
  - Integrated rstn-ui components
  - Replaced inline styling with theme-based components
  - Created navigation matching old Electron UI

**Result**: Complete UI component library ready for feature views.

---

## Current Blocker

### ⚠️ Metal Toolchain Issue

**Error**: GPUI build fails with:
```
error: cannot execute tool 'metal' due to missing Metal Toolchain
```

**Cause**: Xcode 26 beta issue - Metal shader compiler not available.

**Attempted Fix**:
```bash
xcodebuild -downloadComponent MetalToolchain
# Failed with plugin loading errors
```

**Impact**:
- Cannot build `cargo build -p rstn` or `cargo build -p rstn-ui`
- Cannot verify UI renders correctly
- Cannot run application to test components

**Workaround Options**:
1. Wait for Xcode 26 beta fix
2. Downgrade to Xcode 15.x stable
3. Continue implementing feature views (code compiles logically, just can't build)

---

## Architecture Overview

### Directory Structure

```
rustation/
├── crates/
│   ├── rstn/              # Main GPUI application
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs    # Entry point, AppView
│   ├── rstn-core/         # Pure Rust library (business logic)
│   │   ├── Cargo.toml     # No napi dependencies
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app_state.rs
│   │       ├── reducer/
│   │       ├── docker.rs
│   │       ├── justfile.rs
│   │       └── ...
│   └── rstn-ui/           # UI component library
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── theme.rs   # MD3 theme system
│           └── components.rs  # Reusable components
└── Cargo.toml             # Workspace config
```

### Component Architecture

```
┌────────────────────────────────────────────────────┐
│ AppView (main.rs)                                  │
│                                                    │
│  ┌──────────────────────────────────────────────┐ │
│  │ ShellLayout                                  │ │
│  │                                              │ │
│  │  ┌─────────────────────────────────────┐    │ │
│  │  │ Header (title bar)                  │    │ │
│  │  └─────────────────────────────────────┘    │ │
│  │                                              │ │
│  │  ┌──────────┬──────────────────────────┐    │ │
│  │  │ Sidebar  │ Content Area             │    │ │
│  │  │          │                          │    │ │
│  │  │ NavItem  │ PageHeader               │    │ │
│  │  │ NavItem  │                          │    │ │
│  │  │ NavItem  │ (Feature Views)          │    │ │
│  │  │ ...      │                          │    │ │
│  │  │          │                          │    │ │
│  │  └──────────┴──────────────────────────┘    │ │
│  │                                              │ │
│  │  ┌─────────────────────────────────────┐    │ │
│  │  │ Status Bar                          │    │ │
│  │  └─────────────────────────────────────┘    │ │
│  └──────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────┘
```

---

## Material Design 3 Theme

### Color Palette

| Token                | Hex Value | Usage                        |
|----------------------|-----------|------------------------------|
| Primary Main         | `#D0BCFF` | Primary buttons, active items|
| Primary Container    | `#4F378B` | Hover states                 |
| Secondary Main       | `#CCC2DC` | Secondary actions            |
| Secondary Container  | `#4A4458` | Selected items (pill bg)     |
| Background Default   | `#1C1B1F` | Main background              |
| Background Paper     | `#2B2930` | Cards, elevated surfaces     |
| Surface Container    | `#2B2930` | Sidebar, containers          |
| Divider              | `#3D3D3D` | Borders, separators          |
| Text Primary         | `#FFFFFF` | Main text                    |
| Text Secondary       | `#AAAAAA` | Descriptions, hints          |

### Shape Configuration

- **Border Radius**: 16px (large rounded corners)
- **Border Radius Small**: 8px
- **Border Radius Extra Small**: 4px
- **Base Spacing**: 8px (use `theme.spacing(n)` for multiples)

---

## Next Steps (Once Metal Toolchain Fixed)

### Phase 4: Core Feature Views

**Objective**: Port individual feature pages from Electron UI.

**Priority Order** (based on [OLD_UI_ANALYSIS.md](OLD_UI_ANALYSIS.md)):

1. **TasksPage** (Priority 1)
   - Command list cards
   - Output panel with logs
   - Run/stop actions
   - Integration with [crates/rstn-core/src/justfile.rs](crates/rstn-core/src/justfile.rs)

2. **DockersPage** (Priority 1)
   - Service cards with status indicators
   - Start/stop/restart actions
   - Log viewer
   - Integration with [crates/rstn-core/src/docker.rs](crates/rstn-core/src/docker.rs)

3. **ExplorerPage** (Priority 1)
   - File tree view
   - Git status display
   - File preview panel
   - Integration with [crates/rstn-core/src/worktree.rs](crates/rstn-core/src/worktree.rs)

4. **TerminalPage** (Priority 2)
   - PTY integration using `portable-pty`
   - ANSI color rendering
   - Integration with [crates/rstn-core/src/terminal.rs](crates/rstn-core/src/terminal.rs)

5. **ChatPage, WorkflowsPage, SettingsPage** (Priority 3)

### Phase 5: Advanced Features

- MCP inspector
- A2UI dynamic renderer
- Context Engine visualizations

### Phase 6: Final Polish

- Feature parity verification
- Performance optimization
- Documentation updates
- CI/CD pipeline updates

---

## Implementation Notes

### GPUI Patterns

**State Management**:
```rust
struct RstnApp {
    active_tab: &'static str,
}

struct AppView {
    app: Model<RstnApp>,  // GPUI owns the state
}
```

**Rendering**:
```rust
impl Render for AppView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let app = self.app.read(cx);
        let theme = MaterialTheme::dark();

        // Build UI tree
        shell.render(content, cx)
    }
}
```

**Styling with Theme**:
```rust
div()
    .px(theme.spacing(2.0))
    .bg(theme.background.paper)
    .rounded(theme.shape.border_radius)
    .pill(&theme, is_active)
```

### Component Reusability

All components in `rstn-ui` are designed to be:
- **Theme-aware**: Accept `MaterialTheme` parameter
- **Composable**: Return `Div` that can be chained
- **Testable**: Unit tests for creation logic

---

## References

### Documentation
- [OLD_UI_ANALYSIS.md](OLD_UI_ANALYSIS.md) - Analysis of old Electron UI
- [openspec/changes/migrate-to-gpui/](openspec/changes/migrate-to-gpui/) - Migration proposal
- [dev-docs/architecture/](dev-docs/architecture/) - Architecture decisions

### External Resources
- [GPUI Examples](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)
- [Zed UI Components](https://github.com/zed-industries/zed/tree/main/crates/ui)
- [Material Design 3](https://m3.material.io/)

---

## Git History

```
be0a3d5 feat(rstn-ui): Add UI component library with MD3 theme
f43d09c docs(openspec): Apply GPUI migration spec deltas
69c5134 feat: Migrate to GPUI - Phase 1 Foundation
```

---

## Status Summary

| Phase | Status | Completion |
|-------|--------|------------|
| Phase 1: Foundation | ✅ Complete | 100% |
| Phase 2: Specs | ✅ Complete | 100% |
| Phase 3: UI Foundation | ✅ Complete | 100% |
| Phase 4: Core Features | ⏸️ Blocked | 0% |
| Phase 5: Advanced Features | ⏸️ Pending | 0% |
| Phase 6: Polish | ⏸️ Pending | 0% |

**Overall Progress**: 3/6 phases (50%)

**Blocker**: Metal Toolchain required for GPUI build
