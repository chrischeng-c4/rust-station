# Executor Module Contract

**Module**: `rush::executor`
**Purpose**: Execute commands, manage jobs, and handle script execution

---

## Public API

```rust
pub struct CommandExecutor {
    job_manager: JobManager,
    config: Config,
}

impl CommandExecutor {
    /// Create new executor
    pub fn new(config: Config) -> Self;

    /// Execute command in foreground, return exit code
    pub fn execute(&mut self, cmd: &Command) -> Result<i32>;

    /// Execute command in background, return job ID
    pub fn execute_background(&mut self, cmd: &Command) -> Result<usize>;

    /// Execute shell script from file
    pub fn execute_script(&mut self, path: &Path) -> Result<i32>;

    /// Get reference to job manager
    pub fn job_manager(&self) -> &JobManager;

    /// Get mutable reference to job manager
    pub fn job_manager_mut(&mut self) -> &mut JobManager;
}
```

---

## Job Management API

```rust
pub struct JobManager {
    jobs: HashMap<usize, Job>,
    next_job_id: usize,
    rush_pgid: i32,  // Shell's process group ID
}

impl JobManager {
    pub fn new() -> Self;

    /// List all active jobs
    pub fn list(&self) -> Vec<&Job>;

    /// Get job by ID
    pub fn get(&self, job_id: usize) -> Option<&Job>;

    /// Suspend currently running foreground job (Ctrl+Z)
    pub fn suspend_foreground(&mut self) -> Result<()>;

    /// Resume job in foreground
    pub fn foreground(&mut self, job_id: usize) -> Result<i32>;

    /// Resume job in background
    pub fn background(&mut self, job_id: usize) -> Result<()>;

    /// Clean up completed jobs
    pub fn cleanup(&mut self);
}

pub struct Job {
    pub id: usize,
    pub pid: i32,
    pub pgid: i32,
    pub command: String,
    pub state: JobState,
    pub background: bool,
}

pub enum JobState {
    Running,
    Suspended,
    Completed(i32),  // exit code
}
```

---

## Responsibilities

1. **Command Execution**: Spawn processes, capture output, return exit codes
2. **Job Control**: Suspend, resume, background jobs using process groups and signals
3. **Script Execution**: Run shell scripts with command chaining and redirection
4. **Signal Handling**: SIGCHLD to detect job completion, SIGTSTP/SIGCONT for suspend/resume
5. **Process Group Management**: Assign each job to its own process group

---

## Behavior Contracts

### Command Execution

- MUST spawn process with <5ms overhead (constitution requirement)
- MUST stream output to terminal in real-time
- MUST capture exit code accurately
- MUST preserve environment variables
- MUST handle Ctrl+C (SIGINT) - terminate foreground job, not shell

### Job Control

- MUST assign each job to its own process group (for isolation)
- MUST use tcsetpgrp() to control terminal foreground process group
- MUST handle SIGTSTP (Ctrl+Z) - suspend foreground job
- MUST handle SIGCONT - resume suspended job
- MUST track job state transitions accurately

### Script Execution

- MUST support command chaining: `&&`, `||`, `;`
- MUST support output redirection: `>`, `>>`
- MUST execute commands sequentially
- MUST stop on error for `&&` chains
- MUST continue on error for `;` chains

### Signal Handling

- **SIGCHLD**: Detect when jobs complete or state changes
- **SIGTSTP**: Suspend foreground job (not shell itself)
- **SIGINT** (Ctrl+C): Terminate foreground job only
- **SIGCONT**: Resume suspended job

---

## Process Group Strategy

```rust
// macOS job control using process groups

fn spawn_foreground(cmd: &Command) -> Result<i32> {
    let mut child = std::process::Command::new(&cmd.program)
        .args(&cmd.args)
        .spawn()?;

    let pid = Pid::from_raw(child.id() as i32);

    // Put child in its own process group
    setpgid(pid, pid)?;

    // Give terminal control to child's process group
    tcsetpgrp(STDIN_FILENO, pid)?;

    // Wait for child to complete
    let status = child.wait()?;

    // Take back terminal control
    tcsetpgrp(STDIN_FILENO, shell_pgid)?;

    Ok(status.code().unwrap_or(1))
}

fn suspend_foreground(job: &mut Job) -> Result<()> {
    // Send SIGTSTP to job's process group
    signal::kill(Pid::from_raw(-job.pgid), Signal::SIGTSTP)?;

    // Take back terminal control
    tcsetpgrp(STDIN_FILENO, shell_pgid)?;

    job.state = JobState::Suspended;
    Ok(())
}
```

---

## Performance Requirements

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Command spawn | <5ms | Overhead vs direct execution |
| Job state transition | <10ms | Suspend, resume, foreground |
| Script execution | Linear | N commands → N × execution time |

---

## Testing Contract

### Unit Tests

- Command execution with various programs
- Exit code capture (success and failure)
- Background job spawning
- Signal handling (SIGCHLD, SIGTSTP, SIGCONT)

### Integration Tests

- Full job control workflow (spawn → suspend → resume → complete)
- Script execution with chaining operators
- Output redirection
- Multiple concurrent background jobs
- Ctrl+Z in REPL context

---

## Error Cases

- **Program not found**: Return error with helpful message ("command not found")
- **Permission denied**: Return error ("permission denied")
- **Signal delivery failure**: Log error, continue (don't crash shell)
- **Invalid job ID**: Return error ("no such job")
- **No foreground job**: Ctrl+Z does nothing (don't crash)

---

## Example Usage

```rust
use rush::executor::{CommandExecutor, Command};

fn main() -> Result<()> {
    let mut executor = CommandExecutor::new(Config::default());

    // Execute foreground command
    let cmd = Command {
        program: "ls".to_string(),
        args: vec!["-la".to_string()],
        background: false,
        ..Default::default()
    };
    let exit_code = executor.execute(&cmd)?;
    println!("Exit code: {}", exit_code);

    // Execute background command
    let cmd = Command {
        program: "sleep".to_string(),
        args: vec!["30".to_string()],
        background: true,
        ..Default::default()
    };
    let job_id = executor.execute_background(&cmd)?;
    println!("Started job {}", job_id);

    // List jobs
    for job in executor.job_manager().list() {
        println!("[{}] {:?} {}", job.id, job.state, job.command);
    }

    Ok(())
}
```

---

## Platform-Specific Notes

### macOS

- Use `nix` crate for signal handling and process groups
- Process group operations: `setpgid()`, `tcsetpgrp()`
- Signal operations: `kill()` with process group ID (negative PID)
- SIGCHLD delivered when job state changes

### Future (Linux)

- Same process group API (POSIX standard)
- May need different signal handling for job state detection

---

## Dependencies

- `std::process` - Process spawning
- `nix` - Unix system calls (setpgid, tcsetpgrp, kill)
- `signal-hook` - Safe signal handling in Rust

---

## Safety Considerations

- **No unsafe code** except in `nix` crate internals (already audited)
- **Process group isolation** prevents accidental signal delivery to shell
- **Terminal state restoration** ensures shell always responsive after job control
- **Error propagation** prevents panic on system call failures
