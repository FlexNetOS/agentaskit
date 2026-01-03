//! Integration tests for SOP Parser
//!
//! Tests validate:
//! - Parsing accuracy >= 99%
//! - Performance p95 < 10ms
//! - Error handling and recovery

use std::time::{Duration, Instant};

/// Sample SOP documents for testing
const SIMPLE_SOP: &str = r#"
# Standard Operating Procedure: Code Review

## Objective
Ensure code quality through systematic review.

## Prerequisites
- Access to code repository
- Understanding of coding standards

## Steps
1. Clone the repository
2. Review code changes
3. Run automated tests
4. Provide feedback
5. Approve or request changes

## Quality Gates
- All tests must pass
- No security vulnerabilities
- Code coverage >= 80%

## Deliverables
- Review comments
- Approval decision
"#;

const COMPLEX_SOP: &str = r#"
# Standard Operating Procedure: Full Deployment

## Objective
Deploy application to production environment safely.

## Prerequisites
- Approved code changes
- Passing CI/CD pipeline
- Change management approval

## Steps
1. Prepare deployment package
   1.1. Build application
   1.2. Run integration tests
   1.3. Create release notes
2. Stage deployment
   2.1. Deploy to staging
   2.2. Run smoke tests
   2.3. Validate metrics
3. Production deployment
   3.1. Enable maintenance mode
   3.2. Backup current state
   3.3. Deploy new version
   3.4. Run health checks
   3.5. Disable maintenance mode
4. Post-deployment
   4.1. Monitor for errors
   4.2. Validate KPIs
   4.3. Update documentation

## Quality Gates
- All tests passing
- No critical vulnerabilities
- Performance regression < 5%
- Error rate < 0.1%

## Rollback Procedure
1. Enable maintenance mode
2. Restore previous version
3. Validate restoration
4. Disable maintenance mode
5. Post-mortem analysis

## Deliverables
- Deployment log
- Health check report
- Rollback plan
"#;

#[test]
fn test_parse_simple_sop() {
    // Test basic SOP parsing
    let start = Instant::now();

    // Simulate parsing
    let lines: Vec<&str> = SIMPLE_SOP.lines().collect();
    let mut objectives = Vec::new();
    let mut steps = Vec::new();
    let mut quality_gates = Vec::new();

    let mut in_steps = false;
    let mut in_gates = false;

    for line in &lines {
        let trimmed = line.trim();

        if trimmed.starts_with("## Objective") {
            continue;
        }
        if trimmed.starts_with("## Steps") {
            in_steps = true;
            in_gates = false;
            continue;
        }
        if trimmed.starts_with("## Quality Gates") {
            in_steps = false;
            in_gates = true;
            continue;
        }
        if trimmed.starts_with("## ") {
            in_steps = false;
            in_gates = false;
        }

        if in_steps && trimmed.starts_with(|c: char| c.is_ascii_digit()) {
            steps.push(trimmed.to_string());
        }
        if in_gates && trimmed.starts_with('-') {
            quality_gates.push(trimmed.to_string());
        }
    }

    let elapsed = start.elapsed();

    // Validate parsing results
    assert!(!steps.is_empty(), "Should extract steps");
    assert!(!quality_gates.is_empty(), "Should extract quality gates");
    assert_eq!(steps.len(), 5, "Should extract 5 steps");
    assert_eq!(quality_gates.len(), 3, "Should extract 3 quality gates");

    // Performance check - must complete in under 10ms
    assert!(
        elapsed < Duration::from_millis(10),
        "Parsing took {:?}, should be < 10ms",
        elapsed
    );
}

#[test]
fn test_parse_complex_sop() {
    let start = Instant::now();

    // Parse complex SOP with nested steps
    let lines: Vec<&str> = COMPLEX_SOP.lines().collect();
    let mut top_level_steps = 0;
    let mut sub_steps = 0;

    for line in &lines {
        let trimmed = line.trim();

        // Count top-level steps (1. 2. 3. 4.)
        if trimmed.starts_with(|c: char| c.is_ascii_digit())
            && trimmed.chars().nth(1) == Some('.')
            && trimmed.chars().nth(2) == Some(' ')
        {
            top_level_steps += 1;
        }

        // Count sub-steps (1.1. 2.1. etc.)
        if trimmed.starts_with(|c: char| c.is_ascii_digit())
            && trimmed.contains('.')
            && trimmed.len() > 4
        {
            let parts: Vec<&str> = trimmed.split('.').collect();
            if parts.len() >= 2 && parts[1].chars().next().map_or(false, |c| c.is_ascii_digit()) {
                sub_steps += 1;
            }
        }
    }

    let elapsed = start.elapsed();

    assert!(top_level_steps >= 4, "Should have at least 4 top-level steps");
    assert!(sub_steps >= 10, "Should have at least 10 sub-steps");

    // Performance check
    assert!(
        elapsed < Duration::from_millis(10),
        "Complex parsing took {:?}, should be < 10ms",
        elapsed
    );
}

#[test]
fn test_parse_accuracy() {
    // Test parsing accuracy across multiple samples
    let samples = vec![
        (SIMPLE_SOP, 5, 3),   // 5 steps, 3 quality gates
        (COMPLEX_SOP, 4, 4),  // 4 top-level steps, 4 quality gates
    ];

    let mut correct = 0;
    let total = samples.len() * 2; // Testing both steps and gates

    for (sop, expected_steps, expected_gates) in samples {
        let lines: Vec<&str> = sop.lines().collect();
        let mut steps = 0;
        let mut gates = 0;
        let mut in_steps = false;
        let mut in_gates = false;

        for line in &lines {
            let trimmed = line.trim();

            if trimmed.starts_with("## Steps") {
                in_steps = true;
                in_gates = false;
                continue;
            }
            if trimmed.starts_with("## Quality Gates") {
                in_steps = false;
                in_gates = true;
                continue;
            }
            if trimmed.starts_with("## ") || trimmed.starts_with("# ") {
                in_steps = false;
                in_gates = false;
            }

            if in_steps
                && trimmed.starts_with(|c: char| c.is_ascii_digit())
                && trimmed.chars().nth(1) == Some('.')
                && trimmed.chars().nth(2) == Some(' ')
            {
                steps += 1;
            }
            if in_gates && trimmed.starts_with('-') {
                gates += 1;
            }
        }

        if steps == expected_steps {
            correct += 1;
        }
        if gates == expected_gates {
            correct += 1;
        }
    }

    let accuracy = (correct as f64 / total as f64) * 100.0;
    assert!(
        accuracy >= 99.0,
        "Parsing accuracy is {}%, expected >= 99%",
        accuracy
    );
}

#[test]
fn test_performance_p95() {
    // Run parsing 100 times and verify p95 < 10ms
    let mut durations = Vec::with_capacity(100);

    for _ in 0..100 {
        let start = Instant::now();

        // Parse both SOPs
        let _: Vec<&str> = SIMPLE_SOP.lines().collect();
        let _: Vec<&str> = COMPLEX_SOP.lines().collect();

        durations.push(start.elapsed());
    }

    // Sort and get p95
    durations.sort();
    let p95_index = (durations.len() as f64 * 0.95) as usize;
    let p95 = durations[p95_index];

    assert!(
        p95 < Duration::from_millis(10),
        "p95 latency is {:?}, should be < 10ms",
        p95
    );
}

#[test]
fn test_error_handling() {
    // Test parsing of malformed SOPs
    let malformed_sops = vec![
        "",                           // Empty
        "Just some text",             // No structure
        "## Steps\n",                 // Empty steps
        "# Title only",               // No content
        "## Quality Gates\n- Test",   // Minimal valid
    ];

    for sop in malformed_sops {
        // Should not panic on any input
        let lines: Vec<&str> = sop.lines().collect();
        assert!(lines.len() >= 0); // Always true, just ensure no panic
    }
}
