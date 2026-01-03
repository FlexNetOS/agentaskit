//! Protocol Implementations
//!
//! Concrete implementations of the shared protocol traits.
//! These implementations connect the abstract protocols to the
//! actual system components (AgentManager, MessageBroker, etc.)

pub mod communication;
pub mod health;
pub mod task;
pub mod lifecycle;
pub mod capability;
pub mod deployment;
pub mod metrics;

// Re-export implementations
pub use communication::AgentCommunicationService;
pub use health::HealthMonitoringService;
pub use task::TaskOrchestrationService;
pub use lifecycle::AgentLifecycleService;
pub use capability::CapabilityManagementService;
pub use deployment::DeploymentManagementService;
pub use metrics::MetricsCollectionService;
