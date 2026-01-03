//! Model Selector Bridge for llama.cpp Integration
//!
//! Provides a Rust interface to local LLM inference via llama.cpp.
//! This module is gated behind the `llama-cpp` feature flag.

use std::path::PathBuf;

/// Model requirements for selection
#[derive(Debug, Clone)]
pub struct ModelRequirements {
    /// Minimum context length
    pub min_context_length: usize,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: u64,
    /// Required capabilities (e.g., "code", "chat", "instruct")
    pub capabilities: Vec<String>,
    /// Preferred quantization level (e.g., "q4_0", "q8_0")
    pub quantization: Option<String>,
}

/// A local model available for inference
#[derive(Debug, Clone)]
pub struct LocalModel {
    pub name: String,
    pub path: PathBuf,
    pub context_length: usize,
    pub quantization: String,
    pub capabilities: Vec<String>,
}

/// Model selector for local inference
pub struct ModelSelectorBridge {
    models_dir: PathBuf,
    available_models: Vec<LocalModel>,
}

impl ModelSelectorBridge {
    /// Create a new model selector bridge
    pub fn new(models_dir: PathBuf) -> Self {
        Self {
            models_dir,
            available_models: Vec::new(),
        }
    }

    /// Scan for available models in the models directory
    pub fn scan_models(&mut self) -> Result<usize, String> {
        self.available_models.clear();

        if !self.models_dir.exists() {
            return Err(format!("Models directory not found: {:?}", self.models_dir));
        }

        // Scan for GGUF model files
        for entry in std::fs::read_dir(&self.models_dir)
            .map_err(|e| format!("Failed to read models dir: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "gguf") {
                if let Some(model) = self.parse_model_metadata(&path) {
                    self.available_models.push(model);
                }
            }
        }

        Ok(self.available_models.len())
    }

    /// Parse model metadata from filename and file
    fn parse_model_metadata(&self, path: &PathBuf) -> Option<LocalModel> {
        let filename = path.file_stem()?.to_string_lossy().to_string();

        // Parse quantization from filename (e.g., "model-q4_0.gguf")
        let quantization = if filename.contains("q4_0") {
            "q4_0"
        } else if filename.contains("q4_1") {
            "q4_1"
        } else if filename.contains("q5_0") {
            "q5_0"
        } else if filename.contains("q5_1") {
            "q5_1"
        } else if filename.contains("q8_0") {
            "q8_0"
        } else {
            "unknown"
        };

        // Infer capabilities from filename
        let mut capabilities = Vec::new();
        if filename.to_lowercase().contains("code") {
            capabilities.push("code".to_string());
        }
        if filename.to_lowercase().contains("instruct") {
            capabilities.push("instruct".to_string());
        }
        if filename.to_lowercase().contains("chat") {
            capabilities.push("chat".to_string());
        }
        if capabilities.is_empty() {
            capabilities.push("general".to_string());
        }

        Some(LocalModel {
            name: filename,
            path: path.clone(),
            context_length: 4096, // Default, would read from model metadata
            quantization: quantization.to_string(),
            capabilities,
        })
    }

    /// Select the best model matching requirements
    #[cfg(feature = "llama-cpp")]
    pub fn select_model(&self, requirements: &ModelRequirements) -> Option<LocalModel> {
        self.available_models
            .iter()
            .filter(|model| {
                // Check context length
                model.context_length >= requirements.min_context_length
            })
            .filter(|model| {
                // Check capabilities
                requirements.capabilities.iter().all(|cap| {
                    model.capabilities.contains(cap)
                })
            })
            .filter(|model| {
                // Check quantization if specified
                requirements.quantization.as_ref().map_or(true, |q| {
                    &model.quantization == q
                })
            })
            .cloned()
            .next()
    }

    /// Select model (stub when llama-cpp feature is disabled)
    #[cfg(not(feature = "llama-cpp"))]
    pub fn select_model(&self, _requirements: &ModelRequirements) -> Option<LocalModel> {
        None // llama-cpp feature not enabled
    }

    /// Get all available models
    pub fn list_models(&self) -> &[LocalModel] {
        &self.available_models
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_requirements() {
        let req = ModelRequirements {
            min_context_length: 4096,
            max_latency_ms: 100,
            capabilities: vec!["code".to_string()],
            quantization: Some("q4_0".to_string()),
        };

        assert_eq!(req.min_context_length, 4096);
        assert!(req.capabilities.contains(&"code".to_string()));
    }

    #[test]
    fn test_selector_creation() {
        let selector = ModelSelectorBridge::new(PathBuf::from("/tmp/models"));
        assert!(selector.list_models().is_empty());
    }
}
