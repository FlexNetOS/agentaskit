//! Performance Test Suite - TEST-002
//!
//! Comprehensive performance tests including:
//! - Message broker throughput (target: 100k+ msgs/sec)
//! - Task execution latency (target: <50ms p95)
//! - Agent startup time (target: <100ms)
//! - Memory profiling and cleanup verification

pub mod benchmarks;
