# ⚠️ Deprecated - Old Electron E2E Tests

These Playwright tests are for the **old Electron+React architecture**.

**Status**: Outdated and non-functional
**Reason**: Application migrated to GPUI (Zed's native Rust UI framework)

## DO NOT USE

- Tests expect Electron application (now pure Rust/GPUI)
- Tests target React components (now GPUI views)
- `electron.fixture.ts` won't work with new architecture

## What Happened

In Phase 1 of the GPUI migration (Commit: `69c5134`), we removed:
- `desktop/` directory (22,687 lines - entire Electron+React frontend)
- `packages/` directory (napi-rs Node.js bindings)

The application now runs as a native Rust binary using GPUI.

## Future

Will be replaced with GPUI integration tests in Phase 6 Stage 4.

See: [PHASE_6_PLAN.md](../PHASE_6_PLAN.md#testing--optimization-priority-medium)

## Migration Timeline

- **Phase 1-5**: ✅ Complete (UI framework migration)
- **Phase 6**: 🟡 In Progress (Backend integration)
- **Test Rewrite**: ⏸️ Planned for Phase 6 Stage 4

---

**Last Updated**: 2026-01-12
**Migration Docs**: [GPUI_MIGRATION_PROGRESS.md](../GPUI_MIGRATION_PROGRESS.md)
