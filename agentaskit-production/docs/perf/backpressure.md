# Backpressure & Queueing Controls

**REF:** PERF-BACKPRESSURE
**Owner:** @perf-oncall
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the backpressure and queueing control policies for AgentAskit production, ensuring system stability under load.

## Backpressure Thresholds

### Activation Triggers

| Metric | Threshold | Action |
|--------|-----------|--------|
| CPU Utilization | 70% | Begin throttling new requests |
| Memory Utilization | 75% | Pause non-critical tasks |
| Queue Depth | 5000 items | Enable load shedding |
| Response Latency p95 | 100ms | Reduce concurrency |
| Error Rate | 1% | Circuit breaker activation |

### Saturation Levels

```
Level 0 (Normal):    < 50% saturation  → Full throughput
Level 1 (Caution):   50-70% saturation → Monitor closely
Level 2 (Warning):   70-85% saturation → Backpressure active
Level 3 (Critical):  85-95% saturation → Load shedding
Level 4 (Emergency): > 95% saturation  → Controlled drop
```

## Queue Management

### Queue Configuration

```yaml
# configs/queues.yaml
queues:
  high_priority:
    max_depth: 1000
    timeout_seconds: 30
    drop_policy: never

  normal_priority:
    max_depth: 5000
    timeout_seconds: 60
    drop_policy: oldest_first

  low_priority:
    max_depth: 10000
    timeout_seconds: 120
    drop_policy: newest_first
```

### Priority-Based Processing

1. **High Priority:** Critical workflows, system tasks
2. **Normal Priority:** Standard user requests
3. **Low Priority:** Background jobs, analytics

## Controlled Drop Policy

### Drop Criteria

When queues exceed capacity, the following drop policy applies:

1. Drop oldest low-priority items first
2. Reject new low-priority requests
3. If still overloaded, drop oldest normal-priority
4. Never drop high-priority without explicit failure

### Drop Logging

All dropped requests are logged with:
- Request ID
- Priority level
- Queue depth at drop time
- Reason for drop
- Suggested retry-after

## Alerting Configuration

### Queue Depth Alarms

```yaml
# alerts/backpressure.yaml
alerts:
  - name: queue_depth_warning
    condition: queue_depth > 3000
    severity: warning
    action: notify

  - name: queue_depth_critical
    condition: queue_depth > 7000
    severity: critical
    action: page

  - name: backpressure_activated
    condition: backpressure_level >= 2
    severity: warning
    action: notify

  - name: load_shedding_active
    condition: load_shedding_enabled == true
    severity: critical
    action: page
```

## Implementation

### Rust Implementation

```rust
// core/src/backpressure/controller.rs
pub struct BackpressureController {
    cpu_threshold: f64,      // 0.70
    memory_threshold: f64,   // 0.75
    queue_threshold: usize,  // 5000
}

impl BackpressureController {
    pub fn should_apply_backpressure(&self) -> bool {
        let metrics = self.collect_metrics();
        metrics.cpu_utilization > self.cpu_threshold
            || metrics.memory_utilization > self.memory_threshold
            || metrics.queue_depth > self.queue_threshold
    }
}
```

## Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| `backpressure_level` | Current backpressure level (0-4) | < 2 |
| `queue_depth` | Current queue depth | < 5000 |
| `requests_shed` | Requests dropped due to load | < 0.1% |
| `recovery_time_seconds` | Time to return to normal | < 60s |

## Evidence

- Configuration: `configs/queues.yaml`
- Alerts: `alerts/backpressure.yaml`
- Tests: `tests/performance/queueing/`
- Logs: `TEST/perf/backpressure/*.log`

## Related

- [PERF-001](../../.todo) - Performance optimization system
- [PERF-RATE](../../configs/rate_limits.yaml) - Rate limiting
- [PERF-QUOTAS](../../deploy/k8s/limits.yaml) - Resource quotas
