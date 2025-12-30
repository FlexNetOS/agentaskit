//! 4D Methodology Engine
//!
//! Automated 4D method application with quality gates, scoring, and validation systems
//! for the EnhancedWorkflowProcessor framework.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    ChatRequest, DeconstructPhase, DiagnosePhase, DevelopPhase, DeliverPhase,
    SpecificityLevel, ComplexityLevel, RequestType, OptimizationTechnique,
    VerificationProtocol, ExecutionStep, Deliverable, TargetLocation,
};

/// Comprehensive scores for all 4D phases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scores {
    pub deconstruct: u8,
    pub diagnose: u8,
    pub develop: u8,
    pub deliver: u8,
    pub overall: f32,
    pub quality_gate_passed: bool,
}

/// Quality gate thresholds for each phase
#[derive(Debug, Clone)]
pub struct QualityGates {
    pub min_deconstruct_score: u8,
    pub min_diagnose_score: u8,
    pub min_develop_score: u8,
    pub min_deliver_score: u8,
    pub min_overall_score: f32,
}

impl Default for QualityGates {
    fn default() -> Self {
        Self {
            min_deconstruct_score: 70,
            min_diagnose_score: 70,
            min_develop_score: 70,
            min_deliver_score: 70,
            min_overall_score: 0.75,
        }
    }
}

/// 4D Methodology Engine for automated method application
pub struct MethodologyEngine {
    quality_gates: QualityGates,
    scoring_weights: ScoringWeights,
}

#[derive(Debug, Clone)]
struct ScoringWeights {
    core_intent_clarity: f32,
    entity_completeness: f32,
    constraint_coverage: f32,
    ambiguity_resolution: f32,
    technique_alignment: f32,
    execution_feasibility: f32,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            core_intent_clarity: 0.25,
            entity_completeness: 0.20,
            constraint_coverage: 0.15,
            ambiguity_resolution: 0.15,
            technique_alignment: 0.15,
            execution_feasibility: 0.10,
        }
    }
}

impl MethodologyEngine {
    pub fn new() -> Self {
        Self {
            quality_gates: QualityGates::default(),
            scoring_weights: ScoringWeights::default(),
        }
    }

    pub fn with_quality_gates(mut self, gates: QualityGates) -> Self {
        self.quality_gates = gates;
        self
    }

    /// Score all 4D phases and validate quality gates
    pub fn score_all(
        &self,
        deconstruct: &DeconstructPhase,
        diagnose: &DiagnosePhase,
        develop: &DevelopPhase,
        deliver: &DeliverPhase,
    ) -> Scores {
        let deconstruct_score = self.score_deconstruct(deconstruct);
        let diagnose_score = self.score_diagnose(diagnose);
        let develop_score = self.score_develop(develop);
        let deliver_score = self.score_deliver(deliver);

        let overall = (deconstruct_score as f32 * 0.25
            + diagnose_score as f32 * 0.25
            + develop_score as f32 * 0.25
            + deliver_score as f32 * 0.25)
            / 100.0;

        let quality_gate_passed = deconstruct_score >= self.quality_gates.min_deconstruct_score
            && diagnose_score >= self.quality_gates.min_diagnose_score
            && develop_score >= self.quality_gates.min_develop_score
            && deliver_score >= self.quality_gates.min_deliver_score
            && overall >= self.quality_gates.min_overall_score;

        Scores {
            deconstruct: deconstruct_score,
            diagnose: diagnose_score,
            develop: develop_score,
            deliver: deliver_score,
            overall,
            quality_gate_passed,
        }
    }

    /// Score Phase 1: DECONSTRUCT
    pub fn score_deconstruct(&self, phase: &DeconstructPhase) -> u8 {
        let mut score = 0.0;

        // Core intent clarity (0-25 points)
        if !phase.core_intent.is_empty() {
            score += 15.0;
            if phase.core_intent.len() > 50 {
                score += 10.0;
            }
        }

        // Key entities completeness (0-20 points)
        let entity_count = phase.key_entities.len();
        score += (entity_count.min(5) as f32 * 4.0);

        // Output requirements (0-20 points)
        let output_count = phase.output_requirements.len();
        score += (output_count.min(5) as f32 * 4.0);

        // Constraints identified (0-15 points)
        let constraint_count = phase.constraints.len();
        score += (constraint_count.min(5) as f32 * 3.0);

        // Context analysis depth (0-20 points)
        if !phase.context_analysis.is_empty() {
            score += 10.0;
            if phase.context_analysis.len() > 100 {
                score += 10.0;
            }
        }

        score.min(100.0) as u8
    }

    /// Score Phase 2: DIAGNOSE
    pub fn score_diagnose(&self, phase: &DiagnosePhase) -> u8 {
        let mut score = 0.0;

        // Clarity gaps identified (0-20 points)
        score += (phase.clarity_gaps.len().min(5) as f32 * 4.0);

        // Ambiguity points addressed (0-20 points)
        score += (phase.ambiguity_points.len().min(5) as f32 * 4.0);

        // Specificity level (0-20 points)
        score += match phase.specificity_level {
            SpecificityLevel::Vague => 5.0,
            SpecificityLevel::Moderate => 10.0,
            SpecificityLevel::Specific => 15.0,
            SpecificityLevel::Precise => 20.0,
        };

        // Completeness score (0-20 points)
        score += phase.completeness_score * 20.0;

        // Structure needs identified (0-10 points)
        score += (phase.structure_needs.len().min(5) as f32 * 2.0);

        // Complexity assessment (0-10 points)
        score += match phase.complexity_assessment {
            ComplexityLevel::Simple => 10.0,
            ComplexityLevel::Moderate => 8.0,
            ComplexityLevel::Complex => 6.0,
            ComplexityLevel::HighlyComplex => 10.0, // Bonus for identifying high complexity
        };

        score.min(100.0) as u8
    }

    /// Score Phase 3: DEVELOP
    pub fn score_develop(&self, phase: &DevelopPhase) -> u8 {
        let mut score = 0.0;

        // Request type classification (0-15 points)
        score += 15.0;

        // Technique selection (0-30 points)
        let technique_count = phase.selected_techniques.len();
        score += (technique_count.min(5) as f32 * 6.0);

        // AI role assignment (0-20 points)
        if !phase.ai_role_assignment.is_empty() {
            score += 10.0;
            if phase.ai_role_assignment.len() > 30 {
                score += 10.0;
            }
        }

        // Context enhancement (0-20 points)
        if !phase.context_enhancement.is_empty() {
            score += 10.0;
            if phase.context_enhancement.len() > 50 {
                score += 10.0;
            }
        }

        // Logical structure (0-15 points)
        if !phase.logical_structure.is_empty() {
            score += 15.0;
        }

        score.min(100.0) as u8
    }

    /// Score Phase 4: DELIVER
    pub fn score_deliver(&self, phase: &DeliverPhase) -> u8 {
        let mut score = 0.0;

        // Execution plan completeness (0-25 points)
        let step_count = phase.execution_plan.len();
        score += (step_count.min(10) as f32 * 2.5);

        // Verification protocol presence (0-25 points)
        score += 25.0; // Has verification protocol

        // Deliverable specifications (0-20 points)
        let deliverable_count = phase.deliverable_specifications.len();
        score += (deliverable_count.min(8) as f32 * 2.5);

        // Target locations defined (0-15 points)
        let location_count = phase.target_locations.len();
        score += (location_count.min(6) as f32 * 2.5);

        // Timeline present (0-15 points)
        score += 15.0; // Has timeline

        score.min(100.0) as u8
    }

    /// Validate phase against quality gate
    pub fn validate_phase_quality_gate(&self, phase_name: &str, score: u8) -> Result<()> {
        let min_score = match phase_name {
            "DECONSTRUCT" => self.quality_gates.min_deconstruct_score,
            "DIAGNOSE" => self.quality_gates.min_diagnose_score,
            "DEVELOP" => self.quality_gates.min_develop_score,
            "DELIVER" => self.quality_gates.min_deliver_score,
            _ => 0,
        };

        if score < min_score {
            anyhow::bail!(
                "Phase {} failed quality gate: score {} < minimum {}",
                phase_name,
                score,
                min_score
            );
        }

        Ok(())
    }

    /// Generate quality report for all phases
    pub fn generate_quality_report(&self, scores: &Scores) -> String {
        format!(
            r#"4D Methodology Quality Report
===========================

Phase 1 - DECONSTRUCT: {}/100 {}
Phase 2 - DIAGNOSE:    {}/100 {}
Phase 3 - DEVELOP:     {}/100 {}
Phase 4 - DELIVER:     {}/100 {}

Overall Score: {:.2}
Quality Gate: {}

Recommendations:
{}
"#,
            scores.deconstruct,
            if scores.deconstruct >= self.quality_gates.min_deconstruct_score {
                "✓"
            } else {
                "✗"
            },
            scores.diagnose,
            if scores.diagnose >= self.quality_gates.min_diagnose_score {
                "✓"
            } else {
                "✗"
            },
            scores.develop,
            if scores.develop >= self.quality_gates.min_develop_score {
                "✓"
            } else {
                "✗"
            },
            scores.deliver,
            if scores.deliver >= self.quality_gates.min_deliver_score {
                "✓"
            } else {
                "✗"
            },
            scores.overall,
            if scores.quality_gate_passed {
                "PASSED ✓"
            } else {
                "FAILED ✗"
            },
            self.generate_recommendations(scores)
        )
    }

    /// Generate recommendations based on scores
    fn generate_recommendations(&self, scores: &Scores) -> String {
        let mut recommendations = Vec::new();

        if scores.deconstruct < 80 {
            recommendations.push("- Enhance core intent clarity and add more key entities");
        }
        if scores.diagnose < 80 {
            recommendations.push("- Identify more clarity gaps and ambiguity points");
        }
        if scores.develop < 80 {
            recommendations.push("- Select additional optimization techniques");
        }
        if scores.deliver < 80 {
            recommendations.push("- Add more execution steps and deliverable specifications");
        }

        if recommendations.is_empty() {
            "All phases meet high quality standards!".to_string()
        } else {
            recommendations.join("\n")
        }
    }
}

impl Default for MethodologyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to score all phases from complete task subject
pub fn score_task_subject(
    deconstruct: &DeconstructPhase,
    diagnose: &DiagnosePhase,
    develop: &DevelopPhase,
    deliver: &DeliverPhase,
) -> Scores {
    let engine = MethodologyEngine::new();
    engine.score_all(deconstruct, diagnose, develop, deliver)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_all() {
        let deconstruct = DeconstructPhase {
            core_intent: "Test intent with sufficient length to score well".to_string(),
            key_entities: vec!["Entity1".to_string(), "Entity2".to_string()],
            context_analysis: "Context analysis with good depth".to_string(),
            output_requirements: vec!["Req1".to_string()],
            constraints: vec!["Constraint1".to_string()],
            provided_vs_missing: HashMap::new(),
        };

        let diagnose = DiagnosePhase {
            clarity_gaps: vec!["Gap1".to_string()],
            ambiguity_points: vec!["Ambiguity1".to_string()],
            specificity_level: SpecificityLevel::Specific,
            completeness_score: 0.9,
            structure_needs: vec!["Need1".to_string()],
            complexity_assessment: ComplexityLevel::Complex,
        };

        let develop = DevelopPhase {
            request_type: RequestType::Technical,
            selected_techniques: vec![OptimizationTechnique::ConstraintBased],
            ai_role_assignment: "Test Role Assignment".to_string(),
            context_enhancement: "Enhanced context".to_string(),
            logical_structure: "Logical structure".to_string(),
        };

        let deliver = DeliverPhase {
            execution_plan: vec![],
            verification_protocol: VerificationProtocol {
                pass_a_self_check: Default::default(),
                pass_b_independent: Default::default(),
                pass_c_adversarial: Default::default(),
                evidence_ledger: Default::default(),
                truth_gate_requirements: Default::default(),
            },
            deliverable_specifications: vec![],
            target_locations: vec![],
            timeline: Default::default(),
        };

        let engine = MethodologyEngine::new();
        let scores = engine.score_all(&deconstruct, &diagnose, &develop, &deliver);

        assert!(scores.deconstruct > 0);
        assert!(scores.diagnose > 0);
        assert!(scores.develop > 0);
        assert!(scores.deliver > 0);
    }
}
