//! 'jobs' built-in command
//!
//! Lists active jobs with their ID, status, and command string.

use crate::error::Result;
use crate::executor::execute::CommandExecutor;

/// Execute the 'jobs' command
pub fn execute(executor: &mut CommandExecutor, _args: &[String]) -> Result<i32> {
    // Update status first to ensure we show current state
    executor.check_background_jobs();

    let manager = executor.job_manager_mut();
    let mut jobs: Vec<_> = manager.jobs().collect();

    // Sort by ID for consistent output
    jobs.sort_by_key(|j| j.id);

    for job in jobs {
        println!("[{}] {} {}", job.id, job.status, job.command);
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::execute::CommandExecutor;

    #[test]
    fn test_jobs_command() {
        let mut executor = CommandExecutor::new();
        // Just verify it runs without error (hard to mock active jobs without running processes)
        let result = execute(&mut executor, &[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_jobs_with_active_jobs() {
        use crate::executor::job::JobStatus;
        use nix::unistd::Pid;

        let mut executor = CommandExecutor::new();

        // Add multiple jobs
        let manager = executor.job_manager_mut();
        let id1 = manager.add_job(
            Pid::from_raw(1234),
            "sleep 100".to_string(),
            vec![Pid::from_raw(1234)],
        );
        let id2 = manager.add_job(
            Pid::from_raw(5678),
            "cat file".to_string(),
            vec![Pid::from_raw(5678)],
        );

        // Set different statuses
        manager.get_job_mut(id1).unwrap().status = JobStatus::Running;
        manager.get_job_mut(id2).unwrap().status = JobStatus::Stopped;

        // jobs command should list them
        let result = execute(&mut executor, &[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_jobs_prints_active_jobs_directly() {
        use std::process::Command;
        use std::time::Duration;
        use std::thread;

        // Spawn a long-running child process that we actually parent
        // This process will still be alive when we call execute()
        let child = Command::new("sleep")
            .arg("5")
            .spawn()
            .expect("Failed to spawn sleep process");

        let child_pid = child.id() as i32;

        let mut executor = CommandExecutor::new();
        let manager = executor.job_manager_mut();

        let _id = manager.add_job(
            nix::unistd::Pid::from_raw(child_pid),
            "sleep 5".to_string(),
            vec![nix::unistd::Pid::from_raw(child_pid)],
        );

        // Give the process a moment to ensure it's running
        thread::sleep(Duration::from_millis(10));

        // Now call execute() which will:
        // 1. Call check_background_jobs() (updates status)
        // 2. List jobs (the sleep process should still be running)
        // 3. Print each job (executes line 20)
        let result = execute(&mut executor, &[]);

        // Verify the command ran successfully
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }
}
