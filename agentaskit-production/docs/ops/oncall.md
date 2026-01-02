# On-Call Rotation & Escalation

**REF:** OPS-ONCALL
**Owner:** @sre
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the on-call rotation schedule, escalation policies, and paging configuration for AgentAskit production.

## Rotation Schedule

### Primary On-Call

| Week | Primary | Secondary | Dates |
|------|---------|-----------|-------|
| 40 | @engineer-1 | @engineer-2 | Oct 1-7, 2025 |
| 41 | @engineer-2 | @engineer-3 | Oct 8-14, 2025 |
| 42 | @engineer-3 | @engineer-4 | Oct 15-21, 2025 |
| 43 | @engineer-4 | @engineer-1 | Oct 22-28, 2025 |

### Rotation Configuration

```yaml
# configs/oncall.yaml
rotations:
  - name: agentaskit-primary
    schedule:
      type: weekly
      start_day: monday
      handoff_time: "09:00 UTC"

    members:
      - "@engineer-1"
      - "@engineer-2"
      - "@engineer-3"
      - "@engineer-4"

    coverage:
      timezone: UTC
      hours: 24x7

  - name: agentaskit-secondary
    schedule:
      type: weekly
      start_day: monday
      handoff_time: "09:00 UTC"
      offset: 1  # One week behind primary
```

## Escalation Policy

### Escalation Path

```
┌─────────────────┐     5 min     ┌─────────────────┐
│    Primary      │──────────────▶│   Secondary     │
│    On-Call      │               │   On-Call       │
└─────────────────┘               └────────┬────────┘
                                           │
                                           │ 10 min
                                           ▼
┌─────────────────┐     15 min    ┌─────────────────┐
│    Manager      │◀──────────────│   Team Lead     │
│    On-Call      │               │                 │
└─────────────────┘               └─────────────────┘
```

### Escalation Configuration

```yaml
# configs/escalation.yaml
policies:
  - name: default
    steps:
      - delay: 0
        targets:
          - type: schedule
            schedule: agentaskit-primary
        notification:
          - channel: pager
          - channel: slack

      - delay: 5m
        targets:
          - type: schedule
            schedule: agentaskit-secondary
        notification:
          - channel: pager
          - channel: phone

      - delay: 15m
        targets:
          - type: user
            user: "@team-lead"
        notification:
          - channel: phone

      - delay: 30m
        targets:
          - type: user
            user: "@engineering-manager"
        notification:
          - channel: phone
```

## Paging Configuration

### Alert Routing

| Alert Type | Route To | Method |
|------------|----------|--------|
| P1 Critical | Primary + Secondary | Page + Phone |
| P2 High | Primary | Page |
| P3 Medium | Primary | Slack |
| P4 Low | Email only | Email |

### Notification Channels

```yaml
# configs/notifications.yaml
channels:
  pager:
    type: pagerduty
    service_key: "${PAGERDUTY_SERVICE_KEY}"

  slack:
    type: slack
    webhook: "${SLACK_ONCALL_WEBHOOK}"
    channel: "#agentaskit-alerts"

  phone:
    type: twilio
    from: "+1-555-0100"

  email:
    type: email
    from: "alerts@agentaskit.io"
```

## On-Call Expectations

### Response Times

| Severity | Acknowledge | Respond | Escalate If |
|----------|-------------|---------|-------------|
| P1 | 5 min | 15 min | No ack in 5 min |
| P2 | 15 min | 30 min | No ack in 15 min |
| P3 | 1 hour | 4 hours | No ack in 1 hour |

### Responsibilities

1. **Before Shift**
   - Review handoff notes
   - Verify pager/phone working
   - Check pending issues

2. **During Shift**
   - Acknowledge all pages within SLA
   - Triage and resolve or escalate
   - Update status page as needed
   - Document all incidents

3. **After Shift**
   - Complete handoff document
   - Brief incoming on-call
   - Follow up on open items

## Handoff Template

```markdown
## On-Call Handoff: Week 40 → Week 41

### Outgoing: @engineer-1
### Incoming: @engineer-2
### Date: 2025-10-07 09:00 UTC

### Active Incidents
- None

### Recent Incidents (Last 7 Days)
- INC-001: API degradation (resolved, PIR pending)

### Known Issues
- Elevated latency on agent-pool-3 (monitoring)

### Upcoming Changes
- v1.2.3 deployment scheduled Oct 9

### Notes
- New runbook for database failover added
```

## Override Procedures

### Requesting Override

```bash
# Request override
./scripts/oncall/override.sh \
  --from=@engineer-1 \
  --to=@engineer-3 \
  --start="2025-10-05 18:00 UTC" \
  --end="2025-10-06 09:00 UTC" \
  --reason="Personal appointment"
```

### Emergency Coverage

If on-call is unavailable:
1. Page secondary
2. If no response, page team lead
3. If critical, page engineering manager

## Testing

### Pager Test

```bash
# Test pager configuration (weekly)
./scripts/oncall/test-pager.sh --user=@engineer-1

# Test escalation (monthly)
./scripts/oncall/test-escalation.sh --dry-run
```

## Evidence

- Rotation configs: `configs/oncall.yaml`
- Escalation policies: `configs/escalation.yaml`
- Handoff logs: `operational_audit/oncall/`

## Related

- [OPS-INCIDENTS](./incident_management.md) - Incident management
- [SLO-POLICY](../../slo/policies.yaml) - SLO policies
- [OBS-001](../../.todo) - Observability
