---
title: "Data Migration System"
description: "Spec: State Persistence Versioning and Migration"
category: roadmap
status: planned
version: 1.0.0
---

# Feature Spec: Data Migration System

## 1. Overview

**Goal**: Ensure user data is preserved and correctly updated when the application structure (`AppState`) changes between versions.
**Core Value**: Stability. Prevents "White Screen of Death" or data loss after an update.

## 2. Architecture

### Versioning
- **File**: `app_state.json` (and others)
- **Field**: Add `version: u32` to the root JSON object.
- **Current**: Version 1.

### Migration Logic
- **Trait**:
```rust
trait Migration {
    fn from_version(&self) -> u32;
    fn migrate(&self, value: serde_json::Value) -> Result<serde_json::Value>;
}
```

- **Registry**: A list of migrations `1->2`, `2->3`, etc.

## 3. Workflow

1. **Load**: App starts, reads `app_state.json` as generic `Value`.
2. **Check**: Read `version` field.
3. **Loop**: While `version < CURRENT_CODE_VERSION`:
   - Find migration for `version`.
   - Apply migration.
   - `version++`.
4. **Deserialize**: Convert final `Value` to `AppState` struct.
5. **Save**: Write updated structure back to disk.

## 4. Example Scenarios

### Scenario A: Renaming a Field
- **Change**: `project_path` -> `default_path`.
- **Migration**:
  ```rust
  fn migrate(mut v: Value) -> Result<Value> {
      if let Some(path) = v["project_path"].take() {
          v["default_path"] = path;
      }
      Ok(v)
  }
  ```

### Scenario B: New Required Field
- **Change**: `GlobalSettings` added.
- **Migration**: Insert default `GlobalSettings` object if missing.

## 5. Safety
- **Backup**: Always copy `app_state.json` to `app_state.json.bak` before migrating.
- **Fallback**: If migration fails, load default empty state (and notify user "Settings reset due to error").

## 6. Implementation Plan

### Phase 1: Infrastructure
- Implement `MigrationManager` struct.
- Update `persistence.rs` to use it.

### Phase 2: First Migration
- Bump version to 2.
- Test the system by verifying v1 state loads correctly into v2 code.
