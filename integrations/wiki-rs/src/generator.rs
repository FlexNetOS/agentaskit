//! Wiki documentation generator

use crate::{config::WikiConfig, Error, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tracing::{info, warn, error};

/// Manages the wiki documentation generation process
pub struct WikiGenerator {
    config: WikiConfig,
    process: Option<Child>,
}

impl WikiGenerator {
    /// Create a new wiki generator
    pub fn new(config: WikiConfig) -> Self {
        Self {
            config,
            process: None,
        }
    }

    /// Find the deepwiki-rs binary
    fn find_binary(&self) -> Result<PathBuf> {
        // Check common locations
        let locations = [
            PathBuf::from("./wiki-rs/target/release/deepwiki-rs"),
            PathBuf::from("./wiki-rs/target/debug/deepwiki-rs"),
            PathBuf::from("/usr/local/bin/deepwiki-rs"),
            PathBuf::from("/usr/bin/deepwiki-rs"),
        ];

        for loc in &locations {
            if loc.exists() {
                return Ok(loc.clone());
            }
        }

        // Try PATH lookup
        which::which("deepwiki-rs")
            .map_err(|_| Error::GeneratorNotFound(
                "deepwiki-rs binary not found. Install with: cargo install deepwiki-rs".to_string()
            ))
    }

    /// Generate documentation for the configured project
    pub async fn generate(&mut self) -> Result<GenerationResult> {
        let binary = self.find_binary()?;

        info!("Starting documentation generation for {:?}", self.config.project_path);

        let mut cmd = Command::new(&binary);

        // Add project path
        cmd.arg("-p").arg(&self.config.project_path);

        // Add output directory
        cmd.arg("-o").arg(&self.config.output_dir);

        // Add target language
        cmd.arg("--target-language").arg(&self.config.target_language);

        // Add LLM configuration if provided
        if let Some(base_url) = &self.config.llm.api_base_url {
            cmd.arg("--llm-api-base-url").arg(base_url);
        }

        if let Some(api_key) = &self.config.llm.api_key {
            cmd.arg("--llm-api-key").arg(api_key);
        }

        cmd.arg("--model-efficient").arg(&self.config.llm.model_efficient);
        cmd.arg("--model-powerful").arg(&self.config.llm.model_powerful);

        // Add processing options
        if self.config.processing.skip_preprocessing {
            cmd.arg("--skip-preprocessing");
        }

        if self.config.processing.skip_research {
            cmd.arg("--skip-research");
        }

        if !self.config.llm.enable_preset_tools {
            cmd.arg("--disable-preset-tools");
        }

        // Execute
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let start_time = std::time::Instant::now();

        let output = cmd.output().await
            .map_err(|e| Error::GenerationFailed(e.to_string()))?;

        let duration = start_time.elapsed();

        if output.status.success() {
            info!("Documentation generation completed in {:?}", duration);

            // Count generated files
            let files = self.count_generated_files()?;

            Ok(GenerationResult {
                success: true,
                output_dir: self.config.output_dir.clone(),
                files_generated: files,
                duration_seconds: duration.as_secs_f64(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Documentation generation failed: {}", stderr);
            Err(Error::GenerationFailed(stderr.to_string()))
        }
    }

    /// Count generated documentation files
    fn count_generated_files(&self) -> Result<usize> {
        if !self.config.output_dir.exists() {
            return Ok(0);
        }

        let count = walkdir::WalkDir::new(&self.config.output_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path().extension()
                    .map(|ext| ext == "md" || ext == "html")
                    .unwrap_or(false)
            })
            .count();

        Ok(count)
    }

    /// Generate documentation in the background
    pub async fn generate_async(&mut self) -> Result<()> {
        let binary = self.find_binary()?;

        info!("Starting background documentation generation");

        let mut cmd = Command::new(&binary);
        cmd.arg("-p").arg(&self.config.project_path);
        cmd.arg("-o").arg(&self.config.output_dir);
        cmd.arg("--target-language").arg(&self.config.target_language);

        if let Some(base_url) = &self.config.llm.api_base_url {
            cmd.arg("--llm-api-base-url").arg(base_url);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()
            .map_err(|e| Error::GenerationFailed(e.to_string()))?;

        self.process = Some(child);
        Ok(())
    }

    /// Check if generation is in progress
    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    self.process = None;
                    false
                }
                Ok(None) => true,
                Err(e) => {
                    error!("Error checking process status: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Stop the generation process
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            warn!("Stopping documentation generation");
            child.kill().await?;
            child.wait().await?;
        }
        Ok(())
    }

    /// Get the output directory
    pub fn output_dir(&self) -> &PathBuf {
        &self.config.output_dir
    }
}

/// Result of documentation generation
#[derive(Debug, Clone)]
pub struct GenerationResult {
    /// Whether generation succeeded
    pub success: bool,

    /// Output directory path
    pub output_dir: PathBuf,

    /// Number of files generated
    pub files_generated: usize,

    /// Generation duration in seconds
    pub duration_seconds: f64,

    /// Standard output from the process
    pub stdout: String,

    /// Standard error from the process
    pub stderr: String,
}

impl GenerationResult {
    /// Get a summary of the generation
    pub fn summary(&self) -> String {
        if self.success {
            format!(
                "Generated {} documentation files in {:.2}s to {:?}",
                self.files_generated,
                self.duration_seconds,
                self.output_dir
            )
        } else {
            format!("Documentation generation failed: {}", self.stderr)
        }
    }
}

/// Builder for wiki generator with fluent configuration
#[derive(Debug, Clone)]
pub struct WikiGeneratorBuilder {
    config: WikiConfig,
}

impl WikiGeneratorBuilder {
    /// Create a new builder for a project
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            config: WikiConfig::for_project(project_path),
        }
    }

    /// Set output directory
    pub fn output_dir(mut self, dir: PathBuf) -> Self {
        self.config.output_dir = dir;
        self
    }

    /// Set target language
    pub fn language(mut self, lang: &str) -> Self {
        self.config.target_language = lang.to_string();
        self
    }

    /// Set LLM API base URL
    pub fn llm_api_url(mut self, url: &str) -> Self {
        self.config.llm.api_base_url = Some(url.to_string());
        self
    }

    /// Set LLM API key
    pub fn llm_api_key(mut self, key: &str) -> Self {
        self.config.llm.api_key = Some(key.to_string());
        self
    }

    /// Set efficient model
    pub fn model_efficient(mut self, model: &str) -> Self {
        self.config.llm.model_efficient = model.to_string();
        self
    }

    /// Set powerful model
    pub fn model_powerful(mut self, model: &str) -> Self {
        self.config.llm.model_powerful = model.to_string();
        self
    }

    /// Skip preprocessing stage
    pub fn skip_preprocessing(mut self) -> Self {
        self.config.processing.skip_preprocessing = true;
        self
    }

    /// Skip research stage
    pub fn skip_research(mut self) -> Self {
        self.config.processing.skip_research = true;
        self
    }

    /// Build the generator
    pub fn build(self) -> WikiGenerator {
        WikiGenerator::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_builder() {
        let generator = WikiGeneratorBuilder::new(PathBuf::from("./my-project"))
            .output_dir(PathBuf::from("./docs"))
            .language("ja")
            .model_efficient("gpt-3.5-turbo")
            .build();

        assert_eq!(generator.config.project_path, PathBuf::from("./my-project"));
        assert_eq!(generator.config.output_dir, PathBuf::from("./docs"));
        assert_eq!(generator.config.target_language, "ja");
    }

    #[test]
    fn test_generation_result_summary() {
        let result = GenerationResult {
            success: true,
            output_dir: PathBuf::from("./docs"),
            files_generated: 10,
            duration_seconds: 45.5,
            stdout: String::new(),
            stderr: String::new(),
        };

        let summary = result.summary();
        assert!(summary.contains("10"));
        assert!(summary.contains("45.50s"));
    }
}
