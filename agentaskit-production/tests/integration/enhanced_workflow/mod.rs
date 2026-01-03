//! Integration Tests for Enhanced Workflow Components
//!
//! This module contains integration tests validating the complete workflow:
//! - SOP parsing and validation
//! - 4D Methodology engine processing
//! - Deliverable management
//! - Location resolution
//! - Health monitoring integration

mod sop_parser_tests;
mod methodology_engine_tests;
mod deliverable_manager_tests;
mod location_manager_tests;
mod health_monitoring_tests;
mod end_to_end_tests;

/// Re-export test utilities
pub mod test_utils {
    use std::path::PathBuf;
    use std::time::Duration;

    /// Create a temporary test directory
    pub fn temp_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("agentaskit_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("Failed to create temp dir");
        dir
    }

    /// Clean up temporary test directory
    pub fn cleanup(dir: &PathBuf) {
        let _ = std::fs::remove_dir_all(dir);
    }

    /// Assert that an operation completes within a time limit
    pub fn assert_within_time<F, T>(limit: Duration, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start = std::time::Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        assert!(
            elapsed <= limit,
            "Operation took {:?}, expected <= {:?}",
            elapsed,
            limit
        );
        result
    }
}
