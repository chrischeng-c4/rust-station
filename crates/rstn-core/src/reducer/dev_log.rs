use crate::actions::{Action, DevLogSourceData, DevLogTypeData};
use crate::app_state::{AppState, DevLog, DevLogSource, DevLogType};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::AddDevLog { log } => {
            let dev_log = DevLog::new(
                match log.source {
                    DevLogSourceData::Rust => DevLogSource::Rust,
                    DevLogSourceData::Frontend => DevLogSource::Frontend,
                    DevLogSourceData::Claude => DevLogSource::Claude,
                    DevLogSourceData::Ipc => DevLogSource::Ipc,
                },
                match log.log_type {
                    DevLogTypeData::Action => DevLogType::Action,
                    DevLogTypeData::State => DevLogType::State,
                    DevLogTypeData::Claude => DevLogType::Claude,
                    DevLogTypeData::Error => DevLogType::Error,
                    DevLogTypeData::Info => DevLogType::Info,
                },
                log.summary,
                log.data,
            );
            state.add_dev_log(dev_log);
        }

        Action::ClearDevLogs => {
            state.clear_dev_logs();
        }
        _ => {}
    }
}

pub fn log_action_if_interesting(state: &mut AppState, action: &Action) {
    let (action_name, is_interesting) = match action {
        Action::OpenProject { .. } => ("OpenProject", true),
        Action::CloseProject { .. } => ("CloseProject", true),
        Action::SwitchProject { .. } => ("SwitchProject", true),
        Action::AddWorktree { .. } => ("AddWorktree", true),
        Action::AddWorktreeNewBranch { .. } => ("AddWorktreeNewBranch", true),
        Action::RemoveWorktree { .. } => ("RemoveWorktree", true),
        Action::SwitchWorktree { .. } => ("SwitchWorktree", true),
        Action::RefreshWorktrees => ("RefreshWorktrees", true),
        Action::StartMcpServer => ("StartMcpServer", true),
        Action::StopMcpServer => ("StopMcpServer", true),
        Action::SetMcpStatus { .. } => ("SetMcpStatus", true),
        Action::SetMcpError { .. } => ("SetMcpError", true),
        Action::StartDockerService { .. } => ("StartDockerService", true),
        Action::StopDockerService { .. } => ("StopDockerService", true),
        Action::RestartDockerService { .. } => ("RestartDockerService", true),
        Action::SetPortConflict { .. } => ("SetPortConflict", true),
        Action::StartConstitutionWorkflow => ("StartConstitutionWorkflow", true),
        Action::ClearConstitutionWorkflow => ("ClearConstitutionWorkflow", true),
        Action::AnswerConstitutionQuestion { .. } => ("AnswerConstitutionQuestion", true),
        Action::GenerateConstitution => ("GenerateConstitution", true),
        Action::SaveConstitution => ("SaveConstitution", true),
        Action::SetConstitutionError { .. } => ("SetConstitutionError", true),
        Action::CheckConstitutionExists => ("CheckConstitutionExists", true),
        Action::SetConstitutionExists { .. } => ("SetConstitutionExists", true),
        Action::ApplyDefaultConstitution => ("ApplyDefaultConstitution", true),
        Action::ReadConstitution => ("ReadConstitution", true),
        Action::SetConstitutionContent { .. } => ("SetConstitutionContent", true),
        Action::SetClaudeMdExists { .. } => ("SetClaudeMdExists", true),
        Action::ReadClaudeMd => ("ReadClaudeMd", true),
        Action::SetClaudeMdContent { .. } => ("SetClaudeMdContent", true),
        Action::ImportClaudeMd => ("ImportClaudeMd", true),
        Action::SkipClaudeMdImport => ("SkipClaudeMdImport", true),
        Action::SetUseClaudeMdReference { .. } => ("SetUseClaudeMdReference", true),
        _ => ("Other", false),
    };

    if is_interesting {
        let log = DevLog::action(action_name, serde_json::to_value(action).unwrap_or_default());
        state.add_dev_log(log);
    }
}
