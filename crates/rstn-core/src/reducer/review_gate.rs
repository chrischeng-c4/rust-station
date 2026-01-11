use crate::actions::Action;
use crate::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::StartReview { workflow_node_id, content, policy } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    let session_id = uuid::Uuid::new_v4().to_string();
                    let now = chrono::Utc::now().to_rfc3339();

                    let session = crate::app_state::ReviewSession {
                        id: session_id.clone(),
                        workflow_node_id,
                        status: crate::app_state::ReviewStatus::Reviewing,
                        content: crate::app_state::ReviewContent {
                            content_type: match content.content_type {
                                crate::actions::ReviewContentTypeData::Plan => crate::app_state::ReviewContentType::Plan,
                                crate::actions::ReviewContentTypeData::Proposal => crate::app_state::ReviewContentType::Proposal,
                                crate::actions::ReviewContentTypeData::Code => crate::app_state::ReviewContentType::Code,
                                crate::actions::ReviewContentTypeData::Artifact => crate::app_state::ReviewContentType::Artifact,
                            },
                            content: content.content,
                            file_changes: content.file_changes.into_iter().map(|fc| crate::app_state::ReviewFileChange {
                                path: fc.path,
                                action: match fc.action {
                                    crate::actions::ReviewFileActionData::Create => crate::app_state::ReviewFileAction::Create,
                                    crate::actions::ReviewFileActionData::Modify => crate::app_state::ReviewFileAction::Modify,
                                    crate::actions::ReviewFileActionData::Delete => crate::app_state::ReviewFileAction::Delete,
                                },
                                summary: fc.summary,
                            }).collect(),
                        },
                        policy: match policy {
                            crate::actions::ReviewPolicyData::AutoApprove => crate::app_state::ReviewPolicy::AutoApprove,
                            crate::actions::ReviewPolicyData::AgentDecides => crate::app_state::ReviewPolicy::AgentDecides,
                            crate::actions::ReviewPolicyData::AlwaysReview => crate::app_state::ReviewPolicy::AlwaysReview,
                        },
                        comments: vec![],
                        iteration: 1,
                        created_at: now.clone(),
                        updated_at: now,
                    };

                    worktree.tasks.review_gate.sessions.insert(session_id.clone(), session);
                    worktree.tasks.review_gate.active_session_id = Some(session_id);
                }
            }
        }

        Action::AddReviewComment { session_id, target, content } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(session) = worktree.tasks.review_gate.sessions.get_mut(&session_id) {
                        let comment_id = uuid::Uuid::new_v4().to_string();
                        let now = chrono::Utc::now().to_rfc3339();

                        let comment = crate::app_state::ReviewComment {
                            id: comment_id,
                            target: match target {
                                crate::actions::CommentTargetData::Document => crate::app_state::CommentTarget::Document,
                                crate::actions::CommentTargetData::Section { id } => crate::app_state::CommentTarget::Section { id },
                                crate::actions::CommentTargetData::File { path } => crate::app_state::CommentTarget::File { path },
                            },
                            content,
                            author: crate::app_state::CommentAuthor::User,
                            resolved: false,
                            created_at: now.clone(),
                        };

                        session.comments.push(comment);
                        session.updated_at = now;
                    }
                }
            }
        }

        Action::ResolveReviewComment { session_id, comment_id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(session) = worktree.tasks.review_gate.sessions.get_mut(&session_id) {
                        if let Some(comment) = session.comments.iter_mut().find(|c| c.id == comment_id) {
                            comment.resolved = true;
                            session.updated_at = chrono::Utc::now().to_rfc3339();
                        }
                    }
                }
            }
        }

        Action::SubmitReviewFeedback { session_id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(session) = worktree.tasks.review_gate.sessions.get_mut(&session_id) {
                        session.status = crate::app_state::ReviewStatus::Iterating;
                        session.updated_at = chrono::Utc::now().to_rfc3339();
                    }
                    worktree.tasks.review_gate.is_loading = true;
                }
            }
        }

        Action::ApproveReview { session_id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(session) = worktree.tasks.review_gate.sessions.get_mut(&session_id) {
                        session.status = crate::app_state::ReviewStatus::Approved;
                        session.updated_at = chrono::Utc::now().to_rfc3339();
                    }
                }
            }
        }

        Action::RejectReview { session_id, reason } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(session) = worktree.tasks.review_gate.sessions.get_mut(&session_id) {
                        session.status = crate::app_state::ReviewStatus::Rejected;
                        session.updated_at = chrono::Utc::now().to_rfc3339();
                        let comment = crate::app_state::ReviewComment {
                            id: uuid::Uuid::new_v4().to_string(),
                            target: crate::app_state::CommentTarget::Document,
                            content: format!("Rejected: {}", reason),
                            author: crate::app_state::CommentAuthor::System,
                            resolved: false,
                            created_at: chrono::Utc::now().to_rfc3339(),
                        };
                        session.comments.push(comment);
                    }
                }
            }
        }

        Action::UpdateReviewContent { session_id, content } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(session) = worktree.tasks.review_gate.sessions.get_mut(&session_id) {
                        session.content = crate::app_state::ReviewContent {
                            content_type: match content.content_type {
                                crate::actions::ReviewContentTypeData::Plan => crate::app_state::ReviewContentType::Plan,
                                crate::actions::ReviewContentTypeData::Proposal => crate::app_state::ReviewContentType::Proposal,
                                crate::actions::ReviewContentTypeData::Code => crate::app_state::ReviewContentType::Code,
                                crate::actions::ReviewContentTypeData::Artifact => crate::app_state::ReviewContentType::Artifact,
                            },
                            content: content.content,
                            file_changes: content.file_changes.into_iter().map(|fc| crate::app_state::ReviewFileChange {
                                path: fc.path,
                                action: match fc.action {
                                    crate::actions::ReviewFileActionData::Create => crate::app_state::ReviewFileAction::Create,
                                    crate::actions::ReviewFileActionData::Modify => crate::app_state::ReviewFileAction::Modify,
                                    crate::actions::ReviewFileActionData::Delete => crate::app_state::ReviewFileAction::Delete,
                                },
                                summary: fc.summary,
                            }).collect(),
                        };
                        session.iteration += 1;
                        session.status = crate::app_state::ReviewStatus::Reviewing;
                        session.updated_at = chrono::Utc::now().to_rfc3339();
                    }
                    worktree.tasks.review_gate.is_loading = false;
                }
            }
        }

        Action::SetReviewStatus { session_id, status } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(session) = worktree.tasks.review_gate.sessions.get_mut(&session_id) {
                        session.status = match status {
                            crate::actions::ReviewStatusData::Pending => crate::app_state::ReviewStatus::Pending,
                            crate::actions::ReviewStatusData::Reviewing => crate::app_state::ReviewStatus::Reviewing,
                            crate::actions::ReviewStatusData::Iterating => crate::app_state::ReviewStatus::Iterating,
                            crate::actions::ReviewStatusData::Approved => crate::app_state::ReviewStatus::Approved,
                            crate::actions::ReviewStatusData::Rejected => crate::app_state::ReviewStatus::Rejected,
                        };
                        session.updated_at = chrono::Utc::now().to_rfc3339();
                    }
                }
            }
        }

        Action::SetReviewGateLoading { is_loading } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.review_gate.is_loading = is_loading;
                }
            }
        }

        Action::SetReviewGateError { error } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.review_gate.error = error;
                    worktree.tasks.review_gate.is_loading = false;
                }
            }
        }

        Action::SetActiveReviewSession { session_id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.review_gate.active_session_id = session_id;
                }
            }
        }

        Action::ClearReviewSession { session_id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.review_gate.sessions.remove(&session_id);
                    if worktree.tasks.review_gate.active_session_id == Some(session_id) {
                        worktree.tasks.review_gate.active_session_id = None;
                    }
                }
            }
        }
        _ => {}
    }
}
