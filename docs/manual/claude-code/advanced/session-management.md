---
title: "Dual-Layer Session Management"
description: "Track both Claude sessions (LLM) and rstn sessions (workflow)"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "080"
tags: [claude-code, session, sqlite, workflow]
weight: 6
---

## 5. Dual-Layer Session Management

**Status**: ✅ IMPLEMENTED (Phase 1)

### Problem Statement

**User Insight**: "看來我們的狀態除了rstn的session還需要管理claude code的session"

rstn needs to track:
1. **Claude sessions** (UUID from Claude Code) - individual LLM interactions
2. **rstn sessions** (workflow-level) - Prompt, Specify, Plan, Tasks

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│ rstn Session (Workflow Level)                          │
│ ID: rstn-sess-20251221-001                             │
│ Type: Specify                                           │
│ Feature: 082-dark-mode                                  │
│                                                         │
│   ┌───────────────────────────────────────────────┐   │
│   │ Claude Session 1 (Generate Spec)              │   │
│   │ UUID: claude-uuid-abc123                      │   │
│   │ Turns: 3                                       │   │
│   │ Status: Completed                             │   │
│   └───────────────────────────────────────────────┘   │
│                                                         │
│   ┌───────────────────────────────────────────────┐   │
│   │ Claude Session 2 (Clarify Requirements)      │   │
│   │ UUID: claude-uuid-def456                      │   │
│   │ Turns: 1                                       │   │
│   │ Status: Running                               │   │
│   └───────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Data Structures

**File**: `crates/rstn/src/session_manager.rs` (new/updated)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Claude session (from Claude Code)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeSession {
    /// UUID from Claude Code (session_id in stream-json)
    pub uuid: String,

    /// What this Claude session is for
    pub purpose: String,  // "Generate Spec", "Clarify", "Implement Task"

    /// Session metadata
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Session outcome
    pub status: ClaudeSessionStatus,
    pub turns_used: usize,
    pub max_turns: usize,
    pub total_cost_usd: Option<f64>,

    /// Link to parent rstn session
    pub rstn_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaudeSessionStatus {
    Running,
    Completed,
    MaxTurns,
    Error { message: String },
}

/// rstn session (workflow level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RstnSession {
    /// rstn-generated ID
    pub id: String,  // "rstn-sess-20251221-001"

    /// Workflow type
    pub workflow: WorkflowType,

    /// Feature being worked on
    pub feature_number: Option<String>,
    pub feature_name: Option<String>,

    /// Session metadata
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// All Claude sessions in this workflow
    pub claude_sessions: HashMap<String, ClaudeSession>,

    /// Current active Claude session
    pub active_claude_session: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowType {
    Prompt,
    Specify,
    Plan,
    Tasks,
    Implement,
}

/// Session manager
#[derive(Debug, Default)]
pub struct SessionManager {
    /// All rstn sessions (keyed by rstn session ID)
    sessions: HashMap<String, RstnSession>,

    /// Current active rstn session
    current_session: Option<String>,
}

impl SessionManager {
    pub fn start_rstn_session(&mut self, workflow: WorkflowType) -> String {
        let session_id = format!(
            "rstn-sess-{}-{:03}",
            chrono::Utc::now().format("%Y%m%d"),
            self.sessions.len() + 1
        );

        let session = RstnSession {
            id: session_id.clone(),
            workflow,
            feature_number: None,
            feature_name: None,
            started_at: chrono::Utc::now(),
            completed_at: None,
            claude_sessions: HashMap::new(),
            active_claude_session: None,
        };

        self.sessions.insert(session_id.clone(), session);
        self.current_session = Some(session_id.clone());

        session_id
    }

    pub fn start_claude_session(
        &mut self,
        rstn_session_id: &str,
        uuid: String,
        purpose: String,
        max_turns: usize,
    ) -> Result<()> {
        let session = self.sessions.get_mut(rstn_session_id)
            .ok_or_else(|| anyhow::anyhow!("rstn session not found"))?;

        let claude_session = ClaudeSession {
            uuid: uuid.clone(),
            purpose,
            started_at: chrono::Utc::now(),
            completed_at: None,
            status: ClaudeSessionStatus::Running,
            turns_used: 0,
            max_turns,
            total_cost_usd: None,
            rstn_session_id: rstn_session_id.to_string(),
        };

        session.claude_sessions.insert(uuid.clone(), claude_session);
        session.active_claude_session = Some(uuid);

        Ok(())
    }

    pub fn complete_claude_session(
        &mut self,
        rstn_session_id: &str,
        claude_uuid: &str,
        status: ClaudeSessionStatus,
        turns_used: usize,
        total_cost_usd: Option<f64>,
    ) -> Result<()> {
        let session = self.sessions.get_mut(rstn_session_id)
            .ok_or_else(|| anyhow::anyhow!("rstn session not found"))?;

        if let Some(claude_session) = session.claude_sessions.get_mut(claude_uuid) {
            claude_session.completed_at = Some(chrono::Utc::now());
            claude_session.status = status;
            claude_session.turns_used = turns_used;
            claude_session.total_cost_usd = total_cost_usd;
        }

        Ok(())
    }

    /// Get session history for display
    pub fn get_session_history(&self, rstn_session_id: &str) -> Option<Vec<ClaudeSession>> {
        self.sessions.get(rstn_session_id).map(|s| {
            let mut sessions: Vec<_> = s.claude_sessions.values().cloned().collect();
            sessions.sort_by_key(|s| s.started_at);
            sessions
        })
    }
}
```

### Persistence

Store session data in SQLite database:

**File**: `crates/rstn/src/db/schema.sql`

```sql
CREATE TABLE rstn_sessions (
    id TEXT PRIMARY KEY,
    workflow TEXT NOT NULL,
    feature_number TEXT,
    feature_name TEXT,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    metadata TEXT  -- JSON blob
);

CREATE TABLE claude_sessions (
    uuid TEXT PRIMARY KEY,
    rstn_session_id TEXT NOT NULL,
    purpose TEXT NOT NULL,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    status TEXT NOT NULL,
    turns_used INTEGER,
    max_turns INTEGER,
    total_cost_usd REAL,
    FOREIGN KEY (rstn_session_id) REFERENCES rstn_sessions(id)
);
```

### Integration Example

**File**: `crates/rstn/src/commands/prompt.rs`

```rust
pub async fn execute_prompt_cli(args: PromptArgs) -> Result<()> {
    let mut session_mgr = SessionManager::load_from_db().await?;

    // Start rstn session
    let rstn_session_id = session_mgr.start_rstn_session(WorkflowType::Prompt);

    // Start Claude session (UUID comes from "init" message)
    let claude_result = run_claude_command_streaming(&message, &options, |msg| {
        if msg.msg_type == "init" {
            if let Some(session_id) = &msg.session_id {
                session_mgr.start_claude_session(
                    &rstn_session_id,
                    session_id.clone(),
                    "User Prompt".to_string(),
                    options.max_turns.unwrap_or(10),
                )?;
            }
        }
        // ... handle other messages ...
        Ok(())
    }).await?;

    // Complete Claude session
    if let Some(session_id) = claude_result.session_id {
        session_mgr.complete_claude_session(
            &rstn_session_id,
            &session_id,
            ClaudeSessionStatus::Completed,
            claude_result.turns_used,
            claude_result.total_cost_usd,
        )?;
    }

    // Persist to DB
    session_mgr.save_to_db().await?;

    Ok(())
}
```

---

