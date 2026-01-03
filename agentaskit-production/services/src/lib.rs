//! AgentAsKit Production Services
//!
//! This crate provides core services for the AgentAsKit system:
//! - Health monitoring with golden signals (Latency, Traffic, Errors, Saturation)
//! - Capability matching for agent selection
//! - SLO compliance checking

pub mod health;

// Re-export commonly used types
pub use health::{
    GoldenSignals, LatencyMetrics, TrafficMetrics, ErrorMetrics, SaturationMetrics,
    HealthMonitor, AgentCapabilities, CapabilityMatcher,
};
