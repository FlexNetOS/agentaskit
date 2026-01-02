# Incident Management & PIR Process

**REF:** OPS-INCIDENTS
**Owner:** @sre
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the incident management process, severity matrix, and post-incident review (PIR) procedures for AgentAskit production.

## Severity Matrix

| Severity | Criteria | Response Time | Resolution Target |
|----------|----------|---------------|-------------------|
| P1 - Critical | Complete service outage, data loss risk | 15 min | 1 hour |
| P2 - High | Major feature unavailable, >50% users affected | 30 min | 4 hours |
| P3 - Medium | Minor feature degraded, <50% users affected | 2 hours | 24 hours |
| P4 - Low | Cosmetic issues, no user impact | 24 hours | 1 week |

### Severity Examples

| Scenario | Severity |
|----------|----------|
| API completely down | P1 |
| Workflow processing stopped | P1 |
| Agent pool >50% unhealthy | P2 |
| Single region degraded | P2 |
| Slow response times (2× normal) | P3 |
| Dashboard unavailable | P3 |
| Minor UI bug | P4 |

## Incident Response Process

### 1. Detection & Triage

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Alert      │───▶│   On-Call    │───▶│   Triage     │
│   Triggered  │    │   Notified   │    │   Severity   │
└──────────────┘    └──────────────┘    └──────────────┘
```

### 2. Response

```bash
# Acknowledge incident
./scripts/incident/ack.sh --id=INC-001 --oncall=@username

# Start incident channel
./scripts/incident/start-bridge.sh --severity=P1

# Update status page
./scripts/incident/status-update.sh --status=investigating
```

### 3. Investigation

```bash
# Gather diagnostics
./scripts/incident/diagnostics.sh > /tmp/inc-001-diag.txt

# Check recent changes
git log --oneline --since="2 hours ago"

# Review metrics
open https://grafana.agentaskit.io/d/overview
```

### 4. Mitigation

| Action | Command |
|--------|---------|
| Rollback | `kubectl rollout undo deployment/api-gateway` |
| Scale up | `kubectl scale deployment/workers --replicas=200` |
| Failover | `./scripts/dr/failover.sh` |
| Feature flag | `./scripts/flags/disable.sh --flag=new-feature` |

### 5. Resolution

```bash
# Verify resolution
./scripts/incident/verify-resolution.sh

# Update status page
./scripts/incident/status-update.sh --status=resolved

# Close incident
./scripts/incident/close.sh --id=INC-001
```

## On-Call Responsibilities

### On-Call Engineer

- Acknowledge alerts within SLA
- Triage and assign severity
- Lead incident response
- Communicate status updates
- Hand off to next shift if unresolved

### Incident Commander (P1/P2)

- Coordinate response team
- Make escalation decisions
- Communicate with stakeholders
- Ensure documentation

## Communication Templates

### Initial Update

```
[Incident: INC-001] [P1] API Service Degraded

Status: Investigating
Impact: API requests failing for ~30% of users
Started: 2025-10-05 14:30 UTC
Team: @sre-oncall investigating

Next update in 15 minutes.
```

### Resolution Update

```
[Incident: INC-001] [P1] RESOLVED - API Service Degraded

Status: Resolved
Duration: 45 minutes
Impact: API requests failed for ~30% of users
Root Cause: Database connection pool exhaustion
Resolution: Increased pool size, implemented connection recycling

PIR scheduled for 2025-10-06 10:00 UTC.
```

## Post-Incident Review (PIR)

### PIR Template

```yaml
# pir/INC-001.yaml
incident_id: INC-001
severity: P1
date: 2025-10-05
duration: 45m
impact: 30% of API requests failed

timeline:
  - time: "14:30"
    event: "Alert triggered for API errors"
  - time: "14:32"
    event: "On-call acknowledged"
  - time: "14:45"
    event: "Root cause identified"
  - time: "15:00"
    event: "Mitigation applied"
  - time: "15:15"
    event: "Incident resolved"

root_cause: |
  Database connection pool exhausted due to slow query
  introduced in recent deployment.

contributing_factors:
  - No connection pool monitoring
  - Slow query not caught in staging

action_items:
  - id: AI-001
    action: "Add connection pool metrics"
    owner: "@platform"
    due: "2025-10-12"
  - id: AI-002
    action: "Add query performance tests"
    owner: "@qa"
    due: "2025-10-15"

lessons_learned:
  - Always monitor database connection pools
  - Add query performance benchmarks to CI
```

### PIR Meeting Agenda

1. Timeline review (5 min)
2. Root cause analysis (15 min)
3. Contributing factors (10 min)
4. Action items (15 min)
5. Lessons learned (5 min)

## Metrics & Tracking

| Metric | Target | Current |
|--------|--------|---------|
| MTTA (Mean Time to Acknowledge) | <15 min | 8 min |
| MTTD (Mean Time to Detect) | <5 min | 3 min |
| MTTR (Mean Time to Resolve) | <2 hours | 45 min |
| PIR completion rate | 100% | 100% |

## Evidence

- Incident logs: `operational_audit/incidents/`
- PIR documents: `docs/ops/pir/`
- Action item tracking: GitHub Issues with `incident-followup` label

## Related

- [SLO-POLICY](../../slo/policies.yaml) - SLO policies
- [OBS-001](../../.todo) - Observability
- [OPS-ONCALL](./oncall.md) - On-call rotation
