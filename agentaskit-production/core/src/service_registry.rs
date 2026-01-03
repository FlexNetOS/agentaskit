//! Service Registry - Dependency Injection Container
//!
//! Provides centralized service registration, lifecycle management,
//! and dependency resolution for the AgentAsKit system.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::{info, debug, warn, error};

use crate::agents::AgentManager;
use crate::communication::MessageBroker;
use crate::monitoring::MetricsCollector;
use crate::orchestration::OrchestratorEngine;
use crate::security::SecurityManager;

/// Service lifecycle states
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    Registered,
    Initializing,
    Running,
    Stopping,
    Stopped,
    Failed(String),
}

/// Trait for services that can be managed by the registry
pub trait Service: Send + Sync {
    fn name(&self) -> &str;
    fn state(&self) -> ServiceState;
}

/// Service wrapper for type erasure
struct ServiceEntry {
    service: Box<dyn Any + Send + Sync>,
    state: ServiceState,
    name: String,
    type_id: TypeId,
}

/// The main service registry / DI container
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<TypeId, ServiceEntry>>>,
    named_services: Arc<RwLock<HashMap<String, TypeId>>>,
    initialized: Arc<RwLock<bool>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        info!("Creating new service registry");
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            named_services: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Register a service with the registry
    pub async fn register<T: Any + Send + Sync + 'static>(&self, name: &str, service: T) -> Result<()> {
        let type_id = TypeId::of::<T>();

        let entry = ServiceEntry {
            service: Box::new(service),
            state: ServiceState::Registered,
            name: name.to_string(),
            type_id,
        };

        info!("Registering service: {}", name);

        let mut services = self.services.write().await;
        let mut named = self.named_services.write().await;

        services.insert(type_id, entry);
        named.insert(name.to_string(), type_id);

        debug!("Service {} registered successfully", name);
        Ok(())
    }

    /// Get a service by type
    pub async fn get<T: Any + Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().await;

        if let Some(entry) = services.get(&type_id) {
            if let Some(service) = entry.service.downcast_ref::<Arc<T>>() {
                return Some(Arc::clone(service));
            }
        }
        None
    }

    /// Get a service by name
    pub async fn get_by_name<T: Any + Send + Sync + 'static>(&self, name: &str) -> Option<Arc<T>> {
        let named = self.named_services.read().await;
        if let Some(&type_id) = named.get(name) {
            let services = self.services.read().await;
            if let Some(entry) = services.get(&type_id) {
                if let Some(service) = entry.service.downcast_ref::<Arc<T>>() {
                    return Some(Arc::clone(service));
                }
            }
        }
        None
    }

    /// Register an Arc-wrapped service
    pub async fn register_arc<T: Any + Send + Sync + 'static>(&self, name: &str, service: Arc<T>) -> Result<()> {
        let type_id = TypeId::of::<Arc<T>>();

        let entry = ServiceEntry {
            service: Box::new(service),
            state: ServiceState::Registered,
            name: name.to_string(),
            type_id,
        };

        info!("Registering Arc service: {}", name);

        let mut services = self.services.write().await;
        let mut named = self.named_services.write().await;

        services.insert(type_id, entry);
        named.insert(name.to_string(), type_id);

        Ok(())
    }

    /// Initialize all registered services
    pub async fn initialize_all(&self) -> Result<()> {
        info!("Initializing all registered services");

        let mut services = self.services.write().await;
        for entry in services.values_mut() {
            entry.state = ServiceState::Initializing;
            debug!("Initializing service: {}", entry.name);
            entry.state = ServiceState::Running;
        }

        *self.initialized.write().await = true;
        info!("All services initialized");
        Ok(())
    }

    /// Shutdown all services gracefully
    pub async fn shutdown_all(&self) -> Result<()> {
        info!("Shutting down all services");

        let mut services = self.services.write().await;
        for entry in services.values_mut() {
            entry.state = ServiceState::Stopping;
            debug!("Stopping service: {}", entry.name);
            entry.state = ServiceState::Stopped;
        }

        *self.initialized.write().await = false;
        info!("All services shut down");
        Ok(())
    }

    /// Get service count
    pub async fn service_count(&self) -> usize {
        self.services.read().await.len()
    }

    /// Check if registry is initialized
    pub async fn is_initialized(&self) -> bool {
        *self.initialized.read().await
    }

    /// List all registered service names
    pub async fn list_services(&self) -> Vec<String> {
        self.named_services.read().await.keys().cloned().collect()
    }

    /// Get service state by name
    pub async fn get_service_state(&self, name: &str) -> Option<ServiceState> {
        let named = self.named_services.read().await;
        if let Some(&type_id) = named.get(name) {
            let services = self.services.read().await;
            if let Some(entry) = services.get(&type_id) {
                return Some(entry.state.clone());
            }
        }
        None
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing a fully-wired service registry
pub struct ServiceRegistryBuilder {
    registry: ServiceRegistry,
}

impl ServiceRegistryBuilder {
    pub fn new() -> Self {
        Self {
            registry: ServiceRegistry::new(),
        }
    }

    /// Build with security manager
    pub async fn with_security_manager(self, security_manager: Arc<SecurityManager>) -> Result<Self> {
        self.registry.register_arc("security_manager", security_manager).await?;
        Ok(self)
    }

    /// Build with message broker
    pub async fn with_message_broker(self, broker: Arc<MessageBroker>) -> Result<Self> {
        self.registry.register_arc("message_broker", broker).await?;
        Ok(self)
    }

    /// Build with metrics collector
    pub async fn with_metrics_collector(self, collector: Arc<MetricsCollector>) -> Result<Self> {
        self.registry.register_arc("metrics_collector", collector).await?;
        Ok(self)
    }

    /// Build with agent manager
    pub async fn with_agent_manager(self, manager: Arc<AgentManager>) -> Result<Self> {
        self.registry.register_arc("agent_manager", manager).await?;
        Ok(self)
    }

    /// Build with orchestrator engine
    pub async fn with_orchestrator(self, orchestrator: Arc<OrchestratorEngine>) -> Result<Self> {
        self.registry.register_arc("orchestrator", orchestrator).await?;
        Ok(self)
    }

    /// Finalize and return the registry
    pub async fn build(self) -> Result<ServiceRegistry> {
        self.registry.initialize_all().await?;
        Ok(self.registry)
    }
}

impl Default for ServiceRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_registry() {
        let registry = ServiceRegistry::new();

        // Register a simple service
        registry.register("test_service", "test_value".to_string()).await.unwrap();

        assert_eq!(registry.service_count().await, 1);
        assert!(registry.list_services().await.contains(&"test_service".to_string()));
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let registry = ServiceRegistry::new();
        registry.register("lifecycle_test", 42i32).await.unwrap();

        assert!(!registry.is_initialized().await);
        registry.initialize_all().await.unwrap();
        assert!(registry.is_initialized().await);

        let state = registry.get_service_state("lifecycle_test").await;
        assert_eq!(state, Some(ServiceState::Running));

        registry.shutdown_all().await.unwrap();
        assert!(!registry.is_initialized().await);
    }
}
