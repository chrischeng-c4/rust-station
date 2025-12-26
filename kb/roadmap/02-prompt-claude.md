---
title: "Prompt Claude Chat UI"
description: "Spec: Native Chat Interface for AI Assistance"
category: roadmap
status: planned
version: 1.1.0
---

# Feature Spec: Prompt Claude Chat UI

## 1. Overview

**Goal**: A built-in chat interface to interact with an LLM (Claude) that has access to the project context via the native MCP server.
**Core Value**: Seamless "Ask & Code" workflow without switching windows.

## 2. User Stories

1. **Ask Question**: As a user, I want to ask "Where is the auth logic?" and get an answer with file links.
2. **Generate Code**: As a user, I want to ask "Create a login form" and get a code block I can copy.
3. **Context**: As a user, I want the chat to automatically know about the file I currently have open.

## 3. UI Design

### Location
- **View**: Right-side panel (collapsible) or "Assistant" tab in sidebar.

### Layout
```
+-------------------------------------------------------+
| Claude Assistant                                  [x] |
+-------------------------------------------------------+
| [User] How do I run tests?                            |
|                                                       |
| [Claude] You can use `just test`.                     |
| Here is the command from your justfile:               |
| ```bash                                               |
| cargo test && pnpm test                               |
| ```                                                   |
|                                                       |
| [User] Fix the bug in main.rs                         |
|                                                       |
| [Claude] Reading src/main.rs...                       |
| I found a missing semicolon on line 10.               |
+-------------------------------------------------------+
| [ Type a message... ] [Attach] [Send]                 |
+-------------------------------------------------------+
```

## 4. Architecture

### Backend (Rust)
- **Client**: `anthropic-sdk-rs` (or raw HTTP reqwest).
- **Context Gathering**: See "Intelligent Context Engine".
- **Streaming**: Must support SSE to stream tokens to frontend.

### Frontend (React)
- **State**: `messages: Vec<Message>`, `isTyping: boolean`.
- **Rendering**: `react-markdown` with syntax highlighting (`prismjs`).

## 5. Actions & API

| Action | Payload | Description |
|--------|---------|-------------|
| `SendChatMessage` | `{ text, context_ids }` | User sends message |
| `StreamChatResponse` | `{ chunk }` | Server pushes token |
| `ChatError` | `{ message }` | Connection/API error |

## 6. Implementation Plan

### Phase 1: Basic Chat
- API Key management (Settings).
- Simple Text In/Text Out.

### Phase 2: Context Injection
- Append "System Prompt" with project details.
- Attach open file content.

### Phase 3: Streaming & Markdown
- Implement token streaming pipeline.
- Render rich text.

## 7. Edge Cases
- **Network Failure**: Handle offline state gracefully.
- **Cost**: Display token usage/cost estimation if possible.