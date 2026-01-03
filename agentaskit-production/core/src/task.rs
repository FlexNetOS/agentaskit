//! Task types and definitions for the AgentAsKit system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Task execution type variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskExecutionType {
    /// AI/LLM-based task
    AIInstruction {
        instructions: String,
        config: HashMap<String, serde_json::Value>,
    },
    /// Generic shell command execution
    Shell {
        command: String,
        args: Vec<String>,
        working_dir: Option<String>,
        env: HashMap<String, String>,
    },
    /// File system operation
    FileSystem {
        operation: String, // copy, move, create, delete, etc.
        source: String,
        destination: Option<String>,
        options: HashMap<String, String>,
    },
    /// Custom Rust code execution
    RustCode {
        code: String,
        dependencies: Vec<String>,
        entry_point: String,
    },
}

/// Complete task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDefinition {
    /// Human-readable task name
    pub name: String,
    /// Task description
    pub description: String,
    /// Execution type
    pub execution_type: TaskExecutionType,
    /// Task priority (0-100)
    pub priority: u8,
    /// Timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Retry configuration
    pub max_retries: u32,
}
