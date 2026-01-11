use crate::actions::{Action, McpLogDirectionData};
use crate::app_state::{AppState, McpStatus};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::StartMcpServer => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.status = McpStatus::Starting;
                    worktree.mcp.error = None;
                }
            }
        }

        Action::StopMcpServer => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.status = McpStatus::Stopped;
                    worktree.mcp.port = None;
                }
            }
        }

        Action::SetMcpStatus { status } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.status = status.into();
                }
            }
        }

        Action::SetMcpPort { port } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.port = Some(port);
                    worktree.mcp.status = McpStatus::Running;
                }
            }
        }

        Action::SetMcpConfigPath { path } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.config_path = Some(path);
                }
            }
        }

        Action::SetMcpError { error } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.status = McpStatus::Error;
                    worktree.mcp.error = Some(error);
                }
            }
        }

        Action::AddMcpLogEntry { entry } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    let log_entry = crate::app_state::McpLogEntry {
                        timestamp: entry.timestamp,
                        direction: match entry.direction {
                            McpLogDirectionData::In => crate::app_state::McpLogDirection::In,
                            McpLogDirectionData::Out => crate::app_state::McpLogDirection::Out,
                        },
                        method: entry.method,
                        tool_name: entry.tool_name,
                        payload: entry.payload,
                        is_error: entry.is_error,
                    };
                    worktree.mcp.add_log_entry(log_entry);
                }
            }
        }

        Action::ClearMcpLogs => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.clear_logs();
                }
            }
        }

        Action::UpdateMcpTools { tools } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.mcp.available_tools = tools
                        .into_iter()
                        .map(|t| crate::app_state::McpTool {
                            name: t.name,
                            description: t.description,
                            input_schema: t.input_schema,
                        })
                        .collect();
                }
            }
        }
        _ => {}
    }
}
