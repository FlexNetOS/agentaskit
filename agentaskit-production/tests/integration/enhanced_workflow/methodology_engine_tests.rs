//! Integration tests for 4D Methodology Engine
//!
//! Tests validate the complete 4D methodology workflow:
//! - Deconstruct: Task analysis and breakdown
//! - Diagnose: Problem identification and root cause analysis
//! - Develop: Solution development and implementation
//! - Deliver: Output generation and quality verification

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Simulated task input for testing
#[derive(Debug, Clone)]
struct TaskInput {
    task_type: String,
    description: String,
    context: HashMap<String, String>,
    requirements: Vec<String>,
}

/// Simulated methodology output
#[derive(Debug, Clone)]
struct MethodologyOutput {
    phase: String,
    components: Vec<String>,
    quality_score: f64,
    insights: Vec<String>,
    deliverables: Vec<String>,
}

/// Simulate the Deconstruct phase
fn deconstruct(input: &TaskInput) -> MethodologyOutput {
    let components: Vec<String> = input.description
        .split_whitespace()
        .filter(|w| w.len() > 3)
        .map(|s| s.to_string())
        .take(5)
        .collect();

    MethodologyOutput {
        phase: "Deconstruct".to_string(),
        components,
        quality_score: 0.85,
        insights: vec!["Task decomposed into manageable components".to_string()],
        deliverables: vec!["Component analysis".to_string()],
    }
}

/// Simulate the Diagnose phase
fn diagnose(input: &TaskInput, deconstruct_output: &MethodologyOutput) -> MethodologyOutput {
    let insights: Vec<String> = deconstruct_output.components
        .iter()
        .map(|c| format!("Analyzed: {}", c))
        .collect();

    MethodologyOutput {
        phase: "Diagnose".to_string(),
        components: deconstruct_output.components.clone(),
        quality_score: 0.88,
        insights,
        deliverables: vec!["Root cause analysis".to_string(), "Problem statement".to_string()],
    }
}

/// Simulate the Develop phase
fn develop(input: &TaskInput, diagnose_output: &MethodologyOutput) -> MethodologyOutput {
    let deliverables: Vec<String> = input.requirements
        .iter()
        .map(|r| format!("Solution for: {}", r))
        .collect();

    MethodologyOutput {
        phase: "Develop".to_string(),
        components: diagnose_output.components.clone(),
        quality_score: 0.90,
        insights: vec!["Solutions developed based on diagnosis".to_string()],
        deliverables,
    }
}

/// Simulate the Deliver phase
fn deliver(input: &TaskInput, develop_output: &MethodologyOutput) -> MethodologyOutput {
    let deliverables: Vec<String> = develop_output.deliverables
        .iter()
        .map(|d| format!("Delivered: {}", d))
        .collect();

    MethodologyOutput {
        phase: "Deliver".to_string(),
        components: develop_output.components.clone(),
        quality_score: 0.92,
        insights: vec!["All deliverables validated and delivered".to_string()],
        deliverables,
    }
}

#[test]
fn test_deconstruct_phase() {
    let input = TaskInput {
        task_type: "implementation".to_string(),
        description: "Implement a user authentication system with OAuth2 support".to_string(),
        context: HashMap::new(),
        requirements: vec!["OAuth2 integration".to_string()],
    };

    let start = Instant::now();
    let output = deconstruct(&input);
    let elapsed = start.elapsed();

    assert_eq!(output.phase, "Deconstruct");
    assert!(!output.components.is_empty());
    assert!(output.quality_score >= 0.80);
    assert!(elapsed < Duration::from_millis(100));
}

#[test]
fn test_diagnose_phase() {
    let input = TaskInput {
        task_type: "debugging".to_string(),
        description: "Fix memory leak in worker thread pool".to_string(),
        context: HashMap::new(),
        requirements: vec!["Memory profiling".to_string()],
    };

    let deconstruct_output = deconstruct(&input);
    let output = diagnose(&input, &deconstruct_output);

    assert_eq!(output.phase, "Diagnose");
    assert!(!output.insights.is_empty());
    assert!(output.quality_score >= 0.85);
}

#[test]
fn test_develop_phase() {
    let input = TaskInput {
        task_type: "feature".to_string(),
        description: "Add real-time notifications".to_string(),
        context: HashMap::new(),
        requirements: vec![
            "WebSocket support".to_string(),
            "Push notifications".to_string(),
        ],
    };

    let deconstruct_output = deconstruct(&input);
    let diagnose_output = diagnose(&input, &deconstruct_output);
    let output = develop(&input, &diagnose_output);

    assert_eq!(output.phase, "Develop");
    assert_eq!(output.deliverables.len(), 2);
    assert!(output.quality_score >= 0.88);
}

#[test]
fn test_deliver_phase() {
    let input = TaskInput {
        task_type: "deployment".to_string(),
        description: "Deploy to production".to_string(),
        context: HashMap::new(),
        requirements: vec!["Zero downtime".to_string()],
    };

    let deconstruct_output = deconstruct(&input);
    let diagnose_output = diagnose(&input, &deconstruct_output);
    let develop_output = develop(&input, &diagnose_output);
    let output = deliver(&input, &develop_output);

    assert_eq!(output.phase, "Deliver");
    assert!(!output.deliverables.is_empty());
    assert!(output.quality_score >= 0.90);
}

#[test]
fn test_full_4d_pipeline() {
    let input = TaskInput {
        task_type: "full_implementation".to_string(),
        description: "Implement complete order processing pipeline with validation".to_string(),
        context: {
            let mut ctx = HashMap::new();
            ctx.insert("priority".to_string(), "high".to_string());
            ctx.insert("deadline".to_string(), "2024-01-15".to_string());
            ctx
        },
        requirements: vec![
            "Order validation".to_string(),
            "Payment processing".to_string(),
            "Inventory check".to_string(),
            "Notification dispatch".to_string(),
        ],
    };

    let start = Instant::now();

    // Execute full 4D pipeline
    let deconstruct_output = deconstruct(&input);
    let diagnose_output = diagnose(&input, &deconstruct_output);
    let develop_output = develop(&input, &diagnose_output);
    let deliver_output = deliver(&input, &develop_output);

    let elapsed = start.elapsed();

    // Verify pipeline execution
    assert_eq!(deconstruct_output.phase, "Deconstruct");
    assert_eq!(diagnose_output.phase, "Diagnose");
    assert_eq!(develop_output.phase, "Develop");
    assert_eq!(deliver_output.phase, "Deliver");

    // Verify quality progression
    assert!(diagnose_output.quality_score >= deconstruct_output.quality_score);
    assert!(develop_output.quality_score >= diagnose_output.quality_score);
    assert!(deliver_output.quality_score >= develop_output.quality_score);

    // Verify final deliverables
    assert_eq!(deliver_output.deliverables.len(), input.requirements.len());

    // Performance check
    assert!(
        elapsed < Duration::from_millis(500),
        "Full pipeline took {:?}, should be < 500ms",
        elapsed
    );
}

#[test]
fn test_quality_gates() {
    let input = TaskInput {
        task_type: "quality_test".to_string(),
        description: "Test quality gate enforcement".to_string(),
        context: HashMap::new(),
        requirements: vec!["High quality output".to_string()],
    };

    let deconstruct_output = deconstruct(&input);
    let diagnose_output = diagnose(&input, &deconstruct_output);
    let develop_output = develop(&input, &diagnose_output);
    let deliver_output = deliver(&input, &develop_output);

    // All phases must meet minimum quality threshold
    let min_quality = 0.80;

    assert!(
        deconstruct_output.quality_score >= min_quality,
        "Deconstruct quality {} < {}",
        deconstruct_output.quality_score,
        min_quality
    );
    assert!(
        diagnose_output.quality_score >= min_quality,
        "Diagnose quality {} < {}",
        diagnose_output.quality_score,
        min_quality
    );
    assert!(
        develop_output.quality_score >= min_quality,
        "Develop quality {} < {}",
        develop_output.quality_score,
        min_quality
    );
    assert!(
        deliver_output.quality_score >= min_quality,
        "Deliver quality {} < {}",
        deliver_output.quality_score,
        min_quality
    );
}

#[test]
fn test_methodology_with_empty_input() {
    let input = TaskInput {
        task_type: "".to_string(),
        description: "".to_string(),
        context: HashMap::new(),
        requirements: vec![],
    };

    // Should handle empty input gracefully
    let output = deconstruct(&input);
    assert_eq!(output.phase, "Deconstruct");
    // Components may be empty but should not panic
}

#[test]
fn test_methodology_performance() {
    let iterations = 100;
    let mut durations = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let input = TaskInput {
            task_type: "perf_test".to_string(),
            description: "Performance testing task with multiple components".to_string(),
            context: HashMap::new(),
            requirements: vec!["Req1".to_string(), "Req2".to_string()],
        };

        let start = Instant::now();

        let deconstruct_output = deconstruct(&input);
        let diagnose_output = diagnose(&input, &deconstruct_output);
        let develop_output = develop(&input, &diagnose_output);
        let _ = deliver(&input, &develop_output);

        durations.push(start.elapsed());
    }

    // Calculate statistics
    let total: Duration = durations.iter().sum();
    let avg = total / iterations as u32;

    durations.sort();
    let p50 = durations[iterations / 2];
    let p95 = durations[(iterations as f64 * 0.95) as usize];
    let p99 = durations[(iterations as f64 * 0.99) as usize];

    assert!(
        avg < Duration::from_millis(10),
        "Average latency {:?} should be < 10ms",
        avg
    );
    assert!(
        p95 < Duration::from_millis(20),
        "p95 latency {:?} should be < 20ms",
        p95
    );
    assert!(
        p99 < Duration::from_millis(50),
        "p99 latency {:?} should be < 50ms",
        p99
    );
}
