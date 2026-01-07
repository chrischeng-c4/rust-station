---
title: "Testing Guide"
description: "Testing patterns for state-first architecture"
category: workflow
status: active
last_updated: 2025-12-26
version: 3.0.0
---

# Testing Guide

## Testing Pyramid

```
         ╱╲
        ╱  ╲
       ╱ E2E╲      Playwright (few, slow)
      ╱──────╲
     ╱Component╲   Vitest + RTL (medium)
    ╱────────────╲
   ╱ State/Unit   ╲  Rust tests (many, fast)
  ╱────────────────╲
```

---

## Rust Backend Tests

### Location
```
packages/core/src/
├── reducer.rs     # State transition tests
├── actions.rs     # Serialization tests
├── persistence.rs # Load/save tests
├── docker.rs      # Docker API tests
├── justfile.rs    # Parser tests
└── worktree.rs    # Git worktree tests
```

### Run Tests
```bash
cd packages/core
cargo test
```

### Required Test Patterns

#### 1. Round-Trip Serialization

```rust
#[test]
fn test_state_round_trip() {
    let state = AppState::default();
    let json = serde_json::to_string(&state).unwrap();
    let loaded: AppState = serde_json::from_str(&json).unwrap();
    assert_eq!(state, loaded);
}
```

#### 2. State Transitions

```rust
#[test]
fn test_open_project() {
    let mut state = AppState::default();
    reduce(&mut state, Action::OpenProject {
        path: "/test/project".into()
    });
    assert_eq!(state.projects.len(), 1);
    assert_eq!(state.projects[0].name, "project");
}
```

#### 3. Action Serialization

```rust
#[test]
fn test_action_serialization() {
    let action = Action::SetFeatureTab { tab: FeatureTab::Dockers };
    let json = serde_json::to_string(&action).unwrap();
    assert_eq!(json, r#"{"type":"SetFeatureTab","payload":{"tab":"dockers"}}"#);
}
```

---

## React Component Tests

### Location
```
apps/desktop/src/renderer/src/
├── __tests__/
│   └── App.test.tsx
├── components/__tests__/
│   ├── LogPanel.test.tsx
│   └── ProjectTabs.test.tsx
├── features/dockers/__tests__/
│   ├── DockersPage.test.tsx
│   └── DockerServiceCard.test.tsx
└── hooks/__tests__/
    └── useAppState.test.ts
```

### Run Tests
```bash
cd apps/desktop
pnpm test        # Watch mode
pnpm test:run    # Single run
```

### Test Setup

```typescript
// src/renderer/src/test/setup.ts
import '@testing-library/jest-dom'

// Mock window.api
vi.stubGlobal('window', {
  ...window,
  api: {
    docker: {
      isAvailable: vi.fn().mockResolvedValue(true),
      listServices: vi.fn().mockResolvedValue([]),
      // ...
    },
    // ...
  },
  stateApi: {
    dispatch: vi.fn().mockResolvedValue(undefined),
    getState: vi.fn().mockResolvedValue('{}'),
    onStateUpdate: vi.fn().mockReturnValue(() => {}),
  },
})
```

### Example Component Test

```typescript
import { render, screen } from '@testing-library/react'
import { DockerServiceCard } from '../DockerServiceCard'

describe('DockerServiceCard', () => {
  const mockService = {
    id: 'test-postgres',
    name: 'PostgreSQL',
    image: 'postgres:16',
    status: 'running' as const,
    port: 5432,
    service_type: 'Database' as const,
  }

  it('renders service name', () => {
    render(<DockerServiceCard service={mockService} />)
    expect(screen.getByText('PostgreSQL')).toBeInTheDocument()
  })

  it('shows running status', () => {
    render(<DockerServiceCard service={mockService} />)
    expect(screen.getByText('Running')).toBeInTheDocument()
  })
})
```

---

## 4. Visual Regression Testing (MD3 Compliance)

Rustation enforces strict adherence to Material Design 3 using Playwright visual regression tests. Every UI change MUST be accompanied by an updated or new visual snapshot.

### 4.1 Running Visual Tests

To run the visual regression suite:

```bash
cd e2e
pnpm exec playwright test md3-visual-regression.spec.ts
```

### 4.2 Updating Baselines

If a UI change is intentional and follows MD3 standards, update the baseline snapshots:

```bash
cd e2e
pnpm exec playwright test md3-visual-regression.spec.ts --update-snapshots
```

### 4.3 Requirements for New UI Features

1.  **Strict MD3**: All components must use MUI v7 and theme tokens.
2.  **Snapshot Coverage**: Add a test case in `e2e/md3-visual-regression.spec.ts`.
3.  **Color Contrast**: Visual snapshots should be checked for accessibility compliance.


---

## Test Coverage Goals

| Layer | Target | Current |
|-------|--------|---------|
| Rust State | 90%+ | ~80% |
| React Components | 70%+ | ~50% |
| E2E Critical Paths | All | Partial |

---

## CI Integration

Tests run on every PR:
1. `cargo test` - Rust tests
2. `pnpm test:run` - React tests
3. `pnpm test:e2e` - E2E tests (on build)

---

## References

- [State-First Principle](01-state-first.md)
- [Architecture Overview](00-overview.md)
