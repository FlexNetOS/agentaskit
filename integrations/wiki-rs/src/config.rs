//! Wiki generator configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main wiki generator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiConfig {
    /// Project path to analyze
    pub project_path: PathBuf,

    /// Output directory for generated documentation
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    /// Target language for documentation
    #[serde(default = "default_language")]
    pub target_language: String,

    /// LLM configuration
    #[serde(default)]
    pub llm: LlmConfig,

    /// Processing options
    #[serde(default)]
    pub processing: ProcessingConfig,

    /// Output options
    #[serde(default)]
    pub output: OutputConfig,
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("./litho.docs")
}

fn default_language() -> String {
    "en".to_string()
}

impl Default for WikiConfig {
    fn default() -> Self {
        Self {
            project_path: PathBuf::from("."),
            output_dir: default_output_dir(),
            target_language: default_language(),
            llm: LlmConfig::default(),
            processing: ProcessingConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

/// LLM configuration for documentation generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// LLM API base URL
    #[serde(default)]
    pub api_base_url: Option<String>,

    /// LLM API key (use env var in production)
    #[serde(default)]
    pub api_key: Option<String>,

    /// Efficient model for quick operations
    #[serde(default = "default_efficient_model")]
    pub model_efficient: String,

    /// Powerful model for complex analysis
    #[serde(default = "default_powerful_model")]
    pub model_powerful: String,

    /// Enable preset tools (ReAct mode)
    #[serde(default = "default_true")]
    pub enable_preset_tools: bool,

    /// Maximum tokens for responses
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,

    /// Temperature for generation
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_efficient_model() -> String {
    "gpt-4o-mini".to_string()
}

fn default_powerful_model() -> String {
    "gpt-4o".to_string()
}

fn default_true() -> bool {
    true
}

fn default_max_tokens() -> u32 {
    4096
}

fn default_temperature() -> f32 {
    0.7
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            api_base_url: None,
            api_key: None,
            model_efficient: default_efficient_model(),
            model_powerful: default_powerful_model(),
            enable_preset_tools: true,
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
        }
    }
}

/// Processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    /// Skip preprocessing stage
    #[serde(default)]
    pub skip_preprocessing: bool,

    /// Skip research stage
    #[serde(default)]
    pub skip_research: bool,

    /// Enable caching for LLM responses
    #[serde(default = "default_true")]
    pub enable_cache: bool,

    /// Cache directory
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,

    /// File patterns to include
    #[serde(default)]
    pub include_patterns: Vec<String>,

    /// File patterns to exclude
    #[serde(default = "default_exclude_patterns")]
    pub exclude_patterns: Vec<String>,

    /// Maximum file size to process (in bytes)
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
}

fn default_cache_dir() -> PathBuf {
    PathBuf::from("./.litho_cache")
}

fn default_exclude_patterns() -> Vec<String> {
    vec![
        "node_modules/**".to_string(),
        "target/**".to_string(),
        ".git/**".to_string(),
        "*.lock".to_string(),
        "dist/**".to_string(),
        "build/**".to_string(),
    ]
}

fn default_max_file_size() -> u64 {
    1024 * 1024 // 1MB
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            skip_preprocessing: false,
            skip_research: false,
            enable_cache: true,
            cache_dir: default_cache_dir(),
            include_patterns: Vec::new(),
            exclude_patterns: default_exclude_patterns(),
            max_file_size: default_max_file_size(),
        }
    }
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Generate project overview
    #[serde(default = "default_true")]
    pub generate_overview: bool,

    /// Generate architecture documentation
    #[serde(default = "default_true")]
    pub generate_architecture: bool,

    /// Generate workflow documentation
    #[serde(default = "default_true")]
    pub generate_workflow: bool,

    /// Generate module deep dive documentation
    #[serde(default = "default_true")]
    pub generate_deep_dive: bool,

    /// Generate Mermaid diagrams
    #[serde(default = "default_true")]
    pub generate_diagrams: bool,

    /// Fix Mermaid syntax errors
    #[serde(default = "default_true")]
    pub fix_mermaid_errors: bool,

    /// Output format
    #[serde(default)]
    pub format: OutputFormat,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            generate_overview: true,
            generate_architecture: true,
            generate_workflow: true,
            generate_deep_dive: true,
            generate_diagrams: true,
            fix_mermaid_errors: true,
            format: OutputFormat::default(),
        }
    }
}

/// Output format options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Markdown,
    Html,
    Pdf,
}

impl WikiConfig {
    /// Load configuration from a TOML file
    pub fn from_file(path: &PathBuf) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: WikiConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn to_file(&self, path: &PathBuf) -> crate::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::Error::ConfigError(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create configuration for a specific project
    pub fn for_project(project_path: PathBuf) -> Self {
        Self {
            project_path,
            ..Default::default()
        }
    }

    /// Set output directory
    pub fn with_output_dir(mut self, dir: PathBuf) -> Self {
        self.output_dir = dir;
        self
    }

    /// Set target language
    pub fn with_language(mut self, lang: &str) -> Self {
        self.target_language = lang.to_string();
        self
    }

    /// Configure LLM settings
    pub fn with_llm(mut self, config: LlmConfig) -> Self {
        self.llm = config;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WikiConfig::default();
        assert_eq!(config.target_language, "en");
        assert!(config.llm.enable_preset_tools);
    }

    #[test]
    fn test_config_builder() {
        let config = WikiConfig::for_project(PathBuf::from("./my-project"))
            .with_output_dir(PathBuf::from("./docs"))
            .with_language("ja");

        assert_eq!(config.project_path, PathBuf::from("./my-project"));
        assert_eq!(config.output_dir, PathBuf::from("./docs"));
        assert_eq!(config.target_language, "ja");
    }

    #[test]
    fn test_exclude_patterns() {
        let config = ProcessingConfig::default();
        assert!(config.exclude_patterns.contains(&"node_modules/**".to_string()));
        assert!(config.exclude_patterns.contains(&"target/**".to_string()));
    }
}
