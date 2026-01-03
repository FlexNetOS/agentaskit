//! SOP (Standard Operating Procedure) Parser and Analyzer
//! REF: WORKFLOW-002 - AI Model SOP Reading Integration
//!
//! This module provides high-performance parsing of SOP documents with
//! structured output for AI model integration. Target: ≥99% accuracy, p95 <10ms.

use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// Parsed SOP document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSOP {
    /// Document title
    pub title: String,
    /// Document version
    pub version: String,
    /// Last updated timestamp
    pub last_updated: Option<String>,
    /// Document sections
    pub sections: Vec<SOPSection>,
    /// All parsed steps (flattened)
    pub steps: Vec<SOPStep>,
    /// Metadata extracted from document
    pub metadata: HashMap<String, String>,
    /// Parse metrics
    pub parse_metrics: ParseMetrics,
}

/// SOP section with hierarchical structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPSection {
    /// Section number (e.g., "1", "1.1", "2.3.1")
    pub number: String,
    /// Section title
    pub title: String,
    /// Section level (0 = top-level)
    pub level: usize,
    /// Steps within this section
    pub steps: Vec<SOPStep>,
    /// Child sections
    pub subsections: Vec<SOPSection>,
}

/// Individual SOP step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPStep {
    /// Step number/identifier
    pub step_id: String,
    /// Step description
    pub description: String,
    /// Action type
    pub action_type: ActionType,
    /// Prerequisites for this step
    pub prerequisites: Vec<String>,
    /// Expected outcomes
    pub expected_outcomes: Vec<String>,
    /// Verification criteria
    pub verification: Vec<String>,
    /// Notes or warnings
    pub notes: Vec<String>,
    /// Whether this step is mandatory
    pub mandatory: bool,
}

/// Type of action in a step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    Execute,
    Verify,
    Document,
    Review,
    Approve,
    Deploy,
    Monitor,
    Rollback,
    Other(String),
}

/// Parse performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParseMetrics {
    pub parse_time_ms: f64,
    pub lines_processed: usize,
    pub sections_found: usize,
    pub steps_found: usize,
    pub accuracy_score: f64,
}

/// SOP validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPValidation {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
    pub compliance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub location: String,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// Main SOP parser with high-performance implementation
pub struct SOPParser {
    /// Enable strict mode for validation
    strict_mode: bool,
    /// Maximum parse time before timeout (ms)
    timeout_ms: u64,
}

impl SOPParser {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            timeout_ms: 10, // p95 target
        }
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Parse SOP text content with high accuracy
    /// Target: ≥99% accuracy, p95 parse <10ms
    pub fn parse(&self, text: &str) -> Result<ParsedSOP> {
        let start = Instant::now();

        let lines: Vec<&str> = text.lines().collect();
        let lines_count = lines.len();

        // Extract metadata from header
        let (title, version, last_updated) = self.extract_header_metadata(&lines);
        let metadata = self.extract_all_metadata(&lines);

        // Parse sections hierarchically
        let sections = self.parse_sections(&lines)?;

        // Flatten all steps
        let steps = self.flatten_steps(&sections);

        let parse_time = start.elapsed().as_secs_f64() * 1000.0;

        let metrics = ParseMetrics {
            parse_time_ms: parse_time,
            lines_processed: lines_count,
            sections_found: self.count_sections(&sections),
            steps_found: steps.len(),
            accuracy_score: self.calculate_accuracy_score(&sections, &steps),
        };

        Ok(ParsedSOP {
            title,
            version,
            last_updated,
            sections,
            steps,
            metadata,
            parse_metrics: metrics,
        })
    }

    /// Extract header metadata
    fn extract_header_metadata(&self, lines: &[&str]) -> (String, String, Option<String>) {
        let mut title = String::from("Untitled SOP");
        let mut version = String::from("1.0");
        let mut last_updated = None;

        for line in lines.iter().take(20) {
            let trimmed = line.trim();

            // Title extraction
            if trimmed.starts_with("# ") && title == "Untitled SOP" {
                title = trimmed[2..].trim().to_string();
            }

            // Version extraction
            if let Some(v) = self.extract_field(trimmed, "Version:") {
                version = v;
            } else if let Some(v) = self.extract_field(trimmed, "**Version:**") {
                version = v;
            }

            // Date extraction
            if let Some(d) = self.extract_field(trimmed, "Date:") {
                last_updated = Some(d);
            } else if let Some(d) = self.extract_field(trimmed, "Last Updated:") {
                last_updated = Some(d);
            } else if let Some(d) = self.extract_field(trimmed, "**Date:**") {
                last_updated = Some(d);
            }
        }

        (title, version, last_updated)
    }

    fn extract_field(&self, line: &str, prefix: &str) -> Option<String> {
        if line.contains(prefix) {
            line.split(prefix)
                .nth(1)
                .map(|s| s.trim().to_string())
        } else {
            None
        }
    }

    fn extract_all_metadata(&self, lines: &[&str]) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        let metadata_keys = [
            "Author:", "Owner:", "Reviewer:", "Status:",
            "Category:", "Department:", "Priority:"
        ];

        for line in lines.iter().take(50) {
            let trimmed = line.trim();
            for key in &metadata_keys {
                if let Some(value) = self.extract_field(trimmed, key) {
                    let clean_key = key.trim_end_matches(':').to_string();
                    metadata.insert(clean_key, value);
                }
            }
        }

        metadata
    }

    /// Parse sections hierarchically
    fn parse_sections(&self, lines: &[&str]) -> Result<Vec<SOPSection>> {
        let mut sections = Vec::new();
        let mut current_section: Option<SOPSection> = None;
        let mut current_steps = Vec::new();
        let mut in_step = false;
        let mut current_step_lines = Vec::new();

        for line in lines {
            let trimmed = line.trim();

            // Skip empty lines and metadata
            if trimmed.is_empty() {
                continue;
            }

            // Detect section headers (## or ###)
            if let Some(section) = self.parse_section_header(trimmed) {
                // Save previous section
                if let Some(mut sec) = current_section.take() {
                    if !current_step_lines.is_empty() {
                        if let Some(step) = self.parse_step_block(&current_step_lines) {
                            current_steps.push(step);
                        }
                        current_step_lines.clear();
                    }
                    sec.steps = current_steps.clone();
                    sections.push(sec);
                    current_steps.clear();
                }
                current_section = Some(section);
                in_step = false;
                continue;
            }

            // Detect step start
            if self.is_step_start(trimmed) {
                if !current_step_lines.is_empty() {
                    if let Some(step) = self.parse_step_block(&current_step_lines) {
                        current_steps.push(step);
                    }
                    current_step_lines.clear();
                }
                in_step = true;
            }

            if in_step || self.is_step_content(trimmed) {
                current_step_lines.push(trimmed.to_string());
            }
        }

        // Handle last section and step
        if !current_step_lines.is_empty() {
            if let Some(step) = self.parse_step_block(&current_step_lines) {
                current_steps.push(step);
            }
        }
        if let Some(mut sec) = current_section {
            sec.steps = current_steps;
            sections.push(sec);
        }

        Ok(sections)
    }

    fn parse_section_header(&self, line: &str) -> Option<SOPSection> {
        let header_pattern = if line.starts_with("### ") {
            Some((3, &line[4..]))
        } else if line.starts_with("## ") {
            Some((2, &line[3..]))
        } else if line.starts_with("# ") && !line.starts_with("# AgentAsKit") {
            Some((1, &line[2..]))
        } else {
            None
        };

        header_pattern.map(|(level, title)| {
            let (number, title) = self.extract_section_number(title);
            SOPSection {
                number,
                title: title.to_string(),
                level,
                steps: Vec::new(),
                subsections: Vec::new(),
            }
        })
    }

    fn extract_section_number(&self, title: &str) -> (String, &str) {
        let title = title.trim();

        // Try to extract leading number like "1.1" or "2.3.1"
        let mut num_end = 0;
        for (i, c) in title.char_indices() {
            if c.is_numeric() || c == '.' {
                num_end = i + 1;
            } else {
                break;
            }
        }

        if num_end > 0 {
            let number = title[..num_end].trim_end_matches('.').to_string();
            let rest = title[num_end..].trim();
            (number, rest)
        } else {
            (String::new(), title)
        }
    }

    fn is_step_start(&self, line: &str) -> bool {
        // Numbered steps: "1.", "1)", "Step 1:", etc.
        if line.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
            return line.contains('.') || line.contains(')') || line.contains(':');
        }

        // Bullet points
        line.starts_with("- ") || line.starts_with("* ") || line.starts_with("• ")
    }

    fn is_step_content(&self, line: &str) -> bool {
        // Indented content, sub-bullets, verification criteria
        line.starts_with("  ") ||
        line.starts_with("\t") ||
        line.starts_with("    - ") ||
        line.contains("Prerequisite:") ||
        line.contains("Expected:") ||
        line.contains("Verify:") ||
        line.contains("Note:") ||
        line.contains("Warning:")
    }

    fn parse_step_block(&self, lines: &[String]) -> Option<SOPStep> {
        if lines.is_empty() {
            return None;
        }

        let first_line = &lines[0];
        let (step_id, description) = self.extract_step_id_and_desc(first_line);

        let mut prerequisites = Vec::new();
        let mut expected_outcomes = Vec::new();
        let mut verification = Vec::new();
        let mut notes = Vec::new();
        let mut mandatory = true;

        for line in lines.iter().skip(1) {
            let lower = line.to_lowercase();
            if lower.contains("prerequisite:") || lower.contains("requires:") {
                prerequisites.push(self.extract_after_colon(line));
            } else if lower.contains("expected:") || lower.contains("outcome:") {
                expected_outcomes.push(self.extract_after_colon(line));
            } else if lower.contains("verify:") || lower.contains("validation:") {
                verification.push(self.extract_after_colon(line));
            } else if lower.contains("note:") || lower.contains("warning:") {
                notes.push(self.extract_after_colon(line));
            } else if lower.contains("optional") {
                mandatory = false;
            }
        }

        let action_type = self.determine_action_type(&description);

        Some(SOPStep {
            step_id,
            description,
            action_type,
            prerequisites,
            expected_outcomes,
            verification,
            notes,
            mandatory,
        })
    }

    fn extract_step_id_and_desc(&self, line: &str) -> (String, String) {
        let line = line.trim();

        // Handle "1. Description" or "1) Description"
        if let Some(pos) = line.find('.').or_else(|| line.find(')')) {
            let prefix = &line[..pos];
            if prefix.chars().all(|c| c.is_numeric() || c == ' ') {
                let id = prefix.trim().to_string();
                let desc = line[pos + 1..].trim().to_string();
                return (id, desc);
            }
        }

        // Handle "Step X: Description"
        if line.to_lowercase().starts_with("step ") {
            if let Some(colon_pos) = line.find(':') {
                let id = line[5..colon_pos].trim().to_string();
                let desc = line[colon_pos + 1..].trim().to_string();
                return (id, desc);
            }
        }

        // Handle bullet points
        if line.starts_with("- ") || line.starts_with("* ") || line.starts_with("• ") {
            return (String::new(), line[2..].trim().to_string());
        }

        (String::new(), line.to_string())
    }

    fn extract_after_colon(&self, line: &str) -> String {
        line.split_once(':')
            .map(|(_, rest)| rest.trim().to_string())
            .unwrap_or_else(|| line.trim().to_string())
    }

    fn determine_action_type(&self, description: &str) -> ActionType {
        let lower = description.to_lowercase();

        if lower.contains("verify") || lower.contains("check") || lower.contains("validate") {
            ActionType::Verify
        } else if lower.contains("document") || lower.contains("record") || lower.contains("log") {
            ActionType::Document
        } else if lower.contains("review") || lower.contains("inspect") {
            ActionType::Review
        } else if lower.contains("approve") || lower.contains("sign off") {
            ActionType::Approve
        } else if lower.contains("deploy") || lower.contains("release") {
            ActionType::Deploy
        } else if lower.contains("monitor") || lower.contains("observe") {
            ActionType::Monitor
        } else if lower.contains("rollback") || lower.contains("revert") {
            ActionType::Rollback
        } else if lower.contains("execute") || lower.contains("run") || lower.contains("perform") {
            ActionType::Execute
        } else {
            ActionType::Other(String::new())
        }
    }

    fn flatten_steps(&self, sections: &[SOPSection]) -> Vec<SOPStep> {
        let mut steps = Vec::new();
        for section in sections {
            steps.extend(section.steps.clone());
            steps.extend(self.flatten_steps(&section.subsections));
        }
        steps
    }

    fn count_sections(&self, sections: &[SOPSection]) -> usize {
        let mut count = sections.len();
        for section in sections {
            count += self.count_sections(&section.subsections);
        }
        count
    }

    fn calculate_accuracy_score(&self, sections: &[SOPSection], steps: &[SOPStep]) -> f64 {
        let mut score = 1.0;

        // Penalize if no sections found
        if sections.is_empty() {
            score -= 0.1;
        }

        // Penalize if no steps found
        if steps.is_empty() {
            score -= 0.2;
        }

        // Check step quality
        let steps_with_id = steps.iter().filter(|s| !s.step_id.is_empty()).count();
        let id_ratio = if !steps.is_empty() {
            steps_with_id as f64 / steps.len() as f64
        } else {
            0.0
        };
        score = score * 0.7 + id_ratio * 0.3;

        score.max(0.0).min(1.0)
    }

    /// Validate a parsed SOP against compliance requirements
    pub fn validate(&self, sop: &ParsedSOP) -> SOPValidation {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check for title
        if sop.title == "Untitled SOP" || sop.title.is_empty() {
            errors.push(ValidationError {
                location: "header".to_string(),
                message: "SOP must have a title".to_string(),
                severity: ErrorSeverity::High,
            });
        }

        // Check for steps
        if sop.steps.is_empty() {
            errors.push(ValidationError {
                location: "content".to_string(),
                message: "SOP must contain at least one step".to_string(),
                severity: ErrorSeverity::Critical,
            });
        }

        // Check step quality
        for step in &sop.steps {
            if step.description.len() < 10 {
                warnings.push(format!("Step '{}' has very short description", step.step_id));
            }

            if step.verification.is_empty() && self.strict_mode {
                warnings.push(format!("Step '{}' has no verification criteria", step.step_id));
            }
        }

        let compliance_score = if errors.is_empty() {
            1.0 - (warnings.len() as f64 * 0.05).min(0.5)
        } else {
            0.5 - (errors.len() as f64 * 0.1).min(0.5)
        };

        SOPValidation {
            valid: errors.iter().all(|e| !matches!(e.severity, ErrorSeverity::Critical)),
            errors,
            warnings,
            compliance_score,
        }
    }
}

impl Default for SOPParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy parse function for backwards compatibility
#[allow(dead_code)]
pub fn parse_sop(text: &str) -> Vec<String> {
    let parser = SOPParser::new();
    match parser.parse(text) {
        Ok(sop) => sop.steps.iter().map(|s| s.description.clone()).collect(),
        Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sop_basic() {
        let text = r#"
# Test SOP
**Version:** 1.0
**Date:** 2025-01-01

## 1. Setup
1. Install dependencies
2. Configure environment

## 2. Execution
1. Run the application
2. Verify output
"#;

        let parser = SOPParser::new();
        let result = parser.parse(text).unwrap();

        assert_eq!(result.title, "Test SOP");
        assert_eq!(result.version, "1.0");
        assert!(!result.sections.is_empty());
        assert!(!result.steps.is_empty());
        assert!(result.parse_metrics.parse_time_ms < 10.0); // p95 target
    }

    #[test]
    fn test_parse_performance() {
        let text = "# SOP\n".to_string() + &"1. Step\n".repeat(1000);

        let parser = SOPParser::new();
        let result = parser.parse(&text).unwrap();

        assert!(result.parse_metrics.parse_time_ms < 10.0,
                "Parse took {}ms, exceeds 10ms target", result.parse_metrics.parse_time_ms);
    }

    #[test]
    fn test_validation() {
        let parser = SOPParser::new().with_strict_mode(true);
        let sop = ParsedSOP {
            title: "Test".to_string(),
            version: "1.0".to_string(),
            last_updated: None,
            sections: Vec::new(),
            steps: vec![SOPStep {
                step_id: "1".to_string(),
                description: "Test step description".to_string(),
                action_type: ActionType::Execute,
                prerequisites: Vec::new(),
                expected_outcomes: Vec::new(),
                verification: Vec::new(),
                notes: Vec::new(),
                mandatory: true,
            }],
            metadata: HashMap::new(),
            parse_metrics: ParseMetrics::default(),
        };

        let validation = parser.validate(&sop);
        assert!(validation.valid);
    }
}
