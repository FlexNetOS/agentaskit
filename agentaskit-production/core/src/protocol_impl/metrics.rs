//! MetricsCollectionProtocol Implementation

use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

use shared::protocols::{MetricsCollectionProtocol, TimeRange, DashboardData};
use shared::data_models::{AgentId, SystemMetrics};
use crate::monitoring::MetricsCollector;

/// Concrete implementation of MetricsCollectionProtocol
pub struct MetricsCollectionService {
    metrics_collector: Arc<MetricsCollector>,
    metrics_history: Arc<RwLock<HashMap<AgentId, Vec<SystemMetrics>>>>,
    subscriptions: Arc<RwLock<Vec<AgentId>>>,
}

impl MetricsCollectionService {
    pub fn new(metrics_collector: Arc<MetricsCollector>) -> Self {
        Self {
            metrics_collector,
            metrics_history: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl MetricsCollectionProtocol for MetricsCollectionService {
    async fn report_metrics(&self, metrics: SystemMetrics) -> Result<()> {
        let agent_id = metrics.agent_id;

        // Store in history
        let mut history = self.metrics_history.write().await;
        history.entry(agent_id).or_insert_with(Vec::new).push(metrics.clone());

        // Keep only last 1000 entries per agent
        if let Some(agent_metrics) = history.get_mut(&agent_id) {
            if agent_metrics.len() > 1000 {
                agent_metrics.drain(0..agent_metrics.len() - 1000);
            }
        }

        Ok(())
    }

    async fn get_agent_metrics(&self, agent_id: AgentId, time_range: TimeRange) -> Result<Vec<SystemMetrics>> {
        let history = self.metrics_history.read().await;

        if let Some(agent_metrics) = history.get(&agent_id) {
            let filtered: Vec<_> = agent_metrics.iter()
                .filter(|m| m.timestamp >= time_range.start && m.timestamp <= time_range.end)
                .cloned()
                .collect();
            Ok(filtered)
        } else {
            Ok(Vec::new())
        }
    }

    async fn get_system_metrics(&self, time_range: TimeRange) -> Result<HashMap<AgentId, Vec<SystemMetrics>>> {
        let history = self.metrics_history.read().await;
        let mut result = HashMap::new();

        for (agent_id, metrics) in history.iter() {
            let filtered: Vec<_> = metrics.iter()
                .filter(|m| m.timestamp >= time_range.start && m.timestamp <= time_range.end)
                .cloned()
                .collect();
            if !filtered.is_empty() {
                result.insert(*agent_id, filtered);
            }
        }

        Ok(result)
    }

    async fn subscribe_to_metrics(&self, agent_ids: Vec<AgentId>) -> Result<()> {
        let mut subs = self.subscriptions.write().await;
        for agent_id in agent_ids {
            if !subs.contains(&agent_id) {
                subs.push(agent_id);
            }
        }
        Ok(())
    }

    async fn get_dashboard_data(&self) -> Result<DashboardData> {
        let history = self.metrics_history.read().await;

        // Calculate aggregate metrics
        let total_agents = history.len() as u32;
        let active_agents = history.values()
            .filter(|m| !m.is_empty())
            .count() as u32;

        // Get latest system metrics for calculations
        let mut total_cpu = 0.0;
        let mut total_memory = 0.0;
        let mut total_errors = 0u64;
        let mut total_requests = 0u64;
        let mut count = 0;

        for agent_metrics in history.values() {
            if let Some(latest) = agent_metrics.last() {
                total_cpu += latest.cpu_usage;
                total_memory += latest.memory_usage;
                total_errors += latest.error_count;
                total_requests += latest.request_count;
                count += 1;
            }
        }

        let avg_load = if count > 0 { total_cpu / count as f64 } else { 0.0 };
        let avg_memory = if count > 0 { total_memory / count as f64 } else { 0.0 };
        let error_rate = if total_requests > 0 {
            total_errors as f64 / total_requests as f64
        } else {
            0.0
        };

        Ok(DashboardData {
            total_agents,
            active_agents,
            healthy_agents: active_agents, // Simplified
            total_tasks: 0,
            pending_tasks: 0,
            running_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            system_load: avg_load,
            memory_usage: avg_memory,
            network_throughput: 0.0,
            error_rate,
            response_time_avg: 0.0,
            last_updated: chrono::Utc::now(),
        })
    }
}
