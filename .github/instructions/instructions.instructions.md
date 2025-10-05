---
applyTo: '**'
---

# AgentAsKit Development Instructions

## Project Overview

AgentAsKit is a unified multi-agent operating system that combines multiple repositories into a production-ready platform following the "Heal, Don't Harm" principle. This document provides comprehensive guidelines for AI assistants working on this codebase.

## Core Principles

### 1. "Heal, Don't Harm" Philosophy
- **NEVER** remove existing functionality or capabilities
- **ALWAYS** preserve all features from original repositories
- **ENHANCE** existing code rather than replacing it
- **UNIFY** capabilities from multiple sources without loss

### 2. File Unification Rule (CRITICAL)
- **ALWAYS** copy actual source files from source repositories
- **NEVER** create placeholder modules or abstract wrappers
- **PRESERVE** all implementation details and real code
- **MAINTAIN** original file structure and dependencies

## Repository Structure

```
agentaskit/
├── ark-os-production-ready/     # Unified ARK-OS Production System
│   ├── src/
│   │   ├── agents/             # Multi-agent system (24+ agents)
│   │   │   ├── board/          # Strategic governance agents
│   │   │   ├── executive/      # Operational management agents
│   │   │   └── specialized/    # Domain-specific agents
│   │   ├── orchestration/      # Autonomous orchestration engine
│   │   ├── execution/          # Task execution framework
│   │   ├── ui/                 # Tauri desktop application
│   │   └── config/             # System configuration
│   └── Cargo.toml              # Workspace configuration
├── production ready/flexnetos_migration_skeleton/  # FlexNetOS system
├── rustecosys/                 # Tauri desktop framework
├── rustecosys2/                # Advanced orchestration engine
├── agentrs/                    # Multi-agent system source
└── Task/                       # Agent directory and execution kit
```

## Technology Stack & Coding Standards

### Rust Development
- **Language**: Rust 1.70+ with latest stable features
- **Async Runtime**: Tokio for all async operations
- **Serialization**: Serde for JSON, Cap'n Proto for RPC
- **Error Handling**: anyhow::Result for application errors, thiserror for custom errors
- **Logging**: tracing crate with structured logging
- **Testing**: Built-in Rust testing with integration tests

### Code Quality Standards
```rust
// Always use proper error handling
use anyhow::Result;
use thiserror::Error;

// Prefer structured logging
use tracing::{info, error, debug};

// Use async-trait for trait async methods
use async_trait::async_trait;

// Example agent structure
#[async_trait]
pub trait Agent: Send + Sync {
    async fn initialize(&mut self, config: AgentConfig) -> Result<()>;
    async fn execute_task(&self, task: Task) -> Result<TaskResult>;
    async fn health_check(&self) -> Result<HealthStatus>;
}
```

### Workspace Configuration
- Use Cargo workspaces for multi-crate projects
- Shared dependencies in `[workspace.dependencies]`
- Consistent versioning across workspace
- Proper feature flags for optional functionality

## Agent System Architecture

### Agent Hierarchy
1. **Board Agents** (Strategic Level)
   - Finance Board Agent
   - Legal Compliance Board Agent  
   - Operations Board Agent
   - Strategy Board Agent
   - Digest Agent

2. **Executive Agents** (Operational Level)
   - Emergency Responder
   - NOA Commander
   - Priority Manager
   - Resource Allocator
   - System Orchestrator

3. **Specialized Agents** (Domain Expertise)
   - Security Specialist Agent
   - Data Analytics Agent
   - Deployment Agent
   - Monitoring Agent
   - Code Generation Agent
   - Testing Agent
   - Integration Agent
   - Learning Agent

### Agent Implementation Guidelines
```rust
// Standard agent structure
pub struct ExampleAgent {
    id: AgentId,
    config: AgentConfig,
    state: Arc<RwLock<AgentState>>,
    metrics: Arc<RwLock<AgentMetrics>>,
}

impl ExampleAgent {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            id: AgentId::new(),
            config,
            state: Arc::new(RwLock::new(AgentState::Initializing)),
            metrics: Arc::new(RwLock::new(AgentMetrics::default())),
        }
    }
}
```

## Orchestration Engine Guidelines

### Core Features
- **Autonomous Operation**: Self-managing execution cycles
- **Triple Verification**: Three-stage validation for critical operations
- **Auto-Healing**: Automatic error recovery and system repair
- **Resource Management**: Intelligent allocation and constraint handling
- **Parallel Execution**: High-performance concurrent processing

### Configuration Pattern
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationConfig {
    pub max_concurrent_executions: usize,
    pub planning_cycle_duration: Duration,
    pub execution_timeout: Duration,
    pub autonomous_mode: bool,
    pub triple_verification_enabled: bool,
    pub auto_healing_enabled: bool,
}
```

## Desktop Application (Tauri)

### Framework Standards
- **Backend**: Rust with Tauri framework
- **Frontend**: Modern web technologies (HTML/CSS/JS)
- **Communication**: Tauri commands for Rust ↔ Frontend
- **Build**: Cross-platform support (Windows, macOS, Linux)

### Tauri Command Pattern
```rust
#[tauri::command]
async fn get_agent_status(agent_id: String) -> Result<AgentStatus, String> {
    // Implementation
    Ok(status)
}

// Register in main.rs
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_agent_status])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

## Testing Standards

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_agent_initialization() {
        let config = AgentConfig::default();
        let mut agent = ExampleAgent::new(config);
        
        let result = agent.initialize().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
- Place in `tests/` directory
- Test agent interactions and system integration
- Use realistic test data and scenarios
- Mock external dependencies when needed

## Documentation Standards

### Code Documentation
```rust
/// Agent responsible for strategic financial planning and budget allocation.
/// 
/// The Finance Board Agent provides high-level financial oversight and
/// strategic guidance for resource allocation across the entire system.
/// 
/// # Examples
/// 
/// ```rust
/// let config = FinanceBoardConfig::default();
/// let agent = FinanceBoardAgent::new(config);
/// ```
pub struct FinanceBoardAgent {
    // ...
}
```

### README Requirements
- Clear installation instructions
- Usage examples for each major component
- Architecture diagrams and explanations
- Contributing guidelines
- License information

## Dependency Management

### Core Dependencies
```toml
[workspace.dependencies]
anyhow = "1.0"
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
tauri = { version = "1.0", features = ["api-all"] }
```

### Version Management
- Use workspace versioning for consistency
- Pin major versions for stability
- Regular dependency audits for security
- Minimal dependency principle

## Security Guidelines

### Input Validation
```rust
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct TaskRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(range(min = 0, max = 10))]
    pub priority: u8,
}
```

### Error Handling
- Never expose internal errors to external interfaces
- Use structured error types with context
- Log security-relevant events appropriately
- Implement proper timeout and rate limiting

## Performance Guidelines

### Async Best Practices
- Use `Arc<RwLock<T>>` for shared state
- Prefer message passing over shared memory when possible
- Use `tokio::spawn` for CPU-intensive tasks
- Implement proper backpressure mechanisms

### Memory Management
- Use `Box<dyn Trait>` for trait objects
- Implement `Clone` efficiently with `Arc` when needed
- Monitor memory usage in long-running processes
- Use streaming for large data processing

## Git Workflow Standards

### Commit Messages
```
feat: add new security specialist agent

- Implement threat detection capabilities
- Add vulnerability scanning integration
- Include compliance reporting features

Closes #123
```

### Branch Naming
- `feat/agent-security-specialist` - New features
- `fix/orchestration-memory-leak` - Bug fixes
- `docs/api-documentation` - Documentation updates
- `refactor/agent-base-trait` - Code refactoring

### Pull Request Guidelines
- Include comprehensive description
- Add tests for new functionality
- Update documentation as needed
- Ensure all CI checks pass

## AI Assistant Guidelines

### When Working with AgentAsKit:
1. **Always** follow the "Heal, Don't Harm" principle
2. **Never** create placeholder implementations
3. **Preserve** all existing functionality
4. **Use** actual source files from original repositories
5. **Maintain** consistent coding standards
6. **Test** all changes thoroughly
7. **Document** new features and changes

### Code Modification Approach:
```rust
// ... existing code ...
// NEW: Add the specific new functionality here
// ... existing code ...
```

### When Adding New Features:
- Extend existing systems rather than replacing
- Maintain backward compatibility
- Follow established patterns and conventions
- Add comprehensive tests and documentation

## Error Handling Patterns

### Application Errors
```rust
#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Task execution failed: {0}")]
    TaskExecutionFailed(String),
}
```

### Result Types
```rust
pub type AgentResult<T> = Result<T, AgentError>;
pub type SystemResult<T> = Result<T, anyhow::Error>;
```

## Monitoring and Observability

### Metrics Collection
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub average_execution_time: Duration,
    pub resource_utilization: f64,
}
```

### Logging Standards
```rust
use tracing::{info, warn, error, debug, span, Level};

let span = span!(Level::INFO, "agent_execution", agent_id = %self.id);
let _enter = span.enter();

info!("Starting task execution");
// Task execution logic
debug!("Task parameters: {:?}", task.parameters);
```

This document serves as the definitive guide for all AI assistants working on the AgentAsKit project. Follow these guidelines to ensure consistency, quality, and adherence to the project's core principles.