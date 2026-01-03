//! AI Module - Model Selection and SOP Analysis
//!
//! Provides AI-powered capabilities for the AgentAsKit system:
//! - Model selector bridge for local LLM inference via llama.cpp
//! - SOP analyzer for procedure understanding
//!
//! ## Feature Flags
//! - `llama-cpp`: Enable local model inference via llama.cpp

pub mod model_selector_bridge;
pub mod sop_analyzer;

// Re-export commonly used types
pub use model_selector_bridge::{ModelSelectorBridge, ModelRequirements, LocalModel};
pub use sop_analyzer::analyze;
