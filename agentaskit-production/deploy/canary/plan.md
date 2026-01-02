# Canary Release Plan

**REF:** DEP-CANARY
**Owner:** @platform
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the canary deployment strategy, metrics gates, and automated rollback procedures for AgentAskit production.

## Canary Configuration

### Traffic Split

| Phase | Canary % | Duration | Criteria to Advance |
|-------|----------|----------|---------------------|
| Initial | 5% | 5 min | No errors |
| Ramp 1 | 10% | 10 min | Metrics within threshold |
| Ramp 2 | 25% | 15 min | Metrics within threshold |
| Ramp 3 | 50% | 20 min | Metrics within threshold |
| Full | 100% | - | Stable |

### Kubernetes Configuration

```yaml
# deploy/canary/canary.yaml
apiVersion: flagger.app/v1beta1
kind: Canary
metadata:
  name: api-gateway
  namespace: agentaskit-production
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-gateway
  progressDeadlineSeconds: 3600
  service:
    port: 80
    targetPort: 8080
  analysis:
    interval: 1m
    threshold: 5
    maxWeight: 50
    stepWeight: 10
    metrics:
      - name: request-success-rate
        thresholdRange:
          min: 99
        interval: 1m
      - name: request-duration
        thresholdRange:
          max: 500
        interval: 1m
    webhooks:
      - name: load-test
        url: http://flagger-loadtester/
        timeout: 5s
        metadata:
          cmd: "hey -z 1m -q 10 -c 2 http://api-gateway-canary/"
```

## Metrics Gates

### Success Criteria

| Metric | Threshold | Measurement |
|--------|-----------|-------------|
| Success Rate | ≥99% | 1 min rolling |
| P50 Latency | <100ms | 1 min rolling |
| P99 Latency | <500ms | 1 min rolling |
| Error Rate | <1% | 1 min rolling |
| 5xx Errors | <0.1% | 1 min rolling |

### Abort Conditions

Automatically abort and rollback when:

```yaml
# Abort thresholds
abort:
  success_rate_min: 95    # Abort if <95%
  error_rate_max: 5       # Abort if >5%
  latency_p99_max: 2000   # Abort if >2s
  consecutive_failures: 3  # Abort after 3 failed checks
```

## Monitoring Integration

### Prometheus Queries

```yaml
# Canary success rate
sum(rate(http_requests_total{status!~"5.."}[1m]))
/
sum(rate(http_requests_total[1m]))

# Canary latency
histogram_quantile(0.99,
  sum(rate(http_request_duration_seconds_bucket[1m])) by (le)
)
```

### Grafana Dashboard

Dashboard URL: `https://grafana.agentaskit.io/d/canary`

Panels:
- Traffic split visualization
- Error rate comparison
- Latency comparison
- Rollout progress

## Promotion Script

```bash
#!/bin/bash
# scripts/canary/promote.sh

VERSION=$1

echo "Promoting canary to production: $VERSION"

# Verify canary is healthy
CANARY_STATUS=$(kubectl get canary api-gateway -n agentaskit-production -o jsonpath='{.status.phase}')
if [ "$CANARY_STATUS" != "Succeeded" ]; then
    echo "ERROR: Canary not in succeeded state: $CANARY_STATUS"
    exit 1
fi

# Update production deployment
kubectl set image deployment/api-gateway \
    api-gateway=agentaskit/api-gateway:$VERSION \
    -n agentaskit-production

# Wait for rollout
kubectl rollout status deployment/api-gateway -n agentaskit-production

echo "Promotion complete"
```

## Rollback Script

```bash
#!/bin/bash
# scripts/canary/rollback.sh

echo "Rolling back canary deployment..."

# Trigger immediate rollback
kubectl patch canary api-gateway -n agentaskit-production \
    --type='json' -p='[{"op": "replace", "path": "/spec/suspend", "value": true}]'

# Wait for rollback
sleep 10

# Verify rollback
CANARY_WEIGHT=$(kubectl get canary api-gateway -n agentaskit-production \
    -o jsonpath='{.status.canaryWeight}')

if [ "$CANARY_WEIGHT" = "0" ]; then
    echo "Rollback complete"
else
    echo "WARNING: Canary weight still at $CANARY_WEIGHT%"
fi

# Re-enable canary
kubectl patch canary api-gateway -n agentaskit-production \
    --type='json' -p='[{"op": "replace", "path": "/spec/suspend", "value": false}]'
```

## Runbook Integration

### Pre-Canary Checklist

- [ ] Staging tests passed
- [ ] Security scan clean
- [ ] Change record created
- [ ] On-call notified
- [ ] Rollback tested recently

### During Canary

- [ ] Monitor dashboard
- [ ] Watch alerts channel
- [ ] Check error logs
- [ ] Verify user reports

### Post-Canary

- [ ] Confirm full rollout
- [ ] Update SoT
- [ ] Close change record
- [ ] Update documentation

## Evidence

- Canary configs: `deploy/canary/`
- Rollout logs: `TEST/canary/*.log`
- Scripts: `scripts/canary/`
- Metrics: `dashboards/canary/*.json`

## Related

- [OPS-RUNBOOK](../../docs/runbooks/) - Runbooks
- [RELEASE-PROMOTE](../../docs/release/promotion.md) - Promotion process
- [CD-GATES](../../.github/workflows/release.yml) - Release gates
