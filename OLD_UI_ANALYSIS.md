# Old Electron+React UI Analysis

## 概覽

舊的 UI 是基於 **Electron + React + Material UI (MUI) v7** 的桌面應用程式。

### 技術棧
- **框架**: React 19
- **設計系統**: Material UI (MUI) v7 (`@mui/material`)
- **樣式**: Emotion (`@emotion/react`, `@emotion/styled`)
- **圖標**: Material Icons (`@mui/icons-material`)
- **打包**: Electron + Vite
- **測試**: Vitest + Playwright

---

## 目錄結構

```
desktop/src/renderer/src/
├── components/
│   ├── layout/              # 布局組件
│   │   ├── Sidebar.tsx      # 左側導航欄
│   │   └── GlobalIconBar.tsx
│   └── shared/              # 可重用組件
│       ├── PageHeader.tsx   # 頁面標題
│       ├── LogPanel.tsx     # 日誌面板
│       ├── EmptyState.tsx   # 空狀態提示
│       ├── ErrorBanner.tsx  # 錯誤橫幅
│       ├── SourceCodeViewer.tsx
│       └── viewers/         # 檔案檢視器
│           ├── PdfViewer.tsx
│           ├── ImageViewer.tsx
│           ├── ExcelViewer.tsx
│           └── ...
├── features/                # 功能模組
│   ├── explorer/            # 檔案瀏覽器
│   │   ├── ExplorerPage.tsx
│   │   ├── FileTreeView.tsx
│   │   ├── DetailPanel.tsx
│   │   └── FileTabs.tsx
│   ├── tasks/               # 任務管理
│   │   ├── TasksPage.tsx
│   │   └── TaskCard.tsx
│   ├── dockers/             # Docker 管理
│   │   ├── DockersPage.tsx
│   │   └── DockerServiceCard.tsx
│   ├── terminal/            # 終端機
│   │   └── TerminalPage.tsx
│   ├── chat/                # AI 對話
│   │   └── ChatPage.tsx
│   ├── workflows/           # 工作流程
│   │   ├── WorkflowsPage.tsx
│   │   ├── ConstitutionPanel.tsx
│   │   ├── ChangeManagementPanel.tsx
│   │   └── ReviewPanel.tsx
│   ├── mcp/                 # MCP 伺服器檢查器
│   │   └── McpPage.tsx
│   ├── a2ui/                # 動態 UI 渲染器
│   │   └── A2UIPage.tsx
│   └── settings/            # 設定
│       └── SettingsPage.tsx
├── hooks/
│   └── useAppState.ts       # 全局狀態 Hook
├── theme/
│   └── index.ts             # MUI 主題定義
└── App.tsx                  # 根組件
```

---

## 核心組件分析

### 1. Sidebar (導航欄)

**檔案**: `desktop/src/renderer/src/components/layout/Sidebar.tsx`

**功能**:
- 垂直導航列表（左側固定）
- 圖標 + 文字標籤
- 選中狀態高亮（Secondary Container 背景）
- Pill 形狀的選中指示器（16px borderRadius）

**導航項目**:
```typescript
const NAV_ITEMS = [
  { value: 'explorer', label: 'Explorer', icon: <ExplorerIcon /> },
  { value: 'workflows', label: 'Flows', icon: <WorkflowIcon /> },
  { value: 'claude-code', label: 'Claude', icon: <ClaudeIcon /> },
  { value: 'tasks', label: 'Tasks', icon: <TasksIcon /> },
  { value: 'mcp', label: 'rstn', icon: <ServerIcon /> },
  { value: 'chat', label: 'Chat', icon: <ChatIcon /> },
  { value: 'a2ui', label: 'A2UI', icon: <A2UIIcon /> },
  { value: 'terminal', label: 'Term', icon: <TerminalIcon /> },
]
```

**設計特點**:
- 每個項目高度 56px
- Icon 尺寸 24px
- Label 字體 12px (caption)
- 間距 12px (mb: 1.5)
- Pill 背景：`secondary.container` (選中時)
- 文字顏色：`onSecondaryContainer` (選中) / `onSurfaceVariant` (未選中)

---

### 2. TasksPage (任務頁面)

**檔案**: `desktop/src/renderer/src/features/tasks/TasksPage.tsx`

**佈局**:
```
┌─────────────────────────────────────┐
│ PageHeader (Title + Refresh Button)│
├─────────────────┬───────────────────┤
│ Commands List   │ Log Panel         │
│ (50% width)     │ (50% width)       │
│                 │                   │
│ - TaskCard 1    │ Output Lines      │
│ - TaskCard 2    │ ...               │
│ - TaskCard 3    │                   │
└─────────────────┴───────────────────┘
```

**功能**:
1. 載入 Justfile 命令列表
2. 顯示每個命令的卡片（名稱、描述、狀態）
3. 點擊運行命令
4. 即時顯示輸出日誌

**狀態管理**:
```typescript
const { tasks, projectPath, dispatch } = useTasksState()
```

**Actions**:
- `LoadJustfileCommands`: 載入命令列表
- `RunJustCommand`: 執行命令
- `RefreshJustfile`: 重新掃描
- `ClearTaskOutput`: 清除輸出

---

### 3. ExplorerPage (檔案瀏覽器)

**檔案**: `desktop/src/renderer/src/features/explorer/ExplorerPage.tsx`

**佈局**:
```
┌───────────────────────────────────────────┐
│ PathBreadcrumbs                          │
├───────────┬──────────────┬────────────────┤
│ Tree View │ File Table   │ Detail Panel   │
│ (25%)     │ (50%)        │ (25%)          │
│           │              │                │
│ Folders   │ Files List   │ Preview        │
│ ...       │ - file1.txt  │ ...            │
│           │ - file2.rs   │                │
└───────────┴──────────────┴────────────────┘
```

**組件**:
- **FileTreeView**: 樹狀目錄結構
- **FileTable**: 檔案列表（表格形式）
- **DetailPanel**: 檔案預覽（支援多種格式）
  - PDF Viewer
  - Image Viewer
  - Excel Viewer
  - Video Viewer
  - Source Code Viewer (語法高亮)

**功能**:
- Git 狀態顯示（M, A, D, ?? 等）
- 檔案評論（註解功能）
- 多種檔案格式預覽
- 麵包屑導航

---

### 4. WorkflowsPage (工作流程)

**檔案**: `desktop/src/renderer/src/features/workflows/WorkflowsPage.tsx`

**子面板**:
1. **ConstitutionPanel**: 編碼規則管理
2. **ChangeManagementPanel**: 變更管理
3. **ReviewPanel**: 審核閘門
4. **ContextPanel**: 上下文引擎

---

## Material Design 3 主題

**檔案**: `desktop/src/renderer/src/theme/index.ts`

```typescript
export const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: { main: '#D0BCFF' },      // M3 Purple 80
    secondary: { main: '#CCC2DC' },     // M3 Purple 80 (variant)
    background: {
      default: '#1C1B1F',              // M3 Surface
      paper: '#2B2930',                // M3 Surface Container
    },
  },
  shape: {
    borderRadius: 16,                   // 大圓角
  },
  components: {
    MuiButton: {
      defaultProps: {
        size: 'small',                  // 緊湊尺寸
      },
    },
    MuiPaper: {
      defaultProps: {
        variant: 'outlined',            // 預設使用描邊而非陰影
      },
    },
  },
})
```

**色彩系統**:
- Primary: #D0BCFF (紫色)
- Secondary: #CCC2DC (紫色變體)
- Background Default: #1C1B1F (深色背景)
- Background Paper: #2B2930 (容器背景)

**圓角**: 16px（大圓角，MD3 風格）

---

## 共用組件

### PageHeader
```typescript
<PageHeader
  title="Tasks"
  description="Run justfile commands"
>
  <Button>Refresh</Button>
</PageHeader>
```

### LogPanel
```typescript
<LogPanel
  title="Output"
  logs={lines}
  showCopy={true}
/>
```

### EmptyState
```typescript
<EmptyState
  icon={<ListAlt />}
  title="No Commands"
  description="No justfile found"
  action={{
    label: "Scan Again",
    onClick: refresh
  }}
/>
```

---

## 狀態管理架構

**Hook**: `useAppState()`

```typescript
const { state, dispatch } = useAppState()

// 專用 hooks
const { tasks, projectPath, dispatch } = useTasksState()
const { explorer, dispatch } = useExplorerState()
const { dockers, dispatch } = useDockersState()
```

**IPC 通訊**:
```typescript
// Preload Bridge
window.stateApi.dispatch(action)
window.stateApi.subscribe(callback)

// 狀態流向
Frontend → dispatch(Action) → IPC → Rust Backend → State Update → Notify Frontend
```

---

## GPUI 遷移建議

### 需要重新實作的組件

1. **Layout Components**
   - ✅ Sidebar (導航欄) - 圖標 + 文字，Pill 選中指示器
   - ✅ PageHeader - 標題 + 操作按鈕
   - ❌ GlobalIconBar (可能不需要)

2. **Shared Components**
   - ✅ LogPanel - 日誌滾動面板（重要）
   - ✅ EmptyState - 空狀態提示
   - ✅ ErrorBanner - 錯誤顯示
   - ✅ SourceCodeViewer - 語法高亮（使用 tree-sitter）
   - ❌ PDF/Image/Excel Viewer - 後期實作

3. **Feature Pages**
   - 優先級 1: TasksPage, ExplorerPage
   - 優先級 2: DockersPage, TerminalPage
   - 優先級 3: ChatPage, WorkflowsPage
   - 優先級 4: McpPage, A2UIPage, SettingsPage

### 設計遷移

| MUI 組件 | GPUI 等價物 |
|---------|-----------|
| `<Box>` | `div()` |
| `<Stack spacing={2}>` | `div().flex().gap(px(16.0))` |
| `<Paper variant="outlined">` | `div().bg(...).border_1().rounded(...)` |
| `<Typography variant="h6">` | `div().text_lg().font_weight(...)` |
| `<Button variant="outlined">` | `div().px(...).py(...).border_1().rounded().hover(...)` |
| `<IconButton>` | `div().w(px(40.0)).h(px(40.0)).rounded_full()` |

### 色彩對應

| MUI Token | GPUI 實作 |
|----------|----------|
| `primary.main` | `rgb(0xD0BCFF)` |
| `background.default` | `rgb(0x1C1B1F)` |
| `background.paper` | `rgb(0x2B2930)` |
| `divider` | `rgb(0x3D3D3D)` |

### 圓角對應

| MUI | GPUI |
|-----|------|
| `borderRadius: 16` | `rounded(px(16.0))` |
| Pill shape (32px height) | `rounded(px(16.0))` |
| Full circle | `rounded_full()` |

---

## 遷移優先級

### Phase 1: Shell & Layout (Week 1-2)
- [x] Sidebar (導航欄)
- [x] 主視窗佈局
- [x] 基本主題系統

### Phase 2: Core Components (Week 3-4)
- [ ] LogPanel
- [ ] PageHeader
- [ ] EmptyState
- [ ] ErrorBanner

### Phase 3: TasksPage (Week 5)
- [ ] 命令列表卡片
- [ ] 輸出面板
- [ ] 執行功能

### Phase 4: ExplorerPage (Week 6-7)
- [ ] 檔案樹
- [ ] 檔案列表
- [ ] Source Code Viewer
- [ ] Git 狀態

### Phase 5: 其他功能頁面 (Week 8-12)
- [ ] DockersPage
- [ ] TerminalPage
- [ ] ChatPage
- [ ] WorkflowsPage

---

## 參考資料

- **舊架構文檔**: `dev-docs/architecture/01-ui-component-architecture.md` (git HEAD~1)
- **MUI v7 文檔**: https://mui.com/material-ui/
- **Material Design 3**: https://m3.material.io/
- **GPUI Examples**: `~/.cargo/git/checkouts/zed-.../crates/gpui/examples/`
- **Zed UI**: `~/.cargo/git/checkouts/zed-.../crates/ui/`
