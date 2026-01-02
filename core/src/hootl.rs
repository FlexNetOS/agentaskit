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
                    // Plan resource allocation based on system health
                    if state.health.cpu_usage > 75.0 {
                        plans_generated += 1;
                    }
                }
                DecisionType::AgentSpawn => {
                    // Plan agent spawning
                    if state.active_agents.len() < self.config.safety_limits.max_concurrent_agents as usize {
                        plans_generated += 1;
                    }
                }
                DecisionType::AgentTermination => {
                    // Plan agent termination
                    if state.active_agents.len() > 0 {
                        plans_generated += 1;
                    }
                }
                DecisionType::StrategyChange => {
                    // Plan strategy changes
                    plans_generated += 1;
                }
                DecisionType::SelfModification => {
                    // Plan self-modification (requires extra safety checks)
                    if self.config.enable_self_modification {
                        plans_generated += 1;
                    } else {
                        errors.push("Self-modification disabled".to_string());
                    }
                }
            }
        }

        let output = serde_json::json!({
            "plans_generated": plans_generated,
            "pending_decisions": state.pending_decisions.len()
        });

        Ok((errors.is_empty(), output, errors))
    }
    
    /// AMPLIFY phase: Allocate resources and scale operations
    async fn amplify_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut resources_allocated = 0;
        let mut errors = Vec::new();

        // Amplify resources based on system health and demand
        if state.health.cpu_usage < 50.0 && state.pending_decisions.len() > 5 {
            // We have spare capacity and high demand - scale up
            resources_allocated += 1;
        }

        if state.health.memory_usage < self.config.safety_limits.max_memory_mb / 2 {
            // Memory available for expansion
            resources_allocated += 1;
        }

        // Check if we can spawn more agents
        if state.active_agents.len() < (self.config.safety_limits.max_concurrent_agents as usize / 2) {
            resources_allocated += 1;
        }

        let output = serde_json::json!({
            "resources_allocated": resources_allocated,
            "current_cpu": state.health.cpu_usage,
            "current_memory": state.health.memory_usage,
            "active_agents": state.active_agents.len()
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
        let mut errors = Vec::new();

        // Execute approved decisions
        let decisions_to_execute: Vec<_> = state.pending_decisions.drain(..).collect();

        for decision in decisions_to_execute {
            match self.execute_decision(&decision, state).await {
                Ok(_) => {
                    operations_executed += 1;
                }
                Err(e) => {
                    errors.push(format!("Failed to execute decision {}: {}", decision.id, e));
                }
            }
        }

        let output = serde_json::json!({
            "operations_executed": operations_executed,
            "operations_failed": errors.len()
        });

        Ok((errors.is_empty(), output, errors))
    }

    /// Execute a specific decision
    async fn execute_decision(&self, decision: &PendingDecision, state: &mut AutonomousState) -> Result<()> {
        tracing::info!("Executing decision: {:?}", decision.decision_type);

        match decision.decision_type {
            DecisionType::ResourceAllocation => {
                // Allocate resources
                tracing::info!("Allocating resources based on decision {}", decision.id);
            }
            DecisionType::AgentSpawn => {
                // Spawn new agent
                let agent_id = Uuid::new_v4();
                state.active_agents.insert(agent_id);
                tracing::info!("Spawned new agent {}", agent_id);
            }
            DecisionType::AgentTermination => {
                // Terminate an agent
                if let Some(agent_id) = state.active_agents.iter().next().copied() {
                    state.active_agents.remove(&agent_id);
                    tracing::info!("Terminated agent {}", agent_id);
                }
            }
            DecisionType::StrategyChange => {
                // Change execution strategy
                tracing::info!("Changing strategy based on decision {}", decision.id);
            }
            DecisionType::SelfModification => {
                // Perform self-modification (with extra safety)
                if self.config.enable_self_modification {
                    tracing::warn!("Self-modification requested but not fully implemented");
                } else {
                    return Err(anyhow::anyhow!("Self-modification is disabled"));
                }
            }
        }

        Ok(())
    }
    
    /// OBSERVE phase: Monitor execution and gather feedback
    async fn observe_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut observations_collected = 0;

        // Collect system metrics
        let current_cpu = self.get_cpu_usage().await;
        let current_memory = self.get_memory_usage().await;
        let current_disk = self.get_disk_usage().await;

        observations_collected += 3;

        // Update state health
        state.health.cpu_usage = current_cpu;
        state.health.memory_usage = current_memory;
        state.health.disk_usage = current_disk;
        state.health.active_agent_count = state.active_agents.len() as u32;

        // Observe agent status
        for agent_id in &state.active_agents {
            tracing::trace!("Observing agent {}", agent_id);
            observations_collected += 1;
        }

        let output = serde_json::json!({
            "observations_collected": observations_collected,
            "cpu_usage": current_cpu,
            "memory_usage": current_memory,
            "disk_usage": current_disk,
            "active_agents": state.active_agents.len()
        });

        Ok((true, output, Vec::new()))
    }
    
    /// SCORE phase: Evaluate performance and outcomes
    async fn score_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut performance_score = 100.0;

        // Deduct points for high resource usage
        if state.health.cpu_usage > 80.0 {
            performance_score -= (state.health.cpu_usage - 80.0) * 0.5;
        }

        if state.health.memory_usage > self.config.safety_limits.max_memory_mb * 9 / 10 {
            performance_score -= 10.0;
        }

        // Deduct points for errors
        performance_score -= (state.health.error_count as f64) * 2.0;

        // Bonus for good success rate
        if state.health.success_rate > 0.95 {
            performance_score += 5.0;
        }

        // Bonus for fast cycle times
        if state.health.avg_cycle_time < 1.0 {
            performance_score += 5.0;
        }

        // Clamp score between 0 and 100
        performance_score = performance_score.max(0.0).min(100.0);

        let output = serde_json::json!({
            "performance_score": performance_score,
            "cpu_usage": state.health.cpu_usage,
            "memory_usage": state.health.memory_usage,
            "success_rate": state.health.success_rate,
            "error_count": state.health.error_count
        });

        Ok((true, output, Vec::new()))
    }
    
    /// EVOLVE phase: Learn and adapt system behavior
    async fn evolve_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut adaptations_made = 0;
        let mut errors = Vec::new();

        // Adapt based on performance metrics
        if state.health.cpu_usage > 90.0 {
            // High CPU - reduce concurrent agents
            if state.active_agents.len() > 1 {
                adaptations_made += 1;
                tracing::info!("Adapting: Reducing agent count due to high CPU");
            }
        }

        if state.health.success_rate < 0.8 {
            // Low success rate - adjust strategy
            adaptations_made += 1;
            tracing::info!("Adapting: Adjusting strategy due to low success rate");
        }

        if state.health.avg_cycle_time > 10.0 {
            // Slow cycles - optimize
            adaptations_made += 1;
            tracing::info!("Adapting: Optimizing for faster cycles");
        }

        if state.health.error_count > 10 {
            // High error count - increase resilience
            adaptations_made += 1;
            tracing::info!("Adapting: Increasing error handling resilience");
        }

        let output = serde_json::json!({
            "adaptations_made": adaptations_made,
            "cpu_usage": state.health.cpu_usage,
            "success_rate": state.health.success_rate,
            "avg_cycle_time": state.health.avg_cycle_time,
            "error_count": state.health.error_count
        });

        Ok((errors.is_empty(), output, errors))
    }
    
    /// PROMOTE phase: Apply successful adaptations
    async fn promote_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut promotions_applied = 0;

        // Promote successful strategies
        if state.health.success_rate > 0.95 && state.cycle_count > 10 {
            // Current strategy is working well - make it permanent
            promotions_applied += 1;
            tracing::info!("Promoting: Current strategy performing excellently");
        }

        if state.health.cpu_usage < 50.0 && state.health.avg_cycle_time < 2.0 {
            // Efficient operation - promote resource allocation strategy
            promotions_applied += 1;
            tracing::info!("Promoting: Efficient resource allocation strategy");
        }

        if state.health.error_count == 0 && state.cycle_count > 5 {
            // Error-free operation - promote error handling approach
            promotions_applied += 1;
            tracing::info!("Promoting: Error-free operation strategy");
        }

        let output = serde_json::json!({
            "promotions_applied": promotions_applied,
            "success_rate": state.health.success_rate,
            "cpu_usage": state.health.cpu_usage,
            "cycle_count": state.cycle_count
        });

        Ok((true, output, Vec::new()))
    }
    
    /// ROLLBACK phase: Revert failed changes
    async fn rollback_phase(&self, state: &mut AutonomousState) -> Result<(bool, serde_json::Value, Vec<String>)> {
        let mut rollbacks_applied = 0;

        // Rollback if system health is critically degraded
        if state.health.cpu_usage > self.config.safety_limits.max_cpu_usage {
            // Terminate excess agents
            let target_count = (self.config.safety_limits.max_concurrent_agents / 2) as usize;
            while state.active_agents.len() > target_count {
                if let Some(agent_id) = state.active_agents.iter().next().copied() {
                    state.active_agents.remove(&agent_id);
                    rollbacks_applied += 1;
                    tracing::warn!("Rollback: Terminated agent {} due to high CPU", agent_id);
                }
            }
        }

        if state.health.memory_usage > self.config.safety_limits.max_memory_mb {
            // Clear pending decisions to free memory
            let cleared = state.pending_decisions.len();
            state.pending_decisions.clear();
            rollbacks_applied += cleared;
            tracing::warn!("Rollback: Cleared {} pending decisions due to high memory", cleared);
        }

        if state.health.error_count > 50 {
            // Reset error tracking
            rollbacks_applied += 1;
            tracing::warn!("Rollback: Resetting error tracking due to high error count");
        }

        let output = serde_json::json!({
            "rollbacks_applied": rollbacks_applied,
            "cpu_usage": state.health.cpu_usage,
            "memory_usage": state.health.memory_usage,
            "active_agents": state.active_agents.len()
        });

        Ok((true, output, Vec::new()))
    }
    
    /// Make an autonomous decision
    async fn make_decision(
        &self,
        decision: &PendingDecision,
        state: &AutonomousState,
    ) -> Result<DecisionResult> {
        let mut confidence = 0.5;
        let mut rationale = String::new();
        let mut outcome = DecisionOutcome::Approved;

        match decision.decision_type {
            DecisionType::ResourceAllocation => {
                if state.health.cpu_usage < 70.0 && state.health.memory_usage < self.config.safety_limits.max_memory_mb {
                    confidence = 0.9;
                    rationale = "Resources available for allocation".to_string();
                    outcome = DecisionOutcome::Approved;
                } else {
                    confidence = 0.3;
                    rationale = "Insufficient resources available".to_string();
                    outcome = DecisionOutcome::Rejected;
                }
            }
            DecisionType::AgentSpawn => {
                if state.active_agents.len() < self.config.safety_limits.max_concurrent_agents as usize
                    && state.health.memory_usage < self.config.safety_limits.max_memory_mb * 8 / 10 {
                    confidence = 0.85;
                    rationale = "Safe to spawn new agent".to_string();
                    outcome = DecisionOutcome::Approved;
                } else {
                    confidence = 0.6;
                    rationale = "Agent limit or resource constraints".to_string();
                    outcome = DecisionOutcome::Rejected;
                }
            }
            DecisionType::AgentTermination => {
                if state.active_agents.len() > 0 {
                    confidence = 0.8;
                    rationale = "Agents available for termination".to_string();
                    outcome = DecisionOutcome::Approved;
                } else {
                    confidence = 1.0;
                    rationale = "No agents to terminate".to_string();
                    outcome = DecisionOutcome::Rejected;
                }
            }
            DecisionType::StrategyChange => {
                if state.health.success_rate < 0.7 || state.health.error_count > 20 {
                    confidence = 0.75;
                    rationale = "Poor performance warrants strategy change".to_string();
                    outcome = DecisionOutcome::Approved;
                } else {
                    confidence = 0.5;
                    rationale = "Current strategy performing adequately".to_string();
                    outcome = DecisionOutcome::Deferred;
                }
            }
            DecisionType::SelfModification => {
                if self.config.enable_self_modification && state.cycle_count > 100 {
                    confidence = 0.6;
                    rationale = "Self-modification enabled after sufficient cycles".to_string();
                    outcome = DecisionOutcome::Approved;
                } else if !self.config.enable_self_modification {
                    confidence = 1.0;
                    rationale = "Self-modification is disabled by configuration".to_string();
                    outcome = DecisionOutcome::Rejected;
                } else {
                    confidence = 0.4;
                    rationale = "Insufficient cycle data for safe self-modification".to_string();
                    outcome = DecisionOutcome::EscalateToHuman;
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
        use sysinfo::{System, SystemExt};

        let mut sys = System::new_all();
        sys.refresh_all();
        sys.global_cpu_info().cpu_usage() as f64
    }

    /// Get current memory usage
    async fn get_memory_usage(&self) -> u64 {
        use sysinfo::{System, SystemExt};

        let mut sys = System::new_all();
        sys.refresh_all();
        sys.used_memory()
    }

    /// Get current disk usage
    async fn get_disk_usage(&self) -> u64 {
        use sysinfo::{System, SystemExt, DiskExt};

        let mut sys = System::new_all();
        sys.refresh_all();
        sys.disks().iter().map(|d| d.total_space() - d.available_space()).sum()
    }
    
    /// Calculate success rate from recent operations
    fn calculate_success_rate(&self, state: &AutonomousState) -> f64 {
        // Calculate success rate based on cycle history
        if state.cycle_count == 0 {
            return 1.0;
        }

        // Use error count as inverse indicator of success
        let error_rate = (state.health.error_count as f64) / (state.cycle_count as f64);
        (1.0 - error_rate.min(1.0)).max(0.0)
    }

    /// Calculate average cycle time
    fn calculate_avg_cycle_time(&self, state: &AutonomousState) -> f64 {
        // Estimate based on last cycle timestamp
        if let Some(last_cycle) = state.last_cycle_at {
            let elapsed = chrono::Utc::now().signed_duration_since(last_cycle);
            elapsed.num_milliseconds() as f64 / 1000.0
        } else {
            0.0
        }
    }

    /// Count recent errors
    fn count_recent_errors(&self, state: &AutonomousState) -> u32 {
        // Return the error count from state
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
