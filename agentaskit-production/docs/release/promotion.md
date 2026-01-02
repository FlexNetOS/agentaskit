# Release Promotion Process

**REF:** RELEASE-PROMOTE
**Owner:** @platform
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the release promotion process from development through staging to production, including rollback procedures.

## Promotion Pipeline

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    Dev      │────▶│   Staging   │────▶│   Canary    │────▶│ Production  │
│             │     │             │     │   (10%)     │     │   (100%)    │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
      │                   │                   │                   │
      │ CI Tests          │ E2E Tests         │ Metrics           │ Full rollout
      │ Unit Tests        │ Load Tests        │ Error rate        │
      │ Security Scan     │ Security Scan     │ Latency           │
      └───────────────────┴───────────────────┴───────────────────┘
```

## Promotion Stages

### Stage 1: Dev → Staging

**Trigger:** Merge to main branch

**Requirements:**
- [ ] All CI checks pass
- [ ] Unit test coverage ≥80%
- [ ] No critical/high security findings
- [ ] Code review approved

**Automation:**

```yaml
# .github/workflows/promote-staging.yml
on:
  push:
    branches: [main]

jobs:
  promote:
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to staging
        run: ./scripts/deploy.sh staging ${{ github.sha }}
```

### Stage 2: Staging → Canary

**Trigger:** Manual approval or scheduled

**Requirements:**
- [ ] E2E tests pass
- [ ] Load tests pass
- [ ] Staging stable for ≥2 hours
- [ ] Security scan clean

**Approval Process:**

```bash
# Request canary promotion
./scripts/release/request-canary.sh v1.2.3

# Approve (requires RELEASE_APPROVER role)
./scripts/release/approve-canary.sh v1.2.3 --approver=@username
```

### Stage 3: Canary → Production

**Trigger:** Automatic after canary success

**Requirements:**
- [ ] Canary metrics within thresholds
- [ ] No increase in error rate
- [ ] No latency regression
- [ ] Minimum canary duration: 30 minutes

**Metrics Gates:**

| Metric | Threshold | Action if Breached |
|--------|-----------|-------------------|
| Error Rate | <1% increase | Auto-rollback |
| P99 Latency | <10% increase | Alert + pause |
| Availability | ≥99.9% | Auto-rollback |

## Rollback Procedures

### Automatic Rollback

Rollback triggers automatically when:
- Error rate exceeds threshold
- Health checks fail
- Canary metrics breach limits

### Manual Rollback

```bash
# Immediate rollback to previous version
./scripts/release/rollback.sh production

# Rollback to specific version
./scripts/release/rollback.sh production --version=v1.2.2

# Verify rollback
kubectl rollout status deployment/api-gateway -n agentaskit-production
```

### Rollback Verification

```bash
# Verify services healthy
./scripts/release/verify-rollback.sh

# Expected output:
# ✓ api-gateway: v1.2.2 (healthy)
# ✓ orchestrator: v1.2.2 (healthy)
# ✓ workers: v1.2.2 (healthy)
```

## Version Tracking

### SoT Updates

After successful promotion:

```bash
# Update SoT with release
./scripts/release/update-sot.sh v1.2.3 \
  --environment=production \
  --promoted-by=@username \
  --promotion-time=$(date -u +%Y-%m-%dT%H:%M:%SZ)
```

### Evidence Collection

```bash
# Collect promotion evidence
./scripts/release/collect-evidence.sh v1.2.3

# Generates:
# - TEST/release/v1.2.3/promotion.log
# - TEST/release/v1.2.3/metrics.json
# - TEST/release/v1.2.3/approval.json
```

## Hotfix Process

For urgent production fixes:

```bash
# Create hotfix branch
git checkout -b hotfix/v1.2.4 v1.2.3

# Make fix, commit, tag
git commit -m "fix: critical issue"
git tag v1.2.4

# Fast-track promotion (requires 2 approvers)
./scripts/release/hotfix-promote.sh v1.2.4 \
  --skip-staging \
  --approver1=@senior-eng \
  --approver2=@team-lead
```

## Audit Trail

All promotions are logged:

```json
{
  "version": "v1.2.3",
  "timestamp": "2025-10-05T12:00:00Z",
  "action": "promote",
  "from_stage": "staging",
  "to_stage": "canary",
  "approved_by": "@username",
  "ci_run_id": "12345",
  "commit_sha": "abc123"
}
```

## Evidence

- Promotion logs: `TEST/release/*/promotion.log`
- Metrics: `TEST/release/*/metrics.json`
- Approvals: `operational_audit/releases/`

## Related

- [REL-VERSIONING](./versioning.md) - Versioning policy
- [DEP-CANARY](../../deploy/canary/plan.md) - Canary configuration
- [CD-GATES](../../.github/workflows/release.yml) - Release workflow
