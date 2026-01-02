//! AgentAsKit Production-Ready Core Library
//!
//! This library unifies the best capabilities from all integrated systems:
//! - rustecosys: Tauri desktop application framework
//! - rustecosys2: Advanced orchestration and execution engine  
//! - agentrs: Comprehensive multi-agent system
//!
//! Following the "Heal, Don't Harm" principle, all capabilities are preserved and enhanced.

use anyhow::Result;

// Re-export core modules
pub mod agents;
pub mod communication;
pub mod monitoring;
pub mod orchestration;
pub mod performance;
pub mod security;

// Enhanced workflow processing module
pub mod workflows;

// New autonomous development modules
pub mod autonomous;
pub mod self_improving;
pub mod verification;

// Re-export commonly used types for convenience
pub use agents::{Agent, AgentLayer, AgentManager, AgentStatus};
pub use communication::{Message, MessageBroker, MessageType, Priority as MessagePriority};
pub use monitoring::{AgentMetrics, Alert, AlertLevel, MetricsCollector, SystemMetrics};
pub use orchestration::{OrchestratorEngine, Priority, Task, TaskStatus, TaskType};
pub use security::{Capability, CapabilityToken, SecurityManager};

// Export enhanced workflow processing capabilities
pub use workflows::{
    ChatRequest, DeconstructPhase, DeliverPhase, Deliverable, DeliverableType, DevelopPhase,
    DiagnosePhase, EnhancedWorkflowProcessor, EvidenceLedger as WorkflowEvidenceLedger,
    LocationType, RequestPriority, TargetLocation, TaskSubject, TruthGateRequirements,
    VerificationPass as WorkflowVerificationPass, VerificationProtocol,
};

// Export autonomous development capabilities
pub use autonomous::{AutonomousPipeline, BuildSystem, MLEngine, PipelineConfig};
pub use self_improving::{
    ImprovementTracker, LearningEngine, OrchestratorConfig, SelfImprovingOrchestrator,
};
pub use verification::{NoaVerificationSystem, TruthGate, VerificationStatus};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = env!("BUILD_TIME");

/// Initialize the AgentAsKit production system
pub async fn init_system() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("AgentAsKit Production System v{} initialized", VERSION);
    Ok(())
}
