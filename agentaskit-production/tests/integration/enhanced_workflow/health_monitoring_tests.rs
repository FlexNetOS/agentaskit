//! Integration tests for Health Monitoring
//!
//! Tests validate:
//! - Golden signals (Latency, Traffic, Errors, Saturation)
//! - Agent capability matching
//! - Alert thresholds

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Golden signals for health monitoring
#[derive(Debug, Clone)]
struct GoldenSignals {
    latency_p50_ms: f64,
    latency_p95_ms: f64,
    latency_p99_ms: f64,
    requests_per_second: f64,
    error_rate: f64,
    saturation_cpu: f64,
    saturation_memory: f64,
}

impl Default for GoldenSignals {
    fn default() -> Self {
        Self {
            latency_p50_ms: 5.0,
            latency_p95_ms: 15.0,
            latency_p99_ms: 50.0,
            requests_per_second: 100.0,
            error_rate: 0.001,
            saturation_cpu: 0.60,
            saturation_memory: 0.50,
        }
    }
}

/// Agent capability for matching
#[derive(Debug, Clone)]
struct AgentCapability {
    id: String,
    name: String,
    capabilities: Vec<String>,
    health_score: f64,
    load: f64,
}

/// Alert thresholds
struct AlertThresholds {
    latency_warning_ms: f64,
    latency_critical_ms: f64,
    error_rate_warning: f64,
    error_rate_critical: f64,
    saturation_warning: f64,
    saturation_critical: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            latency_warning_ms: 100.0,
            latency_critical_ms: 500.0,
            error_rate_warning: 0.01,
            error_rate_critical: 0.05,
            saturation_warning: 0.70,
            saturation_critical: 0.90,
        }
    }
}

/// Check health status
fn check_health(signals: &GoldenSignals, thresholds: &AlertThresholds) -> String {
    // Check error rate first (most critical)
    if signals.error_rate >= thresholds.error_rate_critical {
        return "critical".to_string();
    }

    // Check saturation
    if signals.saturation_cpu >= thresholds.saturation_critical
        || signals.saturation_memory >= thresholds.saturation_critical
    {
        return "critical".to_string();
    }

    // Check latency
    if signals.latency_p99_ms >= thresholds.latency_critical_ms {
        return "critical".to_string();
    }

    // Check warnings
    if signals.error_rate >= thresholds.error_rate_warning
        || signals.latency_p99_ms >= thresholds.latency_warning_ms
        || signals.saturation_cpu >= thresholds.saturation_warning
        || signals.saturation_memory >= thresholds.saturation_warning
    {
        return "warning".to_string();
    }

    "healthy".to_string()
}

/// Match agent to task based on capabilities
fn match_agent(
    required_capabilities: &[String],
    agents: &[AgentCapability],
) -> Option<AgentCapability> {
    let mut best_match: Option<(AgentCapability, usize)> = None;

    for agent in agents {
        if agent.health_score < 0.5 || agent.load > 0.9 {
            continue; // Skip unhealthy or overloaded agents
        }

        let matches = required_capabilities
            .iter()
            .filter(|cap| agent.capabilities.contains(cap))
            .count();

        if matches == required_capabilities.len() {
            // Perfect match - prefer lower load
            match &best_match {
                None => best_match = Some((agent.clone(), matches)),
                Some((current, _)) => {
                    if agent.load < current.load {
                        best_match = Some((agent.clone(), matches));
                    }
                }
            }
        }
    }

    best_match.map(|(agent, _)| agent)
}

#[test]
fn test_golden_signals_defaults() {
    let signals = GoldenSignals::default();

    assert!(signals.latency_p50_ms < signals.latency_p95_ms);
    assert!(signals.latency_p95_ms < signals.latency_p99_ms);
    assert!(signals.error_rate < 0.01);
    assert!(signals.saturation_cpu <= 1.0);
    assert!(signals.saturation_memory <= 1.0);
}

#[test]
fn test_healthy_status() {
    let signals = GoldenSignals::default();
    let thresholds = AlertThresholds::default();

    let status = check_health(&signals, &thresholds);
    assert_eq!(status, "healthy");
}

#[test]
fn test_warning_status() {
    let signals = GoldenSignals {
        error_rate: 0.02, // Above warning threshold
        ..Default::default()
    };
    let thresholds = AlertThresholds::default();

    let status = check_health(&signals, &thresholds);
    assert_eq!(status, "warning");
}

#[test]
fn test_critical_status_error_rate() {
    let signals = GoldenSignals {
        error_rate: 0.10, // Above critical threshold
        ..Default::default()
    };
    let thresholds = AlertThresholds::default();

    let status = check_health(&signals, &thresholds);
    assert_eq!(status, "critical");
}

#[test]
fn test_critical_status_saturation() {
    let signals = GoldenSignals {
        saturation_cpu: 0.95, // Above critical threshold
        ..Default::default()
    };
    let thresholds = AlertThresholds::default();

    let status = check_health(&signals, &thresholds);
    assert_eq!(status, "critical");
}

#[test]
fn test_critical_status_latency() {
    let signals = GoldenSignals {
        latency_p99_ms: 600.0, // Above critical threshold
        ..Default::default()
    };
    let thresholds = AlertThresholds::default();

    let status = check_health(&signals, &thresholds);
    assert_eq!(status, "critical");
}

#[test]
fn test_agent_matching() {
    let agents = vec![
        AgentCapability {
            id: "agent-1".to_string(),
            name: "Code Agent".to_string(),
            capabilities: vec!["code".to_string(), "rust".to_string()],
            health_score: 0.95,
            load: 0.3,
        },
        AgentCapability {
            id: "agent-2".to_string(),
            name: "Doc Agent".to_string(),
            capabilities: vec!["documentation".to_string(), "markdown".to_string()],
            health_score: 0.90,
            load: 0.5,
        },
    ];

    let required = vec!["code".to_string(), "rust".to_string()];
    let matched = match_agent(&required, &agents);

    assert!(matched.is_some());
    assert_eq!(matched.unwrap().id, "agent-1");
}

#[test]
fn test_agent_matching_prefers_lower_load() {
    let agents = vec![
        AgentCapability {
            id: "agent-1".to_string(),
            name: "Code Agent 1".to_string(),
            capabilities: vec!["code".to_string()],
            health_score: 0.95,
            load: 0.8,
        },
        AgentCapability {
            id: "agent-2".to_string(),
            name: "Code Agent 2".to_string(),
            capabilities: vec!["code".to_string()],
            health_score: 0.90,
            load: 0.2,
        },
    ];

    let required = vec!["code".to_string()];
    let matched = match_agent(&required, &agents);

    assert!(matched.is_some());
    assert_eq!(matched.unwrap().id, "agent-2"); // Lower load
}

#[test]
fn test_agent_matching_skips_unhealthy() {
    let agents = vec![
        AgentCapability {
            id: "agent-1".to_string(),
            name: "Unhealthy Agent".to_string(),
            capabilities: vec!["code".to_string()],
            health_score: 0.3, // Too low
            load: 0.1,
        },
        AgentCapability {
            id: "agent-2".to_string(),
            name: "Healthy Agent".to_string(),
            capabilities: vec!["code".to_string()],
            health_score: 0.8,
            load: 0.5,
        },
    ];

    let required = vec!["code".to_string()];
    let matched = match_agent(&required, &agents);

    assert!(matched.is_some());
    assert_eq!(matched.unwrap().id, "agent-2");
}

#[test]
fn test_agent_matching_skips_overloaded() {
    let agents = vec![
        AgentCapability {
            id: "agent-1".to_string(),
            name: "Overloaded Agent".to_string(),
            capabilities: vec!["code".to_string()],
            health_score: 0.9,
            load: 0.95, // Too high
        },
    ];

    let required = vec!["code".to_string()];
    let matched = match_agent(&required, &agents);

    assert!(matched.is_none());
}

#[test]
fn test_agent_matching_no_match() {
    let agents = vec![
        AgentCapability {
            id: "agent-1".to_string(),
            name: "Wrong Capabilities".to_string(),
            capabilities: vec!["documentation".to_string()],
            health_score: 0.9,
            load: 0.3,
        },
    ];

    let required = vec!["code".to_string(), "rust".to_string()];
    let matched = match_agent(&required, &agents);

    assert!(matched.is_none());
}

#[test]
fn test_latency_recording() {
    let mut latencies = Vec::new();

    // Simulate request latencies
    for i in 0..100 {
        let latency = (i as f64 * 0.5) + 1.0; // 1.0 to 50.5ms
        latencies.push(latency);
    }

    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let p50 = latencies[50];
    let p95 = latencies[95];
    let p99 = latencies[99];

    assert!(p50 < p95);
    assert!(p95 < p99);
    assert!(p99 <= 51.0);
}

#[test]
fn test_error_rate_calculation() {
    let total_requests = 10000;
    let failed_requests = 50;

    let error_rate = failed_requests as f64 / total_requests as f64;

    assert_eq!(error_rate, 0.005);
    assert!(error_rate < AlertThresholds::default().error_rate_warning);
}

#[test]
fn test_saturation_calculation() {
    let cpu_used = 70.0;
    let cpu_total = 100.0;
    let memory_used_gb = 6.0;
    let memory_total_gb = 16.0;

    let cpu_saturation = cpu_used / cpu_total;
    let memory_saturation = memory_used_gb / memory_total_gb;

    assert_eq!(cpu_saturation, 0.70);
    assert!((memory_saturation - 0.375).abs() < 0.001);
}

#[test]
fn test_health_check_performance() {
    let signals = GoldenSignals::default();
    let thresholds = AlertThresholds::default();

    let iterations = 10000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = check_health(&signals, &thresholds);
    }

    let elapsed = start.elapsed();
    let per_check = elapsed / iterations as u32;

    assert!(
        per_check < Duration::from_micros(10),
        "Health check took {:?}, should be < 10us",
        per_check
    );
}
