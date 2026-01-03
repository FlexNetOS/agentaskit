//! Integration health check tests
//!
//! This module provides comprehensive health checks for all integrations.

use std::path::PathBuf;
use std::process::Command;

/// Check if a submodule exists and is initialized
fn check_submodule(name: &str) -> Result<(), String> {
    let path = PathBuf::from(name);
    if !path.exists() {
        return Err(format!("Submodule '{}' not found", name));
    }

    // Check if it's a valid git repo
    let git_dir = path.join(".git");
    if !git_dir.exists() {
        return Err(format!("Submodule '{}' is not initialized", name));
    }

    Ok(())
}

/// Check if a Cargo.toml exists and is valid
fn check_cargo_toml(path: &str) -> Result<(), String> {
    let cargo_path = PathBuf::from(path);
    if !cargo_path.exists() {
        return Err(format!("Cargo.toml not found at: {}", path));
    }

    let content = std::fs::read_to_string(&cargo_path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    // Check for required fields
    if !content.contains("[package]") && !content.contains("[workspace]") {
        return Err(format!("{} missing [package] or [workspace] section", path));
    }

    Ok(())
}

/// Check if integration source files exist
fn check_integration_sources(integration: &str) -> Result<(), String> {
    let base = PathBuf::from("integrations").join(integration);
    let src = base.join("src");

    if !src.exists() {
        return Err(format!("Source directory not found: {:?}", src));
    }

    let lib_rs = src.join("lib.rs");
    if !lib_rs.exists() {
        return Err(format!("lib.rs not found: {:?}", lib_rs));
    }

    Ok(())
}

/// Check if configuration files exist
fn check_config_files(config_dir: &str) -> Result<Vec<String>, String> {
    let path = PathBuf::from("configs").join(config_dir);
    if !path.exists() {
        return Err(format!("Config directory not found: {:?}", path));
    }

    let mut files = Vec::new();
    for entry in std::fs::read_dir(&path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.path().is_file() {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    if files.is_empty() {
        return Err(format!("No config files found in {:?}", path));
    }

    Ok(files)
}

/// Check if a workflow file exists
fn check_workflow(name: &str) -> Result<(), String> {
    let path = PathBuf::from(".github/workflows").join(name);
    if !path.exists() {
        return Err(format!("Workflow not found: {:?}", path));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read workflow: {}", e))?;

    // Check for required workflow sections
    if !content.contains("name:") {
        return Err(format!("Workflow {} missing 'name' field", name));
    }

    if !content.contains("on:") {
        return Err(format!("Workflow {} missing 'on' trigger", name));
    }

    if !content.contains("jobs:") {
        return Err(format!("Workflow {} missing 'jobs' section", name));
    }

    Ok(())
}

// ============================================================================
// Test Cases
// ============================================================================

#[test]
fn test_agentgateway_submodule_exists() {
    check_submodule("agentgateway").expect("Agentgateway submodule check failed");
}

#[test]
fn test_wiki_rs_submodule_exists() {
    check_submodule("wiki-rs").expect("Wiki-rs submodule check failed");
}

#[test]
fn test_agentgateway_integration_structure() {
    // Check Cargo.toml
    check_cargo_toml("integrations/agentgateway/Cargo.toml")
        .expect("Agentgateway integration Cargo.toml check failed");

    // Check source files
    check_integration_sources("agentgateway")
        .expect("Agentgateway integration source check failed");

    // Check required modules exist
    let required_modules = [
        "integrations/agentgateway/src/config.rs",
        "integrations/agentgateway/src/gateway.rs",
        "integrations/agentgateway/src/mcp.rs",
        "integrations/agentgateway/src/a2a.rs",
        "integrations/agentgateway/src/auth.rs",
        "integrations/agentgateway/src/routing.rs",
        "integrations/agentgateway/src/ratelimit.rs",
        "integrations/agentgateway/src/observability.rs",
        "integrations/agentgateway/src/xds.rs",
    ];

    for module in &required_modules {
        assert!(
            PathBuf::from(module).exists(),
            "Required module not found: {}",
            module
        );
    }
}

#[test]
fn test_wiki_rs_integration_structure() {
    // Check Cargo.toml
    check_cargo_toml("integrations/wiki-rs/Cargo.toml")
        .expect("Wiki-rs integration Cargo.toml check failed");

    // Check source files
    check_integration_sources("wiki-rs")
        .expect("Wiki-rs integration source check failed");

    // Check required modules exist
    let required_modules = [
        "integrations/wiki-rs/src/config.rs",
        "integrations/wiki-rs/src/generator.rs",
        "integrations/wiki-rs/src/llm.rs",
        "integrations/wiki-rs/src/output.rs",
    ];

    for module in &required_modules {
        assert!(
            PathBuf::from(module).exists(),
            "Required module not found: {}",
            module
        );
    }
}

#[test]
fn test_agentgateway_config_files() {
    let files = check_config_files("agentgateway")
        .expect("Agentgateway config check failed");

    assert!(files.contains(&"local.yaml".to_string()), "Missing local.yaml");
    assert!(files.contains(&"production.yaml".to_string()), "Missing production.yaml");
}

#[test]
fn test_wiki_config_files() {
    let files = check_config_files("wiki")
        .expect("Wiki config check failed");

    assert!(files.contains(&"local.toml".to_string()), "Missing local.toml");
    assert!(files.contains(&"production.toml".to_string()), "Missing production.toml");
}

#[test]
fn test_agentgateway_workflow() {
    check_workflow("agentgateway-build.yml")
        .expect("Agentgateway workflow check failed");
}

#[test]
fn test_wiki_workflow() {
    check_workflow("wiki-build.yml")
        .expect("Wiki workflow check failed");
}

#[test]
fn test_documentation_exists() {
    let docs = [
        "integrations/agentgateway/README.md",
        "integrations/agentgateway/DEPENDENCY_ANALYSIS.md",
        "integrations/wiki-rs/README.md",
    ];

    for doc in &docs {
        assert!(
            PathBuf::from(doc).exists(),
            "Documentation not found: {}",
            doc
        );
    }
}

#[test]
fn test_gitmodules_configured() {
    let gitmodules = PathBuf::from(".gitmodules");
    assert!(gitmodules.exists(), ".gitmodules file not found");

    let content = std::fs::read_to_string(&gitmodules)
        .expect("Failed to read .gitmodules");

    assert!(content.contains("agentgateway"), ".gitmodules missing agentgateway");
    assert!(content.contains("wiki-rs"), ".gitmodules missing wiki-rs");
}

#[test]
fn test_integration_readme_content() {
    // Check agentgateway README has required sections
    let ag_readme = std::fs::read_to_string("integrations/agentgateway/README.md")
        .expect("Failed to read agentgateway README");

    assert!(ag_readme.contains("# AgentasKit Gateway Integration"), "Missing title");
    assert!(ag_readme.contains("## Features"), "Missing Features section");
    assert!(ag_readme.contains("## Quick Start"), "Missing Quick Start section");

    // Check wiki-rs README has required sections
    let wiki_readme = std::fs::read_to_string("integrations/wiki-rs/README.md")
        .expect("Failed to read wiki-rs README");

    assert!(wiki_readme.contains("# AgentasKit Wiki Integration"), "Missing title");
    assert!(wiki_readme.contains("## Features"), "Missing Features section");
}

/// Summary of all health checks
pub fn run_all_health_checks() -> (usize, usize, Vec<String>) {
    let mut passed = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    let checks: Vec<(&str, Box<dyn Fn() -> Result<(), String>>)> = vec![
        ("Agentgateway submodule", Box::new(|| check_submodule("agentgateway"))),
        ("Wiki-rs submodule", Box::new(|| check_submodule("wiki-rs"))),
        ("Agentgateway Cargo.toml", Box::new(|| check_cargo_toml("integrations/agentgateway/Cargo.toml"))),
        ("Wiki-rs Cargo.toml", Box::new(|| check_cargo_toml("integrations/wiki-rs/Cargo.toml"))),
        ("Agentgateway sources", Box::new(|| check_integration_sources("agentgateway"))),
        ("Wiki-rs sources", Box::new(|| check_integration_sources("wiki-rs"))),
        ("Agentgateway workflow", Box::new(|| check_workflow("agentgateway-build.yml"))),
        ("Wiki workflow", Box::new(|| check_workflow("wiki-build.yml"))),
    ];

    for (name, check) in checks {
        match check() {
            Ok(()) => {
                passed += 1;
                println!("✓ {}", name);
            }
            Err(e) => {
                failed += 1;
                errors.push(format!("{}: {}", name, e));
                println!("✗ {}: {}", name, e);
            }
        }
    }

    (passed, failed, errors)
}
