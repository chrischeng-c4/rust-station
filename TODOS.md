# TODOS - rstn Development Roadmap

> This file tracks feature development, technical debt, and known gaps for rustation.

---

## 🏗️ Track A: State-First Refactoring (Technical Debt)

**Goal:** Eliminate "transitional" gaps where the frontend still uses legacy patterns or handles logic that belongs in the backend.

### Phase A1: Eliminate Legacy API Usage (Renderer)
- [x] Refactor `DockersPage.tsx` to use `dispatch({ type: 'CreateDatabase' })` instead of `window.api.docker.createDatabase`.
- [x] Refactor `DockersPage.tsx` to use `dispatch({ type: 'CreateVhost' })` instead of `window.api.docker.createVhost`.
- [x] Refactor `AddWorktreeDialog.tsx` to use `dispatch({ type: 'FetchBranches' })` instead of `window.api.worktree.listBranches`.
- [x] Refactor `ContextFilesInput.tsx` and `SourceCodeViewer.tsx` to use `dispatch({ type: 'ReadFile' })` instead of `window.api.file.read`.
- [x] Remove `window.api` exposure from `preload/index.ts` once all legacy calls are migrated.

### Phase A2: Centralize Logic in Backend
- [x] Move `justfilePath` construction logic from `TasksPage.tsx` to Rust backend (Backend should return available tasks).
- [x] Verify no other path concatenation logic exists in the frontend.

---

## 📂 Track B: File Explorer & SQLite (New)

**Goal:** Implement a robust, local-first file management system with structured data persistence.

### Phase B1: SQLite Infrastructure
- [x] Add `rusqlite` dependency to `packages/core`.
- [x] Implement `db.rs` for connection management and migrations.
- [x] Implement activity log sink to SQLite.

### Phase B2: File Explorer Backend
- [x] Define `FileExplorerState` in `app_state.rs`.
- [x] Implement directory reading logic with `.gitignore` support.
- [x] Implement sort/filter logic in Rust.
- [x] Implement metadata and preview fetcher.

### Phase B3: File Explorer UI
- [x] Create `ExplorerPage.tsx` container with resizable panels.
- [x] Implement virtualized `FileList` component.
- [x] Implement `DetailPanel` with Metadata, Preview, and Comments tabs.

---

## 🛠️ Track C: Testing & Verification (Gaps)

**Goal:** Ensure comprehensive coverage for both state transitions and end-to-end user flows, including offline/mocked scenarios.

### Phase C1: Backend State Tests (Rust)
- [x] Add reducer tests for MCP actions (start/stop/status/port/error/log/tools).
- [x] Add reducer tests for Notifications actions (add/dismiss/mark read/clear).
- [x] Add reducer tests for Terminal actions (spawn/resize/set session/kill).
- [x] Add reducer tests for Context actions (load/set/init/generate/sync/complete/fail).
- [x] Add reducer tests for Change edge transitions (cancel/fail/archive).
- [x] Add serialization round-trip tests for `ContextState`, `McpState`, `Notification`, and `TerminalState`.

### Phase C2: E2E Test Coverage
- [x] Create "Mocked Backend" E2E tests for Docker flow.
- [x] Implement full-flow E2E for Change Management.
- [x] Implement full-flow E2E for ReviewGate.

---

## 🚀 Track D: Feature Completion (In Progress)

### ReviewGate (Phase B3 - Async Pipeline)
- [x] Implement `SubmitReviewFeedback` async handler in `packages/core/src/lib.rs`.

### Experimental: A2UI
- [x] Implement MCP tool + backend bridge to accept A2UI JSON and push to renderer.

---

## ✅ Completed Tracks (Archive)

### File Operations
- [x] Secure file reader API (`file_reader.rs`)
- [x] Frontend bridge and TypeScript definitions

### Living Context
- [x] Context state and file management
- [x] AI-powered context generation
- [x] Enhanced context synchronization

### UI Architecture
- [x] Migrate to Material Design 3 (MUI v5/v7)
- [x] Remove legacy CSS frameworks (Tailwind/Shadcn)
- [x] Update `ProjectTabs` to MD3
- [x] Stabilize UI tests (`md3-visual-regression.spec.ts`)
