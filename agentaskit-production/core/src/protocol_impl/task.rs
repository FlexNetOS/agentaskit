//! TaskOrchestrationProtocol Implementation

use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;

use shared::protocols::TaskOrchestrationProtocol;
use shared::data_models::{Task, TaskId, TaskStatus, AgentId};
use crate::orchestration::OrchestratorEngine;

/// Concrete implementation of TaskOrchestrationProtocol
pub struct TaskOrchestrationService {
    orchestrator: Arc<OrchestratorEngine>,
}

impl TaskOrchestrationService {
    pub fn new(orchestrator: Arc<OrchestratorEngine>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl TaskOrchestrationProtocol for TaskOrchestrationService {
    async fn submit_task(&self, task: Task) -> Result<TaskId> {
        let internal_task = crate::orchestration::Task {
            id: task.id,
            name: task.name.clone(),
            description: task.description.clone(),
            task_type: crate::orchestration::TaskType::Processing,
            priority: crate::orchestration::Priority::Normal,
            required_capabilities: task.required_capabilities.clone(),
            parameters: task.parameters.clone(),
            dependencies: task.dependencies.clone(),
            deadline: task.deadline,
            created_at: chrono::Utc::now(),
            status: crate::orchestration::TaskStatus::Pending,
            assigned_agent: None,
        };

        self.orchestrator.submit_task(internal_task).await?;
        Ok(task.id)
    }

    async fn get_task_status(&self, task_id: TaskId) -> Result<TaskStatus> {
        let status = self.orchestrator.get_task_status(task_id).await?;
        Ok(match status {
            crate::orchestration::TaskStatus::Pending => TaskStatus::Pending,
            crate::orchestration::TaskStatus::Assigned => TaskStatus::Assigned,
            crate::orchestration::TaskStatus::InProgress => TaskStatus::InProgress,
            crate::orchestration::TaskStatus::Completed => TaskStatus::Completed,
            crate::orchestration::TaskStatus::Failed => TaskStatus::Failed,
            crate::orchestration::TaskStatus::Cancelled => TaskStatus::Cancelled,
        })
    }

    async fn get_task(&self, task_id: TaskId) -> Result<Task> {
        let internal = self.orchestrator.get_task(task_id).await?;
        Ok(Task {
            id: internal.id,
            name: internal.name,
            description: internal.description,
            task_type: format!("{:?}", internal.task_type),
            priority: format!("{:?}", internal.priority),
            required_capabilities: internal.required_capabilities,
            parameters: internal.parameters,
            dependencies: internal.dependencies,
            deadline: internal.deadline,
            created_at: internal.created_at,
            status: match internal.status {
                crate::orchestration::TaskStatus::Pending => TaskStatus::Pending,
                crate::orchestration::TaskStatus::InProgress => TaskStatus::InProgress,
                crate::orchestration::TaskStatus::Completed => TaskStatus::Completed,
                crate::orchestration::TaskStatus::Failed => TaskStatus::Failed,
                _ => TaskStatus::Pending,
            },
            assigned_agent: internal.assigned_agent,
            result: None,
        })
    }

    async fn cancel_task(&self, task_id: TaskId) -> Result<()> {
        self.orchestrator.cancel_task(task_id).await
    }

    async fn assign_task(&self, task_id: TaskId, agent_id: AgentId) -> Result<()> {
        self.orchestrator.assign_task(task_id, agent_id).await
    }

    async fn complete_task(&self, task_id: TaskId, result: serde_json::Value) -> Result<()> {
        self.orchestrator.complete_task(task_id, result).await
    }

    async fn fail_task(&self, task_id: TaskId, error: String) -> Result<()> {
        self.orchestrator.fail_task(task_id, error).await
    }
}
