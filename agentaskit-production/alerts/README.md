# AgentAsKit Alert Rules

This directory contains Prometheus/Alertmanager alert rule definitions for the AgentAsKit multi-agent orchestration system.

## Alert Rule Files

### SLO Alerts (`slo.yaml`)
Alerts based on Service Level Objectives:

**Availability Alerts:**
- `AvailabilitySLOBreach` - Availability drops below 99.9% (critical)
- `ErrorBudgetExhausted` - Error budget below 10% (warning)

**Burn Rate Alerts:**
- `FastBurnRateHigh` - 1h burn rate > 14.4 (critical, budget exhausted in <6h)
- `SlowBurnRateHigh` - 6h burn rate > 6.0 (warning, budget exhausted in <3d)

**Latency Alerts:**
- `P95LatencySLOBreach` - P95 latency exceeds 50ms (warning)
- `P99LatencySLOBreach` - P99 latency exceeds 100ms (warning)
- `AgentStartupTimeSLOBreach` - P95 startup time exceeds 100ms (warning)

### Performance Alerts (`performance.yaml`)
Alerts for system performance metrics:

**Throughput Alerts:**
- `TaskThroughputBelowTarget` - <10K tasks/sec (warning)
- `MessageThroughputBelowTarget` - <100K messages/sec (warning)
- `HighTaskProcessingTime` - P95 task duration >1s (warning)

**Agent Alerts:**
- `AgentStartupSlow` - P95 startup >100ms (warning)
- `HighAgentChurnRate` - >100 agents/sec starting (warning)
- `AgentCapacityNearLimit` - >90% of 928-agent capacity (warning)

**Resource Alerts:**
- `HighMemoryUsage` - >10GB total memory (warning)
- `HighCPUUtilization` - >80% CPU for 10min (warning)

### Backpressure Alerts (`backpressure.yaml`)
Alerts for queue depth and backpressure:

**Queue Alerts:**
- `HighTaskQueueDepth` - Queue >1000 for 5min (warning)
- `CriticalTaskQueueDepth` - Queue >5000 for 2min (critical)
- `MessageQueueBackpressure` - Dropping >100 messages/sec (warning)

**Sandbox Alerts:**
- `SandboxPoolExhaustion` - <10% sandbox pool available (warning)

## Severity Levels

- **Critical**: Immediate action required, SLO breach imminent or occurring
- **Warning**: Investigation needed, degraded performance or approaching limits

## Alert Routing

Alerts should be routed based on component and severity:

```yaml
route:
  receiver: 'default'
  group_by: ['alertname', 'component']
  routes:
    - match:
        severity: critical
      receiver: 'pagerduty-critical'
    - match:
        severity: warning
      receiver: 'slack-warnings'
    - match:
        slo: burn_rate
      receiver: 'sre-oncall'
```

## Integration

These alert rules integrate with:
- **Prometheus**: Rule evaluation engine
- **Alertmanager**: Alert routing and grouping
- **PagerDuty/Slack**: Notification channels
- **Grafana**: Dashboard visualization
- **SLO Policies**: See `../slo/policies.yaml` for targets

## Testing Alerts

To test alert rules locally:

```bash
# Validate YAML syntax
yamllint *.yaml

# Test with promtool
promtool check rules *.yaml

# Validate in CI
.github/workflows/slo-check.yml
```

## Burn Rate Calculation

The burn rate alerts use the multi-window, multi-burn-rate approach:

- **Fast Window (1h)**: Detects severe incidents quickly
  - Burn rate threshold: 14.4x
  - Time to exhaustion: ~6 hours

- **Slow Window (6h)**: Detects gradual degradation
  - Burn rate threshold: 6.0x
  - Time to exhaustion: ~3 days

Formula: `burn_rate = (error_rate / (1 - SLO_target))`

## References

- [SLO Policies](../slo/policies.yaml)
- [Dashboards](../dashboards/)
- [CI SLO Check](../../.github/workflows/slo-check.yml)
- [Google SRE Book - Alerting on SLOs](https://sre.google/workbook/alerting-on-slos/)
