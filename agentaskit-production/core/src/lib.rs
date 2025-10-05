//! ARK-OS Production-Ready Core Library
//! 
//! This library unifies the best capabilities from all three Rust ecosystem repositories:
//! - rustecosys: Tauri desktop application framework
//! - rustecosys2: Advanced orchestration and execution engine  
//! - agentrs: Comprehensive multi-agent system
//! 
//! Following the "Heal, Don't Harm" principle, all capabilities are preserved and enhanced.

use anyhow::Result;

// Re-export core modules
pub mod agents;
pub mod orchestration;
pub mod communication;
pub mod security;
pub mod monitoring;

// New autonomous development modules
pub mod verification;
pub mod autonomous;
pub mod self_improving;

// Re-export commonly used types for convenience
pub use agents::{Agent, AgentLayer, AgentManager, AgentStatus};
pub use orchestration::{OrchestratorEngine, Task, TaskStatus, TaskType, Priority};
pub use communication::{MessageBroker, Message, MessageType, Priority as MessagePriority};
pub use security::{SecurityManager, CapabilityToken, Capability};
pub use monitoring::{MetricsCollector, SystemMetrics, AgentMetrics, Alert, AlertLevel};

// Export autonomous development capabilities
pub use verification::{NoaVerificationSystem, VerificationPass, VerificationStatus, TruthGate, EvidenceLedger};
pub use autonomous::{AutonomousPipeline, PipelineConfig, MLEngine, BuildSystem};
pub use self_improving::{SelfImprovingOrchestrator, OrchestratorConfig, LearningEngine, ImprovementTracker};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = env!("BUILD_TIME");

/// Initialize the ARK-OS production system
pub async fn init_system() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("ARK-OS Production System v{} initialized", VERSION);
    Ok(())
}
