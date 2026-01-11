# Cleanup TODO - GPUI Migration 後清理工作

## 概要

從 Electron+React 遷移到 GPUI 後，許多檔案和配置已經過時，需要清理或更新。

**最後更新**: 2026-01-12

---

## 🔴 必須刪除的檔案/目錄

### 1. Node.js 專案配置（已過時）

| 檔案 | 狀態 | 說明 |
|------|------|------|
| `package.json` | ⚠️ 保留 | 僅用於 VitePress 文檔和 Playwright E2E 測試 |
| `pnpm-workspace.yaml` | ❌ 刪除 | 不再有 pnpm workspace（desktop/、packages/ 已刪除）|
| `.npmrc` | 🔍 檢查 | 如果存在，評估是否需要 |

**建議**:
- ✅ 保留 `package.json` - 用於文檔生成 (VitePress)
- ❌ 刪除 `pnpm-workspace.yaml` - 已無 monorepo 結構
- 🔄 更新 `package.json` - 移除與 Electron 相關的腳本

---

### 2. E2E 測試（針對 Electron UI）

**目錄**: `e2e/`

**現有測試檔案**:
```
e2e/agent-rules.spec.ts
e2e/change-management.spec.ts
e2e/claude-code.spec.ts
e2e/command-palette.spec.ts
e2e/constitution-workflow.spec.ts
e2e/context-engine.spec.ts
e2e/docker.spec.ts
e2e/electron.fixture.ts         # Electron 啟動器
e2e/env-management.spec.ts
e2e/file-explorer.spec.ts
e2e/justfile-tasks.spec.ts
e2e/mcp-server.spec.ts
e2e/review-gate.spec.ts
e2e/terminal.spec.ts
e2e/workflows.spec.ts
```

**問題**:
- 所有測試都基於 Playwright + Electron
- 測試的是 React UI，現在是 GPUI
- `electron.fixture.ts` 啟動舊的 Electron 應用

**選項**:
1. ❌ **刪除全部** - 最簡單，但失去測試覆蓋
2. ⏸️ **保留但標記為過時** - 暫時保留，Phase 6 Stage 4 重寫
3. 🔄 **重寫為 GPUI 測試** - 工作量大，但保持測試覆蓋

**建議**: 選項 2 - 保留檔案，但添加 README 說明這些是舊的 Electron 測試

---

### 3. GitHub Workflows（CI/CD）

**檔案**: `.github/workflows/`

**可能過時的 workflow**:
- `check-mock.yml` - 檢查 `desktop/src/renderer` 中的 MOCK 數據（目錄已刪除）
- 其他與 Node.js/Electron 構建相關的 workflow

**TODO**: 檢查並更新 CI/CD pipeline

---

### 4. 配置檔案

| 檔案 | 狀態 | 說明 |
|------|------|------|
| `tsconfig.json` | 🔍 檢查 | 如果存在，可能只用於 VitePress |
| `playwright.config.ts` | ⚠️ 評估 | E2E 測試配置，視測試策略而定 |
| `.eslintrc.*` | 🔍 檢查 | TypeScript linting，現在是 Rust |
| `.prettierrc.*` | 🔍 檢查 | JS/TS 格式化，現在用 `cargo fmt` |

---

## 🟡 需要更新的檔案

### 1. README.md

**當前內容**: 可能還描述 Electron 架構

**需要更新**:
- 架構說明（Electron → GPUI）
- 安裝步驟（`pnpm install` → `cargo build`）
- 開發指令（`just setup && just dev` → `just dev`）
- 構建說明（`just build-app` → `just build-release`）

**TODO**: 全面更新 README

---

### 2. package.json 腳本

**當前腳本**:
```json
{
  "scripts": {
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui",
    "test:e2e:headed": "playwright test --headed",
    "test:screenshots": "playwright test -c e2e/playwright.config.ts e2e/generate-screenshots.ts",
    "docs:dev": "vitepress dev docs",
    "docs:build": "vitepress build docs",
    "docs:preview": "vitepress preview docs"
  }
}
```

**建議更新**:
```json
{
  "scripts": {
    "docs:dev": "vitepress dev docs",
    "docs:build": "vitepress build docs",
    "docs:preview": "vitepress preview docs"
  }
}
```

**移除**:
- `test:e2e:*` - E2E 測試已過時

---

### 3. 文檔（docs/、dev-docs/）

**可能過時的內容**:
- 安裝指南（還提到 Node.js/Electron）
- 架構文檔（描述 Electron+React）
- API 參考（napi-rs bindings）

**TODO**:
- [ ] 檢查 `docs/` 下的所有 `.md` 檔案
- [ ] 更新安裝步驟
- [ ] 更新架構圖
- [ ] 移除 Electron 相關內容

---

### 4. OpenSpec 規格

**路徑**: `openspec/specs/`

**可能過時的規格**:
- `shared-ui/spec.md` - 提到 MUI/React
- `terminal-pty/spec.md` - 提到 xterm.js

**狀態**: Phase 2 已更新部分，但可能還有遺漏

**TODO**: 全面審查所有規格檔案

---

## 🟢 保留的檔案

### VitePress 文檔系統
- `docs/` - 用戶文檔
- `package.json` - 僅用於 VitePress
- VitePress 相關依賴

**原因**: 文檔生成與應用架構無關，繼續使用 VitePress

---

### Git 配置
- `.gitignore`
- `.gitattributes`

**狀態**: 可能需要小幅更新（移除 `node_modules` 等 Node.js 特定規則）

---

## 清理建議優先級

### 🔴 優先級 1（立即處理）

1. **刪除 `pnpm-workspace.yaml`**
   ```bash
   rm pnpm-workspace.yaml
   ```

2. **更新 `package.json`**
   - 移除 E2E 測試腳本
   - 保留文檔腳本

3. **添加 E2E 目錄說明**
   ```bash
   echo "# Deprecated E2E Tests (Electron)" > e2e/README.md
   echo "These tests are for the old Electron architecture." >> e2e/README.md
   echo "DO NOT USE. Will be rewritten for GPUI in Phase 6 Stage 4." >> e2e/README.md
   ```

---

### 🟡 優先級 2（本週處理）

4. **更新 README.md**
   - 改寫架構說明
   - 更新安裝和開發指令

5. **檢查 GitHub Workflows**
   - 移除 `check-mock.yml`（已無 `desktop/src/renderer` 目錄）
   - 更新 CI 流程為 Rust/Cargo

6. **更新 .gitignore**
   - 移除 Node.js 特定規則（如 `node_modules/`）
   - 確保 Rust 構建產物被忽略

---

### 🟢 優先級 3（Phase 6 完成前處理）

7. **審查文檔**
   - 檢查 `docs/` 下所有內容
   - 更新安裝指南、快速開始、架構文檔

8. **審查 OpenSpec 規格**
   - 確保所有規格反映 GPUI 架構

9. **決定 E2E 測試策略**
   - 刪除舊測試 OR 重寫為 GPUI 測試

---

## 快速清理腳本

```bash
# 1. 刪除 pnpm workspace 配置
rm pnpm-workspace.yaml

# 2. 標記 e2e 為過時
cat > e2e/README.md << 'EOF'
# ⚠️ Deprecated - Old Electron E2E Tests

These Playwright tests are for the **old Electron+React architecture**.

**Status**: Outdated and non-functional
**Reason**: Application migrated to GPUI (Zed's native Rust UI framework)

## DO NOT USE

- Tests expect Electron application (now pure Rust/GPUI)
- Tests target React components (now GPUI views)
- `electron.fixture.ts` won't work with new architecture

## Future

Will be replaced with GPUI integration tests in Phase 6 Stage 4.

See: [PHASE_6_PLAN.md](../PHASE_6_PLAN.md)
EOF

# 3. 更新 package.json（手動編輯，移除 test:e2e 腳本）

# 4. 檢查並移除過時的 GitHub workflow
# （手動檢查 .github/workflows/ 目錄）

echo "✅ 基本清理完成"
echo "⏭️ 接下來請手動檢查："
echo "  1. README.md"
echo "  2. .github/workflows/"
echo "  3. docs/ 文檔內容"
```

---

## 參考資料

- [GPUI_MIGRATION_PROGRESS.md](GPUI_MIGRATION_PROGRESS.md) - Phase 1 刪除了哪些內容
- [PHASE_6_PLAN.md](PHASE_6_PLAN.md) - Phase 6 測試計劃
- [dev-docs/workflow/definition-of-done.md](dev-docs/workflow/definition-of-done.md) - 測試策略

---

## 檢查清單

執行清理後，確認以下項目：

- [ ] `pnpm-workspace.yaml` 已刪除
- [ ] `package.json` 只保留文檔腳本
- [ ] `e2e/README.md` 已添加過時警告
- [ ] README.md 已更新為 GPUI 架構
- [ ] `.github/workflows/` 已更新
- [ ] `.gitignore` 已更新
- [ ] 文檔（docs/）已審查並更新
- [ ] OpenSpec 規格已審查

---

**最後更新**: 2026-01-12
**下次審查**: Phase 6 完成後
