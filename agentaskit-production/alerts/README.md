# Alert Policies

This directory contains Alertmanager and PagerDuty alert configurations.

## Alert Categories

- **latency**: Response time SLO violations
- **error_rate**: Error budget consumption
- **saturation**: Resource utilization alerts
- **availability**: Service health checks

## Configuration Files

- `alerting-rules.yaml`: Prometheus alerting rules
- `alertmanager.yaml`: Alert routing and receivers
- `pagerduty.yaml`: PagerDuty integration (optional)

## SLO-Based Alerts

Alerts are derived from SLO burn rate calculations defined in `../slo/policies.yaml`.
