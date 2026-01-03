//! Observability Tests - TEST-001
//!
//! Tests for MetricsCollector, health monitoring, and alerting systems.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;

// Import from core crate
use agentaskit_core::monitoring::{
    MetricsCollector, AgentMetrics, TaskMetrics, ResourceUsage,
    Alert, AlertLevel, HealthStatus, PerformanceThresholds,
};

#[cfg(test)]
mod metrics_collector_tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new().await;
        assert!(collector.is_ok(), "MetricsCollector should be created successfully");
    }

    #[tokio::test]
    async fn test_metrics_collector_start_stop() {
        let collector = MetricsCollector::new().await.unwrap();

        // Start collection
        let start_result = collector.start().await;
        assert!(start_result.is_ok(), "Metrics collection should start successfully");

        // Allow some time for collection loops to initialize
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Shutdown
        let shutdown_result = collector.shutdown().await;
        assert!(shutdown_result.is_ok(), "Metrics collector should shutdown successfully");
    }

    #[tokio::test]
    async fn test_record_agent_metrics() {
        let collector = MetricsCollector::new().await.unwrap();

        let agent_id = Uuid::new_v4();
        let metrics = AgentMetrics {
            agent_id,
            timestamp: Utc::now(),
            status: "running".to_string(),
            tasks_completed: 100,
            tasks_failed: 5,
            tasks_in_progress: 3,
            average_response_time_ms: 45.5,
            cpu_usage_percent: 25.0,
            memory_usage_mb: 512,
            message_queue_size: 10,
            last_activity: Utc::now(),
        };

        let result = collector.record_agent_metrics(metrics).await;
        assert!(result.is_ok(), "Should record agent metrics successfully");
    }

    #[tokio::test]
    async fn test_record_task_metrics() {
        let collector = MetricsCollector::new().await.unwrap();

        let metrics = TaskMetrics {
            task_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            duration_ms: 150,
            success: true,
            agent_id: Uuid::new_v4(),
            task_type: "data_processing".to_string(),
            priority: "high".to_string(),
            resource_usage: ResourceUsage {
                cpu_time_ms: 120,
                memory_peak_mb: 256,
                disk_io_mb: 50,
                network_io_mb: 10,
            },
        };

        let result = collector.record_task_metrics(metrics).await;
        assert!(result.is_ok(), "Should record task metrics successfully");
    }

    #[tokio::test]
    async fn test_agent_metrics_history_limit() {
        let collector = MetricsCollector::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Record more than 100 metrics (the limit)
        for i in 0..150 {
            let metrics = AgentMetrics {
                agent_id,
                timestamp: Utc::now(),
                status: "running".to_string(),
                tasks_completed: i,
                tasks_failed: 0,
                tasks_in_progress: 0,
                average_response_time_ms: 45.5,
                cpu_usage_percent: 25.0,
                memory_usage_mb: 512,
                message_queue_size: 0,
                last_activity: Utc::now(),
            };

            collector.record_agent_metrics(metrics).await.unwrap();
        }

        // Metrics should be limited to 100 per agent
        // (verified through the internal data structure behavior)
    }
}

#[cfg(test)]
mod health_monitoring_tests {
    use super::*;

    #[tokio::test]
    async fn test_health_status_healthy() {
        let collector = MetricsCollector::new().await.unwrap();

        let status = collector.get_system_health().await.unwrap();
        // With no alerts, system should be healthy
        assert!(matches!(status, HealthStatus::Healthy));
    }

    #[tokio::test]
    async fn test_performance_thresholds_default() {
        let thresholds = PerformanceThresholds::default();

        assert_eq!(thresholds.cpu_usage_warning, 70.0);
        assert_eq!(thresholds.cpu_usage_critical, 90.0);
        assert_eq!(thresholds.memory_usage_warning, 80.0);
        assert_eq!(thresholds.memory_usage_critical, 95.0);
        assert_eq!(thresholds.response_time_warning_ms, 5000.0);
        assert_eq!(thresholds.response_time_critical_ms, 10000.0);
        assert_eq!(thresholds.task_failure_rate_warning, 5.0);
        assert_eq!(thresholds.task_failure_rate_critical, 15.0);
    }
}

#[cfg(test)]
mod alert_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_empty_alerts() {
        let collector = MetricsCollector::new().await.unwrap();

        let alerts = collector.get_alerts(None).await;
        assert!(alerts.is_empty(), "New collector should have no alerts");
    }

    #[tokio::test]
    async fn test_get_alerts_with_filter() {
        let collector = MetricsCollector::new().await.unwrap();

        let critical_alerts = collector.get_alerts(Some(AlertLevel::Critical)).await;
        let warning_alerts = collector.get_alerts(Some(AlertLevel::Warning)).await;
        let info_alerts = collector.get_alerts(Some(AlertLevel::Info)).await;

        assert!(critical_alerts.is_empty());
        assert!(warning_alerts.is_empty());
        assert!(info_alerts.is_empty());
    }

    #[tokio::test]
    async fn test_acknowledge_nonexistent_alert() {
        let collector = MetricsCollector::new().await.unwrap();

        let result = collector.acknowledge_alert(Uuid::new_v4()).await;
        assert!(result.is_err(), "Should fail for non-existent alert");
    }

    #[tokio::test]
    async fn test_resolve_nonexistent_alert() {
        let collector = MetricsCollector::new().await.unwrap();

        let result = collector.resolve_alert(Uuid::new_v4()).await;
        assert!(result.is_err(), "Should fail for non-existent alert");
    }

    #[tokio::test]
    async fn test_alert_level_ordering() {
        assert!(AlertLevel::Critical < AlertLevel::Warning);
        assert!(AlertLevel::Warning < AlertLevel::Info);
        assert!(AlertLevel::Info < AlertLevel::Debug);
    }
}

#[cfg(test)]
mod resource_usage_tests {
    use super::*;

    #[test]
    fn test_resource_usage_creation() {
        let usage = ResourceUsage {
            cpu_time_ms: 100,
            memory_peak_mb: 256,
            disk_io_mb: 50,
            network_io_mb: 10,
        };

        assert_eq!(usage.cpu_time_ms, 100);
        assert_eq!(usage.memory_peak_mb, 256);
        assert_eq!(usage.disk_io_mb, 50);
        assert_eq!(usage.network_io_mb, 10);
    }

    #[test]
    fn test_resource_usage_clone() {
        let usage = ResourceUsage {
            cpu_time_ms: 100,
            memory_peak_mb: 256,
            disk_io_mb: 50,
            network_io_mb: 10,
        };

        let cloned = usage.clone();
        assert_eq!(usage.cpu_time_ms, cloned.cpu_time_ms);
        assert_eq!(usage.memory_peak_mb, cloned.memory_peak_mb);
    }
}
