//! 4D Methodology Engine
//! REF: WORKFLOW-003 - 4D Method Implementation Enhancement
//!
//! Implements the 4D methodology (Deconstruct, Diagnose, Develop, Deliver)
//! with automated quality gates, scoring, and validation.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Phase scores for 4D methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scores {
    pub deconstruct: u8,
    pub diagnose: u8,
    pub develop: u8,
    pub deliver: u8,
    pub overall: u8,
    pub passed: bool,
}

/// Quality gate thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGateConfig {
    /// Minimum score to pass each phase (0-100)
    pub min_phase_score: u8,
    /// Minimum overall score to pass (0-100)
    pub min_overall_score: u8,
    /// Whether to fail on any phase failure
    pub fail_on_any_phase_failure: bool,
    /// Required fields for each phase
    pub required_fields: RequiredFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredFields {
    pub deconstruct: Vec<String>,
    pub diagnose: Vec<String>,
    pub develop: Vec<String>,
    pub deliver: Vec<String>,
}

impl Default for QualityGateConfig {
    fn default() -> Self {
        Self {
            min_phase_score: 70,
            min_overall_score: 75,
            fail_on_any_phase_failure: true,
            required_fields: RequiredFields {
                deconstruct: vec![
                    "core_intent".to_string(),
                    "key_entities".to_string(),
                    "output_requirements".to_string(),
                ],
                diagnose: vec![
                    "specificity_level".to_string(),
                    "completeness_score".to_string(),
                    "complexity_assessment".to_string(),
                ],
                develop: vec![
                    "request_type".to_string(),
                    "selected_techniques".to_string(),
                    "ai_role_assignment".to_string(),
                ],
                deliver: vec![
                    "execution_plan".to_string(),
                    "verification_protocol".to_string(),
                    "deliverable_specifications".to_string(),
                ],
            },
        }
    }
}

/// 4D methodology input for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyInput {
    pub request_id: Uuid,
    pub raw_request: String,
    pub context: HashMap<String, serde_json::Value>,
    pub constraints: Vec<String>,
}

/// 4D methodology output after processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyOutput {
    pub input: MethodologyInput,
    pub deconstruct_result: DeconstructResult,
    pub diagnose_result: DiagnoseResult,
    pub develop_result: DevelopResult,
    pub deliver_result: DeliverResult,
    pub scores: Scores,
    pub gate_result: QualityGateResult,
    pub processed_at: DateTime<Utc>,
}

/// Deconstruct phase result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeconstructResult {
    pub core_intent: String,
    pub key_entities: Vec<String>,
    pub context_analysis: String,
    pub output_requirements: Vec<String>,
    pub constraints: Vec<String>,
    pub provided_vs_missing: HashMap<String, FieldStatus>,
    pub score: u8,
    pub validation_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldStatus {
    Provided,
    Missing,
    Inferred,
    Partial,
}

/// Diagnose phase result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnoseResult {
    pub clarity_gaps: Vec<ClarityGap>,
    pub ambiguity_points: Vec<String>,
    pub specificity_level: SpecificityLevel,
    pub completeness_score: f32,
    pub structure_needs: Vec<String>,
    pub complexity_assessment: ComplexityLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub score: u8,
    pub validation_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarityGap {
    pub area: String,
    pub description: String,
    pub severity: GapSeverity,
    pub suggested_resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GapSeverity {
    Critical,
    Major,
    Minor,
    Cosmetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecificityLevel {
    Vague,
    Moderate,
    Specific,
    Precise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    HighlyComplex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor: String,
    pub impact: RiskImpact,
    pub probability: f32,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskImpact {
    Low,
    Medium,
    High,
    Critical,
}

/// Develop phase result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopResult {
    pub request_type: RequestType,
    pub selected_techniques: Vec<OptimizationTechnique>,
    pub ai_role_assignment: String,
    pub context_enhancement: String,
    pub logical_structure: String,
    pub implementation_approach: String,
    pub score: u8,
    pub validation_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    Creative,
    Technical,
    Educational,
    Complex,
    Research,
    Implementation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationTechnique {
    MultiPerspective,
    ToneEmphasis,
    ConstraintBased,
    PrecisionFocus,
    FewShotExamples,
    ClearStructure,
    ChainOfThought,
    SystematicFrameworks,
    IterativeRefinement,
    DomainExpertise,
}

/// Deliver phase result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverResult {
    pub execution_plan: Vec<ExecutionStep>,
    pub verification_protocol: VerificationProtocol,
    pub deliverable_specifications: Vec<DeliverableSpec>,
    pub target_locations: Vec<TargetLocation>,
    pub timeline_estimate: TimelineEstimate,
    pub score: u8,
    pub validation_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_id: String,
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub verification_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProtocol {
    pub pass_a: VerificationPass,
    pub pass_b: VerificationPass,
    pub pass_c: VerificationPass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationPass {
    pub name: String,
    pub criteria: Vec<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverableSpec {
    pub name: String,
    pub deliverable_type: String,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetLocation {
    pub path: String,
    pub location_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEstimate {
    pub total_hours: f32,
    pub phases: Vec<PhaseEstimate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseEstimate {
    pub phase: String,
    pub hours: f32,
}

/// Quality gate evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGateResult {
    pub passed: bool,
    pub phase_results: HashMap<String, PhaseGateResult>,
    pub blocking_issues: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseGateResult {
    pub phase: String,
    pub passed: bool,
    pub score: u8,
    pub missing_requirements: Vec<String>,
    pub notes: Vec<String>,
}

/// Main 4D Methodology Engine
pub struct MethodologyEngine {
    config: QualityGateConfig,
}

impl MethodologyEngine {
    pub fn new() -> Self {
        Self {
            config: QualityGateConfig::default(),
        }
    }

    pub fn with_config(config: QualityGateConfig) -> Self {
        Self { config }
    }

    /// Process input through all 4D phases
    pub fn process(&self, input: MethodologyInput) -> Result<MethodologyOutput> {
        // Phase 1: Deconstruct
        let deconstruct_result = self.deconstruct(&input)?;

        // Phase 2: Diagnose
        let diagnose_result = self.diagnose(&input, &deconstruct_result)?;

        // Phase 3: Develop
        let develop_result = self.develop(&input, &deconstruct_result, &diagnose_result)?;

        // Phase 4: Deliver
        let deliver_result = self.deliver(&input, &deconstruct_result, &diagnose_result, &develop_result)?;

        // Calculate scores
        let scores = self.calculate_scores(
            &deconstruct_result,
            &diagnose_result,
            &develop_result,
            &deliver_result,
        );

        // Evaluate quality gates
        let gate_result = self.evaluate_quality_gates(&scores, &[
            ("deconstruct", &deconstruct_result.validation_notes),
            ("diagnose", &diagnose_result.validation_notes),
            ("develop", &develop_result.validation_notes),
            ("deliver", &deliver_result.validation_notes),
        ]);

        Ok(MethodologyOutput {
            input,
            deconstruct_result,
            diagnose_result,
            develop_result,
            deliver_result,
            scores,
            gate_result,
            processed_at: Utc::now(),
        })
    }

    /// Phase 1: Deconstruct the request
    fn deconstruct(&self, input: &MethodologyInput) -> Result<DeconstructResult> {
        let mut validation_notes = Vec::new();
        let mut provided_vs_missing = HashMap::new();

        // Extract core intent from request
        let core_intent = self.extract_core_intent(&input.raw_request);
        if core_intent.is_empty() {
            validation_notes.push("Unable to extract clear core intent".to_string());
        }

        // Extract key entities
        let key_entities = self.extract_key_entities(&input.raw_request);
        provided_vs_missing.insert(
            "key_entities".to_string(),
            if key_entities.is_empty() { FieldStatus::Missing } else { FieldStatus::Provided }
        );

        // Analyze context
        let context_analysis = self.analyze_context(&input.context);

        // Extract output requirements
        let output_requirements = self.extract_output_requirements(&input.raw_request);
        provided_vs_missing.insert(
            "output_requirements".to_string(),
            if output_requirements.is_empty() { FieldStatus::Inferred } else { FieldStatus::Provided }
        );

        // Calculate phase score
        let score = self.calculate_deconstruct_score(&core_intent, &key_entities, &output_requirements);

        Ok(DeconstructResult {
            core_intent,
            key_entities,
            context_analysis,
            output_requirements,
            constraints: input.constraints.clone(),
            provided_vs_missing,
            score,
            validation_notes,
        })
    }

    /// Phase 2: Diagnose requirements
    fn diagnose(&self, input: &MethodologyInput, deconstruct: &DeconstructResult) -> Result<DiagnoseResult> {
        let mut validation_notes = Vec::new();

        // Identify clarity gaps
        let clarity_gaps = self.identify_clarity_gaps(deconstruct);

        // Find ambiguity points
        let ambiguity_points = self.find_ambiguity_points(&input.raw_request);

        // Determine specificity level
        let specificity_level = self.determine_specificity(deconstruct);

        // Calculate completeness
        let completeness_score = self.calculate_completeness(deconstruct);
        if completeness_score < 0.7 {
            validation_notes.push(format!("Low completeness score: {:.2}", completeness_score));
        }

        // Identify structure needs
        let structure_needs = self.identify_structure_needs(deconstruct);

        // Assess complexity
        let complexity_assessment = self.assess_complexity(deconstruct, &clarity_gaps);

        // Identify risk factors
        let risk_factors = self.identify_risks(deconstruct, &complexity_assessment);

        let score = self.calculate_diagnose_score(&clarity_gaps, &completeness_score, &specificity_level);

        Ok(DiagnoseResult {
            clarity_gaps,
            ambiguity_points,
            specificity_level,
            completeness_score,
            structure_needs,
            complexity_assessment,
            risk_factors,
            score,
            validation_notes,
        })
    }

    /// Phase 3: Develop approach
    fn develop(&self, input: &MethodologyInput, deconstruct: &DeconstructResult, diagnose: &DiagnoseResult) -> Result<DevelopResult> {
        let mut validation_notes = Vec::new();

        // Determine request type
        let request_type = self.determine_request_type(&input.raw_request, diagnose);

        // Select optimization techniques
        let selected_techniques = self.select_techniques(&request_type, diagnose);
        if selected_techniques.is_empty() {
            validation_notes.push("No optimization techniques selected".to_string());
        }

        // Assign AI role
        let ai_role_assignment = self.assign_ai_role(&request_type, deconstruct);

        // Enhance context
        let context_enhancement = self.enhance_context(deconstruct, diagnose);

        // Define logical structure
        let logical_structure = self.define_logical_structure(diagnose);

        // Define implementation approach
        let implementation_approach = self.define_implementation_approach(&request_type, diagnose);

        let score = self.calculate_develop_score(&selected_techniques, &ai_role_assignment);

        Ok(DevelopResult {
            request_type,
            selected_techniques,
            ai_role_assignment,
            context_enhancement,
            logical_structure,
            implementation_approach,
            score,
            validation_notes,
        })
    }

    /// Phase 4: Deliver plan
    fn deliver(&self, input: &MethodologyInput, deconstruct: &DeconstructResult, diagnose: &DiagnoseResult, develop: &DevelopResult) -> Result<DeliverResult> {
        let mut validation_notes = Vec::new();

        // Create execution plan
        let execution_plan = self.create_execution_plan(deconstruct, diagnose, develop);
        if execution_plan.is_empty() {
            validation_notes.push("No execution steps defined".to_string());
        }

        // Define verification protocol
        let verification_protocol = self.create_verification_protocol(diagnose);

        // Define deliverables
        let deliverable_specifications = self.define_deliverables(deconstruct, develop);

        // Determine target locations
        let target_locations = self.determine_target_locations(&deliverable_specifications);

        // Estimate timeline
        let timeline_estimate = self.estimate_timeline(&execution_plan, diagnose);

        let score = self.calculate_deliver_score(&execution_plan, &deliverable_specifications);

        Ok(DeliverResult {
            execution_plan,
            verification_protocol,
            deliverable_specifications,
            target_locations,
            timeline_estimate,
            score,
            validation_notes,
        })
    }

    // Helper methods for each phase
    fn extract_core_intent(&self, request: &str) -> String {
        // Extract the main intent from the request
        let sentences: Vec<&str> = request.split(['.', '!', '?']).collect();
        sentences.first().map(|s| s.trim().to_string()).unwrap_or_default()
    }

    fn extract_key_entities(&self, request: &str) -> Vec<String> {
        // Extract key nouns and concepts
        let words: Vec<&str> = request.split_whitespace().collect();
        words.iter()
            .filter(|w| w.len() > 3 && w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
            .map(|w| w.to_string())
            .collect()
    }

    fn analyze_context(&self, context: &HashMap<String, serde_json::Value>) -> String {
        if context.is_empty() {
            "No additional context provided".to_string()
        } else {
            format!("Context includes {} items", context.len())
        }
    }

    fn extract_output_requirements(&self, request: &str) -> Vec<String> {
        let mut requirements = Vec::new();
        let lower = request.to_lowercase();

        if lower.contains("create") || lower.contains("build") {
            requirements.push("Implementation artifact".to_string());
        }
        if lower.contains("document") || lower.contains("explain") {
            requirements.push("Documentation".to_string());
        }
        if lower.contains("test") || lower.contains("verify") {
            requirements.push("Test suite".to_string());
        }

        requirements
    }

    fn calculate_deconstruct_score(&self, intent: &str, entities: &[String], requirements: &[String]) -> u8 {
        let mut score = 50u8;

        if !intent.is_empty() { score += 20; }
        if !entities.is_empty() { score += 15; }
        if !requirements.is_empty() { score += 15; }

        score.min(100)
    }

    fn identify_clarity_gaps(&self, deconstruct: &DeconstructResult) -> Vec<ClarityGap> {
        let mut gaps = Vec::new();

        for (field, status) in &deconstruct.provided_vs_missing {
            if matches!(status, FieldStatus::Missing | FieldStatus::Partial) {
                gaps.push(ClarityGap {
                    area: field.clone(),
                    description: format!("{} is not fully specified", field),
                    severity: GapSeverity::Major,
                    suggested_resolution: format!("Please clarify {}", field),
                });
            }
        }

        gaps
    }

    fn find_ambiguity_points(&self, request: &str) -> Vec<String> {
        let mut points = Vec::new();

        // Check for ambiguous words
        let ambiguous_words = ["some", "maybe", "possibly", "might", "could", "various"];
        for word in ambiguous_words {
            if request.to_lowercase().contains(word) {
                points.push(format!("Request contains ambiguous term: '{}'", word));
            }
        }

        points
    }

    fn determine_specificity(&self, deconstruct: &DeconstructResult) -> SpecificityLevel {
        let score = deconstruct.score;

        if score >= 90 { SpecificityLevel::Precise }
        else if score >= 75 { SpecificityLevel::Specific }
        else if score >= 50 { SpecificityLevel::Moderate }
        else { SpecificityLevel::Vague }
    }

    fn calculate_completeness(&self, deconstruct: &DeconstructResult) -> f32 {
        let total_fields = deconstruct.provided_vs_missing.len() as f32;
        let provided = deconstruct.provided_vs_missing.values()
            .filter(|s| matches!(s, FieldStatus::Provided))
            .count() as f32;

        if total_fields > 0.0 { provided / total_fields } else { 0.0 }
    }

    fn identify_structure_needs(&self, deconstruct: &DeconstructResult) -> Vec<String> {
        let mut needs = Vec::new();

        if deconstruct.output_requirements.len() > 3 {
            needs.push("Consider breaking down into smaller tasks".to_string());
        }
        if deconstruct.constraints.len() > 5 {
            needs.push("Many constraints - prioritize key ones".to_string());
        }

        needs
    }

    fn assess_complexity(&self, deconstruct: &DeconstructResult, gaps: &[ClarityGap]) -> ComplexityLevel {
        let req_count = deconstruct.output_requirements.len();
        let gap_count = gaps.len();

        if req_count > 5 || gap_count > 3 { ComplexityLevel::HighlyComplex }
        else if req_count > 3 || gap_count > 1 { ComplexityLevel::Complex }
        else if req_count > 1 { ComplexityLevel::Moderate }
        else { ComplexityLevel::Simple }
    }

    fn identify_risks(&self, _deconstruct: &DeconstructResult, complexity: &ComplexityLevel) -> Vec<RiskFactor> {
        let mut risks = Vec::new();

        if matches!(complexity, ComplexityLevel::HighlyComplex | ComplexityLevel::Complex) {
            risks.push(RiskFactor {
                factor: "High complexity may lead to scope creep".to_string(),
                impact: RiskImpact::Medium,
                probability: 0.4,
                mitigation: "Define clear boundaries and milestones".to_string(),
            });
        }

        risks
    }

    fn calculate_diagnose_score(&self, gaps: &[ClarityGap], completeness: &f32, specificity: &SpecificityLevel) -> u8 {
        let mut score = 50u8;

        // Penalize for gaps
        score = score.saturating_sub((gaps.len() * 5) as u8);

        // Bonus for completeness
        score = score.saturating_add((completeness * 30.0) as u8);

        // Bonus for specificity
        score += match specificity {
            SpecificityLevel::Precise => 20,
            SpecificityLevel::Specific => 15,
            SpecificityLevel::Moderate => 10,
            SpecificityLevel::Vague => 0,
        };

        score.min(100)
    }

    fn determine_request_type(&self, request: &str, _diagnose: &DiagnoseResult) -> RequestType {
        let lower = request.to_lowercase();

        if lower.contains("implement") || lower.contains("build") || lower.contains("create") {
            RequestType::Implementation
        } else if lower.contains("research") || lower.contains("analyze") {
            RequestType::Research
        } else if lower.contains("teach") || lower.contains("explain") {
            RequestType::Educational
        } else if lower.contains("design") || lower.contains("creative") {
            RequestType::Creative
        } else {
            RequestType::Technical
        }
    }

    fn select_techniques(&self, request_type: &RequestType, _diagnose: &DiagnoseResult) -> Vec<OptimizationTechnique> {
        match request_type {
            RequestType::Technical | RequestType::Implementation => vec![
                OptimizationTechnique::ConstraintBased,
                OptimizationTechnique::PrecisionFocus,
                OptimizationTechnique::SystematicFrameworks,
            ],
            RequestType::Creative => vec![
                OptimizationTechnique::MultiPerspective,
                OptimizationTechnique::ToneEmphasis,
            ],
            RequestType::Educational => vec![
                OptimizationTechnique::ClearStructure,
                OptimizationTechnique::FewShotExamples,
            ],
            _ => vec![
                OptimizationTechnique::ChainOfThought,
                OptimizationTechnique::IterativeRefinement,
            ],
        }
    }

    fn assign_ai_role(&self, request_type: &RequestType, _deconstruct: &DeconstructResult) -> String {
        match request_type {
            RequestType::Technical => "Senior Systems Engineer".to_string(),
            RequestType::Implementation => "Senior Software Developer".to_string(),
            RequestType::Research => "Research Analyst".to_string(),
            RequestType::Educational => "Technical Educator".to_string(),
            RequestType::Creative => "Creative Director".to_string(),
            RequestType::Complex => "Systems Architect".to_string(),
        }
    }

    fn enhance_context(&self, deconstruct: &DeconstructResult, _diagnose: &DiagnoseResult) -> String {
        format!("Enhanced with {} entities and {} requirements",
                deconstruct.key_entities.len(),
                deconstruct.output_requirements.len())
    }

    fn define_logical_structure(&self, _diagnose: &DiagnoseResult) -> String {
        "Sequential processing with verification gates".to_string()
    }

    fn define_implementation_approach(&self, request_type: &RequestType, diagnose: &DiagnoseResult) -> String {
        match (&diagnose.complexity_assessment, request_type) {
            (ComplexityLevel::Simple, _) => "Direct implementation".to_string(),
            (ComplexityLevel::Moderate, _) => "Phased implementation with checkpoints".to_string(),
            (_, RequestType::Implementation) => "Iterative development with continuous testing".to_string(),
            _ => "Systematic approach with milestone verification".to_string(),
        }
    }

    fn calculate_develop_score(&self, techniques: &[OptimizationTechnique], role: &str) -> u8 {
        let mut score = 60u8;

        score += (techniques.len() * 10).min(30) as u8;
        if !role.is_empty() { score += 10; }

        score.min(100)
    }

    fn create_execution_plan(&self, deconstruct: &DeconstructResult, _diagnose: &DiagnoseResult, _develop: &DevelopResult) -> Vec<ExecutionStep> {
        deconstruct.output_requirements.iter().enumerate().map(|(i, req)| {
            ExecutionStep {
                step_id: format!("step-{}", i + 1),
                name: format!("Implement: {}", req),
                description: format!("Complete implementation of {}", req),
                dependencies: if i > 0 { vec![format!("step-{}", i)] } else { vec![] },
                verification_criteria: vec!["Unit tests pass".to_string(), "Code review complete".to_string()],
            }
        }).collect()
    }

    fn create_verification_protocol(&self, _diagnose: &DiagnoseResult) -> VerificationProtocol {
        VerificationProtocol {
            pass_a: VerificationPass {
                name: "Self-check".to_string(),
                criteria: vec!["Code compiles".to_string(), "Tests pass".to_string()],
                required: true,
            },
            pass_b: VerificationPass {
                name: "Independent review".to_string(),
                criteria: vec!["Code review".to_string(), "Documentation review".to_string()],
                required: true,
            },
            pass_c: VerificationPass {
                name: "Adversarial validation".to_string(),
                criteria: vec!["Edge case testing".to_string(), "Security review".to_string()],
                required: false,
            },
        }
    }

    fn define_deliverables(&self, deconstruct: &DeconstructResult, _develop: &DevelopResult) -> Vec<DeliverableSpec> {
        deconstruct.output_requirements.iter().map(|req| {
            DeliverableSpec {
                name: req.clone(),
                deliverable_type: "Implementation".to_string(),
                acceptance_criteria: vec![
                    "Meets requirements".to_string(),
                    "Tests pass".to_string(),
                    "Documentation complete".to_string(),
                ],
            }
        }).collect()
    }

    fn determine_target_locations(&self, deliverables: &[DeliverableSpec]) -> Vec<TargetLocation> {
        deliverables.iter().map(|d| {
            TargetLocation {
                path: format!("agentaskit-production/{}", d.name.to_lowercase().replace(' ', "_")),
                location_type: "ProductionDirectory".to_string(),
            }
        }).collect()
    }

    fn estimate_timeline(&self, steps: &[ExecutionStep], diagnose: &DiagnoseResult) -> TimelineEstimate {
        let complexity_factor = match diagnose.complexity_assessment {
            ComplexityLevel::Simple => 1.0,
            ComplexityLevel::Moderate => 1.5,
            ComplexityLevel::Complex => 2.0,
            ComplexityLevel::HighlyComplex => 3.0,
        };

        let hours_per_step = 2.0 * complexity_factor;
        let total_hours = steps.len() as f32 * hours_per_step;

        TimelineEstimate {
            total_hours,
            phases: vec![
                PhaseEstimate { phase: "Implementation".to_string(), hours: total_hours * 0.6 },
                PhaseEstimate { phase: "Testing".to_string(), hours: total_hours * 0.25 },
                PhaseEstimate { phase: "Documentation".to_string(), hours: total_hours * 0.15 },
            ],
        }
    }

    fn calculate_deliver_score(&self, steps: &[ExecutionStep], deliverables: &[DeliverableSpec]) -> u8 {
        let mut score = 50u8;

        if !steps.is_empty() { score += 25; }
        if !deliverables.is_empty() { score += 25; }

        score.min(100)
    }

    /// Calculate overall scores
    fn calculate_scores(&self, deconstruct: &DeconstructResult, diagnose: &DiagnoseResult, develop: &DevelopResult, deliver: &DeliverResult) -> Scores {
        let overall = ((deconstruct.score as u16 + diagnose.score as u16 + develop.score as u16 + deliver.score as u16) / 4) as u8;

        let passed = overall >= self.config.min_overall_score &&
            (!self.config.fail_on_any_phase_failure || (
                deconstruct.score >= self.config.min_phase_score &&
                diagnose.score >= self.config.min_phase_score &&
                develop.score >= self.config.min_phase_score &&
                deliver.score >= self.config.min_phase_score
            ));

        Scores {
            deconstruct: deconstruct.score,
            diagnose: diagnose.score,
            develop: develop.score,
            deliver: deliver.score,
            overall,
            passed,
        }
    }

    /// Evaluate quality gates
    fn evaluate_quality_gates(&self, scores: &Scores, phase_notes: &[(&str, &Vec<String>)]) -> QualityGateResult {
        let mut phase_results = HashMap::new();
        let mut blocking_issues = Vec::new();
        let mut recommendations = Vec::new();

        for (phase, notes) in phase_notes {
            let score = match *phase {
                "deconstruct" => scores.deconstruct,
                "diagnose" => scores.diagnose,
                "develop" => scores.develop,
                "deliver" => scores.deliver,
                _ => 0,
            };

            let passed = score >= self.config.min_phase_score;

            if !passed {
                blocking_issues.push(format!("Phase {} failed with score {}", phase, score));
            }

            phase_results.insert(phase.to_string(), PhaseGateResult {
                phase: phase.to_string(),
                passed,
                score,
                missing_requirements: Vec::new(),
                notes: notes.clone(),
            });
        }

        if scores.overall < 80 {
            recommendations.push("Consider additional clarification on requirements".to_string());
        }

        QualityGateResult {
            passed: scores.passed,
            phase_results,
            blocking_issues,
            recommendations,
        }
    }
}

impl Default for MethodologyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy compatibility function
#[allow(dead_code)]
pub fn score_all() -> Scores {
    Scores {
        deconstruct: 0,
        diagnose: 0,
        develop: 0,
        deliver: 0,
        overall: 0,
        passed: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methodology_processing() {
        let engine = MethodologyEngine::new();

        let input = MethodologyInput {
            request_id: Uuid::new_v4(),
            raw_request: "Create a health monitoring service for the AgentAskit system".to_string(),
            context: HashMap::new(),
            constraints: vec!["Must be production-ready".to_string()],
        };

        let result = engine.process(input).unwrap();

        assert!(!result.deconstruct_result.core_intent.is_empty());
        assert!(result.scores.overall > 0);
    }

    #[test]
    fn test_quality_gates() {
        let config = QualityGateConfig {
            min_phase_score: 50,
            min_overall_score: 50,
            ..Default::default()
        };
        let engine = MethodologyEngine::with_config(config);

        let input = MethodologyInput {
            request_id: Uuid::new_v4(),
            raw_request: "Implement a complete testing framework".to_string(),
            context: HashMap::new(),
            constraints: Vec::new(),
        };

        let result = engine.process(input).unwrap();
        assert!(result.gate_result.passed);
    }
}
