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
}
