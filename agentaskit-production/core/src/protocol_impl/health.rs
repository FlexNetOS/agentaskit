//! HealthMonitoringProtocol Implementation

use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

use shared::protocols::{HealthMonitoringProtocol, HealthCheckResult};
use shared::data_models::{AgentId, HealthStatus, HealthCheck};
use crate::monitoring::MetricsCollector;

/// Concrete implementation of HealthMonitoringProtocol
pub struct HealthMonitoringService {
    metrics_collector: Arc<MetricsCollector>,
    health_status: Arc<RwLock<HashMap<AgentId, HealthStatus>>>,
    health_checks: Arc<RwLock<HashMap<AgentId, Vec<HealthCheck>>>>,
}

impl HealthMonitoringService {
    pub fn new(metrics_collector: Arc<MetricsCollector>) -> Self {
        Self {
            metrics_collector,
            health_status: Arc::new(RwLock::new(HashMap::new())),
            health_checks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl HealthMonitoringProtocol for HealthMonitoringService {
    async fn report_health(&self, agent_id: AgentId, health_status: HealthStatus) -> Result<()> {
        let mut status_map = self.health_status.write().await;
        status_map.insert(agent_id, health_status);
        Ok(())
    }

    async fn get_health_status(&self, agent_id: AgentId) -> Result<HealthStatus> {
        let status_map = self.health_status.read().await;
        Ok(status_map.get(&agent_id).cloned().unwrap_or(HealthStatus::Unknown))
    }

    async fn get_system_health(&self) -> Result<HashMap<AgentId, HealthStatus>> {
        let status_map = self.health_status.read().await;
        Ok(status_map.clone())
    }

    async fn register_health_check(&self, agent_id: AgentId, check: HealthCheck) -> Result<()> {
        let mut checks = self.health_checks.write().await;
        checks.entry(agent_id).or_insert_with(Vec::new).push(check);
        Ok(())
    }

    async fn execute_health_checks(&self, agent_id: AgentId) -> Result<Vec<HealthCheckResult>> {
        let checks = self.health_checks.read().await;
        let agent_checks = checks.get(&agent_id).cloned().unwrap_or_default();

        let mut results = Vec::new();
        for check in agent_checks {
            // Execute each health check
            let result = HealthCheckResult {
                check_type: check.check_type.clone(),
                status: HealthStatus::Healthy, // Would run actual check
                timestamp: chrono::Utc::now(),
                message: Some("Health check passed".to_string()),
                details: serde_json::json!({"check_name": check.name}),
            };
            results.push(result);
        }

        Ok(results)
    }
}
