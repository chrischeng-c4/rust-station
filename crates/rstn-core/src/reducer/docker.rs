use crate::actions::Action;
use crate::app_state::{AppState, ServiceStatus, PendingConflict};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::CheckDockerAvailability => {
            state.docker.is_loading = true;
        }

        Action::SetDockerAvailable { available } => {
            state.docker.docker_available = Some(available);
            state.docker.is_loading = false;
        }

        Action::RefreshDockerServices => {
            state.docker.is_loading = true;
        }

        Action::SetDockerServices { services } => {
            state.docker.services = services.into_iter().map(|s| s.into()).collect();
            state.docker.is_loading = false;
        }

        Action::StartDockerService { service_id } => {
            if let Some(service) = state
                .docker
                .services
                .iter_mut()
                .find(|s| s.id == service_id)
            {
                service.status = ServiceStatus::Starting;
            }
        }

        Action::StopDockerService { service_id } => {
            if let Some(service) = state
                .docker
                .services
                .iter_mut()
                .find(|s| s.id == service_id)
            {
                service.status = ServiceStatus::Stopping;
            }
        }

        Action::RestartDockerService { service_id } => {
            if let Some(service) = state
                .docker
                .services
                .iter_mut()
                .find(|s| s.id == service_id)
            {
                service.status = ServiceStatus::Starting;
            }
        }

        Action::SelectDockerService { service_id } => {
            state.docker.selected_service_id = service_id;
            state.docker.logs.clear();
        }

        Action::FetchDockerLogs { .. } => {
            state.docker.is_loading_logs = true;
        }

        Action::SetDockerLogs { logs } => {
            state.docker.logs = logs;
            state.docker.is_loading_logs = false;
        }

        Action::CreateDatabase { .. } | Action::CreateVhost { .. } => {
            // Async triggers
        }

        Action::SetDockerConnectionString { connection_string } => {
            state.docker.last_connection_string = connection_string;
        }

        Action::SetPortConflict { service_id, conflict } => {
            state.docker.pending_conflict = Some(PendingConflict {
                service_id,
                conflict: conflict.into(),
            });
        }

        Action::ClearPortConflict => {
            state.docker.pending_conflict = None;
        }

        Action::StartDockerServiceWithPort { ref service_id, port } => {
            state.docker.port_overrides.insert(service_id.clone(), port);
            state.docker.pending_conflict = None;
            if let Some(service) = state
                .docker
                .services
                .iter_mut()
                .find(|s| s.id == *service_id)
            {
                service.status = ServiceStatus::Starting;
            }
        }

        Action::ResolveConflictByStoppingContainer { ref service_id, .. } => {
            state.docker.pending_conflict = None;
            if let Some(service) = state
                .docker
                .services
                .iter_mut()
                .find(|s| s.id == *service_id)
            {
                service.status = ServiceStatus::Starting;
            }
        }

        Action::SetDockerLoading { is_loading } => {
            state.docker.is_loading = is_loading;
        }

        Action::SetDockerLogsLoading { is_loading } => {
            state.docker.is_loading_logs = is_loading;
        }
        _ => {}
    }
}
