use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Enhanced: Common agent identifier type with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentId(pub Uuid);

/// Namespace UUID for AgentAsKit agent IDs (deterministic generation)
pub const AGENT_NAMESPACE: Uuid = Uuid::from_bytes([
    0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
    0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
]);

/// Create a deterministic AgentId from a string name
/// Uses UUID v5 (SHA-1 based) for consistent IDs across runs
#[inline]
pub fn agent_id_from_name(name: &str) -> AgentId {
    Uuid::new_v5(&AGENT_NAMESPACE, name.as_bytes())
}

/// Common task identifier type
pub type TaskId = Uuid;

/// Universal agent status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentStatus {
    Inactive,
    Initializing,
    Active,
    Busy,
    Error,
    Maintenance,
    Terminating,
    Shutdown,
}

/// Universal task status enumeration (unified from core and shared)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Assigned,     // Added from core/orchestration
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

/// Priority levels used across all systems (unified from core and shared)
/// Lower numeric values = higher priority
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Emergency = 0,    // Highest priority - from core/orchestration ordering
    Critical = 1,
    High = 2,
    Medium = 3,       // Added from core/orchestration
    Normal = 4,
    Low = 5,
    Maintenance = 6,  // Added from core/orchestration - lowest priority
}

/// Health status for NOA monitoring integration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    NeedsRepair,
    Critical,
    Unknown,
}

/// Common agent metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: AgentId,
    pub name: String,
    pub agent_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
    pub health_status: HealthStatus,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub resource_requirements: ResourceRequirements,
    pub tags: HashMap<String, String>,
}

/// Resource requirements specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_cores: Option<u32>,
    pub memory_mb: Option<u64>,
    pub storage_mb: Option<u64>,
    pub network_bandwidth_mbps: Option<u32>,
    pub gpu_required: bool,
    pub special_capabilities: Vec<String>,
}

/// Common task structure used across FlexNetOS and NOA (unified from core and shared)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub description: String,
    pub task_type: String,
    pub priority: Priority,
    pub status: TaskStatus,
    pub assigned_agent: Option<AgentId>,
    pub dependencies: Vec<TaskId>,
    pub input_data: serde_json::Value,
    pub output_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,  // Added from core/orchestration
    pub timeout: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub error_message: Option<String>,
    pub tags: HashMap<String, String>,
    pub required_capabilities: Vec<String>,
}

impl Task {
    /// Get task parameters (compatibility alias for input_data)
    pub fn parameters(&self) -> &serde_json::Value {
        &self.input_data
    }

    /// Set task parameters (compatibility alias for input_data)
    pub fn set_parameters(&mut self, params: serde_json::Value) {
        self.input_data = params;
    }
}

/// Task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: TaskId,
    pub status: TaskStatus,
    pub output_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub completed_at: DateTime<Utc>,
}

/// Capability token structure for FlexNetOS integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityToken {
    pub token_id: Uuid,
    pub capability: String,
    pub granted_to: AgentId,
    pub granted_by: AgentId,
    pub valid_until: DateTime<Utc>,
    pub permissions: Vec<String>,
    pub restrictions: HashMap<String, serde_json::Value>,
    pub signature: String,
}

/// NOA deployment manifest entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentManifestEntry {
    pub agent_id: AgentId,
    pub agent_name: String,
    pub agent_type: String,
    pub deployment_config: serde_json::Value,
    pub health_checks: Vec<HealthCheck>,
    pub scaling_policy: ScalingPolicy,
    pub dependencies: Vec<AgentId>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub check_type: String,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub threshold: u32,
    pub endpoint: Option<String>,
    pub command: Option<String>,
}

/// Scaling policy for NOA deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    pub min_instances: u32,
    pub max_instances: u32,
    pub target_cpu_percent: Option<f64>,
    pub target_memory_percent: Option<f64>,
    pub scale_up_cooldown_seconds: u64,
    pub scale_down_cooldown_seconds: u64,
}

/// Message structure for inter-agent communication
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enhanced: Agent message with type-safe IDs
pub struct AgentMessage {
    pub message_id: MessageId,
    pub from_agent: AgentId,
    pub to_agent: AgentId,
    pub message_type: String,
    pub priority: Priority,
    pub timestamp: DateTime<Utc>,
    pub payload: serde_json::Value,
    pub correlation_id: Option<TaskId>,
    pub reply_to: Option<AgentId>,
    pub ttl: Option<DateTime<Utc>>,
}

/// System metrics for monitoring and health assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub agent_id: AgentId,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub memory_usage_percent: f64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub disk_usage_mb: u64,
    pub disk_usage_percent: f64,
    pub task_queue_size: u32,
    pub active_tasks: u32,
    pub completed_tasks_last_hour: u32,
    pub error_count_last_hour: u32,
    pub response_time_ms: f64,
    pub custom_metrics: HashMap<String, f64>,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

impl Default for AgentStatus {
    fn default() -> Self {
        AgentStatus::Inactive
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::Unknown
    }
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu_cores: None,
            memory_mb: None,
            storage_mb: None,
            network_bandwidth_mbps: None,
            gpu_required: false,
            special_capabilities: Vec::new(),
        }
    }
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_mb: u64,
    pub storage_mb: u64,
    pub network_bandwidth_mbps: f64,
    pub timestamp: DateTime<Utc>,
}

/// Agent context for execution environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub agent_id: AgentId,
    pub environment: HashMap<String, String>,
    pub working_directory: String,
    pub resource_limits: ResourceRequirements,
    pub permissions: Vec<String>,
}

/// Agent role in the system hierarchy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentRole {
    Executive,   // High-level coordination and decision-making
    Board,       // Strategic and governance functions
    Specialized, // Domain-specific expertise
    Worker,      // Task execution
    Monitor,     // Observation and reporting
}

impl Default for AgentRole {
    fn default() -> Self {
        AgentRole::Worker
    }
}
