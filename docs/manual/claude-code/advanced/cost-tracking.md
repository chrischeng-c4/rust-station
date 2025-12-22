---
title: "Real-time Cost Tracking"
description: "Monitor cumulative API costs during Claude sessions"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
tags: [claude-code, cost, budget, monitoring]
weight: 9
---

## 8. Real-time Cost Tracking

**Status**: ✅ IMPLEMENTED (Phase 3)

### Implementation Details

**File**: [crates/rstn/src/tui/views/session_output.rs:28-31](../../crates/rstn/src/tui/views/session_output.rs#L28-L31)

**SessionOutputView State**:
```rust
pub struct SessionOutputView {
    // ... existing fields ...

    /// Cumulative cost in USD (updated in real-time)
    cumulative_cost_usd: f64,  // ✅ IMPLEMENTED

    /// Budget warning threshold in USD (warn if exceeded)
    budget_warning_threshold: f64,  // ✅ IMPLEMENTED (default: $0.50)
}
```

**Real-time Update** ([session_output.rs:85-89](../../crates/rstn/src/tui/views/session_output.rs#L85-L89)):
```rust
pub fn add_message(&mut self, message: &ClaudeStreamMessage) {
    // Update cumulative cost if available (real-time tracking)
    if let Some(cost) = message.total_cost_usd {
        self.cumulative_cost_usd = cost;
    }
    // ... rest of message handling ...
}
```

**Status Line Display** ([session_output.rs:251-309](../../crates/rstn/src/tui/views/session_output.rs#L251-L309)):
```rust
pub fn status_line(&self) -> String {
    // Budget warning indicator
    let budget_warning = if self.cumulative_cost_usd > self.budget_warning_threshold {
        "⚠️ "
    } else {
        ""
    };

    match &self.completion_status {
        Some(CompletionStatus::Complete { turns, duration_secs }) => {
            format!(
                "✓ Complete ({} turn{}, {}s, {}${:.4})",
                turns,
                if *turns == 1 { "" } else { "s" },
                duration_secs,
                budget_warning,
                self.cumulative_cost_usd
            )
        }
        None if self.start_time.is_some() => {
            let duration = self.start_time.unwrap().elapsed().as_secs();
            format!(
                "🤖 Running... (Turn {}/{}, {}s, {}${:.4})",
                self.current_turn.max(1),
                self.max_turns,
                duration,
                budget_warning,
                self.cumulative_cost_usd
            )
        }
        // ... other states ...
    }
}
```

**Budget Configuration** ([session_output.rs:55-67](../../crates/rstn/src/tui/views/session_output.rs#L55-L67)):
```rust
/// Create a new session with custom budget threshold
pub fn with_budget(max_turns: usize, budget_threshold: f64) -> Self {
    Self {
        budget_warning_threshold: budget_threshold,
        // ... other fields ...
    }
}
```

**Tests** ([session_output.rs:423-545](../../crates/rstn/src/tui/views/session_output.rs#L423-L545)):
```rust
#[test]
fn test_cost_tracking_real_time() {
    let mut view = SessionOutputView::new(5);
    view.start_session("Test prompt", 5);

    let msg = ClaudeStreamMessage {
        total_cost_usd: Some(0.025),
        // ...
    };
    view.add_message(&msg);
    assert_eq!(view.cumulative_cost_usd, 0.025);

    let status = view.status_line();
    assert!(status.contains("$0.0250"));
}

#[test]
fn test_budget_warning() {
    let mut view = SessionOutputView::with_budget(5, 0.01);  // Low threshold

    let msg = ClaudeStreamMessage {
        total_cost_usd: Some(0.025),  // Above threshold
        // ...
    };
    view.add_message(&msg);

    let status = view.status_line();
    assert!(status.contains("⚠️"));  // Budget warning shown
}
```

### Cumulative Cost Tracking (Dual-Layer)

**File**: [crates/rstn/src/session_manager.rs:58-67](../../crates/rstn/src/session_manager.rs#L58-67)

Track cost across multiple Claude sessions within an rstn workflow:

```rust
pub struct RstnSession {
    // ... existing fields ...

    /// Total cost across all Claude sessions in this workflow
    pub total_cost_usd: f64,  // ✅ IMPLEMENTED
}

impl SessionManager {
    pub fn complete_claude_session(&mut self, ...) -> Result<()> {
        // ... existing logic ...

        // Update rstn session total cost (already implemented)
        // This aggregates costs from all Claude sessions in the workflow
        Ok(())
    }
}
```

### Budget Warnings

**File**: `crates/rstn/src/tui/app.rs`

```rust
const BUDGET_WARNING_THRESHOLD: f64 = 1.0;  // $1.00
const BUDGET_CRITICAL_THRESHOLD: f64 = 5.0; // $5.00

fn check_budget_warning(&self, session: &RstnSession) {
    if session.total_cost_usd > BUDGET_CRITICAL_THRESHOLD {
        self.show_warning(format!(
            "⚠️  Session cost ${:.2} exceeds critical threshold!",
            session.total_cost_usd
        ));
    } else if session.total_cost_usd > BUDGET_WARNING_THRESHOLD {
        self.show_info(format!(
            "ℹ️  Session cost: ${:.2}",
            session.total_cost_usd
        ));
    }
}
```

---

