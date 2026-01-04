//! AI Model Interface for SOP Analysis
//!
//! Provides AI-powered content analysis and procedure validation
//! for Standard Operating Procedures.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::sop_parser::{SOPDocument, SOPProcedure, SOPStep};

/// AI-powered SOP analyzer
pub struct AISopAnalyzer {
    model_name: String,
    confidence_threshold: f32,
}

impl AISopAnalyzer {
    pub fn new(model_name: String) -> Self {
        Self {
            model_name,
            confidence_threshold: 0.75,
        }
    }

    pub fn with_confidence_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Analyze SOP content for completeness
    pub async fn analyze_content(&self, sop: &SOPDocument) -> Result<ContentAnalysis> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut completeness_score = 1.0f32;

        // Check title and purpose
        if sop.title.is_empty() {
            issues.push("Missing SOP title".to_string());
            completeness_score -= 0.1;
        }
        if sop.purpose.is_empty() {
            issues.push("Missing SOP purpose section".to_string());
            completeness_score -= 0.15;
        }

        // Enhanced: Check scope coverage (direct field access - more efficient)
        if sop.scope.inclusions.is_empty() {
            recommendations.push("Define explicit scope inclusions for clarity".to_string());
            completeness_score -= 0.05;
        }
        if sop.scope.exclusions.is_empty() {
            recommendations.push("Consider documenting scope exclusions".to_string());
        }
        if sop.scope.limitations.is_empty() {
            recommendations.push("Document known limitations for transparency".to_string());
        }

        // Check roles definition
        if sop.roles.is_empty() {
            issues.push("No roles defined - clarify responsibilities".to_string());
            completeness_score -= 0.1;
        }

        // Check materials and resources
        // Enhanced: Check materials (direct field access)
        if sop.materials.required_tools.is_empty() {
            recommendations.push("List required tools for reproducibility".to_string());
            completeness_score -= 0.05;
        }
        if sop.materials.environment_variables.is_empty() {
            recommendations.push("Document environment variables if applicable".to_string());
        }

        // Check procedures
        if sop.procedures.is_empty() {
            issues.push("No procedures defined - SOP has no actionable steps".to_string());
            completeness_score -= 0.3;
        } else {
            // Analyze procedure quality
            for procedure in &sop.procedures {
                if procedure.steps.is_empty() {
                    issues.push(format!("Procedure '{}' has no steps", procedure.name));
                    completeness_score -= 0.05;
                }
                if procedure.description.is_empty() {
                    recommendations
                        .push(format!("Add description to procedure '{}'", procedure.name));
                }
            }
        }

        // Enhanced: Check quality checks (direct field access)
        if sop.quality_checks.build_time_gates.is_empty() && sop.quality_checks.runtime_guards.is_empty() {
            recommendations.push("Define quality gates for verification".to_string());
            completeness_score -= 0.05;
        }

        let confidence = completeness_score.max(0.0).min(1.0);
        let status = if confidence >= self.confidence_threshold {
            AnalysisStatus::Complete
        } else if confidence >= 0.5 {
            AnalysisStatus::NeedsImprovement
        } else {
            AnalysisStatus::Incomplete
        };

        Ok(ContentAnalysis {
            status,
            completeness_score: confidence,
            issues,
            recommendations,
            analyzed_sections: self.get_analyzed_sections(sop),
        })
    }

    /// Validate procedure against task requirements
    pub async fn validate_procedure(
        &self,
        procedure: &SOPProcedure,
        task_description: &str,
    ) -> Result<ProcedureValidation> {
        let mut alignment_score = 1.0f32;
        let mut gaps = Vec::new();
        let mut relevant_steps = Vec::new();

        // Simple keyword matching (in production, use actual AI model)
        let task_keywords = self.extract_keywords(task_description);

        for step in &procedure.steps {
            let step_text = format!(
                "{} {}",
                step.command.as_ref().unwrap_or(&String::new()),
                step.description
            );

            let mut relevance = 0.0f32;
            for keyword in &task_keywords {
                if step_text.to_lowercase().contains(&keyword.to_lowercase()) {
                    relevance += 0.1;
                }
            }

            if relevance > 0.0 {
                relevant_steps.push(RelevantStep {
                    substep: step.substep.clone(),
                    relevance_score: relevance.min(1.0),
                    reason: format!("Matches keywords: {}", task_keywords.join(", ")),
                });
            }
        }

        // Check for missing required steps
        if procedure.steps.iter().any(|s| s.required) && relevant_steps.is_empty() {
            gaps.push("No relevant steps found for task requirements".to_string());
            alignment_score -= 0.3;
        }

        // Check for dependencies
        if !procedure.dependencies.is_empty() && relevant_steps.len() < procedure.steps.len() / 2 {
            gaps.push("Procedure has unmet dependencies".to_string());
            alignment_score -= 0.2;
        }

        let is_aligned = alignment_score >= self.confidence_threshold;

        Ok(ProcedureValidation {
            is_aligned,
            alignment_score: alignment_score.max(0.0),
            relevant_steps,
            gaps,
            required_steps: procedure
                .steps
                .iter()
                .filter(|s| s.required)
                .map(|s| s.substep.clone())
                .collect(),
        })
    }

    /// Extract key concepts from SOP
    pub async fn extract_key_concepts(&self, sop: &SOPDocument) -> Result<Vec<Concept>> {
        let mut concepts = Vec::new();

        // Extract from purpose
        if !sop.purpose.is_empty() {
            concepts.push(Concept {
                name: "Purpose".to_string(),
                category: ConceptCategory::Objective,
                description: sop.purpose.clone(),
                importance: 1.0,
            });
        }

        // Extract from procedures
        for procedure in &sop.procedures {
            concepts.push(Concept {
                name: procedure.name.clone(),
                category: ConceptCategory::Procedure,
                description: procedure.description.clone(),
                importance: 0.8,
            });
        }

        // Extract from roles
        for role in &sop.roles {
            concepts.push(Concept {
                name: role.name.clone(),
                category: ConceptCategory::Role,
                description: role.responsibilities.join("; "),
                importance: 0.7,
            });
        }

        // Enhanced: Extract from quality checks (direct field access)
        for gate in &sop.quality_checks.build_time_gates {
            concepts.push(Concept {
                name: format!("Build Gate: {}", gate),
                category: ConceptCategory::QualityCheck,
                description: gate.clone(),
                importance: 0.9,
            });
        }

        Ok(concepts)
    }

    /// Find relevant procedures for a task
    pub async fn find_relevant_procedures(
        &self,
        sop: &SOPDocument,
        task_description: &str,
    ) -> Result<Vec<RelevantProcedure>> {
        let mut relevant = Vec::new();
        let task_keywords = self.extract_keywords(task_description);

        for procedure in &sop.procedures {
            let procedure_text = format!(
                "{} {} {}",
                procedure.name,
                procedure.description,
                procedure
                    .steps
                    .iter()
                    .map(|s| s.description.clone())
                    .collect::<Vec<_>>()
                    .join(" ")
            );

            let mut relevance = 0.0f32;
            for keyword in &task_keywords {
                if procedure_text
                    .to_lowercase()
                    .contains(&keyword.to_lowercase())
                {
                    relevance += 0.15;
                }
            }

            if relevance > 0.1 {
                relevant.push(RelevantProcedure {
                    procedure_id: procedure.id.clone(),
                    procedure_name: procedure.name.clone(),
                    relevance_score: relevance.min(1.0),
                    matching_steps: procedure.steps.len(),
                    reason: format!("Matches task keywords: {}", task_keywords.join(", ")),
                });
            }
        }

        // Sort by relevance
        relevant.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(relevant)
    }

    // Helper methods
    /// Enhanced: Get analyzed sections (structs are always present, not Optional)
    fn get_analyzed_sections(&self, sop: &SOPDocument) -> Vec<String> {
        let mut sections = vec!["Title".to_string(), "Purpose".to_string()];

        // Scope is always present as a struct
        sections.push("Scope".to_string());

        if !sop.roles.is_empty() {
            sections.push("Roles".to_string());
        }

        // Materials is always present as a struct
        sections.push("Materials".to_string());

        // Architecture is always present as a struct
        sections.push("Architecture".to_string());

        if !sop.procedures.is_empty() {
            sections.push("Procedures".to_string());
        }

        // Quality checks is always present as a struct
        sections.push("Quality Checks".to_string());
        if !sop.glossary.is_empty() {
            sections.push("Glossary".to_string());
        }

        sections
    }

    fn extract_keywords(&self, text: &str) -> Vec<String> {
        // Simple keyword extraction (in production, use NLP)
        text.split_whitespace()
            .filter(|w| w.len() > 3)
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| !w.is_empty())
            .collect()
    }
}

/// Content analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysis {
    pub status: AnalysisStatus,
    pub completeness_score: f32,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub analyzed_sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalysisStatus {
    Complete,
    NeedsImprovement,
    Incomplete,
}

/// Procedure validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureValidation {
    pub is_aligned: bool,
    pub alignment_score: f32,
    pub relevant_steps: Vec<RelevantStep>,
    pub gaps: Vec<String>,
    pub required_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevantStep {
    pub substep: String,
    pub relevance_score: f32,
    pub reason: String,
}

/// Key concept extracted from SOP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub name: String,
    pub category: ConceptCategory,
    pub description: String,
    pub importance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptCategory {
    Objective,
    Procedure,
    Role,
    Tool,
    QualityCheck,
    Constraint,
}

/// Relevant procedure for a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevantProcedure {
    pub procedure_id: String,
    pub procedure_name: String,
    pub relevance_score: f32,
    pub matching_steps: usize,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_analyzer() {
        let analyzer = AISopAnalyzer::new("test-model".to_string());

        let mut sop = SOPDocument {
            version: "1.0".to_string(),
            generated: "2025-01-01".to_string(),
            title: "Test SOP".to_string(),
            purpose: "Test purpose".to_string(),
            scope: Default::default(),
            roles: vec![],
            materials: Default::default(),
            architecture: Default::default(),
            procedures: vec![],
            quality_checks: Default::default(),
            glossary: Default::default(),
        };

        let analysis = analyzer.analyze_content(&sop).await.unwrap();
        assert!(analysis.completeness_score < 1.0);
        assert!(!analysis.issues.is_empty());
    }

    #[test]
    fn test_extract_keywords() {
        let analyzer = AISopAnalyzer::new("test-model".to_string());
        let keywords = analyzer.extract_keywords("implement workflow processing system");
        assert!(keywords.contains(&"implement".to_string()));
        assert!(keywords.contains(&"workflow".to_string()));
    }
}
