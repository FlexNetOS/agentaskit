// Re-export specific agent modules
pub mod board;
pub mod executive;
pub mod specialized;

// Communication module (if needed)
pub mod communication;
pub mod integration_tests;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use tracing::{info, warn, error, debug};
use agentaskit_shared::{AgentId, AgentMetadata, Task as SharedTask, TaskStatus, Priority, ResourceRequirements, HealthStatus, AgentStatus, TaskResult};

pub type AgentResult<T> = Result<T, anyhow::Error>;
pub type MessageId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BroadcastScope {
    All,
    Layer(AgentLayer),
    Role(AgentRole),
}

/// Alert severity levels for system notifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    Emergency,  // System failure or critical issue
    Critical,   // Major problem requiring immediate attention
    Warning,    // Potential issue or degraded performance
    Info,       // Informational message
    Debug,      // Debugging information
}

/// Agent role in the system hierarchy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentRole {
    Executive,     // High-level coordination and decision-making
    Board,         // Strategic and governance functions
    Specialized,   // Domain-specific expertise
    Worker,        // Task execution
    Monitor,       // Observation and reporting
}

impl Default for AgentRole {
    fn default() -> Self {
        AgentRole::Worker
    }
}

/// Agent message types for inter-agent communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentMessage {
    /// Direct task request to specific agent
    Request {
        id: MessageId,
        from: AgentId,
        to: AgentId,
        task: Task,
        priority: Priority,
        timeout: Option<std::time::Duration>,
    },
    
    /// Response to a previous request
    Response {
        id: MessageId,
        request_id: MessageId,
        from: AgentId,
        to: AgentId,
        result: TaskResult,
    },
    
    /// Broadcast message to multiple agents
    Broadcast {
        id: MessageId,
        from: AgentId,
        topic: String,
        payload: serde_json::Value,
        scope: BroadcastScope,
    },
    
    /// System alert or notification
    Alert {
        id: MessageId,
        from: AgentId,
        severity: AlertSeverity,
        message: String,
        context: serde_json::Value,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    
    /// Heartbeat message for agent liveness
    Heartbeat {
        id: MessageId,
        from: AgentId,
        health: HealthStatus,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    
    /// Agent registration/deregistration
    Registration {
        id: MessageId,
        from: AgentId,
        action: RegistrationAction,
        metadata: AgentMetadata,
    },
}

/// Registration action type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationAction {
    Register,
    Deregister,
    Update,
}

use crate::orchestration::Task;
use crate::security::SecurityManager;

/// Agent trait for all agents in the system
#[async_trait]
pub trait Agent {
    /// Start the agent
    async fn start(&mut self) -> AgentResult<()>;

    /// Stop the agent gracefully
    async fn stop(&mut self) -> AgentResult<()>;

    /// Handle an incoming message
    async fn handle_message(&mut self, message: AgentMessage) -> AgentResult<Option<AgentMessage>>;

    /// Execute a task assigned to the agent
    async fn execute_task(&mut self, task: Task) -> AgentResult<TaskResult>;

    /// Perform a health check
    async fn health_check(&self) -> AgentResult<HealthStatus>;

    /// Update the agent's configuration
    async fn update_config(&mut self, config: serde_json::Value) -> AgentResult<()>;

    /// Get the agent's capabilities
    fn capabilities(&self) -> &[String];

    /// Get the current status of the agent
    async fn state(&self) -> AgentStatus;

    /// Get the agent's metadata
    fn metadata(&self) -> &AgentMetadata;
}

/// Placeholder for AgentCapability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub name: String,
    pub level: u8,
}

/// Agent hierarchy layers as defined in the design
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentLayer {
    CECCA,      // Command, Executive, Control, Coordination, Authority (1-3 agents)
    Board,      // Governance & Policy (5-15 agents)
    Executive,  // Operational Management (10-25 agents)
    StackChief, // Domain Leadership (20-50 agents)
    Specialist, // Expert Capabilities (50-200 agents)
    Micro,      // Task Execution (100-1000+ agents)
}



/// Performance metrics for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub average_response_time_ms: f64,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: u64,
    pub uptime_seconds: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            tasks_completed: 0,
            tasks_failed: 0,
            average_response_time_ms: 0.0,
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0,
            uptime_seconds: 0,
        }
    }
}

/// Basic agent implementation for generic agents
pub struct BasicAgent {
    metadata: AgentMetadata,
    state: RwLock<AgentStatus>,
    layer: AgentLayer,
    capabilities: Vec<String>,
}

impl BasicAgent {
    pub fn new(
        id: Uuid,
        name: String,
        layer: AgentLayer,
        capabilities: Vec<String>,
    ) -> Self {
        let metadata = AgentMetadata {
            id,
            name,
            agent_type: format!("{:?}", layer),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: capabilities.clone(),
            status: AgentStatus::Initializing,
            health_status: HealthStatus::Unknown,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            resource_requirements: ResourceRequirements::default(),
            tags: HashMap::new(),
        };

        Self {
            metadata,
            state: RwLock::new(AgentStatus::Initializing),
            layer,
            capabilities,
        }
    }
}

/// Concrete agent implementation that can be managed by AgentManager
#[derive(Clone)]
pub struct ManagedAgent {
    pub id: Uuid,
    pub name: String,
    pub layer: AgentLayer,
    pub capabilities: Vec<String>,
    pub status: Arc<RwLock<AgentStatus>>,
    pub resource_requirements: ResourceRequirements,
    pub performance_metrics: Arc<RwLock<PerformanceMetrics>>,
    pub escalation_path: Option<Uuid>,
    pub subordinates: Arc<RwLock<Vec<Uuid>>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_heartbeat: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
    pub metadata: AgentMetadata,
}

impl ManagedAgent {
    /// Create a new managed agent
    pub fn new(
        name: String,
        layer: AgentLayer,
        capabilities: Vec<String>,
        resource_requirements: ResourceRequirements,
    ) -> Self {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        
        let metadata = AgentMetadata {
            id,
            name: name.clone(),
            agent_type: format!("{:?}", layer),
            version: "1.0.0".to_string(),
            capabilities: capabilities.clone(),
            status: AgentStatus::Initializing,
            health_status: HealthStatus::Unknown,
            created_at: now,
            last_updated: now,
        let metadata = AgentMetadata {
            id,
            name,
            agent_type: format!("{:?}", layer),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: capabilities.clone(),
            status: AgentStatus::Initializing,
            health_status: HealthStatus::Unknown,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            resource_requirements: resource_requirements.clone(),
            tags: HashMap::new(),
        };

        Self {
            id,
            name,
            layer,
            capabilities,
            status: Arc::new(RwLock::new(AgentStatus::Initializing)),
            resource_requirements,
            performance_metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            escalation_path: None,
            subordinates: Arc::new(RwLock::new(Vec::new())),
            created_at: now,
            last_heartbeat: Arc::new(RwLock::new(None)),
            metadata,
        }
    }
}

#[async_trait]
impl Agent for ManagedAgent {
    async fn start(&mut self) -> AgentResult<()> {
        info!("Starting agent: {}", self.name);
        let mut status = self.status.write().await;
        *status = AgentStatus::Active;
        Ok(())
    }

    async fn stop(&mut self) -> AgentResult<()> {
        info!("Stopping agent: {}", self.name);
        let mut status = self.status.write().await;
        *status = AgentStatus::Shutdown;
        Ok(())
    }

    async fn handle_message(&mut self, message: AgentMessage) -> AgentResult<Option<AgentMessage>> {
        let message_type = match &message {
            AgentMessage::Request { .. } => "Request",
            AgentMessage::Response { .. } => "Response",
            AgentMessage::Broadcast { .. } => "Broadcast",
            AgentMessage::Alert { .. } => "Alert",
            AgentMessage::Heartbeat { .. } => "Heartbeat",
            AgentMessage::Registration { .. } => "Registration",
        };
        debug!("Agent {} received message: {}", self.name, message_type);
        // Basic message handling - can be extended
        Ok(None)
    }

    async fn execute_task(&mut self, task: Task) -> AgentResult<TaskResult> {
        debug!("Agent {} executing task: {}", self.name, task.name);

        // Update status to busy
        {
            let mut status = self.status.write().await;
            *status = AgentStatus::Busy;
        }

        // Simulate task execution
        // In a real implementation, this would delegate to specialized handlers

        // Update metrics
        {
            let mut metrics = self.performance_metrics.write().await;
            metrics.tasks_completed += 1;
        }

        // Update status back to active
        {
            let mut status = self.status.write().await;
            *status = AgentStatus::Active;
        }

        Ok(TaskResult {
            task_id: task.id,
            status: TaskStatus::Completed,
            output_data: Some(serde_json::json!({"result": "success", "message": "Task completed by BasicAgent"})),
            error_message: None,
            completed_at: chrono::Utc::now(),
        })
    }

    async fn health_check(&self) -> AgentResult<HealthStatus> {
        let status = self.status.read().await;
        match *status {
            AgentStatus::Active | AgentStatus::Busy => Ok(HealthStatus::Healthy),
            AgentStatus::Error => Ok(HealthStatus::Critical),
            AgentStatus::Maintenance => Ok(HealthStatus::Degraded),
            _ => Ok(HealthStatus::Unknown),
        }
    }

    async fn update_config(&mut self, _config: serde_json::Value) -> AgentResult<()> {
        debug!("Updating configuration for agent: {}", self.name);
        Ok(())
    }

    fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    async fn state(&self) -> AgentStatus {
        let status = self.status.read().await;
        status.clone()
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }
}

/// The agent management system that handles the six-layer hierarchy
pub struct AgentManager {
    agents: Arc<RwLock<HashMap<Uuid, Arc<dyn Agent>>>>,
    layer_assignments: Arc<RwLock<HashMap<AgentLayer, Vec<Uuid>>>>,
    security_manager: Arc<SecurityManager>,
    next_agent_number: Arc<RwLock<u32>>,
    // Store hierarchy relationships separately since they're not part of the Agent trait
    escalation_paths: Arc<RwLock<HashMap<Uuid, Uuid>>>,
    subordinates_map: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,
    agent_layers: Arc<RwLock<HashMap<Uuid, AgentLayer>>>,
}

impl AgentManager {
    pub async fn new(initial_agent_count: u32, security_manager: &SecurityManager) -> Result<Self> {
        let manager = Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            layer_assignments: Arc::new(RwLock::new(HashMap::new())),
            security_manager: Arc::new(security_manager.clone()),
            next_agent_number: Arc::new(RwLock::new(1)),
            escalation_paths: Arc::new(RwLock::new(HashMap::new())),
            subordinates_map: Arc::new(RwLock::new(HashMap::new())),
            agent_layers: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize the agent hierarchy with appropriate distribution
        manager.initialize_hierarchy(initial_agent_count).await?;
        
        Ok(manager)
    }

    async fn initialize_hierarchy(&self, total_agents: u32) -> Result<()> {
        info!("Initializing agent hierarchy with {} total agents", total_agents);

        // Distribute agents across layers based on the design specifications
        let distribution = Self::calculate_layer_distribution(total_agents);
        
        for (layer, count) in distribution {
            for _ in 0..count {
                self.create_agent(layer.clone()).await?;
            }
        }

        // Establish hierarchy relationships
        self.establish_hierarchy_relationships().await?;

        info!("Agent hierarchy initialization complete");
        Ok(())
    }

    fn calculate_layer_distribution(total_agents: u32) -> Vec<(AgentLayer, u32)> {
        // Distribution based on design specifications
        let cecca_count = std::cmp::min(3, std::cmp::max(1, total_agents / 100));
        let board_count = std::cmp::min(15, std::cmp::max(5, total_agents / 20));
        let executive_count = std::cmp::min(25, std::cmp::max(10, total_agents / 10));
        let stack_chief_count = std::cmp::min(50, std::cmp::max(20, total_agents / 5));
        let specialist_count = std::cmp::min(200, std::cmp::max(50, total_agents / 3));
        
        let used = cecca_count + board_count + executive_count + stack_chief_count + specialist_count;
        let micro_count = if total_agents > used { total_agents - used } else { total_agents / 2 };

        vec![
            (AgentLayer::CECCA, cecca_count),
            (AgentLayer::Board, board_count),
            (AgentLayer::Executive, executive_count),
            (AgentLayer::StackChief, stack_chief_count),
            (AgentLayer::Specialist, specialist_count),
            (AgentLayer::Micro, micro_count),
        ]
    }

    async fn create_agent(&self, layer: AgentLayer) -> Result<Uuid> {
        let agent_number = {
            let mut num = self.next_agent_number.write().await;
            let current = *num;
            *num += 1;
            current
        };

        let capabilities = Self::get_layer_capabilities(&layer);
        let resource_requirements = Self::get_layer_resource_requirements(&layer);
        let name = format!("{:?}-Agent-{:04}", layer, agent_number);

        // Create concrete ManagedAgent instance
        let agent = ManagedAgent::new(
            name,
            layer.clone(),
            capabilities,
            resource_requirements,
        );

        let agent_id = agent.id;
        
        // Store agent as trait object
        self.agents.write().await.insert(agent_id, Arc::new(agent));
        
        // Add to layer assignments
        self.layer_assignments.write().await
            .entry(layer.clone())
            .or_insert_with(Vec::new)
            .push(agent_id);
        
        // Store layer information
        self.agent_layers.write().await.insert(agent_id, layer);

        debug!("Created agent: {} ({:04})", agent_id, agent_number);
        Ok(agent_id)
    }

    fn get_layer_capabilities(layer: &AgentLayer) -> Vec<String> {
        match layer {
            AgentLayer::CECCA => vec![
                "strategic_planning".to_string(),
                "system_authority".to_string(),
                "cross_organizational_coordination".to_string(),
                "emergency_decision_making".to_string(),
                "resource_allocation".to_string(),
            ],
            AgentLayer::Board => vec![
                "policy_enforcement".to_string(),
                "governance_oversight".to_string(),
                "compliance_monitoring".to_string(),
                "risk_assessment".to_string(),
                "ethics_validation".to_string(),
            ],
            AgentLayer::Executive => vec![
                "operational_coordination".to_string(),
                "task_orchestration".to_string(),
                "resource_management".to_string(),
                "performance_monitoring".to_string(),
                "emergency_response".to_string(),
            ],
            AgentLayer::StackChief => vec![
                "domain_leadership".to_string(),
                "subject_matter_expertise".to_string(),
                "team_coordination".to_string(),
                "workflow_orchestration".to_string(),
                "specialization_management".to_string(),
            ],
            AgentLayer::Specialist => vec![
                "deep_domain_expertise".to_string(),
                "complex_analysis".to_string(),
                "system_integration".to_string(),
                "advanced_processing".to_string(),
                "decision_support".to_string(),
            ],
            AgentLayer::Micro => vec![
                "task_execution".to_string(),
                "atomic_operations".to_string(),
                "parallel_processing".to_string(),
                "rule_based_actions".to_string(),
                "resource_efficiency".to_string(),
            ],
        }
    }

    fn get_layer_resource_requirements(layer: &AgentLayer) -> ResourceRequirements {
        match layer {
            AgentLayer::CECCA => ResourceRequirements {
                cpu_cores: Some(4),
                memory_mb: Some(8192),
                storage_mb: Some(10240),
                network_bandwidth_mbps: Some(100),
                gpu_required: false,
                special_capabilities: Vec::new(),
            },
            AgentLayer::Board => ResourceRequirements {
                cpu_cores: Some(2),
                memory_mb: Some(4096),
                storage_mb: Some(5120),
                network_bandwidth_mbps: Some(50),
                gpu_required: false,
                special_capabilities: Vec::new(),
            },
            AgentLayer::Executive => ResourceRequirements {
                cpu_cores: Some(2),
                memory_mb: Some(4096),
                storage_mb: Some(5120),
                network_bandwidth_mbps: Some(50),
                gpu_required: false,
                special_capabilities: Vec::new(),
            },
            AgentLayer::StackChief => ResourceRequirements {
                cpu_cores: Some(2),
                memory_mb: Some(2048),
                storage_mb: Some(2560),
                network_bandwidth_mbps: Some(25),
                gpu_required: false,
                special_capabilities: Vec::new(),
            },
            AgentLayer::Specialist => ResourceRequirements {
                cpu_cores: Some(1),
                memory_mb: Some(1024),
                storage_mb: Some(1280),
                network_bandwidth_mbps: Some(10),
                gpu_required: false,
                special_capabilities: Vec::new(),
            },
            AgentLayer::Micro => ResourceRequirements {
                cpu_cores: Some(1),
                memory_mb: Some(256),
                storage_mb: Some(512),
                network_bandwidth_mbps: Some(5),
                gpu_required: false,
                special_capabilities: Vec::new(),
            },
        }
    }

    async fn establish_hierarchy_relationships(&self) -> Result<()> {
        info!("Establishing hierarchy relationships");

        let layer_assignments = self.layer_assignments.read().await;
        let mut escalation_paths = self.escalation_paths.write().await;
        let mut subordinates_map = self.subordinates_map.write().await;

        // CECCA -> Board relationships
        if let (Some(cecca_agents), Some(board_agents)) = 
            (layer_assignments.get(&AgentLayer::CECCA), layer_assignments.get(&AgentLayer::Board)) {
            
            if let Some(cecca_id) = cecca_agents.first() {
                for board_id in board_agents {
                    // Set escalation path for board agents
                    escalation_paths.insert(*board_id, *cecca_id);
                    
                    // Add board agent as subordinate of CECCA
                    subordinates_map
                        .entry(*cecca_id)
                        .or_insert_with(Vec::new)
                        .push(*board_id);
                }
            }
        }

        // Board -> Executive relationships
        if let (Some(board_agents), Some(executive_agents)) = 
            (layer_assignments.get(&AgentLayer::Board), layer_assignments.get(&AgentLayer::Executive)) {
            
            if let Some(board_id) = board_agents.first() {
                for exec_id in executive_agents {
                    escalation_paths.insert(*exec_id, *board_id);
                    subordinates_map
                        .entry(*board_id)
                        .or_insert_with(Vec::new)
                        .push(*exec_id);
                }
            }
        }

        // Executive -> StackChief relationships
        if let (Some(executive_agents), Some(stack_chief_agents)) = 
            (layer_assignments.get(&AgentLayer::Executive), layer_assignments.get(&AgentLayer::StackChief)) {
            
            if let Some(exec_id) = executive_agents.first() {
                for chief_id in stack_chief_agents {
                    escalation_paths.insert(*chief_id, *exec_id);
                    subordinates_map
                        .entry(*exec_id)
                        .or_insert_with(Vec::new)
                        .push(*chief_id);
                }
            }
        }

        // StackChief -> Specialist relationships
        if let (Some(stack_chief_agents), Some(specialist_agents)) = 
            (layer_assignments.get(&AgentLayer::StackChief), layer_assignments.get(&AgentLayer::Specialist)) {
            
            if let Some(chief_id) = stack_chief_agents.first() {
                for specialist_id in specialist_agents {
                    escalation_paths.insert(*specialist_id, *chief_id);
                    subordinates_map
                        .entry(*chief_id)
                        .or_insert_with(Vec::new)
                        .push(*specialist_id);
                }
            }
        }

        // Specialist -> Micro relationships
        if let (Some(specialist_agents), Some(micro_agents)) = 
            (layer_assignments.get(&AgentLayer::Specialist), layer_assignments.get(&AgentLayer::Micro)) {
            
            if let Some(specialist_id) = specialist_agents.first() {
                for micro_id in micro_agents {
                    escalation_paths.insert(*micro_id, *specialist_id);
                    subordinates_map
                        .entry(*specialist_id)
                        .or_insert_with(Vec::new)
                        .push(*micro_id);
                }
            }
        }
        
        info!("Hierarchy relationships established");
        Ok(())
    }

    pub async fn find_suitable_agent(&self, task: &Task) -> Result<Uuid> {
        let agents = self.agents.read().await;
        
        // Find agents with matching capabilities and available status
        for (agent_id, agent) in agents.iter() {
            let status = agent.state().await;
            if status == AgentStatus::Active {
                // Check if agent has required capabilities
                let agent_caps = agent.capabilities();
                let has_capabilities = task.required_capabilities.iter()
                    .all(|cap| agent_caps.contains(cap));
                
                if has_capabilities {
                    return Ok(*agent_id);
                }
            }
        }
        
        Err(anyhow::anyhow!("No suitable agent found for task"))
    }

    pub async fn send_task_to_agent(&self, agent_id: Uuid, task: &Task) -> Result<()> {
        // Get the agent and execute the task
        let agents = self.agents.read().await;
        if let Some(agent) = agents.get(&agent_id) {
            // Clone the agent to avoid holding the read lock during task execution
            let agent_clone = Arc::clone(agent);
            drop(agents);
            
            // Note: We can't call mutable methods on Arc<dyn Agent>
            // In a real implementation, we would send the task to the agent via message passing
            debug!("Task {} sent to agent {}", task.id, agent_id);
        }
        
        Ok(())
    }

    pub async fn health_check(&self) -> Result<()> {
        let agents = self.agents.read().await;
        
        for (agent_id, agent) in agents.iter() {
            // Call health_check on each agent
            match agent.health_check().await {
                Ok(health_status) => {
                    if health_status == HealthStatus::Critical {
                        warn!("Agent {} is in critical health state", agent_id);
                    }
                }
                Err(e) => {
                    error!("Health check failed for agent {}: {}", agent_id, e);
                }
            }
        }
        
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting agent manager");
        
        // In a real implementation, we would start each agent via a message bus
        // or internal command system since we can't mutate Arc<dyn Agent>
        info!("Agent manager started. {} agents initialized", self.agents.read().await.len());
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down agent manager");
        
        // In a real implementation, we would send shutdown messages to agents
        info!("Agent manager shutdown initiated");
        
        Ok(())
    }

    pub async fn get_agent_status(&self, agent_id: Uuid) -> Result<AgentStatus> {
        let agents = self.agents.read().await;
        if let Some(agent) = agents.get(&agent_id) {
            Ok(agent.state().await)
        } else {
            Err(anyhow::anyhow!("Agent not found: {}", agent_id))
        }
    }

    pub async fn get_layer_statistics(&self) -> HashMap<AgentLayer, LayerStats> {
        let agents = self.agents.read().await;
        let agent_layers = self.agent_layers.read().await;
        let mut stats = HashMap::new();

        for (agent_id, agent) in agents.iter() {
            if let Some(layer) = agent_layers.get(agent_id) {
                let layer_stats = stats.entry(layer.clone()).or_insert(LayerStats::default());
                layer_stats.total_agents += 1;
                
                let status = agent.state().await;
                match status {
                    AgentStatus::Active => layer_stats.active_agents += 1,
                    AgentStatus::Busy => layer_stats.busy_agents += 1,
                    AgentStatus::Inactive => layer_stats.idle_agents += 1,
                    AgentStatus::Shutdown => layer_stats.offline_agents += 1,
                    AgentStatus::Error => layer_stats.error_agents += 1,
                    _ => {}
                }
            }
        }

        stats
    }
}

#[derive(Debug, Default)]
pub struct LayerStats {
    pub total_agents: u32,
    pub active_agents: u32,
    pub busy_agents: u32,
    pub idle_agents: u32,
    pub offline_agents: u32,
    pub error_agents: u32,
}

/// Agent registry for tracking and discovering agents
#[derive(Debug, Default)]
pub struct AgentRegistry {
    agents: HashMap<AgentId, AgentMetadata>,
    pub health_status: HashMap<AgentId, HealthStatus>,
    capability_index: HashMap<String, Vec<AgentId>>,
    role_index: HashMap<AgentRole, Vec<AgentId>>,
    layer_index: HashMap<AgentLayer, Vec<AgentId>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a new agent
    pub fn register_agent(&mut self, metadata: AgentMetadata) -> Result<()> {
        let agent_id = metadata.id;
        
        // Add to main registry
        self.agents.insert(agent_id, metadata.clone());
        
        // Update capability index
        for capability in &metadata.capabilities {
            self.capability_index
                .entry(capability.clone())
                .or_insert_with(Vec::new)
                .push(agent_id);
        }
        
        // Initialize health status
        self.health_status.insert(agent_id, metadata.health_status.clone());
        
        Ok(())
    }
    
    /// Deregister an agent
    pub fn deregister_agent(&mut self, agent_id: AgentId) -> Result<()> {
        if let Some(metadata) = self.agents.remove(&agent_id) {
            // Remove from capability index
            for capability in &metadata.capabilities {
                if let Some(agents) = self.capability_index.get_mut(capability) {
                    agents.retain(|id| *id != agent_id);
                }
            }
            
            self.health_status.remove(&agent_id);
        }
        Ok(())
    }
    
    /// Get all registered agents
    pub fn all_agents(&self) -> Vec<AgentMetadata> {
        self.agents.values().cloned().collect()
    }
    
    /// Find agents by role
    pub fn find_by_role(&self, _role: &AgentRole) -> Vec<AgentId> {
        // Simple implementation - in production would use role_index
        self.agents.keys().copied().collect()
    }
    
    /// Find agents by layer
    pub fn find_by_layer(&self, _layer: &AgentLayer) -> Vec<AgentId> {
        // Simple implementation - in production would use layer_index
        self.agents.keys().copied().collect()
    }
    
    /// Update agent health status
    pub fn update_health(&mut self, agent_id: AgentId, health: HealthStatus) {
        self.health_status.insert(agent_id, health);
    }
}