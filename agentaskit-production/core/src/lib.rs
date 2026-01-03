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
pub mod orchestration;
pub mod communication;
pub mod security;
pub mod monitoring;
pub mod ai;

// Enhanced workflow processing module
pub mod workflows;

// New autonomous development modules
pub mod verification;
pub mod autonomous;
pub mod self_improving;

// Integrated orphaned modules (previously in core/src root but not declared)
// Orchestration components
pub mod orchestrator;      // Core orchestrator implementation (406 lines)
pub mod executor;          // Task executor (253 lines)
pub mod expansion;         // Agent scaling system (555 lines)
pub mod engine;            // Core engine (183 lines)
pub mod rust_workers;      // Worker pool (100 lines)
pub mod task;              // Task definitions (30 lines)

// Governance modules
pub mod governance_integration;        // Full governance (319 lines)
pub mod governance_integration_simple; // Lightweight governance (168 lines)
#[cfg(test)]
pub mod governance_stubs;              // Governance test stubs (158 lines)

// Human oversight and autonomous operation
pub mod hootl;             // Human-On-Outside-The-Loop (516 lines)
pub mod evolution;         // Evolution logic for self-improvement (83 lines)

// Communication and messaging
pub mod broker;            // Message broker stub (19 lines)

// Configuration
pub mod production_config; // Production configuration (164 lines)
pub mod build;             // Build information utilities

// CLI and shell
pub mod shell;             // Interactive shell interface (247 lines)

// Effects and utilities
pub mod effect;            // Effect system (12 lines)
#[cfg(test)]
pub mod noop;              // No-op implementations for testing (95 lines)

// Agent base (re-exported through agents module)
pub mod agent;             // Agent base implementation (94 lines)

// Re-export commonly used types for convenience
pub use agents::{Agent, AgentLayer, AgentManager, AgentStatus};
pub use orchestration::{OrchestratorEngine, Task, TaskStatus, TaskType, Priority};
pub use communication::{MessageBroker, Message, MessageType, Priority as MessagePriority};
pub use security::{SecurityManager, CapabilityToken, Capability};
pub use monitoring::{MetricsCollector, SystemMetrics, AgentMetrics, Alert, AlertLevel};

// Export enhanced workflow processing capabilities
pub use workflows::{
    EnhancedWorkflowProcessor, ChatRequest, RequestPriority, TaskSubject,
    DeconstructPhase, DiagnosePhase, DevelopPhase, DeliverPhase,
    Deliverable, DeliverableType, TargetLocation, LocationType,
    VerificationProtocol, VerificationPass, EvidenceLedger, TruthGateRequirements
};

// Export autonomous development capabilities
pub use verification::{NoaVerificationSystem, VerificationPass, VerificationStatus, TruthGate, EvidenceLedger};
pub use autonomous::{AutonomousPipeline, PipelineConfig, MLEngine, BuildSystem};
pub use self_improving::{SelfImprovingOrchestrator, OrchestratorConfig, LearningEngine, ImprovementTracker};

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
