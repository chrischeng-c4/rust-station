# Architecture Diagrams

## 1. State Diagram
```mermaid
stateDiagram-v2
    [*] --> ThemeLoading
    ThemeLoading --> ThemeReady: Theme Config Loaded
    ThemeReady --> RenderUI: ThemeProvider Applied
    RenderUI --> [*]
```

## 2. Flow Chart
```mermaid
flowchart TD
    A[App Start] --> B{Load Theme}
    B --> C[Create MUI Theme]
    C --> D[Apply Compact Overrides]
    D --> E[Inject ThemeProvider]
    E --> F[Render Components]
```

## 3. Sequence Diagram
```mermaid
sequenceDiagram
    participant App
    participant ThemeProvider
    participant ThemeConfig
    participant Component

    App->>ThemeConfig: import theme
    ThemeConfig-->>App: return theme object
    App->>ThemeProvider: pass theme
    ThemeProvider->>Component: provide style context
    Component->>ThemeConfig: access palette/spacing
    Component-->>App: render with new styles
```

## 4. UI Layout Diagram
```
┌──────────────────────────────────────────────────┐
│ ┌──────────────┐ ┌─────────────────────────────┐ │
│ │  ProjectTabs │ │      Global Toolbar         │ │
│ └──────────────┘ └─────────────────────────────┘ │
├──────┬───────────────────────────────────────────┤
│ Sidebar  │  Main Content Area                        │
│      │                                           │
│ Task │  ┌─────────────────────────────────────┐  │
│      │  │ Card (Compact)                      │  │
│ Exp  │  │ [Button] [Button]                   │  │
│      │  └─────────────────────────────────────┘  │
│ Set  │                                           │
│      │  ┌─────────────────────────────────────┐  │
│      │  │ Table (Dense)                       │  │
│      │  │ Row 1: Data                         │  │
│      │  │ Row 2: Data                         │  │
│      │  └─────────────────────────────────────┘  │
│      │                                           │
└──────┴───────────────────────────────────────────┘
```
