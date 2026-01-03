//! Real-time Health Monitoring and Capability Matching Service
//! REF: OBS-001 - Real-time health & capability matching
//!
//! This module implements golden-signal monitoring with agent capability matching
//! for the AgentAskit orchestration system.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Golden signals for observability (Latency, Traffic, Errors, Saturation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSignals {
    /// Request latency metrics
    pub latency: LatencyMetrics,
    /// Traffic/throughput metrics
    pub traffic: TrafficMetrics,
    /// Error rate metrics
    pub errors: ErrorMetrics,
    /// Resource saturation metrics
    pub saturation: SaturationMetrics,
    /// Timestamp of measurement
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LatencyMetrics {
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrafficMetrics {
    pub requests_per_second: f64,
    pub messages_per_second: f64,
    pub tasks_per_second: f64,
    pub bytes_per_second: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorMetrics {
    pub error_rate: f64,
    pub error_count: u64,
    pub error_budget_remaining: f64,
    pub errors_by_type: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaturationMetrics {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub disk_utilization: f64,
    pub network_utilization: f64,
    pub queue_depth: u64,
    pub thread_pool_saturation: f64,
}

/// Agent capability for matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub agent_id: String,
    pub capabilities: Vec<String>,
    pub current_load: f64,
    pub max_capacity: u64,
    pub health_status: HealthStatus,
    pub specializations: Vec<String>,
    pub performance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub status: HealthStatus,
    pub checks: Vec<ComponentHealthCheck>,
    pub overall_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealthCheck {
    pub component: String,
    pub status: HealthStatus,
    pub latency_ms: f64,
    pub message: Option<String>,
}

/// Real-time health monitoring service
pub struct HealthMonitoringService {
    /// Current golden signals
    signals: Arc<RwLock<GoldenSignals>>,
    /// Agent capabilities registry
    agent_registry: Arc<RwLock<HashMap<String, AgentCapability>>>,
    /// Health check history
    health_history: Arc<RwLock<Vec<HealthCheckResult>>>,
    /// Alert thresholds
    thresholds: AlertThresholds,
    /// Metrics window (for percentile calculations)
    latency_samples: Arc<RwLock<Vec<f64>>>,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub latency_p99_max_ms: f64,
    pub error_rate_max: f64,
    pub saturation_max: f64,
    pub health_capability_latency_ms: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            latency_p99_max_ms: 50.0,  // SLA target
            error_rate_max: 0.02,       // 2% FPR for alerts
            saturation_max: 0.70,       // 70% saturation threshold
            health_capability_latency_ms: 20.0, // p95 <20ms for health->capability
        }
    }
}

impl HealthMonitoringService {
    pub fn new() -> Self {
        Self {
            signals: Arc::new(RwLock::new(GoldenSignals {
                latency: LatencyMetrics::default(),
                traffic: TrafficMetrics::default(),
                errors: ErrorMetrics::default(),
                saturation: SaturationMetrics::default(),
                timestamp: chrono::Utc::now(),
            })),
            agent_registry: Arc::new(RwLock::new(HashMap::new())),
            health_history: Arc::new(RwLock::new(Vec::new())),
            thresholds: AlertThresholds::default(),
            latency_samples: Arc::new(RwLock::new(Vec::with_capacity(10000))),
        }
    }

    /// Record a latency sample
    pub async fn record_latency(&self, latency_ms: f64) {
        let mut samples = self.latency_samples.write().await;
        samples.push(latency_ms);

        // Keep only last 10000 samples for percentile calculations
        if samples.len() > 10000 {
            samples.remove(0);
        }

        // Update golden signals
        drop(samples);
        self.update_latency_metrics().await;
    }

    /// Update latency metrics from samples
    async fn update_latency_metrics(&self) {
        let samples = self.latency_samples.read().await;
        if samples.is_empty() {
            return;
        }

        let mut sorted: Vec<f64> = samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let len = sorted.len();
        let p50_idx = (len as f64 * 0.50) as usize;
        let p95_idx = (len as f64 * 0.95) as usize;
        let p99_idx = (len as f64 * 0.99) as usize;

        let mut signals = self.signals.write().await;
        signals.latency = LatencyMetrics {
            p50_ms: sorted.get(p50_idx).copied().unwrap_or(0.0),
            p95_ms: sorted.get(p95_idx.min(len - 1)).copied().unwrap_or(0.0),
            p99_ms: sorted.get(p99_idx.min(len - 1)).copied().unwrap_or(0.0),
            avg_ms: sorted.iter().sum::<f64>() / len as f64,
            min_ms: sorted.first().copied().unwrap_or(0.0),
            max_ms: sorted.last().copied().unwrap_or(0.0),
        };
        signals.timestamp = chrono::Utc::now();
    }

    /// Record an error
    pub async fn record_error(&self, error_type: &str) {
        let mut signals = self.signals.write().await;
        signals.errors.error_count += 1;
        *signals.errors.errors_by_type.entry(error_type.to_string()).or_insert(0) += 1;
    }

    /// Update traffic metrics
    pub async fn update_traffic(&self, traffic: TrafficMetrics) {
        let mut signals = self.signals.write().await;
        signals.traffic = traffic;
        signals.timestamp = chrono::Utc::now();
    }

    /// Update saturation metrics
    pub async fn update_saturation(&self, saturation: SaturationMetrics) {
        let mut signals = self.signals.write().await;
        signals.saturation = saturation;
        signals.timestamp = chrono::Utc::now();
    }

    /// Get current golden signals
    pub async fn get_golden_signals(&self) -> GoldenSignals {
        self.signals.read().await.clone()
    }

    /// Register an agent with capabilities
    pub async fn register_agent(&self, capability: AgentCapability) {
        let mut registry = self.agent_registry.write().await;
        registry.insert(capability.agent_id.clone(), capability);
    }

    /// Update agent health status
    pub async fn update_agent_health(&self, agent_id: &str, status: HealthStatus) {
        let mut registry = self.agent_registry.write().await;
        if let Some(agent) = registry.get_mut(agent_id) {
            agent.health_status = status;
        }
    }

    /// Match capabilities to find best agent for a task
    /// Target: p95 <20ms for health->capability matching
    pub async fn match_capability(&self, required_capabilities: &[String]) -> Result<Option<String>> {
        let start = Instant::now();

        let registry = self.agent_registry.read().await;

        let mut best_match: Option<(String, f64)> = None;

        for (agent_id, capability) in registry.iter() {
            // Skip unhealthy agents
            if capability.health_status != HealthStatus::Healthy {
                continue;
            }

            // Calculate capability match score
            let matched_caps: usize = required_capabilities.iter()
                .filter(|req| capability.capabilities.contains(req) ||
                              capability.specializations.contains(req))
                .count();

            if matched_caps == 0 {
                continue;
            }

            // Score based on: capability match, load, performance
            let match_ratio = matched_caps as f64 / required_capabilities.len() as f64;
            let load_score = 1.0 - capability.current_load;
            let score = match_ratio * 0.5 + load_score * 0.3 + capability.performance_score * 0.2;

            if best_match.is_none() || score > best_match.as_ref().unwrap().1 {
                best_match = Some((agent_id.clone(), score));
            }
        }

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        self.record_latency(elapsed).await;

        // Check if we met the p95 target
        if elapsed > self.thresholds.health_capability_latency_ms {
            log::warn!("Capability matching took {}ms, exceeds {}ms target",
                      elapsed, self.thresholds.health_capability_latency_ms);
        }

        Ok(best_match.map(|(id, _)| id))
    }

    /// Perform health check on all components
    pub async fn health_check(&self) -> HealthCheckResult {
        let mut checks = Vec::new();
        let mut total_score = 0.0;

        // Check agent registry health
        let registry = self.agent_registry.read().await;
        let healthy_agents = registry.values()
            .filter(|a| a.health_status == HealthStatus::Healthy)
            .count();
        let total_agents = registry.len();
        let agent_health_ratio = if total_agents > 0 {
            healthy_agents as f64 / total_agents as f64
        } else {
            1.0
        };

        checks.push(ComponentHealthCheck {
            component: "agent_registry".to_string(),
            status: if agent_health_ratio > 0.9 { HealthStatus::Healthy }
                   else if agent_health_ratio > 0.5 { HealthStatus::Degraded }
                   else { HealthStatus::Unhealthy },
            latency_ms: 0.1,
            message: Some(format!("{}/{} agents healthy", healthy_agents, total_agents)),
        });
        total_score += agent_health_ratio;
        drop(registry);

        // Check golden signals health
        let signals = self.signals.read().await;

        // Latency check
        let latency_status = if signals.latency.p99_ms <= self.thresholds.latency_p99_max_ms {
            HealthStatus::Healthy
        } else if signals.latency.p99_ms <= self.thresholds.latency_p99_max_ms * 2.0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };
        checks.push(ComponentHealthCheck {
            component: "latency".to_string(),
            status: latency_status.clone(),
            latency_ms: signals.latency.p99_ms,
            message: Some(format!("p99: {:.2}ms", signals.latency.p99_ms)),
        });
        total_score += if latency_status == HealthStatus::Healthy { 1.0 }
                       else if latency_status == HealthStatus::Degraded { 0.5 }
                       else { 0.0 };

        // Error rate check
        let error_status = if signals.errors.error_rate <= self.thresholds.error_rate_max {
            HealthStatus::Healthy
        } else if signals.errors.error_rate <= self.thresholds.error_rate_max * 2.0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };
        checks.push(ComponentHealthCheck {
            component: "errors".to_string(),
            status: error_status.clone(),
            latency_ms: 0.0,
            message: Some(format!("error_rate: {:.4}", signals.errors.error_rate)),
        });
        total_score += if error_status == HealthStatus::Healthy { 1.0 }
                       else if error_status == HealthStatus::Degraded { 0.5 }
                       else { 0.0 };

        // Saturation check
        let max_saturation = signals.saturation.cpu_utilization
            .max(signals.saturation.memory_utilization)
            .max(signals.saturation.thread_pool_saturation);
        let saturation_status = if max_saturation <= self.thresholds.saturation_max {
            HealthStatus::Healthy
        } else if max_saturation <= 0.9 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };
        checks.push(ComponentHealthCheck {
            component: "saturation".to_string(),
            status: saturation_status.clone(),
            latency_ms: 0.0,
            message: Some(format!("max: {:.2}%", max_saturation * 100.0)),
        });
        total_score += if saturation_status == HealthStatus::Healthy { 1.0 }
                       else if saturation_status == HealthStatus::Degraded { 0.5 }
                       else { 0.0 };

        let overall_score = total_score / 4.0;
        let overall_status = if overall_score >= 0.9 { HealthStatus::Healthy }
                            else if overall_score >= 0.5 { HealthStatus::Degraded }
                            else { HealthStatus::Unhealthy };

        let result = HealthCheckResult {
            status: overall_status,
            checks,
            overall_score,
            timestamp: chrono::Utc::now(),
        };

        // Store in history
        let mut history = self.health_history.write().await;
        history.push(result.clone());
        if history.len() > 1000 {
            history.remove(0);
        }

        result
    }

    /// Export metrics in Prometheus format
    pub async fn export_prometheus_metrics(&self) -> String {
        let signals = self.signals.read().await;
        let registry = self.agent_registry.read().await;

        let mut output = String::new();

        // Latency metrics
        output.push_str(&format!("# HELP agentaskit_latency_seconds Request latency\n"));
        output.push_str(&format!("# TYPE agentaskit_latency_seconds summary\n"));
        output.push_str(&format!("agentaskit_latency_seconds{{quantile=\"0.5\"}} {:.6}\n", signals.latency.p50_ms / 1000.0));
        output.push_str(&format!("agentaskit_latency_seconds{{quantile=\"0.95\"}} {:.6}\n", signals.latency.p95_ms / 1000.0));
        output.push_str(&format!("agentaskit_latency_seconds{{quantile=\"0.99\"}} {:.6}\n", signals.latency.p99_ms / 1000.0));

        // Traffic metrics
        output.push_str(&format!("# HELP agentaskit_requests_per_second Request throughput\n"));
        output.push_str(&format!("# TYPE agentaskit_requests_per_second gauge\n"));
        output.push_str(&format!("agentaskit_requests_per_second {:.2}\n", signals.traffic.requests_per_second));
        output.push_str(&format!("agentaskit_tasks_per_second {:.2}\n", signals.traffic.tasks_per_second));
        output.push_str(&format!("agentaskit_messages_per_second {:.2}\n", signals.traffic.messages_per_second));

        // Error metrics
        output.push_str(&format!("# HELP agentaskit_errors_total Total error count\n"));
        output.push_str(&format!("# TYPE agentaskit_errors_total counter\n"));
        output.push_str(&format!("agentaskit_errors_total {}\n", signals.errors.error_count));
        output.push_str(&format!("agentaskit_error_rate {:.6}\n", signals.errors.error_rate));

        // Saturation metrics
        output.push_str(&format!("# HELP agentaskit_saturation Resource saturation\n"));
        output.push_str(&format!("# TYPE agentaskit_saturation gauge\n"));
        output.push_str(&format!("agentaskit_saturation{{resource=\"cpu\"}} {:.4}\n", signals.saturation.cpu_utilization));
        output.push_str(&format!("agentaskit_saturation{{resource=\"memory\"}} {:.4}\n", signals.saturation.memory_utilization));
        output.push_str(&format!("agentaskit_saturation{{resource=\"disk\"}} {:.4}\n", signals.saturation.disk_utilization));

        // Agent metrics
        output.push_str(&format!("# HELP agentaskit_agents_total Total registered agents\n"));
        output.push_str(&format!("# TYPE agentaskit_agents_total gauge\n"));
        output.push_str(&format!("agentaskit_agents_total {}\n", registry.len()));

        let healthy = registry.values().filter(|a| a.health_status == HealthStatus::Healthy).count();
        output.push_str(&format!("agentaskit_agents_healthy {}\n", healthy));

        output
    }
}

impl Default for HealthMonitoringService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_latency_recording() {
        let service = HealthMonitoringService::new();

        for i in 0..100 {
            service.record_latency(i as f64).await;
        }

        let signals = service.get_golden_signals().await;
        assert!(signals.latency.p50_ms > 0.0);
        assert!(signals.latency.p99_ms >= signals.latency.p50_ms);
    }

    #[tokio::test]
    async fn test_capability_matching() {
        let service = HealthMonitoringService::new();

        service.register_agent(AgentCapability {
            agent_id: "agent-1".to_string(),
            capabilities: vec!["rust".to_string(), "testing".to_string()],
            current_load: 0.3,
            max_capacity: 100,
            health_status: HealthStatus::Healthy,
            specializations: vec!["performance".to_string()],
            performance_score: 0.9,
        }).await;

        let result = service.match_capability(&["rust".to_string()]).await.unwrap();
        assert_eq!(result, Some("agent-1".to_string()));
    }

    #[tokio::test]
    async fn test_health_check() {
        let service = HealthMonitoringService::new();
        let result = service.health_check().await;
        assert!(!result.checks.is_empty());
    }
}
