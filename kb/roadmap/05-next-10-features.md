---
title: "Next 10 Big Features"
description: "Prioritized feature roadmap for Q1 2026"
category: roadmap
status: planned
last_updated: 2025-12-26
version: 1.0.0
weight: 10
---

# Next 10 Big Features

This document outlines the next 10 major features to be implemented, prioritized by architectural dependency and user value.

## 1. Env Management UI (Project Scope)
**Status**: Backend Ready | **Gap**: UI Missing

A dedicated interface for the **Three-Scope** project level to manage environment variables across worktrees.
- **UI**: `EnvPage` accessible from the second top bar.
- **Features**:
  - List tracked patterns (`.env`, `.envrc`, etc.)
  - "Copy to Sibling" wizard
  - Toggle "Auto-Copy" on/off
  - View copy history/logs
- **Dependency**: `kb/architecture/07-env-management.md`

## 2. Settings UI (Global & Worktree Scope)
**Status**: Partial Backend | **Gap**: UI Missing

A unified settings interface handling both global application preferences and worktree-specific overrides.
- **UI**: `SettingsPage` accessible from sidebar.
- **Features**:
  - **Global**: Theme (Light/Dark), Default Project Path, Notification preferences.
  - **Worktree**: Justfile location override, MCP server port, Terminal font size.
- **Architecture**: Split state into `GlobalSettings` (persisted in `app.json`) and `WorktreeSettings`.

## 3. Global Command Palette (`Cmd+K`)
**Status**: Not Started

A keyboard-centric interface to navigate the app and execute commands without leaving the keyboard.
- **Scope**: Global (Overlay)
- **Features**:
  - **Navigation**: Jump to Project, Jump to Worktree, Switch View (Tasks/Docker/Env).
  - **Actions**: Run Just task, Start/Stop Docker service, Toggle Theme.
  - **Search**: Filter projects and branches.

## 4. Native MCP Server (Rust)
**Status**: Planned

Embedded Model Context Protocol (MCP) server implemented in Rust (using `axum` or `mcp-rs`), exposing project context to AI tools.
- **Scope**: Worktree (One server per worktree)
- **Tools**:
  - `read_file`: Safe file reading within worktree.
  - `list_tasks`: Get available Just commands.
  - `get_docker_status`: Check container health.
- **Transport**: SSE (Server-Sent Events) for Claude Desktop / Claude Code compatibility.

## 5. MCP Inspector Dashboard
**Status**: Not Started

Visual dashboard to monitor the status and activity of the embedded MCP server.
- **Scope**: Worktree
- **UI**: New tab or section in Settings.
- **Features**:
  - Server Status (Running/Stopped, Port).
  - Connected Clients count.
  - **Traffic Log**: Real-time log of tool calls and responses (great for debugging prompts).

## 6. Prompt Claude Chat UI
**Status**: Not Started

A native chat interface to interact with Claude directly within the app, bypassing the CLI for simple queries.
- **Scope**: Worktree
- **UI**: Split pane or dedicated view.
- **Features**:
  - Streaming message response.
  - Markdown rendering (code blocks).
  - History persistence (session storage).

## 7. Intelligent Context Engine
**Status**: Not Started

Backend system to aggregate and format project context for the "Prompt Claude" workflow.
- **Scope**: Worktree
- **Features**:
  - **Auto-Context**: Current file, open editor tabs, active Docker errors, last failed Task output.
  - **Token Management**: Optimize context to fit window.
  - **Injection**: Dynamically inject context into the Chat UI system prompt.

## 8. Integrated PTY Terminal
**Status**: Not Started

Embedded terminal emulation to run CLI tools without leaving the app.
- **Scope**: Worktree
- **Tech**: `xterm.js` (frontend) + `portable-pty` (Rust backend).
- **Features**:
  - Persisted sessions per worktree.
  - "Run in Terminal" action for Just tasks.
  - clickable links (file paths).

## 9. Notification Center
**Status**: Backend Ready | **Gap**: UI Missing

Centralized history of application events, errors, and background task completions.
- **Scope**: Global
- **UI**: Slide-over drawer or popover.
- **Features**:
  - History of "Env Copy" results.
  - Docker container crash alerts.
  - MCP connection events.
  - "Clear All" and "Dismiss" actions.

## 10. Data Migration System
**Status**: Not Started

Robust versioning system for state persistence to handle schema changes as the app evolves.
- **Scope**: Global
- **Features**:
  - `version` field in `app_state.json`.
  - Migration traits in Rust (`fn migrate(old: Value) -> NewState`).
  - Backup before migration.
  - Critical for preventing data loss during updates.
