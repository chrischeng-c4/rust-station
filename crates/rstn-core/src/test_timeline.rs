//! Test execution timeline tracking
//!
//! Provides tools for recording and visualizing test execution flow.
//! Useful for debugging test failures and understanding execution order.

use std::sync::Mutex;
use std::time::Instant;

/// A single event in the test execution timeline
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    /// Elapsed seconds since timeline start
    pub timestamp: f64,
    /// Event level (STATE, RENDER, EVENT, ASSERT, etc.)
    pub level: String,
    /// Human-readable message
    pub message: String,
    /// Optional structured data
    pub data: Option<serde_json::Value>,
}

/// Records a chronological timeline of test execution events
///
/// # Examples
///
/// ```
/// use rstn_core::test_timeline::TestTimeline;
///
/// let timeline = TestTimeline::new();
///
/// timeline.record("STATE", "Creating state", None);
/// timeline.record("DATA", "Loading data", Some(serde_json::json!({"count": 5})));
/// timeline.record("ASSERT", "Verifying result", None);
///
/// timeline.print_timeline();
/// ```
pub struct TestTimeline {
    start_time: Instant,
    events: Mutex<Vec<TimelineEvent>>,
}

impl TestTimeline {
    /// Create a new timeline starting now
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            events: Mutex::new(Vec::new()),
        }
    }

    /// Record a new event in the timeline
    ///
    /// # Arguments
    ///
    /// * `level` - Event level (e.g., "STATE", "RENDER", "EVENT", "ASSERT")
    /// * `message` - Human-readable description
    /// * `data` - Optional structured data (will be pretty-printed as JSON)
    pub fn record(
        &self,
        level: &str,
        message: impl Into<String>,
        data: Option<serde_json::Value>,
    ) {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let event = TimelineEvent {
            timestamp: elapsed,
            level: level.to_string(),
            message: message.into(),
            data,
        };
        self.events.lock().unwrap().push(event);
    }

    /// Print the timeline to stdout in a readable format
    ///
    /// Output format:
    /// ```text
    /// 📊 Test Execution Timeline:
    ///   Time |  Level   | Message
    /// -------|----------|----------
    ///  0.000s |  STATE   | Creating state
    ///  0.005s |  DATA    | Loading data
    ///         |          | Data: {"count": 5}
    ///  0.010s |  ASSERT  | Verifying result
    /// ```
    pub fn print_timeline(&self) {
        println!("\n📊 Test Execution Timeline:");
        println!("{:>6} | {:^8} | {}", "Time", "Level", "Message");
        println!("{:-<60}", "");

        for event in self.events.lock().unwrap().iter() {
            println!(
                "{:>6.3}s | {:^8} | {}",
                event.timestamp, event.level, event.message
            );
            if let Some(data) = &event.data {
                if let Ok(pretty) = serde_json::to_string_pretty(data) {
                    println!("       |          | Data: {}", pretty);
                } else {
                    println!("       |          | Data: {:?}", data);
                }
            }
        }
        println!();
    }

    /// Get all events as a Vec
    pub fn events(&self) -> Vec<TimelineEvent> {
        self.events.lock().unwrap().clone()
    }

    /// Export timeline as JSON
    pub fn to_json(&self) -> serde_json::Value {
        let events: Vec<_> = self
            .events
            .lock()
            .unwrap()
            .iter()
            .map(|e| {
                serde_json::json!({
                    "timestamp": e.timestamp,
                    "level": e.level,
                    "message": e.message,
                    "data": e.data,
                })
            })
            .collect();

        serde_json::json!({
            "start_time": format!("{:?}", self.start_time),
            "events": events,
        })
    }

    /// Get the total duration of the timeline
    pub fn duration(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// Find events by level
    pub fn filter_by_level(&self, level: &str) -> Vec<TimelineEvent> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.level == level)
            .cloned()
            .collect()
    }

    /// Get a summary string
    pub fn summary(&self) -> String {
        let events = self.events.lock().unwrap();
        let duration = self.duration();
        format!(
            "Timeline: {} events in {:.3}s",
            events.len(),
            duration
        )
    }
}

impl Default for TestTimeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timeline_basic() {
        let timeline = TestTimeline::new();

        timeline.record("STATE", "Creating state", None);
        timeline.record("DATA", "Loading data", None);
        timeline.record("ASSERT", "Verifying", None);

        let events = timeline.events();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].level, "STATE");
        assert_eq!(events[1].level, "DATA");
        assert_eq!(events[2].level, "ASSERT");
    }

    #[test]
    fn test_timeline_with_data() {
        let timeline = TestTimeline::new();

        timeline.record(
            "DATA",
            "Test data",
            Some(serde_json::json!({"count": 42, "status": "ok"})),
        );

        let events = timeline.events();
        assert_eq!(events.len(), 1);
        assert!(events[0].data.is_some());
    }

    #[test]
    fn test_timeline_timestamps() {
        let timeline = TestTimeline::new();

        timeline.record("EVENT1", "First", None);
        thread::sleep(Duration::from_millis(10));
        timeline.record("EVENT2", "Second", None);

        let events = timeline.events();
        assert_eq!(events.len(), 2);
        assert!(events[1].timestamp > events[0].timestamp);
        assert!(events[1].timestamp >= 0.01); // At least 10ms
    }

    #[test]
    fn test_filter_by_level() {
        let timeline = TestTimeline::new();

        timeline.record("STATE", "State 1", None);
        timeline.record("ASSERT", "Assert 1", None);
        timeline.record("STATE", "State 2", None);
        timeline.record("ASSERT", "Assert 2", None);

        let state_events = timeline.filter_by_level("STATE");
        let assert_events = timeline.filter_by_level("ASSERT");

        assert_eq!(state_events.len(), 2);
        assert_eq!(assert_events.len(), 2);
    }

    #[test]
    fn test_to_json() {
        let timeline = TestTimeline::new();

        timeline.record("TEST", "Test event", Some(serde_json::json!({"key": "value"})));

        let json = timeline.to_json();
        assert!(json["events"].is_array());
        assert_eq!(json["events"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_duration() {
        let timeline = TestTimeline::new();

        thread::sleep(Duration::from_millis(10));
        timeline.record("EVENT", "Test", None);

        let duration = timeline.duration();
        assert!(duration >= 0.01); // At least 10ms
    }

    #[test]
    fn test_summary() {
        let timeline = TestTimeline::new();

        timeline.record("EVENT", "Event 1", None);
        timeline.record("EVENT", "Event 2", None);

        let summary = timeline.summary();
        assert!(summary.contains("2 events"));
    }

    #[test]
    fn test_print_timeline() {
        let timeline = TestTimeline::new();

        timeline.record("STATE", "Test state", None);
        timeline.record(
            "DATA",
            "Test data",
            Some(serde_json::json!({"count": 5})),
        );

        // Should not panic
        timeline.print_timeline();
    }
}
