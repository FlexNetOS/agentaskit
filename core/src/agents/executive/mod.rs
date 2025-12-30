pub mod noa_commander;
pub mod system_orchestrator;
pub mod resource_allocator;
pub mod priority_manager;
pub mod emergency_responder;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use noa_commander::{NoaCommander, CommanderConfig};
pub use system_orchestrator::{SystemOrchestrator, OrchestratorConfig};
pub use resource_allocator::{ResourceAllocator, ResourceAllocatorConfig};
pub use priority_manager::{PriorityManager, PriorityManagerConfig};
pub use emergency_responder::{EmergencyResponder, EmergencyResponderConfig};

use crate::agents::Agent;
use agentaskit_shared::{
    AgentContext, AgentId, AgentMessage, AgentMetadata, AgentRole, AgentStatus,
    HealthStatus, Priority, ResourceRequirements, ResourceUsage, Task, TaskResult, TaskStatus,
};
    use uuid::Uuid;
use crate::agents::MessageId;
    use std::time::Duration;
    
    /// Coordinate a strategic decision across multiple executive agents
    pub async fn coordinate_strategic_decision(
        communication_manager: &CommunicationManager,
        decision_task: Task,
        timeout: Duration,
    ) -> Result<serde_json::Value> {
        tracing::info!("Coordinating strategic decision: {}", decision_task.name);
        
        // Send task to NOA Commander for strategic decision-making
        let commander_id = AgentId::from_name("noa-commander");
        let message_id = MessageId::new();
        
        let request = AgentMessage::Request {
            id: message_id,
            from: AgentId::from_name("executive-layer-coordinator"),
            to: commander_id,
            task: decision_task,
            priority: Priority::High,
            timeout: Some(timeout),
        };
        
        communication_manager.send_message(request).await?;
        
        // TODO: Wait for response and handle coordination
        // This would involve setting up a response handler and timeout
        
        Ok(serde_json::json!({
            "status": "coordinated",
            "decision_initiated": true,
            "coordinator": "noa-commander",
        }))
    }
    
    /// Broadcast emergency alert to all executive agents
    pub async fn broadcast_emergency_alert(
        communication_manager: &CommunicationManager,
        alert_message: String,
        context: serde_json::Value,
    ) -> Result<()> {
        tracing::error!("Broadcasting emergency alert: {}", alert_message);
        
        let alert = AgentMessage::Alert {
            id: MessageId::new(),
            from: AgentId::from_name("executive-layer-coordinator"),
            severity: crate::agents::AlertSeverity::Emergency,
            message: alert_message,
            context,
            timestamp: std::time::Instant::now(),
        };
        
        // Broadcast to all executive agents
        communication_manager.send_message(AgentMessage::Broadcast {
            id: MessageId::new(),
            from: AgentId::from_name("executive-layer-coordinator"),
            topic: "emergency-alert".to_string(),
            payload: serde_json::to_value(alert)?,
            scope: crate::agents::BroadcastScope::Role(crate::agents::AgentRole::Executive),
        }).await?;
        
        Ok(())
    }
    
    /// Coordinate resource reallocation across the system
    pub async fn coordinate_resource_reallocation(
        communication_manager: &CommunicationManager,
        reallocation_request: serde_json::Value,
    ) -> Result<serde_json::Value> {
        tracing::info!("Coordinating resource reallocation");
        
        // Create task for resource allocator
        let task = Task {
            id: Uuid::new_v4(),
            name: "resource-reallocation".to_string(),
            description: "Coordinate system-wide resource reallocation".to_string(),
            parameters: reallocation_request,
            required_capabilities: vec!["resource-allocation".to_string()],
            deadline: Some(std::time::Instant::now() + Duration::from_secs(300)),
            dependencies: Vec::new(),
        };
        
        let resource_allocator_id = AgentId::from_name("resource-allocator");
        let message = AgentMessage::Request {
            id: MessageId::new(),
            from: AgentId::from_name("executive-layer-coordinator"),
            to: resource_allocator_id,
            task,
            priority: Priority::High,
            timeout: Some(Duration::from_secs(300)),
        };
        
        communication_manager.send_message(message).await?;
        
        Ok(serde_json::json!({
            "status": "reallocation_initiated",
            "coordinator": "resource-allocator",
            "estimated_completion": 300,
        }))
    }
