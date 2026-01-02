# AgentAsKit Service Level Objectives (SLO)

This directory contains the Service Level Objective policies for the AgentAsKit multi-agent orchestration system.

## SLO Policy File

### `policies.yaml`

Defines the baseline SLO targets and burn rate thresholds for the system.

**Service:** agentaskit

### Availability SLO

**Target:** 99.9% (three nines)
- **Monthly downtime budget:** ~43 minutes
- **Error budget:** 0.1% of requests

**Burn Rate Windows:**

| Window | Lookback | Burn Rate | Time to Exhaustion | Severity |
|--------|----------|-----------|-------------------|----------|
| Fast   | 1 hour   | 14.4x     | ~6 hours          | Critical |
| Slow   | 6 hours  | 6.0x      | ~3 days           | Warning  |

The burn rate thresholds are calculated using the multi-window, multi-burn-rate approach from Google's SRE Workbook.

### Latency SLO

**P95 Latency:** ≤50ms
- 95% of requests must complete within 50 milliseconds

**P99 Latency:** ≤100ms
- 99% of requests must complete within 100 milliseconds

### Agent Startup Time SLO

**P95 Startup Time:** ≤100ms
- 95% of agent initializations must complete within 100 milliseconds
- Critical for achieving 10K+ tasks/sec throughput target

## SLO Validation

The SLO policies are validated automatically via GitHub Actions:

```yaml
# Workflow: .github/workflows/slo-check.yml
# Runs: Every 30 minutes + on policy changes
```

**Validation Checks:**
1. YAML syntax validation
2. Required fields presence (service, availability, latency, startup_time)
3. Availability target minimum (≥99.9%)
4. Burn rate window configuration (fast & slow)
5. Burn rate threshold reasonableness
6. Dashboard and alert alignment

## Performance Targets

Based on COMPREHENSIVE-7PHASE-001 task requirements:

- **Task Throughput:** 10,000+ tasks/second
- **Message Throughput:** 100,000+ messages/second
- **Agent Startup:** <100ms (P95)
- **Response Time:** <50ms (P95)
- **System Availability:** 99.99%

Note: The availability SLO is set at 99.9% with the understanding that the system target is 99.99%. This provides a buffer for operational flexibility.

## Error Budget Calculation

**Error Budget = (1 - SLO_target) × Total_requests**

For 99.9% availability:
- Error budget: 0.1% of requests
- Example: 1M requests/day = 1,000 failed requests allowed
- Monthly budget: ~43 minutes of downtime

**Burn Rate Formula:**
```
burn_rate = (current_error_rate / (1 - SLO_target))
```

A burn rate of 1.0 means consuming error budget at exactly the sustainable rate.
- Burn rate > 1.0: Consuming budget faster than sustainable
- Burn rate 14.4: Will exhaust budget in ~6 hours (critical)
- Burn rate 6.0: Will exhaust budget in ~3 days (warning)

## Integration

The SLO policies integrate with:

### Dashboards
- `../dashboards/sla.json` - SLA/SLO overview dashboard
- Burn rate visualization for fast and slow windows
- Error budget tracking and remaining budget

### Alerts
- `../alerts/slo.yaml` - SLO-based alert rules
- Burn rate alerts (fast and slow windows)
- Availability breach alerts
- Latency SLO breach alerts

### CI/CD
- `.github/workflows/slo-check.yml` - Automated validation
- Runs every 30 minutes to validate configuration
- Generates compliance reports

## Monitoring Strategy

### 1. Error Budget-Based Alerting
- Alert when burn rate exceeds thresholds
- Multiple time windows for different severities
- Prevents alert fatigue while catching issues early

### 2. Multi-Window Approach
**Fast Window (1h):**
- Detects severe incidents quickly
- Higher burn rate threshold (14.4x)
- Critical severity

**Slow Window (6h):**
- Detects gradual degradation
- Lower burn rate threshold (6.0x)
- Warning severity

### 3. Latency Monitoring
- Track P95 and P99 independently
- Alert on SLO breaches
- Correlate with availability issues

## Updating SLO Policies

To modify SLO targets:

1. Update `policies.yaml` with new targets
2. Recalculate burn rate thresholds if needed
3. Update corresponding alert rules in `../alerts/slo.yaml`
4. Update dashboard thresholds in `../dashboards/sla.json`
5. Run CI validation: `.github/workflows/slo-check.yml`
6. Document changes and rationale

**Burn Rate Calculation Tool:**
```python
# For a given SLO and time window
slo_target = 0.999  # 99.9%
error_budget = 1 - slo_target  # 0.001
window_hours = 1  # or 6 for slow window
budget_hours = 30 * 24  # 30 days in hours

# Time to exhaust budget at this rate
exhaustion_time = budget_hours / burn_rate

# Solve for burn rate given desired exhaustion time
desired_exhaustion_hours = 6  # Want to know in 6 hours
burn_rate = budget_hours / desired_exhaustion_hours
# Result: 14.4 for 1h window alerting on 6h exhaustion
```

## References

- [Google SRE Book - Service Level Objectives](https://sre.google/sre-book/service-level-objectives/)
- [Google SRE Workbook - Alerting on SLOs](https://sre.google/workbook/alerting-on-slos/)
- [Implementing SLOs](https://sre.google/workbook/implementing-slos/)
- [Alert Rules](../alerts/)
- [Dashboards](../dashboards/)
- [CI Validation](.github/workflows/slo-check.yml)

## Appendix: SLO Targets by System Component

| Component | Availability | P95 Latency | P99 Latency | Throughput |
|-----------|--------------|-------------|-------------|------------|
| Task Orchestrator | 99.9% | 50ms | 100ms | 10K+ tasks/s |
| Message Bus | 99.9% | 10ms | 20ms | 100K+ msgs/s |
| Agent Pool | 99.9% | 100ms startup | 150ms startup | 928 agents |
| API Gateway | 99.9% | 50ms | 100ms | - |
| Sandbox Pool | 99.9% | - | - | Tri-sandbox |

These component-level SLOs aggregate to the system-wide targets defined in `policies.yaml`.
