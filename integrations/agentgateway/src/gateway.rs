//! Gateway process manager

use crate::{config::GatewayConfig, Error, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tracing::{info, warn, error};

/// Manages the agentgateway process lifecycle
pub struct GatewayManager {
    config: GatewayConfig,
    process: Option<Child>,
    config_file: PathBuf,
}

impl GatewayManager {
    /// Create a new gateway manager
    pub fn new(config: GatewayConfig) -> Self {
        let config_file = config.config_file.clone()
            .unwrap_or_else(|| PathBuf::from("/tmp/agentgateway-config.yaml"));

        Self {
            config,
            process: None,
            config_file,
        }
    }

    /// Find the agentgateway binary
    fn find_binary(&self) -> Result<PathBuf> {
        if let Some(path) = &self.config.binary_path {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // Check common locations
        let locations = [
            PathBuf::from("./agentgateway/target/release/agentgateway"),
            PathBuf::from("./agentgateway/target/debug/agentgateway"),
            PathBuf::from("/usr/local/bin/agentgateway"),
            PathBuf::from("/usr/bin/agentgateway"),
        ];

        for loc in &locations {
            if loc.exists() {
                return Ok(loc.clone());
            }
        }

        // Try PATH lookup
        which::which("agentgateway")
            .map_err(|_| Error::GatewayNotFound(
                "agentgateway binary not found. Build it with: cd agentgateway && make build".to_string()
            ))
    }

    /// Write configuration file for the gateway
    fn write_config(&self) -> Result<()> {
        let gateway_config = self.config.to_gateway_config();
        let yaml = serde_yaml::to_string(&gateway_config)
            .map_err(|e| Error::ConfigError(e.to_string()))?;
        std::fs::write(&self.config_file, yaml)?;
        info!("Wrote gateway config to {:?}", self.config_file);
        Ok(())
    }

    /// Start the gateway process
    pub async fn start(&mut self) -> Result<()> {
        if self.process.is_some() {
            warn!("Gateway already running");
            return Ok(());
        }

        let binary = self.find_binary()?;
        self.write_config()?;

        info!("Starting agentgateway from {:?}", binary);

        let child = Command::new(&binary)
            .arg("--file")
            .arg(&self.config_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| Error::GatewayStartFailed(e.to_string()))?;

        self.process = Some(child);

        // Give the gateway time to start
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        info!("Gateway started successfully");
        Ok(())
    }

    /// Stop the gateway process
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            info!("Stopping gateway...");
            child.kill().await?;
            child.wait().await?;
            info!("Gateway stopped");
        }
        Ok(())
    }

    /// Check if the gateway is running
    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    self.process = None;
                    false
                }
                Ok(None) => true,
                Err(e) => {
                    error!("Error checking gateway status: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Reload gateway configuration
    pub async fn reload(&mut self) -> Result<()> {
        self.write_config()?;
        // Gateway uses file watch for dynamic reload
        info!("Configuration reloaded (gateway will pick up changes)");
        Ok(())
    }

    /// Get the admin UI URL
    pub fn admin_url(&self) -> String {
        format!("http://{}/ui", self.config.static_config.admin_address)
    }

    /// Get health check URL
    pub fn health_url(&self) -> String {
        format!("http://{}/health", self.config.static_config.admin_address)
    }

    /// Check gateway health
    pub async fn health_check(&self) -> Result<bool> {
        let client = reqwest::Client::new();
        match client.get(&self.health_url()).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

impl Drop for GatewayManager {
    fn drop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.start_kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_manager_new() {
        let config = GatewayConfig::default();
        let manager = GatewayManager::new(config);
        assert!(!manager.is_running());
    }

    #[test]
    fn test_admin_url() {
        let config = GatewayConfig::default();
        let manager = GatewayManager::new(config);
        assert_eq!(manager.admin_url(), "http://127.0.0.1:15000/ui");
    }
}
