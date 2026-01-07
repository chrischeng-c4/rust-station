# GEMINI Context File

> This file serves as the long-term memory and context handover between sessions for the Gemini CLI agent.

---

## 📅 Session Info
- **Last Updated**: January 7, 2026
- **Current Phase**: Post-MD3 Migration Stabilization
- **System Status**: 🟢 Stable (Builds Passing, UI Tests Passing)

---

## 📝 Recent Accomplishments

### 1. Material Design 3 (MD3) Migration Completed
The application has been fully migrated to use Material UI (MUI) with a custom MD3 theme.
- **Removed**: Tailwind CSS, Shadcn UI, and legacy CSS files.
- **Refactored**: `App.tsx` now correctly imports and uses the MD3 `ThemeProvider`.
- **New Components**:
  - `desktop/src/renderer/src/features/projects/ProjectTabs.tsx`: Replaced the legacy tabs with MUI `Tabs` and `Tab`.
  - `desktop/src/renderer/src/components/shared/ErrorBoundary.tsx`: Added to catch React rendering errors.
- **Fixes**:
  - Solved `ReferenceError: useCallback is not defined` in `App.tsx`.
  - Solved `TypeError` in `LogPanel` by adding default props.

### 2. Test Verification
- **Visual Regression**: `e2e/md3-visual-regression.spec.ts` has been updated to handle the initial "Empty State" correctly.
- **Status**: All 5 tests in `md3-visual-regression.spec.ts` are PASSING.

---

## 📍 Current File System State

### Key Modified Files
- `desktop/src/renderer/src/App.tsx`: Main entry point, MD3 setup.
- `desktop/src/renderer/src/features/projects/ProjectTabs.tsx`: Project navigation.
- `desktop/src/renderer/src/components/shared/LogPanel.tsx`: Logs display.
- `e2e/md3-visual-regression.spec.ts`: E2E tests.

### Architecture Notes
- **Frontend**: React 19 + MUI v5/v7.
- **Backend**: Rust (napi-rs).
- **State**: `useAppState` hook drives the UI from Rust state.
- **KB**: `dev-docs/architecture/01-ui-component-architecture.md` is the source of truth for UI patterns.

---

## ⏭️ Next Steps (Prioritized)

1.  **Refactoring (Track A)**:
    - Continue with "Track A: State-First Refactoring" in `TODOS.md`.
    - Specifically, replace legacy `window.api.*` calls in `DockersPage.tsx` and `AddWorktreeDialog.tsx` with dispatch actions.

2.  **File Explorer (Track B)**:
    - Begin "Phase B1: SQLite Infrastructure" to support robust file management.

3.  **Cleanup**:
    - Monitor `ErrorBoundary` logs for any edge case crashes.

---

## 🧠 Memory Bank
- **Fact**: The project uses `just` for task running.
- **Fact**: E2E tests run via `pnpm exec playwright test` in the `e2e` folder.
- **Fact**: Frontend dev runs via `cd apps/desktop && pnpm dev`.