//! HOOTL (Human-Out-Of-The-Loop) Autonomy Engine
//! 
//! Implements the core autonomous operation cycle:
//! SENSE → DECIDE → PLAN → AMPLIFY → GATES → RUN → OBSERVE → SCORE → EVOLVE → PROMOTE → ROLLBACK

use crate::{
    AutonomousComponent, AutonomousConfig, AutonomousPhase, AutonomousState, ComponentHealth,
    DecisionType, HealthStatus, PendingDecision, SystemHealth,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use parking_lot::Mutex;
use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use tokio::time::sleep;
use uuid::Uuid;

/// HOOTL autonomous operation engine
#[derive(Debug, Clone)]
pub struct HootlEngine {
    pub id: Uuid,
    pub config: AutonomousConfig,
    pub running: bool,
    pub cycle_count: u64,
    /// System monitor for resource usage tracking
    system_monitor: Arc<Mutex<System>>,
    /// Recent operation results for metrics calculation
    recent_operations: Arc<Mutex<Vec<OperationResult>>>,
    /// Cycle timing history
    cycle_times: Arc<Mutex<Vec<f64>>>,
    /// Error history
    error_history: Arc<Mutex<Vec<ErrorRecord>>>,
}

#[derive(Debug, Clone)]
struct OperationResult {
    timestamp: chrono::DateTime<chrono::Utc>,
    success: bool,
}

#[derive(Debug, Clone)]
struct ErrorRecord {
    timestamp: chrono::DateTime<chrono::Utc>,
    error_type: String,
}

/// HOOTL cycle execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HootlCycleResult {
    /// Cycle number
    pub cycle: u64,
    /// Phase results
    pub phase_results: Vec<PhaseResult>,
    /// Overall cycle success
    pub success: bool,
    /// Cycle duration in seconds
    pub duration: f64,
    /// Decisions made during cycle
    pub decisions_made: Vec<DecisionResult>,
    /// Errors encountered
    pub errors: Vec<String>,
}

/// Result of a single phase execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseResult {
    /// Phase that was executed
    pub phase: AutonomousPhase,
    /// Phase success status
    pub success: bool,
    /// Phase duration in seconds
    pub duration: f64,
    /// Phase output data
    pub output: serde_json::Value,
    /// Phase errors
    pub errors: Vec<String>,
}

/// Result of a decision made during autonomous operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionResult {
    /// Decision ID
    pub decision_id: Uuid,
    /// Decision type
    pub decision_type: DecisionType,
    /// Decision outcome
    pub outcome: DecisionOutcome,
    /// Decision rationale
    pub rationale: String,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
}

/// Outcome of an autonomous decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionOutcome {
    /// Decision was approved and executed
    Approved,
    /// Decision was rejected
    Rejected,
    /// Decision was deferred for later
    Deferred,
    /// Decision requires human intervention
    EscalateToHuman,
}

impl HootlEngine {
    /// Create a new HOOTL engine
    pub fn new(id: Uuid, config: AutonomousConfig) -> Self {
        Self {
            id,
            config,
            running: false,
            cycle_count: 0,
            system_monitor: Arc::new(Mutex::new(System::new_all())),
            recent_operations: Arc::new(Mutex::new(Vec::new())),
            cycle_times: Arc::new(Mutex::new(Vec::new())),
            error_history: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Start the HOOTL autonomous loop
    pub async fn start_loop(&mut self, state: &mut AutonomousState) -> Result<()> {
        tracing::info!("Starting HOOTL autonomous loop for engine {}", self.id);
        
        self.running = true;
        
        while self.running && self.cycle_count < self.config.max_cycles {
            let cycle_result = self.execute_cycle(state).await?;
            
            tracing::info!(
                "Completed HOOTL cycle {} with success: {}",
                cycle_result.cycle,
                cycle_result.success
            );
            
            // Sleep between cycles
            sleep(Duration::from_secs(self.config.cycle_interval)).await;
        }
        
        tracing::info!("HOOTL autonomous loop stopped for engine {}", self.id);
        Ok(())
    }
    
    /// Stop the HOOTL autonomous loop
    pub fn stop_loop(&mut self) {
        tracing::info!("Stopping HOOTL autonomous loop for engine {}", self.id);
        self.running = false;
    }
    
    /// Execute a single HOOTL cycle
    pub async fn execute_cycle(&mut self, state: &mut AutonomousState) -> Result<HootlCycleResult> {
        let cycle_start = std::time::Instant::now();
        self.cycle_count += 1;
        state.cycle_count = self.cycle_count;
        
        tracing::debug!("Starting HOOTL cycle {}", self.cycle_count);
        
        let mut phase_results = Vec::new();
        let decisions_made = Vec::new();
        let mut errors = Vec::new();
        let mut overall_success = true;
        
        // Execute each phase of the HOOTL cycle
        let phases = vec![
            AutonomousPhase::Sense,
            AutonomousPhase::Decide,
            AutonomousPhase::Plan,
            AutonomousPhase::Amplify,
            AutonomousPhase::Gates,
            AutonomousPhase::Run,
            AutonomousPhase::Observe,
            AutonomousPhase::Score,
            AutonomousPhase::Evolve,
            AutonomousPhase::Promote,
        ];
        
        for phase in phases {
            state.current_phase = phase.clone();
            
            let phase_result = self.execute_phase(&phase, state).await;
            
            match phase_result {
                Ok(result) => {
                    if !result.success {
                        overall_success = false;
                    }
                    phase_results.push(result);
                }
                Err(e) => {
                    overall_success = false;
                    errors.push(format!("Phase {:?} failed: {}", phase, e));
                    
                    // Create error phase result
                    phase_results.push(PhaseResult {
                        phase: phase.clone(),
                        success: false,
                        duration: 0.0,
                        output: serde_json::json!({"error": e.to_string()}),
                        errors: vec![e.to_string()],
                    });
                    
                    // If a critical phase fails, consider rollback
                    if matches!(phase, AutonomousPhase::Gates | AutonomousPhase::Run) {
                        tracing::warn!("Critical phase failed, initiating rollback");
                        state.current_phase = AutonomousPhase::Rollback;
                        let _ = self.execute_phase(&AutonomousPhase::Rollback, state).await;
                        break;
                    }
                }
            }
        }
        
        // Update state
        state.current_phase = AutonomousPhase::Idle;
        state.last_cycle_at = Some(Utc::now());
        
        let cycle_duration = cycle_start.elapsed().as_secs_f64();
        
        Ok(HootlCycleResult {
            cycle: self.cycle_count,
            phase_results,
            success: overall_success,
            duration: cycle_duration,
            decisions_made,
            errors,
        })
    }
    
    /// Execute a single phase of the HOOTL cycle
    async fn execute_phase(
        &self,
        phase: &AutonomousPhase,
        state: &mut AutonomousState,
    ) -> Result<PhaseResult> {
        let phase_start = std::time::Instant::now();
        
        tracing::debug!("Executing HOOTL phase: {:?}", phase);
        
        let (success, output, errors) = match phase {
            AutonomousPhase::Sense => self.sense_phase(state).await?,
            AutonomousPhase::Decide => self.decide_phase(state).await?,
            AutonomousPhase::Plan => self.plan_phase(state).await?,
            AutonomousPhase::Amplify => self.amplify_phase(state).await?,
            AutonomousPhase::Gates => self.gates_phase(state).await?,
            AutonomousPhase::Run => self.run_phase(state).await?,
            AutonomousPhase::Observe => self.observe_phase(state).await?,
            AutonomousPhase::Score => self.score_phase(state).await?,
            AutonomousPhase::Evolve => self.evolve_phase(state).await?,
            AutonomousPhase::Promote => self.promote_phase(state).await?,
            AutonomousPhase::Rollback => self.rollback_phase(state).await?,
            AutonomousPhase::Idle => (true, serde_json::json!({"status": "idle"}), Vec::new()),
        };
        
        let duration = phase_start.elapsed().as_secs_f64();
        
        Ok(PhaseResult {
            phase: phase.clone(),
            success,
            duration,
            output,
            errors,
        })
    }
    
    /// SENSE phase: Gather system state and environmental data
    async fn sense_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        // Gather system metrics
        let health = SystemHealth {
            cpu_usage: self.get_cpu_usage().await,
            memory_usage: self.get_memory_usage().await,
            disk_usage: self.get_disk_usage().await,
            active_agent_count: state.active_agents.len() as u32,
            success_rate: self.calculate_success_rate(state),
            avg_cycle_time: self.calculate_avg_cycle_time(state),
            error_count: self.count_recent_errors(state),
        };
        
        state.health = health.clone();
        
        let output = serde_json::json!({
            "health": health,
            "timestamp": Utc::now(),
            "active_agents": state.active_agents.len(),
            "pending_decisions": state.pending_decisions.len()
        });
        
        Ok((true, output, Vec::new()))
    }
    
    /// DECIDE phase: Analyze data and make decisions
    async fn decide_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut decisions_made = 0;
        let mut errors = Vec::new();
        
        // Process pending decisions
        for decision in &state.pending_decisions {
            match self.make_decision(decision, state).await {
                Ok(_) => decisions_made += 1,
                Err(e) => errors.push(format!("Decision {} failed: {}", decision.id, e)),
            }
        }
        
        // Generate new decisions based on system state
        if state.health.cpu_usage > self.config.safety_limits.max_cpu_usage {
            let decision = PendingDecision {
                id: Uuid::new_v4(),
                decision_type: DecisionType::ResourceAllocation,
                context: serde_json::json!({"reason": "high_cpu_usage", "current": state.health.cpu_usage}),
                priority: 8,
                created_at: Utc::now(),
                deadline: Some(Utc::now() + chrono::Duration::minutes(5)),
            };
            state.pending_decisions.push(decision);
        }
        
        let output = serde_json::json!({
            "decisions_made": decisions_made,
            "pending_decisions": state.pending_decisions.len(),
            "errors": errors.len()
        });
        
        Ok((errors.is_empty(), output, errors))
    }
    
    /// PLAN phase: Generate execution plans
    async fn plan_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut plans_generated = 0;
        let mut errors = Vec::new();

        // Generate plans based on pending tasks and decisions
        for task in &state.pending_tasks {
            // Create execution plan for each pending task
            tracing::debug!("Generating plan for task: {}", task.id);
            plans_generated += 1;
        }

        // Generate resource allocation plans based on current load
        if state.health.cpu_usage > 50.0 {
            tracing::debug!("Planning CPU optimization due to {}% usage", state.health.cpu_usage);
            plans_generated += 1;
        }

        let output = serde_json::json!({
            "plans_generated": plans_generated,
            "pending_tasks": state.pending_tasks.len()
        });

        self.record_operation(true);
        Ok((true, output, errors))
    }
    
    /// AMPLIFY phase: Allocate resources and scale operations
    async fn amplify_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut resources_allocated = 0;
        let mut errors = Vec::new();

        // Determine resource needs based on workload
        let cpu_usage = state.health.cpu_usage;
        let memory_usage = state.health.memory_usage;
        let pending_work = state.pending_tasks.len();

        // Scale up agents if needed and within limits
        if pending_work > 10 && state.health.active_agent_count < self.config.safety_limits.max_concurrent_agents {
            let agents_to_add = ((pending_work / 10).min(5)) as u32;
            tracing::info!("Amplifying: requesting {} additional agents for {} pending tasks",
                agents_to_add, pending_work);
            resources_allocated += agents_to_add;
        }

        // Allocate additional memory if usage is moderate but workload is high
        let memory_threshold =
            (self.config.safety_limits.max_memory_mb as f64 * 0.7) as u64;
        if memory_usage < memory_threshold && pending_work > 20 {
            tracing::debug!("Amplifying: memory allocation recommended");
            resources_allocated += 1;
        }

        let output = serde_json::json!({
            "resources_allocated": resources_allocated,
            "cpu_usage": cpu_usage,
            "memory_usage_mb": memory_usage,
            "active_agents": state.health.active_agent_count
        });

        self.record_operation(true);
        Ok((true, output, errors))
    }
    
    /// GATES phase: Safety checks and verification
    async fn gates_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut checks_passed = 0;
        let mut checks_failed = 0;
        let mut errors = Vec::new();
        
        // Safety check: CPU usage
        if state.health.cpu_usage > self.config.safety_limits.max_cpu_usage {
            checks_failed += 1;
            errors.push(format!("CPU usage too high: {}%", state.health.cpu_usage));
        } else {
            checks_passed += 1;
        }
        
        // Safety check: Memory usage
        if state.health.memory_usage > self.config.safety_limits.max_memory_mb {
            checks_failed += 1;
            errors.push(format!("Memory usage too high: {} MB", state.health.memory_usage));
        } else {
            checks_passed += 1;
        }
        
        // Safety check: Agent count
        if state.health.active_agent_count > self.config.safety_limits.max_concurrent_agents {
            checks_failed += 1;
            errors.push(format!("Too many active agents: {}", state.health.active_agent_count));
        } else {
            checks_passed += 1;
        }
        
        let output = serde_json::json!({
            "checks_passed": checks_passed,
            "checks_failed": checks_failed,
            "safety_status": if checks_failed == 0 { "PASS" } else { "FAIL" }
        });
        
        Ok((checks_failed == 0, output, errors))
    }
    
    /// RUN phase: Execute planned operations
    async fn run_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut operations_executed = 0;
        let mut operations_failed = 0;
        let mut errors = Vec::new();

        // Execute pending tasks (simulation - actual execution would be delegated to agents)
        let tasks_to_execute = state.pending_tasks.len().min(self.config.max_concurrent_operations as usize);

        for i in 0..tasks_to_execute {
            match self.execute_operation(i, state).await {
                Ok(_) => {
                    operations_executed += 1;
                    self.record_operation(true);
                }
                Err(e) => {
                    operations_failed += 1;
                    let error_msg = format!("Operation {} failed: {}", i, e);
                    errors.push(error_msg.clone());
                    self.record_operation(false);
                    self.record_error("execution_failure".to_string());
                }
            }
        }

        let output = serde_json::json!({
            "operations_executed": operations_executed,
            "operations_failed": operations_failed,
            "success_rate": if operations_executed + operations_failed > 0 {
                operations_executed as f64 / (operations_executed + operations_failed) as f64
            } else {
                1.0
            }
        });

        Ok((operations_failed == 0, output, errors))
    }

    /// Execute a single operation (helper for run_phase)
    async fn execute_operation(&self, _operation_id: usize, _state: &AutonomousState) -> Result<()> {
        // INTENTIONAL ASYNC PLACEHOLDER:
        // This function is async by design because real implementations will perform
        // asynchronous work (e.g., delegating to agents, IO-bound tasks, RPC calls).
        // For now it is a synchronous no-op used for HOOTL simulations.
        Ok(())
    }
    
    /// OBSERVE phase: Monitor execution and gather feedback
    async fn observe_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut observations_collected = 0;
        let errors = Vec::new();

        // Collect system health metrics
        let cpu_usage = self.get_cpu_usage().await;
        let memory_usage = self.get_memory_usage().await;
        let disk_usage = self.get_disk_usage().await;

        // Update state with current observations
        state.health.cpu_usage = cpu_usage;
        state.health.memory_usage = memory_usage;
        state.health.disk_usage = disk_usage;
        observations_collected += 3;

        // Collect performance metrics
        let success_rate = self.calculate_success_rate(state);
        let avg_cycle_time = self.calculate_avg_cycle_time(state);
        let recent_errors = self.count_recent_errors(state);
        observations_collected += 3;

        tracing::debug!(
            "Observations: CPU={:.1}%, Mem={}MB, Disk={}GB, Success={:.2}%, AvgCycle={:.2}s, Errors={}",
            cpu_usage, memory_usage / 1_000_000, disk_usage / 1_000_000_000,
            success_rate * 100.0, avg_cycle_time, recent_errors
        );

        let output = serde_json::json!({
            "observations_collected": observations_collected,
            "system_health": {
                "cpu_usage_percent": cpu_usage,
                "memory_usage_mb": memory_usage / 1_000_000,
                "disk_usage_gb": disk_usage / 1_000_000_000
            },
            "performance": {
                "success_rate": success_rate,
                "avg_cycle_time_seconds": avg_cycle_time,
                "recent_errors": recent_errors
            }
        });

        Ok((true, output, errors))
    }
    
    /// SCORE phase: Evaluate performance and outcomes
    async fn score_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let errors = Vec::new();

        // Calculate composite performance score
        let success_rate = self.calculate_success_rate(state);
        let avg_cycle_time = self.calculate_avg_cycle_time(state);
        let error_count = self.count_recent_errors(state);

        // Score components (0.0 to 1.0 scale)
        let success_score = success_rate;
        let latency_score = if avg_cycle_time > 0.0 {
            (1.0 / (1.0 + avg_cycle_time / 10.0)).max(0.0).min(1.0)
        } else {
            1.0
        };
        let reliability_score = if error_count == 0 {
            1.0
        } else {
            (1.0 / (1.0 + error_count as f64 / 10.0)).max(0.0).min(1.0)
        };

        // Weighted composite score
        let performance_score = (success_score * 0.5) + (latency_score * 0.3) + (reliability_score * 0.2);

        tracing::info!(
            "Performance score: {:.3} (success={:.2}, latency={:.2}, reliability={:.2})",
            performance_score, success_score, latency_score, reliability_score
        );

        let output = serde_json::json!({
            "performance_score": performance_score,
            "components": {
                "success_rate": success_score,
                "latency_score": latency_score,
                "reliability_score": reliability_score
            },
            "metrics": {
                "success_rate_percent": success_rate * 100.0,
                "avg_cycle_time_seconds": avg_cycle_time,
                "recent_errors": error_count
            }
        });

        Ok((performance_score >= 0.7, output, errors))
    }
    
    /// EVOLVE phase: Learn and adapt system behavior
    async fn evolve_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut adaptations_made = 0;
        let errors = Vec::new();

        // Analyze recent performance to identify adaptation opportunities
        let success_rate = self.calculate_success_rate(state);
        let avg_cycle_time = self.calculate_avg_cycle_time(state);
        let error_count = self.count_recent_errors(state);

        // Adapt based on success rate
        if success_rate < 0.8 {
            tracing::info!("Evolving: Low success rate ({:.2}%), adapting error handling strategies", success_rate * 100.0);
            adaptations_made += 1;
        }

        // Adapt based on cycle time
        if avg_cycle_time > 5.0 {
            tracing::info!("Evolving: High cycle time ({:.2}s), adapting scheduling strategy", avg_cycle_time);
            adaptations_made += 1;
        }

        // Adapt based on error rate
        if error_count > 10 {
            tracing::info!("Evolving: High error count ({}), adapting retry and recovery policies", error_count);
            adaptations_made += 1;
        }

        // Resource optimization adaptations
        if state.health.cpu_usage > 80.0 {
            tracing::info!("Evolving: High CPU usage ({:.1}%), adapting workload distribution", state.health.cpu_usage);
            adaptations_made += 1;
        }

        let output = serde_json::json!({
            "adaptations_made": adaptations_made,
            "triggers": {
                "low_success_rate": success_rate < 0.8,
                "high_cycle_time": avg_cycle_time > 5.0,
                "high_error_count": error_count > 10,
                "high_cpu": state.health.cpu_usage > 80.0
            }
        });

        Ok((true, output, errors))
    }
    
    /// PROMOTE phase: Apply successful adaptations
    async fn promote_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut promotions_applied = 0;
        let errors = Vec::new();

        // Promote adaptations that have proven successful
        let success_rate = self.calculate_success_rate(state);

        if success_rate >= 0.9 {
            // High success rate - promote current configuration
            tracing::info!("Promoting: Current configuration shows {:.1}% success rate", success_rate * 100.0);
            promotions_applied += 1;

            // In production, this would save configuration, update policies, etc.
        }

        // Check for stable performance over time
        let avg_cycle_time = self.calculate_avg_cycle_time(state);
        if avg_cycle_time > 0.0 && avg_cycle_time < 2.0 {
            tracing::debug!("Promoting: Stable cycle time of {:.2}s", avg_cycle_time);
            promotions_applied += 1;
        }

        let output = serde_json::json!({
            "promotions_applied": promotions_applied,
            "current_performance": {
                "success_rate": success_rate,
                "avg_cycle_time": avg_cycle_time
            }
        });

        Ok((true, output, errors))
    }
    
    /// ROLLBACK phase: Revert failed changes
    async fn rollback_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut rollbacks_applied = 0;
        let mut errors = Vec::new();

        // Check if rollback is needed based on performance degradation
        let success_rate = self.calculate_success_rate(state);
        let error_count = self.count_recent_errors(state);

        // Rollback if critical failures detected
        if success_rate < 0.5 {
            tracing::warn!("Rollback triggered: Critical success rate drop to {:.1}%", success_rate * 100.0);
            rollbacks_applied += 1;
            // In production: revert to last known good configuration
        }

        if error_count > 50 {
            tracing::warn!("Rollback triggered: Excessive errors ({})", error_count);
            rollbacks_applied += 1;
            self.record_error("rollback_triggered_errors".to_string());
            // In production: revert problematic changes
        }

        // Check for resource exhaustion
        if state.health.cpu_usage > 95.0 {
            tracing::warn!("Rollback triggered: CPU exhaustion ({:.1}%)", state.health.cpu_usage);
            rollbacks_applied += 1;
            // In production: scale back recent resource allocations
        }

        let output = serde_json::json!({
            "rollbacks_applied": rollbacks_applied,
            "triggers": {
                "low_success_rate": success_rate < 0.5,
                "excessive_errors": error_count > 50,
                "cpu_exhaustion": state.health.cpu_usage > 95.0
            }
        });

        if rollbacks_applied > 0 {
            errors.push(format!("Applied {} emergency rollbacks", rollbacks_applied));
        }

        Ok((rollbacks_applied == 0, output, errors))
    }
    
    /// Make an autonomous decision
    async fn make_decision(
        &self,
        decision: &PendingDecision,
        state: &AutonomousState,
    ) -> Result<DecisionResult> {
        // Determine decision outcome based on type, priority, and system state
        let (outcome, rationale, confidence) = match decision.decision_type {
            DecisionType::ResourceAllocation => {
                // Check if we can safely allocate more resources
                if state.health.active_agent_count < self.config.safety_limits.max_concurrent_agents
                    && state.health.memory_usage
                        < self.config.safety_limits.max_memory_mb * 1024 * 1024 * 80 / 100
                {
                    (
                        DecisionOutcome::Approved,
                        "Resources available, allocation approved".to_string(),
                        0.9,
                    )
                } else {
                    (
                        DecisionOutcome::Rejected,
                        "Resource limits approached, allocation rejected".to_string(),
                        0.85,
                    )
                }
            }
            DecisionType::TaskPrioritization => {
                // Prioritize based on system load and task urgency
                if decision.priority >= 7 || state.pending_tasks.len() < 5 {
                    (
                        DecisionOutcome::Approved,
                        "High priority or low load, task prioritized".to_string(),
                        0.85,
                    )
                } else {
                    (
                        DecisionOutcome::Deferred,
                        "System busy, task deferred".to_string(),
                        0.75,
                    )
                }
            }
            DecisionType::CapabilityInvocation => {
                // Check if capability can be safely invoked
                let success_rate = self.calculate_success_rate(state);
                if success_rate > 0.8 && state.health.cpu_usage < 80.0 {
                    (
                        DecisionOutcome::Approved,
                        "System healthy, capability invocation approved".to_string(),
                        0.88,
                    )
                } else {
                    (
                        DecisionOutcome::EscalateToHuman,
                        "System degraded, escalating to human".to_string(),
                        0.65,
                    )
                }
            }
            DecisionType::SystemAdaptation => {
                // Approve adaptations during stable operation
                let error_count = self.count_recent_errors(state);
                if error_count < 5 && state.health.cpu_usage < 70.0 {
                    (
                        DecisionOutcome::Approved,
                        "System stable, adaptation approved".to_string(),
                        0.82,
                    )
                } else {
                    (
                        DecisionOutcome::Rejected,
                        "System unstable, adaptation rejected".to_string(),
                        0.78,
                    )
                }
            }
        };

        tracing::debug!(
            "Decision {} ({:?}): {:?} - {} (confidence: {:.2})",
            decision.id,
            decision.decision_type,
            outcome,
            rationale,
            confidence
        );

        Ok(DecisionResult {
            decision_id: decision.id,
            decision_type: decision.decision_type.clone(),
            outcome,
            rationale,
            confidence,
        })
    }
    
    /// Get current CPU usage
    async fn get_cpu_usage(&self) -> f64 {
        let mut sys = self.system_monitor.lock();
        sys.refresh_cpu();
        // Wait a moment for accurate CPU measurement
        tokio::time::sleep(Duration::from_millis(200)).await;
        sys.refresh_cpu();
        sys.global_cpu_info().cpu_usage() as f64
    }

    /// Get current memory usage
    async fn get_memory_usage(&self) -> u64 {
        let mut sys = self.system_monitor.lock();
        sys.refresh_memory();
        sys.used_memory()
    }

    /// Get current disk usage
    async fn get_disk_usage(&self) -> u64 {
        let sys = self.system_monitor.lock();
        // Sum up used space across all disks
        sys.disks().iter().map(|disk| disk.total_space() - disk.available_space()).sum()
    }
    
    /// Calculate success rate from recent operations
    fn calculate_success_rate(&self, _state: &AutonomousState) -> f64 {
        let operations = self.recent_operations.lock();
        if operations.is_empty() {
            return 1.0; // No data, assume success
        }

        // Calculate success rate from recent operations (last 100)
        let recent: Vec<_> = operations.iter().rev().take(100).collect();
        let successful = recent.iter().filter(|op| op.success).count();
        successful as f64 / recent.len() as f64
    }

    /// Calculate average cycle time
    fn calculate_avg_cycle_time(&self, _state: &AutonomousState) -> f64 {
        let times = self.cycle_times.lock();
        if times.is_empty() {
            return 0.0;
        }

        // Calculate average from recent cycles (last 50)
        let recent: Vec<_> = times.iter().rev().take(50).copied().collect();
        recent.iter().sum::<f64>() / recent.len() as f64
    }

    /// Count recent errors
    fn count_recent_errors(&self, _state: &AutonomousState) -> u32 {
        let errors = self.error_history.lock();
        let one_hour_ago = Utc::now() - chrono::Duration::hours(1);

        // Count errors from the last hour
        errors.iter()
            .filter(|e| e.timestamp > one_hour_ago)
            .count() as u32
    }

    /// Record an operation result for metrics tracking
    fn record_operation(&self, success: bool) {
        let mut operations = self.recent_operations.lock();
        operations.push(OperationResult {
            timestamp: Utc::now(),
            success,
        });

        // Keep only last 1000 operations
        if operations.len() > 1000 {
            operations.drain(0..operations.len() - 1000);
        }
    }

    /// Record cycle time for metrics tracking
    fn record_cycle_time(&self, duration: f64) {
        let mut times = self.cycle_times.lock();
        times.push(duration);

        // Keep only last 200 cycles
        if times.len() > 200 {
            times.drain(0..times.len() - 200);
        }
    }

    /// Record an error for metrics tracking
    fn record_error(&self, error_type: String) {
        let mut errors = self.error_history.lock();
        errors.push(ErrorRecord {
            timestamp: Utc::now(),
            error_type,
        });

        // Keep only last 24 hours of errors
        let one_day_ago = Utc::now() - chrono::Duration::hours(24);
        errors.retain(|e| e.timestamp > one_day_ago);
    }
}

#[async_trait]
impl AutonomousComponent for HootlEngine {
    async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing HOOTL engine {}", self.id);
        Ok(())
    }
    
    async fn execute_cycle(&mut self, state: &mut AutonomousState) -> Result<()> {
        self.execute_cycle(state).await?;
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        tracing::info!("Shutting down HOOTL engine {}", self.id);
        self.stop_loop();
        Ok(())
    }
    
    fn health_check(&self) -> Result<ComponentHealth> {
        Ok(ComponentHealth {
            component: "HootlEngine".to_string(),
            status: if self.running { HealthStatus::Healthy } else { HealthStatus::Degraded },
            message: format!("Cycle count: {}, Running: {}", self.cycle_count, self.running),
            checked_at: Utc::now(),
            metrics: [
                ("cycle_count".to_string(), self.cycle_count as f64),
                ("running".to_string(), if self.running { 1.0 } else { 0.0 }),
            ].into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AutonomousState;
    
    #[tokio::test]
    async fn test_hootl_engine_creation() {
        let config = AutonomousConfig::default();
        let engine = HootlEngine::new(Uuid::new_v4(), config);
        assert!(!engine.running);
        assert_eq!(engine.cycle_count, 0);
    }
    
    #[tokio::test]
    async fn test_hootl_cycle_execution() {
        let config = AutonomousConfig::default();
        let mut engine = HootlEngine::new(Uuid::new_v4(), config);
        let mut state = AutonomousState::new();
        
        let result = engine.execute_cycle(&mut state).await.unwrap();
        assert_eq!(result.cycle, 1);
        assert_eq!(state.cycle_count, 1);
    }
    
    #[tokio::test]
    async fn test_phase_execution() {
        let config = AutonomousConfig::default();
        let engine = HootlEngine::new(Uuid::new_v4(), config);
        let mut state = AutonomousState::new();
        
        let result = engine.execute_phase(&AutonomousPhase::Sense, &mut state).await.unwrap();
        assert!(result.success);
        assert_eq!(result.phase, AutonomousPhase::Sense);
    }
}
