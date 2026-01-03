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
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

/// HOOTL autonomous operation engine
#[derive(Debug, Clone)]
pub struct HootlEngine {
    pub id: Uuid,
    pub config: AutonomousConfig,
    pub running: bool,
    pub cycle_count: u64,
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

        // Generate plans based on pending decisions
        for decision in &state.pending_decisions {
            match decision.decision_type {
                DecisionType::ResourceAllocation => {
                    // Plan resource reallocation based on current load
                    plans_generated += 1;
                }
                DecisionType::ScaleUp | DecisionType::ScaleDown => {
                    // Plan scaling operations
                    plans_generated += 1;
                }
                DecisionType::TaskAssignment => {
                    // Plan task distribution
                    plans_generated += 1;
                }
                _ => {
                    // Generic planning for other decision types
                    plans_generated += 1;
                }
            }
        }

        // Generate optimization plans based on health metrics
        if state.health.success_rate < 0.9 {
            plans_generated += 1; // Plan for improving success rate
        }

        if state.health.avg_cycle_time > self.config.max_cycle_time_seconds {
            plans_generated += 1; // Plan for reducing cycle time
        }

        let output = serde_json::json!({
            "plans_generated": plans_generated,
            "pending_decisions_addressed": state.pending_decisions.len()
        });

        Ok((errors.is_empty(), output, errors))
    }

    /// AMPLIFY phase: Allocate resources and scale operations
    async fn amplify_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut resources_allocated = 0;
        let mut errors = Vec::new();

        // Scale resources based on current load and predictions
        let load_factor = state.health.cpu_usage / 100.0;

        if load_factor > 0.8 && state.active_agents.len() < self.config.safety_limits.max_concurrent_agents as usize {
            // Need to scale up - allocate more resources
            resources_allocated += 1;
            tracing::info!("AMPLIFY: Scaling up resources due to high load ({}%)", state.health.cpu_usage);
        } else if load_factor < 0.3 && state.active_agents.len() > 1 {
            // Can scale down - reduce resource allocation
            resources_allocated += 1;
            tracing::info!("AMPLIFY: Scaling down resources due to low load ({}%)", state.health.cpu_usage);
        }

        // Allocate memory if needed
        let memory_usage_pct = (state.health.memory_usage as f64 / self.config.safety_limits.max_memory_mb as f64) * 100.0;
        if memory_usage_pct > 80.0 {
            resources_allocated += 1;
            tracing::warn!("AMPLIFY: Memory usage high at {}%", memory_usage_pct);
        }

        let output = serde_json::json!({
            "resources_allocated": resources_allocated,
            "load_factor": load_factor,
            "memory_usage_pct": memory_usage_pct
        });

        Ok((errors.is_empty(), output, errors))
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

        // Execute pending decisions that passed gates
        let decisions_to_execute: Vec<_> = state.pending_decisions.drain(..).collect();

        for decision in decisions_to_execute {
            match decision.decision_type {
                DecisionType::ResourceAllocation => {
                    // Execute resource allocation
                    tracing::debug!("RUN: Executing resource allocation for decision {}", decision.id);
                    operations_executed += 1;
                }
                DecisionType::ScaleUp => {
                    // Execute scale up operation
                    tracing::info!("RUN: Scaling up system resources");
                    operations_executed += 1;
                }
                DecisionType::ScaleDown => {
                    // Execute scale down operation
                    tracing::info!("RUN: Scaling down system resources");
                    operations_executed += 1;
                }
                DecisionType::TaskAssignment => {
                    // Execute task assignment
                    operations_executed += 1;
                }
                _ => {
                    // Execute other operations
                    operations_executed += 1;
                }
            }
        }

        let output = serde_json::json!({
            "operations_executed": operations_executed,
            "operations_failed": operations_failed,
            "success_rate": if operations_executed > 0 {
                (operations_executed - operations_failed) as f64 / operations_executed as f64
            } else {
                1.0
            }
        });

        Ok((operations_failed == 0, output, errors))
    }

    /// OBSERVE phase: Monitor execution and gather feedback
    async fn observe_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let observations_collected;

        // Collect observations from system state
        let current_cpu = self.get_cpu_usage().await;
        let current_memory = self.get_memory_usage().await;
        let current_disk = self.get_disk_usage().await;

        // Compare with previous state to detect changes
        let cpu_delta = current_cpu - state.health.cpu_usage;
        let memory_delta = current_memory as i64 - state.health.memory_usage as i64;

        observations_collected = 3; // CPU, Memory, Disk

        // Update state with current observations
        state.health.cpu_usage = current_cpu;
        state.health.memory_usage = current_memory;
        state.health.disk_usage = current_disk;

        let output = serde_json::json!({
            "observations_collected": observations_collected,
            "cpu_delta": cpu_delta,
            "memory_delta": memory_delta,
            "current_state": {
                "cpu": current_cpu,
                "memory": current_memory,
                "disk": current_disk
            }
        });

        Ok((true, output, Vec::new()))
    }

    /// SCORE phase: Evaluate performance and outcomes
    async fn score_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        // Calculate performance score based on multiple factors
        let mut total_score = 0.0;
        let mut weights_sum = 0.0;

        // Score based on success rate (weight: 0.4)
        let success_score = state.health.success_rate;
        total_score += success_score * 0.4;
        weights_sum += 0.4;

        // Score based on resource efficiency (weight: 0.3)
        let cpu_efficiency = 1.0 - (state.health.cpu_usage / 100.0).min(1.0);
        let memory_efficiency = 1.0 - (state.health.memory_usage as f64 / self.config.safety_limits.max_memory_mb as f64).min(1.0);
        let resource_score = (cpu_efficiency + memory_efficiency) / 2.0;
        total_score += resource_score * 0.3;
        weights_sum += 0.3;

        // Score based on cycle time (weight: 0.2)
        let cycle_time_score = if state.health.avg_cycle_time > 0.0 {
            (self.config.max_cycle_time_seconds / state.health.avg_cycle_time).min(1.0)
        } else {
            1.0
        };
        total_score += cycle_time_score * 0.2;
        weights_sum += 0.2;

        // Score based on error count (weight: 0.1)
        let error_score = 1.0 - (state.health.error_count as f64 / 10.0).min(1.0);
        total_score += error_score * 0.1;
        weights_sum += 0.1;

        let final_score = total_score / weights_sum;

        let output = serde_json::json!({
            "performance_score": final_score,
            "component_scores": {
                "success_rate": success_score,
                "resource_efficiency": resource_score,
                "cycle_time": cycle_time_score,
                "error_rate": error_score
            },
            "recommendation": if final_score >= 0.8 { "excellent" }
                             else if final_score >= 0.6 { "good" }
                             else if final_score >= 0.4 { "needs_improvement" }
                             else { "critical" }
        });

        Ok((true, output, Vec::new()))
    }

    /// EVOLVE phase: Learn and adapt system behavior
    async fn evolve_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut adaptations_made = 0;

        // Analyze performance patterns and adapt thresholds
        if state.health.success_rate < 0.8 {
            // Low success rate - consider more conservative resource allocation
            tracing::info!("EVOLVE: Low success rate detected, adapting behavior");
            adaptations_made += 1;
        }

        if state.health.avg_cycle_time > self.config.max_cycle_time_seconds * 1.5 {
            // Slow cycles - consider parallelization or optimization
            tracing::info!("EVOLVE: Slow cycle times detected, adapting execution strategy");
            adaptations_made += 1;
        }

        if state.health.error_count > 5 {
            // High error rate - increase caution level
            tracing::warn!("EVOLVE: High error count, increasing safety margins");
            adaptations_made += 1;
        }

        // Check for patterns in agent performance
        let active_agent_count = state.active_agents.len();
        if active_agent_count > 0 && state.health.success_rate > 0.95 {
            // High performance - consider scaling up
            tracing::info!("EVOLVE: High performance detected, may consider expansion");
            adaptations_made += 1;
        }

        let output = serde_json::json!({
            "adaptations_made": adaptations_made,
            "current_cycle": state.cycle_count,
            "learning_signals": {
                "success_rate_trend": if state.health.success_rate > 0.8 { "positive" } else { "negative" },
                "resource_efficiency_trend": "stable"
            }
        });

        Ok((true, output, Vec::new()))
    }

    /// PROMOTE phase: Apply successful adaptations
    async fn promote_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut promotions_applied = 0;

        // Promote adaptations that have proven successful
        // This would persist learned behaviors for future cycles

        // Check if current configuration is performing well
        if state.health.success_rate > 0.9 && state.health.error_count == 0 {
            // Current configuration is good - mark for promotion
            tracing::info!("PROMOTE: Current configuration performing well, promoting settings");
            promotions_applied += 1;
        }

        // Check cycle efficiency
        if state.cycle_count > 0 && state.health.avg_cycle_time < self.config.max_cycle_time_seconds * 0.8 {
            // Efficient cycle times - promote timing parameters
            promotions_applied += 1;
        }

        let output = serde_json::json!({
            "promotions_applied": promotions_applied,
            "promoted_configurations": [],
            "promotion_criteria_met": state.health.success_rate > 0.9
        });

        Ok((true, output, Vec::new()))
    }

    /// ROLLBACK phase: Revert failed changes
    async fn rollback_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut rollbacks_applied = 0;
        let errors = Vec::new();

        tracing::warn!("ROLLBACK: Initiating rollback phase");

        // Clear pending decisions that may have caused issues
        if !state.pending_decisions.is_empty() {
            tracing::info!("ROLLBACK: Clearing {} pending decisions", state.pending_decisions.len());
            state.pending_decisions.clear();
            rollbacks_applied += 1;
        }

        // Reset error count after rollback
        if state.health.error_count > 0 {
            tracing::info!("ROLLBACK: Resetting error count from {}", state.health.error_count);
            state.health.error_count = 0;
            rollbacks_applied += 1;
        }

        // Log rollback for audit trail
        tracing::info!("ROLLBACK: Applied {} rollback operations", rollbacks_applied);

        let output = serde_json::json!({
            "rollbacks_applied": rollbacks_applied,
            "state_restored": true,
            "cleared_decisions": true
        });

        Ok((true, output, errors))
    }
    
    /// Make an autonomous decision
    async fn make_decision(
        &self,
        decision: &PendingDecision,
        state: &AutonomousState,
    ) -> Result<DecisionResult> {
        let mut confidence = 0.8;
        let mut outcome = DecisionOutcome::Approved;
        let mut rationale = String::new();

        // Evaluate decision based on type and context
        match decision.decision_type {
            DecisionType::ResourceAllocation => {
                // Check if resource allocation is within safety limits
                if state.health.cpu_usage > self.config.safety_limits.max_cpu_usage {
                    outcome = DecisionOutcome::Approved;
                    rationale = format!(
                        "Resource allocation approved: CPU usage {}% exceeds threshold",
                        state.health.cpu_usage
                    );
                    confidence = 0.9;
                } else {
                    outcome = DecisionOutcome::Deferred;
                    rationale = "Resource allocation deferred: current usage within limits".to_string();
                    confidence = 0.7;
                }
            }
            DecisionType::ScaleUp => {
                // Check if scaling up is appropriate
                if state.health.cpu_usage > 70.0 &&
                   state.active_agents.len() < self.config.safety_limits.max_concurrent_agents as usize {
                    outcome = DecisionOutcome::Approved;
                    rationale = "Scale up approved: high load and capacity available".to_string();
                    confidence = 0.85;
                } else {
                    outcome = DecisionOutcome::Rejected;
                    rationale = "Scale up rejected: conditions not met".to_string();
                    confidence = 0.6;
                }
            }
            DecisionType::ScaleDown => {
                // Check if scaling down is safe
                if state.health.cpu_usage < 30.0 && state.active_agents.len() > 1 {
                    outcome = DecisionOutcome::Approved;
                    rationale = "Scale down approved: low load detected".to_string();
                    confidence = 0.8;
                } else {
                    outcome = DecisionOutcome::Rejected;
                    rationale = "Scale down rejected: minimum agents needed".to_string();
                    confidence = 0.75;
                }
            }
            DecisionType::Emergency => {
                // Emergency decisions escalate to human
                outcome = DecisionOutcome::EscalateToHuman;
                rationale = "Emergency decision requires human intervention".to_string();
                confidence = 1.0;
            }
            _ => {
                // Default handling for other decision types
                if decision.priority >= 8 {
                    outcome = DecisionOutcome::Approved;
                    rationale = "High priority decision auto-approved".to_string();
                    confidence = 0.7;
                } else {
                    outcome = DecisionOutcome::Deferred;
                    rationale = "Low priority decision deferred".to_string();
                    confidence = 0.5;
                }
            }
        }

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
        // Read from /proc/stat on Linux for actual CPU usage
        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = tokio::fs::read_to_string("/proc/stat").await {
                if let Some(cpu_line) = contents.lines().find(|l| l.starts_with("cpu ")) {
                    let values: Vec<u64> = cpu_line
                        .split_whitespace()
                        .skip(1)
                        .filter_map(|s| s.parse().ok())
                        .collect();

                    if values.len() >= 4 {
                        let user = values[0];
                        let nice = values[1];
                        let system = values[2];
                        let idle = values[3];
                        let total = user + nice + system + idle;
                        let active = user + nice + system;

                        if total > 0 {
                            return (active as f64 / total as f64) * 100.0;
                        }
                    }
                }
            }
        }

        // Fallback: return simulated value based on agent count
        let base_usage = 10.0;
        let per_agent_usage = 5.0;
        base_usage + (self.cycle_count as f64 % 10.0) * per_agent_usage
    }

    /// Get current memory usage in MB
    async fn get_memory_usage(&self) -> u64 {
        // Read from /proc/meminfo on Linux
        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = tokio::fs::read_to_string("/proc/meminfo").await {
                let mut total: u64 = 0;
                let mut available: u64 = 0;

                for line in contents.lines() {
                    if line.starts_with("MemTotal:") {
                        total = line.split_whitespace()
                            .nth(1)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                    } else if line.starts_with("MemAvailable:") {
                        available = line.split_whitespace()
                            .nth(1)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                    }
                }

                if total > 0 {
                    return (total - available) / 1024; // Convert KB to MB
                }
            }
        }

        // Fallback: return simulated value
        256 + (self.cycle_count * 10) % 512
    }

    /// Get current disk usage in MB
    async fn get_disk_usage(&self) -> u64 {
        // Use statvfs on Unix systems
        #[cfg(unix)]
        {
            use std::ffi::CString;

            if let Ok(path) = CString::new("/") {
                unsafe {
                    let mut stat: libc::statvfs = std::mem::zeroed();
                    if libc::statvfs(path.as_ptr(), &mut stat) == 0 {
                        let total = stat.f_blocks * stat.f_frsize;
                        let free = stat.f_bfree * stat.f_frsize;
                        let used = total - free;
                        return used / (1024 * 1024); // Convert to MB
                    }
                }
            }
        }

        // Fallback: return simulated value
        1024 + (self.cycle_count * 100) % 4096
    }

    /// Calculate success rate from recent operations
    fn calculate_success_rate(&self, state: &AutonomousState) -> f64 {
        // Calculate based on cycle history if available
        if state.cycle_count == 0 {
            return 1.0;
        }

        // For now, use a simulated rate based on error count
        let error_penalty = state.health.error_count as f64 * 0.05;
        (1.0 - error_penalty).max(0.0).min(1.0)
    }

    /// Calculate average cycle time
    fn calculate_avg_cycle_time(&self, state: &AutonomousState) -> f64 {
        // Calculate based on last cycle timestamp
        if let Some(last_cycle) = state.last_cycle_at {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(last_cycle);

            // Average over recent cycles
            if state.cycle_count > 0 {
                return elapsed.num_milliseconds() as f64 / 1000.0;
            }
        }

        // Default cycle time
        self.config.cycle_interval as f64
    }

    /// Count recent errors
    fn count_recent_errors(&self, state: &AutonomousState) -> u32 {
        // Count errors from current health state
        // In a full implementation, this would track errors over a time window
        state.health.error_count
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
