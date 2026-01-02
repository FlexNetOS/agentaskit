//! SOP Parser - Parse AgentTask Standard Operating Procedure files
//!
//! This module provides comprehensive parsing and analysis of .sop files
//! following the AgentTask SOP format with three-plane architecture.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete SOP document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPDocument {
    pub version: String,
    pub generated: String,
    pub title: String,
    pub purpose: String,
    pub scope: SOPScope,
    pub roles: Vec<SOPRole>,
    pub materials: SOPMaterials,
    pub architecture: SOPArchitecture,
    pub procedures: Vec<SOPProcedure>,
    pub quality_checks: SOPQualityChecks,
    pub glossary: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPScope {
    pub applies_to: String,
    pub inclusions: Vec<String>,
    pub exclusions: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPRole {
    pub name: String,
    pub responsibilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPMaterials {
    pub required_tools: Vec<String>,
    pub environment_variables: HashMap<String, String>,
    pub optional_tools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPArchitecture {
    pub root_paths: Vec<String>,
    pub orchestrator: Vec<String>,
    pub sandbox: Vec<String>,
    pub execution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPProcedure {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<SOPStep>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPStep {
    pub substep: String,
    pub command: Option<String>,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPQualityChecks {
    pub build_time_gates: Vec<String>,
    pub runtime_guards: Vec<String>,
    pub metrics_to_watch: Vec<String>,
}

/// Parse SOP file content into structured document
pub fn parse_sop(text: &str) -> Result<SOPDocument> {
    let mut doc = SOPDocument {
        version: String::new(),
        generated: String::new(),
        title: String::new(),
        purpose: String::new(),
        scope: SOPScope {
            applies_to: String::new(),
            inclusions: Vec::new(),
            exclusions: Vec::new(),
            limitations: Vec::new(),
        },
        roles: Vec::new(),
        materials: SOPMaterials {
            required_tools: Vec::new(),
            environment_variables: HashMap::new(),
            optional_tools: Vec::new(),
        },
        architecture: SOPArchitecture {
            root_paths: Vec::new(),
            orchestrator: Vec::new(),
            sandbox: Vec::new(),
            execution: Vec::new(),
        },
        procedures: Vec::new(),
        quality_checks: SOPQualityChecks {
            build_time_gates: Vec::new(),
            runtime_guards: Vec::new(),
            metrics_to_watch: Vec::new(),
        },
        glossary: HashMap::new(),
    };

    let lines: Vec<&str> = text.lines().collect();
    let mut current_section = String::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Parse version and generated timestamp
        if line.starts_with("Version:") {
            if let Some(version) = line.split("Version:").nth(1) {
                doc.version = version.split('•').next().unwrap_or("").trim().to_string();
            }
            if let Some(generated) = line.split("Generated:").nth(1) {
                doc.generated = generated.trim().to_string();
            }
        }

        // Detect section headers
        if line.contains("====") && i + 1 < lines.len() {
            current_section = lines[i + 1].trim().to_string();
            i += 2;
            continue;
        }

        match current_section.as_str() {
            "TITLE / HEADER" => {
                if !line.is_empty() && !line.contains("====") {
                    doc.title = line.to_string();
                }
            }
            "PURPOSE" => {
                if !line.is_empty() && !line.contains("====") {
                    doc.purpose.push_str(line);
                    doc.purpose.push(' ');
                }
            }
            "SCOPE" => {
                if line.starts_with("Applies to") {
                    doc.scope.applies_to = line.to_string();
                } else if line.starts_with("Limitations:") {
                    // Parse limitations
                    i += 1;
                    while i < lines.len() && lines[i].trim().starts_with('-') {
                        doc.scope
                            .limitations
                            .push(lines[i].trim()[1..].trim().to_string());
                        i += 1;
                    }
                    i -= 1;
                }
            }
            "ROLES & RESPONSIBILITIES" => {
                if !line.is_empty() && !line.starts_with("====") && !line.starts_with("•") {
                    let role_name = line.to_string();
                    let mut responsibilities = Vec::new();
                    i += 1;
                    while i < lines.len() && lines[i].trim().starts_with("•") {
                        responsibilities.push(lines[i].trim()[1..].trim().to_string());
                        i += 1;
                    }
                    doc.roles.push(SOPRole {
                        name: role_name,
                        responsibilities,
                    });
                    i -= 1;
                }
            }
            "MATERIALS & RESOURCES" => {
                if line.starts_with("Required tools") {
                    i += 1;
                    while i < lines.len() && lines[i].trim().starts_with("•") {
                        doc.materials
                            .required_tools
                            .push(lines[i].trim()[1..].trim().to_string());
                        i += 1;
                    }
                    i -= 1;
                } else if line.starts_with("Environment") {
                    i += 1;
                    while i < lines.len() && lines[i].trim().starts_with("•") {
                        let env_line = lines[i].trim()[1..].trim();
                        if let Some(dash_pos) = env_line.find('–') {
                            let key = env_line[..dash_pos].trim().to_string();
                            let value = env_line[dash_pos + 3..].trim().to_string();
                            doc.materials.environment_variables.insert(key, value);
                        }
                        i += 1;
                    }
                    i -= 1;
                }
            }
            "ARCHITECTURE TREE (REFERENCE)" => {
                if line.starts_with('/') {
                    doc.architecture.root_paths.push(line.to_string());
                }
            }
            "PROCEDURES / INSTRUCTIONS" => {
                if line.starts_with(char::is_numeric) && line.contains(')') {
                    // Parse procedure
                    let parts: Vec<&str> = line.splitn(2, ')').collect();
                    let id = parts[0].trim().to_string();
                    let name = parts.get(1).unwrap_or(&"").trim().to_string();
                    let mut steps = Vec::new();

                    i += 1;
                    while i < lines.len() {
                        let step_line = lines[i].trim();
                        if step_line.starts_with(char::is_alphabetic) && step_line.contains('.') {
                            let substep =
                                step_line.split('.').next().unwrap_or("").trim().to_string();
                            let description = step_line
                                .split('.')
                                .skip(1)
                                .collect::<Vec<_>>()
                                .join(".")
                                .trim()
                                .to_string();
                            steps.push(SOPStep {
                                substep,
                                command: None,
                                description,
                                required: true,
                            });
                        } else if step_line.starts_with(char::is_numeric) || step_line.is_empty() {
                            break;
                        }
                        i += 1;
                    }

                    doc.procedures.push(SOPProcedure {
                        id,
                        name,
                        description: String::new(),
                        steps,
                        dependencies: Vec::new(),
                    });
                    i -= 1;
                }
            }
            "QUALITY CHECKS / MONITORING" => {
                if line.starts_with("Build-time Gates") {
                    i += 1;
                    while i < lines.len() && lines[i].trim().starts_with("•") {
                        doc.quality_checks
                            .build_time_gates
                            .push(lines[i].trim()[1..].trim().to_string());
                        i += 1;
                    }
                    i -= 1;
                } else if line.starts_with("Runtime Guards") {
                    i += 1;
                    while i < lines.len() && lines[i].trim().starts_with("•") {
                        doc.quality_checks
                            .runtime_guards
                            .push(lines[i].trim()[1..].trim().to_string());
                        i += 1;
                    }
                    i -= 1;
                } else if line.starts_with("Metrics to Watch") {
                    i += 1;
                    while i < lines.len() && lines[i].trim().starts_with("•") {
                        doc.quality_checks
                            .metrics_to_watch
                            .push(lines[i].trim()[1..].trim().to_string());
                        i += 1;
                    }
                    i -= 1;
                }
            }
            "GLOSSARY & KEY TERMS" => {
                if line.contains(':') && !line.is_empty() {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        doc.glossary
                            .insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                    }
                }
            }
            _ => {}
        }

        i += 1;
    }

    Ok(doc)
}

/// Extract steps as simple string list (backward compatibility)
pub fn parse_sop_steps(text: &str) -> Vec<String> {
    let doc = match parse_sop(text) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    doc.procedures
        .into_iter()
        .flat_map(|proc| {
            proc.steps
                .into_iter()
                .map(|step| format!("{}: {} - {}", proc.name, step.substep, step.description))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Validate SOP compliance for a given task
pub fn validate_sop_compliance(sop: &SOPDocument, task_description: &str) -> Vec<String> {
    let mut violations = Vec::new();

    // Check for required procedures mentioned
    for procedure in &sop.procedures {
        if task_description.contains(&procedure.name) {
            // Verify all required steps are acknowledged
            for step in &procedure.steps {
                if step.required && !task_description.contains(&step.description) {
                    violations.push(format!(
                        "Missing required step {} in procedure {}",
                        step.substep, procedure.name
                    ));
                }
            }
        }
    }

    violations
}

/// Get required environment variables from SOP
pub fn get_required_env_vars(sop: &SOPDocument) -> Vec<String> {
    sop.materials
        .environment_variables
        .keys()
        .cloned()
        .collect()
}

/// Get procedures by ID
pub fn get_procedure_by_id(sop: &SOPDocument, id: &str) -> Option<&SOPProcedure> {
    sop.procedures.iter().find(|p| p.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sop_basic() {
        let sample_sop = r#"
AGENTASK STANDARD OPERATING PROCEDURE (.sop)
Version: 1.0 • Generated: 2025-10-05T08:12:14Z

============================================================
TITLE / HEADER
============================================================
Agent Task Lifecycle & Release Integrity SOP

============================================================
PURPOSE
============================================================
Guarantee low-latency, contract-based delivery.
        "#;

        let doc = parse_sop(sample_sop).unwrap();
        assert_eq!(doc.version, "1.0");
        assert!(doc.title.contains("Agent Task Lifecycle"));
    }

    #[test]
    fn test_parse_sop_steps() {
        let sample_sop = r#"
============================================================
PROCEDURES / INSTRUCTIONS
============================================================
1) Author & Prepare
  a. Place inputs in sandbox/inputs/
  b. Update contracts/inference.capnp
        "#;

        let steps = parse_sop_steps(sample_sop);
        assert!(steps.len() > 0);
    }
}
