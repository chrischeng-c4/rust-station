use crate::actions::Action;
use crate::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::StartConstitutionWorkflow => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    let use_claude_md = worktree.tasks.claude_md_exists.unwrap_or(false);
                    worktree.tasks.constitution_workflow = Some(crate::app_state::ConstitutionWorkflow {
                        current_question: 0,
                        answers: std::collections::HashMap::new(),
                        output: String::new(),
                        status: crate::app_state::WorkflowStatus::Collecting,
                        use_claude_md_reference: use_claude_md,
                        error: None,
                    });
                }
            }
        }

        Action::ClearConstitutionWorkflow => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.constitution_workflow = None;
                }
            }
        }

        Action::AnswerConstitutionQuestion { answer } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(workflow) = &mut worktree.tasks.constitution_workflow {
                        const QUESTIONS: &[&str] = &["tech_stack", "security", "code_quality", "architecture"];
                        if workflow.current_question < QUESTIONS.len() {
                            let key = QUESTIONS[workflow.current_question];
                            workflow.answers.insert(key.to_string(), answer);
                            workflow.current_question += 1;
                        }
                    }
                }
            }
        }

        Action::GenerateConstitution => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(workflow) = &mut worktree.tasks.constitution_workflow {
                        workflow.status = crate::app_state::WorkflowStatus::Generating;
                        workflow.output.clear();
                    }
                }
            }
        }

        Action::AppendConstitutionOutput { content } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(workflow) = &mut worktree.tasks.constitution_workflow {
                        workflow.output.push_str(&content);
                    }
                }
            }
        }

        Action::SaveConstitution => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(workflow) = &mut worktree.tasks.constitution_workflow {
                        workflow.status = crate::app_state::WorkflowStatus::Complete;
                    }
                }
            }
        }

        Action::SetConstitutionError { error } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(workflow) = &mut worktree.tasks.constitution_workflow {
                        workflow.status = crate::app_state::WorkflowStatus::Error;
                        workflow.error = Some(error);
                    }
                }
            }
        }

        Action::SetConstitutionExists { exists } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.constitution_exists = Some(exists);
                }
            }
        }

        Action::SetConstitutionContent { content } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.constitution_content = content;
                }
            }
        }

        Action::SetClaudeMdExists { exists } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.claude_md_exists = Some(exists);
                }
            }
        }

        Action::SetClaudeMdContent { content } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.claude_md_content = content;
                }
            }
        }

        Action::SkipClaudeMdImport => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.claude_md_skipped = true;
                }
            }
        }

        Action::SetUseClaudeMdReference { use_reference } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(workflow) = &mut worktree.tasks.constitution_workflow {
                        workflow.use_claude_md_reference = use_reference;
                    }
                }
            }
        }

        Action::SetConstitutionMode { mode } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.constitution_mode = mode.into();
                }
            }
        }

        Action::SelectConstitutionPreset { preset_id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.constitution_presets.active_preset_id = preset_id;
                }
            }
        }

        Action::CreateConstitutionPreset { name, prompt } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    let now = chrono::Utc::now().to_rfc3339();
                    let preset = crate::app_state::ConstitutionPreset {
                        id: uuid::Uuid::new_v4().to_string(),
                        name,
                        prompt,
                        is_builtin: false,
                        created_at: now.clone(),
                        updated_at: now,
                    };
                    worktree.tasks.constitution_presets.presets.push(preset);
                }
            }
        }

        Action::UpdateConstitutionPreset { id, name, prompt } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    if let Some(preset) = worktree.tasks.constitution_presets.presets.iter_mut().find(|p| p.id == id && !p.is_builtin) {
                        preset.name = name;
                        preset.prompt = prompt;
                        preset.updated_at = chrono::Utc::now().to_rfc3339();
                    }
                }
            }
        }

        Action::DeleteConstitutionPreset { id } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.constitution_presets.presets.retain(|p| p.id != id || p.is_builtin);
                    if worktree.tasks.constitution_presets.active_preset_id.as_ref() == Some(&id) {
                        worktree.tasks.constitution_presets.active_preset_id = None;
                    }
                }
            }
        }

        Action::SetConstitutionPresetTempFile { path } => {
            if let Some(project) = state.active_project_mut() {
                if let Some(worktree) = project.active_worktree_mut() {
                    worktree.tasks.constitution_presets.temp_file_path = path;
                }
            }
        }
        _ => {}
    }
}
