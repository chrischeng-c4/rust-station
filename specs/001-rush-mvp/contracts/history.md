# History Module Contract

**Module**: `rush::history`
**Purpose**: Command history storage, retrieval, and persistence

---

## Public API

```rust
pub struct HistoryManager {
    entries: VecDeque<HistoryEntry>,
    file_path: PathBuf,
    max_entries: usize,
}

impl HistoryManager {
    /// Create new history manager with default path
    /// Path: ~/.config/rush/history
    pub fn new() -> Result<Self>;

    /// Create history manager with custom path
    pub fn with_path(path: PathBuf) -> Result<Self>;

    /// Load history from disk
    pub fn load(&mut self) -> Result<()>;

    /// Append command to history (in-memory and disk)
    pub fn append(&mut self, command: String) -> Result<()>;

    /// Get history entry by index (0 = oldest)
    pub fn get(&self, index: usize) -> Option<&str>;

    /// Get most recent N entries
    pub fn recent(&self, n: usize) -> Vec<&str>;

    /// Search history for commands matching prefix
    pub fn search_prefix(&self, prefix: &str) -> Vec<&str>;

    /// Get total number of history entries
    pub fn len(&self) -> usize;

    /// Clear all history (in-memory and disk)
    pub fn clear(&mut self) -> Result<()>;
}

pub struct HistoryEntry {
    pub command: String,
    pub timestamp: u64,
    pub exit_code: Option<i32>,
    pub working_dir: PathBuf,
}
```

---

## Responsibilities

1. **Persistence**: Save commands to disk atomically (crash-safe)
2. **Loading**: Read history from disk at startup
3. **Appending**: Add new commands efficiently
4. **Searching**: Find commands matching prefix for autosuggestions
5. **Cleanup**: Enforce max_entries limit, remove oldest when exceeded

---

## Storage Format

```
# ~/.config/rush/history
# Plain text, one command per line, newest at bottom

echo "hello world"
git status
cargo build --release
ls -la /etc
```

**Future**: May extend to include metadata (timestamp, exit code, working directory)

---

## Behavior Contracts

### Persistence

- MUST use atomic writes (temp file + rename) for crash safety
- MUST append synchronously after each command (<5ms target)
- MUST create parent directory if it doesn't exist
- MUST handle concurrent access (multiple rush instances)

### Loading

- MUST load history asynchronously to avoid blocking startup
- MUST handle corrupted history file (skip invalid lines, log warning)
- MUST handle missing history file (start with empty history)
- MUST enforce max_entries limit when loading

### Searching

- MUST return results in reverse chronological order (newest first)
- MUST handle empty prefix (return all entries)
- MUST be case-sensitive (shell commands are case-sensitive)
- MUST complete search in <10ms for typical history size (10,000 entries)

### Memory Management

- MUST enforce max_entries limit (default: 10,000)
- MUST remove oldest entries when limit exceeded
- MUST keep in-memory representation synchronized with disk

---

## Performance Requirements

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Load from disk | <50ms | For 10,000 entries |
| Append to disk | <5ms | Synchronous write |
| Search prefix | <10ms | For 10,000 entries |
| Memory usage | <1MB | For 10,000 entries |

---

## Testing Contract

### Unit Tests

- Append and retrieve entries
- Persistence across load/save cycles
- Prefix search with various patterns
- Max entries enforcement
- Corrupted file handling

### Integration Tests

- Multiple processes appending concurrently
- History persistence across shell restarts
- Large history files (100,000+ entries)

---

## Error Cases

- **File unreadable**: Log warning, return empty history
- **File corrupted**: Skip invalid lines, load valid entries
- **Directory not writable**: Log error, continue in-memory only
- **Disk full**: Log error, continue in-memory (don't crash)

---

## Example Usage

```rust
use rush::history::HistoryManager;

fn main() -> Result<()> {
    let mut history = HistoryManager::new()?;
    history.load()?;

    // Add commands
    history.append("ls -la".to_string())?;
    history.append("git status".to_string())?;

    // Search for autosuggestions
    let matches = history.search_prefix("gi");
    assert_eq!(matches, vec!["git status"]);

    Ok(())
}
```

---

## Thread Safety

- `HistoryManager` is NOT thread-safe by default
- REPL wraps it in `Arc<Mutex<>>` for safe access
- File locking NOT implemented in MVP (future enhancement)
