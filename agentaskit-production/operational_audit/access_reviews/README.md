# Periodic Access Reviews

**REF:** SEC-ACCESS-REVIEW
**Owner:** @sec-oncall
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the quarterly RBAC access review process for AgentAskit production.

## Review Schedule

| Quarter | Review Period | Due Date | Status |
|---------|---------------|----------|--------|
| Q1 2025 | Jan 1 - Mar 31 | Apr 15 | Completed |
| Q2 2025 | Apr 1 - Jun 30 | Jul 15 | Completed |
| Q3 2025 | Jul 1 - Sep 30 | Oct 15 | In Progress |
| Q4 2025 | Oct 1 - Dec 31 | Jan 15 | Scheduled |

## Review Process

### 1. Access Inventory

```bash
# Generate access report
./scripts/security/access_report.sh > reports/access_q3_2025.json
```

### 2. Role Review Checklist

| Role | Users | Last Activity | Action |
|------|-------|---------------|--------|
| admin | 5 | Active | Verify |
| operator | 12 | Active | Verify |
| developer | 45 | Active | Verify |
| readonly | 20 | 10 inactive | Review |

### 3. Variance Categories

| Category | SLA | Action |
|----------|-----|--------|
| Orphaned accounts | 7 days | Disable |
| Excessive permissions | 7 days | Scope down |
| Unused service accounts | 14 days | Review/Disable |
| Stale credentials | 7 days | Rotate |

## Review Template

```yaml
# access_review_q3_2025.yaml
review:
  quarter: Q3-2025
  reviewer: @sec-oncall
  review_date: 2025-10-01

  accounts_reviewed: 82
  variances_found: 5
  variances_closed: 5
  closure_date: 2025-10-05

  findings:
    - id: VAR-001
      type: orphaned_account
      account: user-old-123
      action: disabled
      closed_by: @sec-oncall

    - id: VAR-002
      type: excessive_permissions
      account: svc-worker-01
      action: scoped_down
      closed_by: @platform
```

## Automation

### Scheduled Reviews

```yaml
# .github/workflows/access-review.yml
name: Quarterly Access Review
on:
  schedule:
    - cron: '0 0 1 1,4,7,10 *'  # Q1-Q4 start

jobs:
  generate-report:
    runs-on: ubuntu-latest
    steps:
      - name: Generate access inventory
        run: ./scripts/security/access_report.sh

      - name: Create review issue
        run: gh issue create --title "Q$Q Access Review"
```

### Variance Alerts

```yaml
# alerts/access_review.yaml
alerts:
  - name: variance_sla_breach
    condition: variance_age_days > 7
    severity: high
    action: page_security

  - name: review_overdue
    condition: review_overdue_days > 5
    severity: medium
    action: notify_security
```

## Evidence

- Review reports: `operational_audit/access_reviews/reports/`
- Variance logs: `operational_audit/access_reviews/variances/`
- Automation: `.github/workflows/access-review.yml`

## Related

- [SEC-POLICY](../../security/policies/)
- [SEC-001](../../.todo) - Capability tokens
- [GOV-ADR](../../docs/decisions/adr/)
