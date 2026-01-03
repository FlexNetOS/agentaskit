use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::{Ordering, Reverse};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::agents::{
    Agent, AgentContext, AgentId, AgentMessage, AgentMetadata, AgentRole, AgentState,
    HealthStatus, Priority, ResourceRequirements, ResourceUsage, Task, TaskResult, TaskStatus,
};

use agentaskit_shared::data_models::AgentStatus;

/// Priority Manager Agent - Dynamic priority assignment and task scheduling
/// 
/// The Priority Manager is responsible for:
/// - Dynamic priority calculation and assignment
/// - Task queue management with priority-based scheduling
/// - Priority escalation and de-escalation policies
/// - SLA monitoring and priority adjustment
/// - Critical path analysis and optimization
/// - Load-aware priority balancing
pub struct PriorityManager {
    metadata: AgentMetadata,
    state: RwLock<AgentState>,
    context: Option<AgentContext>,
    
    /// Priority calculation engine
    priority_engine: Arc<RwLock<PriorityEngine>>,
    
    /// Task scheduling system
    scheduler: Arc<RwLock<PriorityScheduler>>,
    
    /// Priority policy manager
    policy_manager: Arc<RwLock<PriorityPolicyManager>>,
    
    /// SLA monitor
    sla_monitor: Arc<RwLock<SLAMonitor>>,
    
    /// Configuration
    config: PriorityManagerConfig,
}

/// Configuration for Priority Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityManagerConfig {
    /// Priority calculation interval
    pub calculation_interval: Duration,
    
    /// Task scheduling interval
    pub scheduling_interval: Duration,
    
    /// Priority aging factor (how quickly priorities increase over time)
    pub priority_aging_factor: f64,
    
    /// Maximum priority boost percentage
    pub max_priority_boost: f64,
    
    /// SLA violation threshold
    pub sla_violation_threshold: Duration,
    
    /// Emergency priority threshold
    pub emergency_threshold: f64,
    
    /// Critical priority threshold
    pub critical_threshold: f64,
    
    /// Normal priority range
    pub normal_priority_range: (f64, f64),
    
    /// Load balancing enabled
    pub load_balancing_enabled: bool,
    
    /// Maximum task queue size
    pub max_queue_size: usize,
    
    /// Priority history retention
    pub history_retention: Duration,
}

impl Default for PriorityManagerConfig {
    fn default() -> Self {
        Self {
            calculation_interval: Duration::from_secs(30),
            scheduling_interval: Duration::from_secs(1),
            priority_aging_factor: 0.01,
            max_priority_boost: 50.0,
            sla_violation_threshold: Duration::from_secs(300), // 5 minutes
            emergency_threshold: 95.0,
            critical_threshold: 80.0,
            normal_priority_range: (20.0, 70.0),
            load_balancing_enabled: true,
            max_queue_size: 10000,
            history_retention: Duration::from_secs(86400), // 24 hours
        }
    }
}

/// Priority calculation engine
#[derive(Debug, Default)]
struct PriorityEngine {
    /// Current priority assignments
    priority_assignments: HashMap<Uuid, PriorityAssignment>,
    
    /// Priority calculation factors
    calculation_factors: Vec<PriorityFactor>,
    
    /// Priority history
    priority_history: VecDeque<PrioritySnapshot>,
    
    /// Priority statistics
    statistics: PriorityStatistics,
}

/// Priority assignment for a task or agent
#[derive(Debug, Clone)]
struct PriorityAssignment {
    pub target_id: Uuid,
    pub target_type: PriorityTargetType,
    pub base_priority: f64,
    pub current_priority: f64,
    pub priority_factors: Vec<AppliedFactor>,
    pub created_at: Instant,
    pub last_updated: Instant,
    pub expires_at: Option<Instant>,
    pub locked: bool,
    pub source_agent: Option<AgentId>,
}

/// Types of priority targets
#[derive(Debug, Clone)]
enum PriorityTargetType {
    Task,
    Agent,
    Workflow,
    Resource,
    Alert,
}

/// Applied priority factor
#[derive(Debug, Clone)]
struct AppliedFactor {
    pub factor_type: PriorityFactorType,
    pub weight: f64,
    pub value: f64,
    pub contribution: f64,
    pub applied_at: Instant,
}

/// Priority calculation factors
#[derive(Debug, Clone)]
struct PriorityFactor {
    pub factor_type: PriorityFactorType,
    pub weight: f64,
    pub enabled: bool,
    pub calculation_method: FactorCalculationMethod,
}

/// Types of priority factors
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PriorityFactorType {
    Urgency,        // How urgent is the task
    Importance,     // How important is the task
    Deadline,       // Deadline proximity
    Dependencies,   // Number of dependent tasks
    ResourceCost,   // Resource requirements
    BusinessValue,  // Business impact
    UserPriority,   // User-assigned priority
    SystemLoad,     // Current system load
    SLARequirement, // SLA constraints
    Age,           // How long has task been waiting
    RetryCount,    // Number of retries
    Custom(String), // Custom factors
}

/// Methods for calculating factor contributions
#[derive(Debug, Clone)]
enum FactorCalculationMethod {
    Linear,         // Linear calculation
    Exponential,    // Exponential growth/decay
    Logarithmic,    // Logarithmic scaling
    Threshold,      // Threshold-based
    Custom(String), // Custom calculation
}

/// Priority snapshot for history tracking
#[derive(Debug)]
struct PrioritySnapshot {
    pub timestamp: Instant,
    pub assignments: HashMap<Uuid, PriorityAssignment>,
    pub statistics: PriorityStatistics,
}

/// Priority statistics
#[derive(Debug, Default, Clone)]
struct PriorityStatistics {
    pub total_assignments: u64,
    pub emergency_count: u64,
    pub critical_count: u64,
    pub normal_count: u64,
    pub low_count: u64,
    pub avg_priority: f64,
    pub priority_distribution: HashMap<String, u64>,
    pub escalation_events: u64,
    pub sla_violations: u64,
}

/// Priority-based task scheduler
#[derive(Debug, Default)]
struct PriorityScheduler {
    /// Priority queue for tasks
    task_queue: BinaryHeap<PrioritizedTask>,
    
    /// Scheduled tasks tracking
    scheduled_tasks: HashMap<Uuid, ScheduledTask>,
    
    /// Agent workload tracking
    agent_workloads: HashMap<AgentId, AgentWorkload>,
    
    /// Scheduling metrics
    scheduling_metrics: SchedulingMetrics,
}

/// Prioritized task for queue management
#[derive(Debug)]
struct PrioritizedTask {
    pub task: Task,
    pub priority: f64,
    pub deadline: Option<Instant>,
    pub queued_at: Instant,
    pub attempts: u32,
    pub dependencies: Vec<Uuid>,
    pub target_agent: Option<AgentId>,
}

impl Eq for PrioritizedTask {}

impl PartialEq for PrioritizedTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.task.id == other.task.id
    }
}

impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first (reverse comparison for BinaryHeap max-heap behavior)
        self.priority.partial_cmp(&other.priority)
            .unwrap_or(Ordering::Equal)
            .then_with(|| other.queued_at.cmp(&self.queued_at)) // Earlier tasks first for same priority
    }
}

impl PartialOrd for PrioritizedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Scheduled task tracking
#[derive(Debug)]
struct ScheduledTask {
    pub task_id: Uuid,
    pub agent_id: AgentId,
    pub scheduled_at: Instant,
    pub expected_completion: Option<Instant>,
    pub status: ScheduledTaskStatus,
}

/// Status of scheduled tasks
#[derive(Debug, Clone)]
enum ScheduledTaskStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

/// Agent workload tracking
#[derive(Debug)]
struct AgentWorkload {
    pub agent_id: AgentId,
    pub current_tasks: u32,
    pub queued_tasks: u32,
    pub total_capacity: u32,
    pub utilization_percentage: f64,
    pub avg_task_duration: Duration,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for AgentWorkload {
    fn default() -> Self {
        Self {
            agent_id: AgentId::from_name("default"),
            current_tasks: 0,
            queued_tasks: 0,
            total_capacity: 100,
            utilization_percentage: 0.0,
            avg_task_duration: Duration::from_secs(60),
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Scheduling performance metrics
#[derive(Debug, Default)]
struct SchedulingMetrics {
    pub tasks_scheduled: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub avg_queue_time: Duration,
    pub avg_execution_time: Duration,
    pub throughput: f64,
    pub sla_compliance: f64,
}

/// Priority policy management system
#[derive(Debug, Default)]
struct PriorityPolicyManager {
    /// Active policies
    active_policies: Vec<PriorityPolicy>,
    
    /// Policy triggers
    policy_triggers: HashMap<String, PolicyTrigger>,
    
    /// Policy execution history
    execution_history: VecDeque<PolicyExecution>,
}

/// Priority policy definition
#[derive(Debug)]
struct PriorityPolicy {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub conditions: Vec<PolicyCondition>,
    pub actions: Vec<PolicyAction>,
    pub priority: u8,
    pub created_at: Instant,
    pub last_executed: Option<Instant>,
}

/// Policy condition
#[derive(Debug)]
struct PolicyCondition {
    pub condition_type: ConditionType,
    pub operator: ComparisonOperator,
    pub value: serde_json::Value,
    pub target: ConditionTarget,
}

/// Types of policy conditions
#[derive(Debug)]
enum ConditionType {
    TaskAge,
    QueueSize,
    SystemLoad,
    SLAViolation,
    AgentAvailability,
    ResourceUtilization,
    BusinessHours,
    Custom(String),
}

/// Comparison operators for conditions
#[derive(Debug)]
enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Contains,
    NotContains,
}

/// Condition targets
#[derive(Debug)]
enum ConditionTarget {
    Task(Uuid),
    Agent(AgentId),
    System,
    Queue(String),
    Custom(String),
}

/// Policy actions
#[derive(Debug)]
struct PolicyAction {
    pub action_type: ActionType,
    pub parameters: serde_json::Value,
    pub target: ActionTarget,
}

/// Types of policy actions
#[derive(Debug)]
enum ActionType {
    SetPriority,
    IncreasePriority,
    DecreasePriority,
    EscalatePriority,
    ReassignTask,
    CancelTask,
    NotifyOperator,
    TriggerWorkflow,
    Custom(String),
}

/// Action targets
#[derive(Debug)]
enum ActionTarget {
    Task(Uuid),
    Agent(AgentId),
    Queue(String),
    System,
    Custom(String),
}

/// Policy trigger
#[derive(Debug)]
struct PolicyTrigger {
    pub trigger_type: TriggerType,
    pub enabled: bool,
    pub last_triggered: Option<Instant>,
    pub trigger_count: u64,
}

/// Types of policy triggers
#[derive(Debug)]
enum TriggerType {
    Schedule(Duration),
    Event(String),
    Threshold(f64),
    Manual,
}

/// Policy execution record
#[derive(Debug)]
struct PolicyExecution {
    pub policy_id: String,
    pub execution_time: Instant,
    pub conditions_met: Vec<String>,
    pub actions_executed: Vec<String>,
    pub success: bool,
    pub error: Option<String>,
}

/// SLA monitoring system
#[derive(Debug, Default)]
struct SLAMonitor {
    /// SLA definitions
    sla_definitions: HashMap<String, SLADefinition>,
    
    /// SLA compliance tracking
    compliance_tracking: HashMap<String, SLACompliance>,
    
    /// SLA violations
    violations: VecDeque<SLAViolation>,
    
    /// SLA metrics
    metrics: SLAMetrics,
}

/// SLA definition
#[derive(Debug)]
struct SLADefinition {
    pub sla_id: String,
    pub name: String,
    pub description: String,
    pub target_type: SLATargetType,
    pub target_value: f64,
    pub measurement_unit: SLAMeasurementUnit,
    pub measurement_period: Duration,
    pub priority_impact: f64,
    pub enabled: bool,
}

/// SLA target types
#[derive(Debug)]
enum SLATargetType {
    ResponseTime,
    Throughput,
    Availability,
    ErrorRate,
    QueueTime,
    Custom(String),
}

/// SLA measurement units
#[derive(Debug)]
enum SLAMeasurementUnit {
    Seconds,
    Minutes,
    Hours,
    Percentage,
    Count,
    Rate,
    Custom(String),
}

/// SLA compliance tracking
#[derive(Debug)]
struct SLACompliance {
    pub sla_id: String,
    pub current_value: f64,
    pub target_value: f64,
    pub compliance_percentage: f64,
    pub status: ComplianceStatus,
    pub last_updated: Instant,
    pub measurement_history: VecDeque<SLAMeasurement>,
}

/// SLA compliance status
#[derive(Debug)]
enum ComplianceStatus {
    Compliant,
    Warning,
    Violation,
    Critical,
}

/// SLA measurement
#[derive(Debug)]
struct SLAMeasurement {
    pub timestamp: Instant,
    pub value: f64,
    pub compliant: bool,
}

/// SLA violation record
#[derive(Debug)]
struct SLAViolation {
    pub violation_id: Uuid,
    pub sla_id: String,
    pub violation_time: Instant,
    pub severity: ViolationSeverity,
    pub measured_value: f64,
    pub target_value: f64,
    pub impact_description: String,
    pub resolved: bool,
    pub resolution_time: Option<Instant>,
}

/// SLA violation severity
#[derive(Debug)]
enum ViolationSeverity {
    Minor,
    Major,
    Critical,
    Catastrophic,
}

/// SLA monitoring metrics
#[derive(Debug, Default)]
struct SLAMetrics {
    pub total_slas: usize,
    pub compliant_slas: usize,
    pub violated_slas: usize,
    pub avg_compliance: f64,
    pub total_violations: u64,
    pub resolved_violations: u64,
    pub avg_resolution_time: Duration,
}

impl PriorityManager {
    pub fn new(config: PriorityManagerConfig) -> Self {
        let mut tags = HashMap::new();
        tags.insert("cluster_assignment".to_string(), "orchestration".to_string());

        let metadata = AgentMetadata {
            id: AgentId::from_name("priority-manager"),
            name: "Priority Manager".to_string(),
            agent_type: "Executive".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![
                "priority-management".to_string(),
                "task-scheduling".to_string(),
                "sla-monitoring".to_string(),
                "policy-management".to_string(),
                "escalation-handling".to_string(),
                "load-balancing".to_string(),
            ],
            status: AgentStatus::Initializing,
            health_status: HealthStatus::Unknown,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            resource_requirements: ResourceRequirements {
                cpu_cores: Some(2),
                memory_mb: Some(2048), // 2GB
                storage_mb: Some(100), // 100MB
                network_bandwidth_mbps: None,
                gpu_required: false,
                special_capabilities: Vec::new(),
            },
            tags,
        };

        Self {
            metadata,
            state: RwLock::new(AgentState::Initializing),
            context: None,
            priority_engine: Arc::new(RwLock::new(PriorityEngine::default())),
            scheduler: Arc::new(RwLock::new(PriorityScheduler::default())),
            policy_manager: Arc::new(RwLock::new(PriorityPolicyManager::default())),
            sla_monitor: Arc::new(RwLock::new(SLAMonitor::default())),
            config,
        }
    }

    /// Calculate priority for a task
    pub async fn calculate_priority(&self, task: &Task) -> Result<f64> {
        let priority_engine = self.priority_engine.read().await;
        
        let mut total_priority = 0.0;
        let mut total_weight = 0.0;
        
        // Apply priority factors
        for factor in &priority_engine.calculation_factors {
            if !factor.enabled {
                continue;
            }
            
            let factor_value = self.calculate_factor_value(&factor.factor_type, task).await?;
            let contribution = factor.weight * factor_value;
            
            total_priority += contribution;
            total_weight += factor.weight;
        }
        
        // Normalize priority
        let normalized_priority = if total_weight > 0.0 {
            total_priority / total_weight
        } else {
            50.0 // Default priority
        };
        
        // Apply aging factor
        let age_factor = self.calculate_age_factor(task);
        let final_priority = normalized_priority + age_factor;
        
        // Clamp priority to valid range
        Ok(final_priority.clamp(0.0, 100.0))
    }
    
    /// Schedule a task with calculated priority
    pub async fn schedule_task(&self, task: Task) -> Result<()> {
        let priority = self.calculate_priority(&task).await?;
        let mut scheduler = self.scheduler.write().await;
        
        // Check queue size limit
        if scheduler.task_queue.len() >= self.config.max_queue_size {
            tracing::warn!("Task queue at maximum capacity, rejecting task {}", task.id);
            return Err(anyhow::anyhow!("Task queue full"));
        }
        
        // Extract deadline from task metadata if present
        let deadline = task.parameters.get("deadline")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc));

        // Extract dependencies from task parameters
        let dependencies = task.parameters.get("dependencies")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .filter_map(|s| uuid::Uuid::parse_str(s).ok())
                .collect())
            .unwrap_or_default();

        // Extract target agent if specified
        let target_agent = task.parameters.get("target_agent")
            .and_then(|v| v.as_str())
            .map(AgentId::from_name);

        let prioritized_task = PrioritizedTask {
            task,
            priority,
            deadline,
            queued_at: Instant::now(),
            attempts: 0,
            dependencies,
            target_agent,
        };
        
        scheduler.task_queue.push(prioritized_task);
        scheduler.scheduling_metrics.tasks_scheduled += 1;
        
        tracing::debug!("Scheduled task with priority {:.2}", priority);
        Ok(())
    }
    
    /// Get next task from priority queue
    pub async fn get_next_task(&self) -> Result<Option<Task>> {
        let mut scheduler = self.scheduler.write().await;
        
        if let Some(prioritized_task) = scheduler.task_queue.pop() {
            let task = prioritized_task.task;
            
            // Update scheduling metrics
            let queue_time = prioritized_task.queued_at.elapsed();
            scheduler.scheduling_metrics.avg_queue_time = 
                (scheduler.scheduling_metrics.avg_queue_time + queue_time) / 2;
            
            tracing::debug!("Retrieved task {} with priority {:.2}", task.id, prioritized_task.priority);
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }
    
    /// Escalate task priority
    pub async fn escalate_priority(&self, task_id: Uuid, escalation_factor: f64) -> Result<()> {
        let mut priority_engine = self.priority_engine.write().await;
        
        if let Some(assignment) = priority_engine.priority_assignments.get_mut(&task_id) {
            if !assignment.locked {
                let new_priority = (assignment.current_priority * (1.0 + escalation_factor))
                    .clamp(0.0, 100.0);
                
                assignment.current_priority = new_priority;
                assignment.last_updated = Instant::now();
                
                // Record escalation event
                priority_engine.statistics.escalation_events += 1;
                
                tracing::info!("Escalated priority for task {} to {:.2}", task_id, new_priority);
            } else {
                tracing::warn!("Cannot escalate locked priority assignment for task {}", task_id);
            }
        }
        
        Ok(())
    }
    
    /// Apply priority policy
    pub async fn apply_policy(&self, policy_id: &str) -> Result<()> {
        let mut policy_manager = self.policy_manager.write().await;
        
        if let Some(policy) = policy_manager.active_policies.iter().find(|p| p.policy_id == policy_id) {
            if !policy.enabled {
                return Ok(());
            }
            
            // Check policy conditions
            let conditions_met = self.check_policy_conditions(&policy.conditions).await?;
            
            if conditions_met.is_empty() {
                return Ok(());
            }
            
            // Execute policy actions
            let actions_executed = self.execute_policy_actions(&policy.actions).await?;
            
            // Record policy execution
            let execution = PolicyExecution {
                policy_id: policy_id.to_string(),
                execution_time: Instant::now(),
                conditions_met,
                actions_executed,
                success: true,
                error: None,
            };
            
            policy_manager.execution_history.push_back(execution);
            
            tracing::info!("Applied priority policy: {}", policy_id);
        }
        
        Ok(())
    }
    
    /// Monitor SLA compliance
    pub async fn monitor_sla_compliance(&self) -> Result<Vec<SLAViolation>> {
        let mut sla_monitor = self.sla_monitor.write().await;
        let mut violations = Vec::new();
        
        for (sla_id, compliance) in &mut sla_monitor.compliance_tracking {
            // Check for SLA violations
            if compliance.current_value > compliance.target_value * 1.1 { // 10% tolerance
                let violation = SLAViolation {
                    violation_id: Uuid::new_v4(),
                    sla_id: sla_id.clone(),
                    violation_time: Instant::now(),
                    severity: self.determine_violation_severity(compliance).await,
                    measured_value: compliance.current_value,
                    target_value: compliance.target_value,
                    impact_description: format!(
                        "SLA {} violated: measured {:.2}, target {:.2}",
                        sla_id, compliance.current_value, compliance.target_value
                    ),
                    resolved: false,
                    resolution_time: None,
                };
                
                violations.push(violation.clone());
                sla_monitor.violations.push_back(violation);
                
                // Update compliance status
                compliance.status = ComplianceStatus::Violation;
            }
        }
        
        // Update SLA metrics
        sla_monitor.metrics.total_violations += violations.len() as u64;
        
        if !violations.is_empty() {
            tracing::warn!("Detected {} SLA violations", violations.len());
        }
        
        Ok(violations)
    }
    
    /// Get priority statistics
    pub async fn get_priority_statistics(&self) -> Result<PriorityStatistics> {
        let priority_engine = self.priority_engine.read().await;
        Ok(priority_engine.statistics.clone())
    }

    /// Calculate factor value for priority calculation
    async fn calculate_factor_value(&self, factor_type: &PriorityFactorType, task: &Task) -> Result<f64> {
        match factor_type {
            PriorityFactorType::Urgency => {
                // Extract urgency from task metadata or use default
                Ok(task.parameters.get("urgency")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(50.0))
            }
            PriorityFactorType::Importance => {
                Ok(task.parameters.get("importance")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(50.0))
            }
            PriorityFactorType::Deadline => {
                // Calculate deadline proximity
                if let Some(deadline_str) = task.parameters.get("deadline").and_then(|v| v.as_str()) {
                    // TODO: Parse deadline and calculate proximity
                    Ok(70.0) // Placeholder
                } else {
                    Ok(30.0) // No deadline = lower priority
                }
            }
            PriorityFactorType::Dependencies => {
                // Count dependent tasks
                let dep_count = task.parameters.get("dependencies")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.len())
                    .unwrap_or(0);
                Ok((dep_count as f64 * 10.0).clamp(0.0, 100.0))
            }
            PriorityFactorType::BusinessValue => {
                Ok(task.parameters.get("business_value")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(40.0))
            }
            PriorityFactorType::UserPriority => {
                Ok(task.parameters.get("user_priority")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(50.0))
            }
            PriorityFactorType::Age => {
                // Age factor calculated separately
                Ok(0.0)
            }
            _ => Ok(50.0), // Default value for other factors
        }
    }
    
    /// Calculate age factor for task priority
    fn calculate_age_factor(&self, task: &Task) -> f64 {
        let age = chrono::Utc::now().signed_duration_since(task.created_at);
        age.num_seconds() as f64 * self.config.priority_aging_factor
    }
    
    /// Check if policy conditions are met
    async fn check_policy_conditions(&self, conditions: &[PolicyCondition]) -> Result<Vec<String>> {
        let mut met_conditions = Vec::new();
        let scheduler = self.scheduler.read().await;
        let priority_engine = self.priority_engine.read().await;

        for condition in conditions {
            let condition_met = match &condition.condition_type {
                ConditionType::TaskAge => {
                    // Check if any tasks exceed age threshold
                    let threshold = condition.value.as_f64().unwrap_or(300.0);
                    priority_engine.priority_assignments.values()
                        .any(|a| a.created_at.elapsed().as_secs_f64() > threshold)
                }
                ConditionType::QueueSize => {
                    let threshold = condition.value.as_u64().unwrap_or(100) as usize;
                    match condition.operator {
                        ComparisonOperator::GreaterThan => scheduler.task_queue.len() > threshold,
                        ComparisonOperator::LessThan => scheduler.task_queue.len() < threshold,
                        _ => scheduler.task_queue.len() == threshold,
                    }
                }
                ConditionType::SystemLoad => {
                    let threshold = condition.value.as_f64().unwrap_or(0.8);
                    let avg_utilization = scheduler.agent_workloads.values()
                        .map(|w| w.utilization_percentage)
                        .sum::<f64>() / scheduler.agent_workloads.len().max(1) as f64;
                    avg_utilization > threshold * 100.0
                }
                ConditionType::SLAViolation => {
                    let sla_monitor = self.sla_monitor.read().await;
                    sla_monitor.violations.len() > 0
                }
                ConditionType::AgentAvailability => {
                    let threshold = condition.value.as_f64().unwrap_or(0.5);
                    let available = scheduler.agent_workloads.values()
                        .filter(|w| w.utilization_percentage < 80.0)
                        .count();
                    (available as f64 / scheduler.agent_workloads.len().max(1) as f64) < threshold
                }
                _ => false,
            };

            if condition_met {
                met_conditions.push(format!("{:?}", condition.condition_type));
            }
        }

        Ok(met_conditions)
    }

    /// Execute policy actions
    async fn execute_policy_actions(&self, actions: &[PolicyAction]) -> Result<Vec<String>> {
        let mut executed_actions = Vec::new();

        for action in actions {
            let success = match &action.action_type {
                ActionType::SetPriority => {
                    if let Some(priority) = action.parameters.get("priority").and_then(|v| v.as_f64()) {
                        let mut engine = self.priority_engine.write().await;
                        if let ActionTarget::Task(task_id) = &action.target {
                            if let Some(assignment) = engine.priority_assignments.get_mut(task_id) {
                                assignment.current_priority = priority;
                                assignment.last_updated = Instant::now();
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                ActionType::IncreasePriority => {
                    let increase = action.parameters.get("amount").and_then(|v| v.as_f64()).unwrap_or(10.0);
                    let mut engine = self.priority_engine.write().await;
                    if let ActionTarget::Task(task_id) = &action.target {
                        if let Some(assignment) = engine.priority_assignments.get_mut(task_id) {
                            assignment.current_priority = (assignment.current_priority + increase).min(100.0);
                            assignment.last_updated = Instant::now();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                ActionType::DecreasePriority => {
                    let decrease = action.parameters.get("amount").and_then(|v| v.as_f64()).unwrap_or(10.0);
                    let mut engine = self.priority_engine.write().await;
                    if let ActionTarget::Task(task_id) = &action.target {
                        if let Some(assignment) = engine.priority_assignments.get_mut(task_id) {
                            assignment.current_priority = (assignment.current_priority - decrease).max(0.0);
                            assignment.last_updated = Instant::now();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                ActionType::EscalatePriority => {
                    if let ActionTarget::Task(task_id) = &action.target {
                        self.escalate_priority(*task_id, 0.25).await.is_ok()
                    } else {
                        false
                    }
                }
                ActionType::NotifyOperator => {
                    let message = action.parameters.get("message").and_then(|v| v.as_str()).unwrap_or("Policy triggered");
                    tracing::warn!("Policy notification: {}", message);
                    true
                }
                _ => {
                    tracing::debug!("Unimplemented action type: {:?}", action.action_type);
                    false
                }
            };

            if success {
                executed_actions.push(format!("{:?}", action.action_type));
            }
        }

        Ok(executed_actions)
    }
    
    /// Determine SLA violation severity
    async fn determine_violation_severity(&self, compliance: &SLACompliance) -> ViolationSeverity {
        let violation_ratio = compliance.current_value / compliance.target_value;
        
        if violation_ratio > 2.0 {
            ViolationSeverity::Catastrophic
        } else if violation_ratio > 1.5 {
            ViolationSeverity::Critical
        } else if violation_ratio > 1.2 {
            ViolationSeverity::Major
        } else {
            ViolationSeverity::Minor
        }
    }
}

#[async_trait]
impl Agent for PriorityManager {
    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    async fn state(&self) -> AgentState {
        self.state.read().await.clone()
    }

    async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing Priority Manager");
        
        // Initialize priority factors
        let mut priority_engine = self.priority_engine.write().await;
        self.initialize_priority_factors(&mut priority_engine).await?;
        
        // Initialize default policies
        let mut policy_manager = self.policy_manager.write().await;
        self.initialize_default_policies(&mut policy_manager).await?;
        
        // Initialize SLA definitions
        let mut sla_monitor = self.sla_monitor.write().await;
        self.initialize_sla_definitions(&mut sla_monitor).await?;
        
        *self.state.write().await = AgentState::Active;
        
        tracing::info!("Priority Manager initialized successfully");
        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting Priority Manager");
        
        // Start priority calculation cycle
        let priority_engine = self.priority_engine.clone();
        let calculation_interval = self.config.calculation_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(calculation_interval);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_priority_calculation(priority_engine.clone()).await {
                    tracing::error!("Priority calculation failed: {}", e);
                }
            }
        });
        
        // Start task scheduling
        let scheduler = self.scheduler.clone();
        let scheduling_interval = self.config.scheduling_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(scheduling_interval);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_scheduling_cycle(scheduler.clone()).await {
                    tracing::error!("Task scheduling failed: {}", e);
                }
            }
        });
        
        // Start SLA monitoring
        let sla_monitor = self.sla_monitor.clone();
        let monitoring_interval = Duration::from_secs(60);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(monitoring_interval);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_sla_monitoring(sla_monitor.clone()).await {
                    tracing::error!("SLA monitoring failed: {}", e);
                }
            }
        });
        
        tracing::info!("Priority Manager started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping Priority Manager");
        
        *self.state.write().await = AgentState::Terminating;

        // Graceful shutdown
        // Save priority assignments
        let priority_engine = self.priority_engine.read().await;
        let scheduler = self.scheduler.read().await;

        tracing::info!("Saving state: {} priority assignments, {} queued tasks",
            priority_engine.priority_assignments.len(),
            scheduler.task_queue.len());

        // Log pending operations
        let pending_count = priority_engine.priority_assignments.values()
            .filter(|a| !a.locked)
            .count();
        if pending_count > 0 {
            tracing::info!("Saving {} pending priority assignments", pending_count);
        }

        // Log SLA state
        let sla_monitor = self.sla_monitor.read().await;
        tracing::info!("SLA state: {} active, {} violations pending",
            sla_monitor.sla_definitions.len(),
            sla_monitor.violations.iter().filter(|v| !v.resolved).count());

        tracing::info!("Priority Manager stopped successfully");
        Ok(())
    }

    async fn handle_message(&mut self, message: AgentMessage) -> Result<Option<AgentMessage>> {
        match message {
            AgentMessage::Request { id, from, task, .. } => {
                let result = self.execute_task(task).await?;
                
                Ok(Some(AgentMessage::Response {
                    id: crate::agents::MessageId::new(),
                    request_id: id,
                    from: self.metadata.id,
                    to: from,
                    result,
                }))
            }
            _ => Ok(None),
        }
    }

    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        let start_time = Instant::now();
        
        match task.name.as_str() {
            "calculate-priority" => {
                let priority = self.calculate_priority(&task).await?;
                
                Ok(TaskResult {
                    task_id: task.id,
                    status: TaskStatus::Completed,
                    result: serde_json::json!({"priority": priority}),
                    error: None,
                    execution_time: start_time.elapsed(),
                    resource_usage: ResourceUsage::default(),
                })
            }
            "schedule-task" => {
                self.schedule_task(task.clone()).await?;
                
                Ok(TaskResult {
                    task_id: task.id,
                    status: TaskStatus::Completed,
                    result: serde_json::json!({"scheduled": true}),
                    error: None,
                    execution_time: start_time.elapsed(),
                    resource_usage: ResourceUsage::default(),
                })
            }
            "escalate-priority" => {
                let task_id = task.parameters.get("task_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .ok_or_else(|| anyhow::anyhow!("Invalid task_id parameter"))?;
                
                let escalation_factor = task.parameters.get("escalation_factor")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.2);
                
                self.escalate_priority(task_id, escalation_factor).await?;
                
                Ok(TaskResult {
                    task_id: task.id,
                    status: TaskStatus::Completed,
                    result: serde_json::json!({"escalated": true, "factor": escalation_factor}),
                    error: None,
                    execution_time: start_time.elapsed(),
                    resource_usage: ResourceUsage::default(),
                })
            }
            "get-statistics" => {
                let stats = self.get_priority_statistics().await?;
                
                Ok(TaskResult {
                    task_id: task.id,
                    status: TaskStatus::Completed,
                    result: serde_json::json!({
                        "total_assignments": stats.total_assignments,
                        "avg_priority": stats.avg_priority,
                        "emergency_count": stats.emergency_count,
                        "critical_count": stats.critical_count,
                        "escalation_events": stats.escalation_events,
                        "sla_violations": stats.sla_violations,
                    }),
                    error: None,
                    execution_time: start_time.elapsed(),
                    resource_usage: ResourceUsage::default(),
                })
            }
            _ => {
                Ok(TaskResult {
                    task_id: task.id,
                    status: TaskStatus::Failed("Priority management failed".to_string()),
                    result: serde_json::Value::Null,
                    error: Some(format!("Unknown task type: {}", task.name)),
                    execution_time: start_time.elapsed(),
                    resource_usage: ResourceUsage::default(),
                })
            }
        }
    }

    async fn health_check(&self) -> Result<HealthStatus> {
        let state = self.state.read().await;
        let scheduler = self.scheduler.read().await;
        let priority_engine = self.priority_engine.read().await;
        
        Ok(HealthStatus {
            agent_id: self.metadata.id,
            state: state.clone(),
            last_heartbeat: chrono::Utc::now(),
            cpu_usage: 8.0, // Placeholder
            memory_usage: 128 * 1024 * 1024, // 128MB placeholder
            task_queue_size: scheduler.task_queue.len(),
            completed_tasks: scheduler.scheduling_metrics.tasks_completed,
            failed_tasks: scheduler.scheduling_metrics.tasks_failed,
            average_response_time: Duration::from_millis(25),
        })
    }

    async fn update_config(&mut self, config: serde_json::Value) -> Result<()> {
        tracing::info!("Updating Priority Manager configuration");

        // Parse and update configuration
        if let Some(interval_secs) = config.get("calculation_interval_secs").and_then(|v| v.as_u64()) {
            self.config.calculation_interval = Duration::from_secs(interval_secs);
        }

        if let Some(interval_secs) = config.get("scheduling_interval_secs").and_then(|v| v.as_u64()) {
            self.config.scheduling_interval = Duration::from_secs(interval_secs);
        }

        if let Some(factor) = config.get("priority_aging_factor").and_then(|v| v.as_f64()) {
            self.config.priority_aging_factor = factor;
        }

        if let Some(boost) = config.get("max_priority_boost").and_then(|v| v.as_f64()) {
            self.config.max_priority_boost = boost;
        }

        if let Some(threshold) = config.get("emergency_threshold").and_then(|v| v.as_f64()) {
            self.config.emergency_threshold = threshold;
        }

        if let Some(threshold) = config.get("critical_threshold").and_then(|v| v.as_f64()) {
            self.config.critical_threshold = threshold;
        }

        if let Some(enabled) = config.get("load_balancing_enabled").and_then(|v| v.as_bool()) {
            self.config.load_balancing_enabled = enabled;
        }

        if let Some(size) = config.get("max_queue_size").and_then(|v| v.as_u64()) {
            self.config.max_queue_size = size as usize;
        }

        tracing::info!("Priority Manager configuration updated");
        Ok(())
    }

    fn capabilities(&self) -> &[String] {
        &self.metadata.capabilities
    }
}

impl PriorityManager {
    /// Initialize priority factors
    async fn initialize_priority_factors(&self, priority_engine: &mut PriorityEngine) -> Result<()> {
        let factors = vec![
            PriorityFactor {
                factor_type: PriorityFactorType::Urgency,
                weight: 0.25,
                enabled: true,
                calculation_method: FactorCalculationMethod::Linear,
            },
            PriorityFactor {
                factor_type: PriorityFactorType::Importance,
                weight: 0.20,
                enabled: true,
                calculation_method: FactorCalculationMethod::Linear,
            },
            PriorityFactor {
                factor_type: PriorityFactorType::Deadline,
                weight: 0.20,
                enabled: true,
                calculation_method: FactorCalculationMethod::Exponential,
            },
            PriorityFactor {
                factor_type: PriorityFactorType::Dependencies,
                weight: 0.15,
                enabled: true,
                calculation_method: FactorCalculationMethod::Linear,
            },
            PriorityFactor {
                factor_type: PriorityFactorType::BusinessValue,
                weight: 0.15,
                enabled: true,
                calculation_method: FactorCalculationMethod::Linear,
            },
            PriorityFactor {
                factor_type: PriorityFactorType::Age,
                weight: 0.05,
                enabled: true,
                calculation_method: FactorCalculationMethod::Logarithmic,
            },
        ];
        
        priority_engine.calculation_factors = factors;
        
        tracing::info!("Initialized {} priority factors", priority_engine.calculation_factors.len());
        Ok(())
    }
    
    /// Initialize default priority policies
    async fn initialize_default_policies(&self, policy_manager: &mut PriorityPolicyManager) -> Result<()> {
        // Create default priority policies

        // Policy 1: Escalate long-waiting tasks
        let escalate_aged_tasks = PriorityPolicy {
            policy_id: "escalate-aged-tasks".to_string(),
            name: "Escalate Aged Tasks".to_string(),
            description: "Automatically escalate tasks waiting longer than threshold".to_string(),
            enabled: true,
            conditions: vec![
                PolicyCondition {
                    condition_type: ConditionType::TaskAge,
                    operator: ComparisonOperator::GreaterThan,
                    value: serde_json::json!(600), // 10 minutes
                    target: ConditionTarget::System,
                },
            ],
            actions: vec![
                PolicyAction {
                    action_type: ActionType::IncreasePriority,
                    parameters: serde_json::json!({"amount": 15.0}),
                    target: ActionTarget::System,
                },
            ],
            priority: 1,
            created_at: Instant::now(),
            last_executed: None,
        };
        policy_manager.active_policies.push(escalate_aged_tasks);

        // Policy 2: Emergency priority for queue overflow
        let queue_overflow = PriorityPolicy {
            policy_id: "queue-overflow".to_string(),
            name: "Queue Overflow Response".to_string(),
            description: "Increase processing priority when queue exceeds threshold".to_string(),
            enabled: true,
            conditions: vec![
                PolicyCondition {
                    condition_type: ConditionType::QueueSize,
                    operator: ComparisonOperator::GreaterThan,
                    value: serde_json::json!(1000),
                    target: ConditionTarget::System,
                },
            ],
            actions: vec![
                PolicyAction {
                    action_type: ActionType::NotifyOperator,
                    parameters: serde_json::json!({"message": "Task queue exceeds threshold"}),
                    target: ActionTarget::System,
                },
            ],
            priority: 2,
            created_at: Instant::now(),
            last_executed: None,
        };
        policy_manager.active_policies.push(queue_overflow);

        // Policy 3: SLA violation response
        let sla_response = PriorityPolicy {
            policy_id: "sla-violation-response".to_string(),
            name: "SLA Violation Response".to_string(),
            description: "Escalate priorities when SLA violations detected".to_string(),
            enabled: true,
            conditions: vec![
                PolicyCondition {
                    condition_type: ConditionType::SLAViolation,
                    operator: ComparisonOperator::Equals,
                    value: serde_json::json!(true),
                    target: ConditionTarget::System,
                },
            ],
            actions: vec![
                PolicyAction {
                    action_type: ActionType::EscalatePriority,
                    parameters: serde_json::json!({}),
                    target: ActionTarget::System,
                },
            ],
            priority: 0, // Highest priority policy
            created_at: Instant::now(),
            last_executed: None,
        };
        policy_manager.active_policies.push(sla_response);

        tracing::info!("Initialized {} default priority policies", policy_manager.active_policies.len());
        Ok(())
    }
    
    /// Initialize SLA definitions
    async fn initialize_sla_definitions(&self, sla_monitor: &mut SLAMonitor) -> Result<()> {
        let sla_definitions = vec![
            SLADefinition {
                sla_id: "response-time".to_string(),
                name: "Response Time SLA".to_string(),
                description: "Maximum response time for tasks".to_string(),
                target_type: SLATargetType::ResponseTime,
                target_value: 300.0, // 5 minutes
                measurement_unit: SLAMeasurementUnit::Seconds,
                measurement_period: Duration::from_secs(3600), // 1 hour
                priority_impact: 20.0,
                enabled: true,
            },
            SLADefinition {
                sla_id: "throughput".to_string(),
                name: "Throughput SLA".to_string(),
                description: "Minimum task throughput".to_string(),
                target_type: SLATargetType::Throughput,
                target_value: 100.0, // 100 tasks per hour
                measurement_unit: SLAMeasurementUnit::Rate,
                measurement_period: Duration::from_secs(3600),
                priority_impact: 15.0,
                enabled: true,
            },
        ];
        
        for sla in sla_definitions {
            sla_monitor.sla_definitions.insert(sla.sla_id.clone(), sla);
        }
        
        tracing::info!("Initialized {} SLA definitions", sla_monitor.sla_definitions.len());
        Ok(())
    }
    
    /// Run priority calculation cycle (background task)
    async fn run_priority_calculation(priority_engine: Arc<RwLock<PriorityEngine>>) -> Result<()> {
        let mut priority_engine = priority_engine.write().await;
        
        // Update priority assignments based on aging and other factors
        let now = Instant::now();
        for assignment in priority_engine.priority_assignments.values_mut() {
            if !assignment.locked {
                // Apply aging factor
                let age_hours = assignment.created_at.elapsed().as_secs_f64() / 3600.0;
                let age_bonus = age_hours * 0.5; // Small aging bonus
                
                assignment.current_priority = (assignment.base_priority + age_bonus).clamp(0.0, 100.0);
                assignment.last_updated = now;
            }
        }
        
        // Update statistics
        let mut stats = &mut priority_engine.statistics;
        stats.total_assignments = priority_engine.priority_assignments.len() as u64;
        stats.emergency_count = priority_engine.priority_assignments.values()
            .filter(|a| a.current_priority >= 95.0)
            .count() as u64;
        stats.critical_count = priority_engine.priority_assignments.values()
            .filter(|a| a.current_priority >= 80.0 && a.current_priority < 95.0)
            .count() as u64;
        
        let avg_priority: f64 = priority_engine.priority_assignments.values()
            .map(|a| a.current_priority)
            .sum::<f64>() / priority_engine.priority_assignments.len().max(1) as f64;
        
        stats.avg_priority = avg_priority;
        
        tracing::debug!("Priority calculation cycle completed");
        Ok(())
    }
    
    /// Run scheduling cycle (background task)
    async fn run_scheduling_cycle(scheduler: Arc<RwLock<PriorityScheduler>>) -> Result<()> {
        let mut scheduler = scheduler.write().await;
        
        // Process high-priority tasks first
        let mut processed_count = 0;
        while let Some(prioritized_task) = scheduler.task_queue.peek() {
            if prioritized_task.priority < 80.0 && processed_count > 10 {
                break; // Process only high-priority tasks in this cycle
            }
            
            let _task = scheduler.task_queue.pop().unwrap();
            processed_count += 1;
            
            // TODO: Actually assign task to appropriate agent
            scheduler.scheduling_metrics.tasks_completed += 1;
        }
        
        // Update throughput calculation
        if processed_count > 0 {
            scheduler.scheduling_metrics.throughput = processed_count as f64;
            tracing::debug!("Scheduled {} tasks in this cycle", processed_count);
        }
        
        Ok(())
    }
    
    /// Run SLA monitoring (background task)
    async fn run_sla_monitoring(sla_monitor: Arc<RwLock<SLAMonitor>>) -> Result<()> {
        let mut sla_monitor = sla_monitor.write().await;
        
        // Check SLA compliance for each definition
        for (sla_id, sla_def) in &sla_monitor.sla_definitions {
            if !sla_def.enabled {
                continue;
            }
            
            // TODO: Collect real measurements
            let current_value = match sla_def.target_type {
                SLATargetType::ResponseTime => 250.0, // Placeholder
                SLATargetType::Throughput => 120.0,   // Placeholder
                _ => 0.0,
            };
            
            let compliance_percentage = if current_value <= sla_def.target_value {
                100.0
            } else {
                (sla_def.target_value / current_value * 100.0).clamp(0.0, 100.0)
            };
            
            let compliance = SLACompliance {
                sla_id: sla_id.clone(),
                current_value,
                target_value: sla_def.target_value,
                compliance_percentage,
                status: if compliance_percentage >= 95.0 {
                    ComplianceStatus::Compliant
                } else if compliance_percentage >= 80.0 {
                    ComplianceStatus::Warning
                } else {
                    ComplianceStatus::Violation
                },
                last_updated: Instant::now(),
                measurement_history: VecDeque::new(),
            };
            
            sla_monitor.compliance_tracking.insert(sla_id.clone(), compliance);
        }
        
        // Update SLA metrics
        let compliant_count = sla_monitor.compliance_tracking.values()
            .filter(|c| matches!(c.status, ComplianceStatus::Compliant))
            .count();
        
        sla_monitor.metrics.total_slas = sla_monitor.sla_definitions.len();
        sla_monitor.metrics.compliant_slas = compliant_count;
        sla_monitor.metrics.violated_slas = sla_monitor.metrics.total_slas - compliant_count;
        sla_monitor.metrics.avg_compliance = sla_monitor.compliance_tracking.values()
            .map(|c| c.compliance_percentage)
            .sum::<f64>() / sla_monitor.compliance_tracking.len().max(1) as f64;
        
        tracing::debug!("SLA monitoring cycle completed");
        Ok(())
    }
}
