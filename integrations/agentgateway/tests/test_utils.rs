//! Test utilities for agentgateway integration tests
//! 
//! This module provides common utilities for testing, including
//! dynamic timestamp generation to avoid hardcoded test dates.

use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a dynamic ISO8601 timestamp for test use
/// 
/// Returns a timestamp in the format: "YYYY-MM-DDTHH:MM:SSZ"
/// This ensures tests don't rely on hardcoded dates that could become outdated.
pub fn generate_test_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch");
    
    // Format as ISO8601 with seconds precision
    let secs = now.as_secs();
    let datetime = format_unix_timestamp(secs);
    datetime
}

/// Generate a test timestamp with a specific offset from now
/// 
/// # Arguments
/// * `offset_secs` - Number of seconds to offset from current time (can be negative)
/// 
/// # Returns
/// ISO8601 formatted timestamp string
pub fn generate_test_timestamp_offset(offset_secs: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch");
    
    let secs = (now.as_secs() as i64 + offset_secs) as u64;
    format_unix_timestamp(secs)
}

/// Format a Unix timestamp as ISO8601
fn format_unix_timestamp(secs: u64) -> String {
    // Simple conversion to ISO8601 format
    const SECS_PER_DAY: u64 = 86400;
    const SECS_PER_HOUR: u64 = 3600;
    const SECS_PER_MINUTE: u64 = 60;
    
    // Days since epoch (1970-01-01)
    let days = secs / SECS_PER_DAY;
    let remaining = secs % SECS_PER_DAY;
    
    let hours = remaining / SECS_PER_HOUR;
    let remaining = remaining % SECS_PER_HOUR;
    
    let minutes = remaining / SECS_PER_MINUTE;
    let seconds = remaining % SECS_PER_MINUTE;
    
    // Simplified date calculation (good enough for tests)
    let year = 1970 + (days / 365);
    let day_of_year = days % 365;
    let month = 1 + (day_of_year / 30).min(11);
    let day = 1 + (day_of_year % 30);
    
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", 
            year, month, day, hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_timestamp() {
        let ts = generate_test_timestamp();
        // Verify format (basic check)
        assert!(ts.contains('T'));
        assert!(ts.ends_with('Z'));
        assert!(ts.len() >= 19); // YYYY-MM-DDTHH:MM:SSZ
    }

    #[test]
    fn test_generate_timestamp_offset() {
        let ts1 = generate_test_timestamp_offset(0);
        let ts2 = generate_test_timestamp_offset(1);
        
        // Second timestamp should be different (1 second later)
        assert_ne!(ts1, ts2);
    }
}
