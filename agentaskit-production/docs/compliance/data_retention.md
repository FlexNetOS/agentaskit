# Data Retention & Privacy Policy

**REF:** COMPL-DATARET
**Owner:** @program
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the data retention policies, PII handling procedures, and deletion workflows for AgentAskit production.

## Retention Policy

### Data Categories

| Category | Description | Retention Period | Storage Location |
|----------|-------------|------------------|------------------|
| Operational Logs | System, application logs | 30 days hot, 365 days archive |  Loki/S3 |
| Audit Logs | Security, access events | 2 years | Immutable S3 |
| User Data | User-submitted content | Duration of account + 30 days | PostgreSQL |
| Task Results | Processed outputs | 90 days | S3 |
| Metrics | System telemetry | 13 months | Prometheus/Thanos |
| Traces | Distributed traces | 7 days | Jaeger/Tempo |
| Session Data | Active sessions | Session duration + 24h | Redis |

### Retention Schedule

```yaml
# configs/retention.yaml
retention:
  operational_logs:
    hot: 30d
    warm: 90d
    archive: 365d
    deletion: automatic

  audit_logs:
    hot: 90d
    warm: 365d
    archive: 730d
    deletion: manual_review

  user_data:
    active: indefinite
    post_deletion: 30d
    legal_hold: indefinite
    deletion: upon_request

  task_results:
    default: 90d
    extended: 365d
    deletion: automatic
```

## PII Handling

### PII Classification

| Level | Examples | Handling |
|-------|----------|----------|
| High | SSN, Financial data | Encrypted at rest, masked in logs |
| Medium | Email, Phone | Encrypted, redacted in exports |
| Low | Name, Preferences | Standard encryption |

### PII Processing

```rust
// core/src/privacy/pii_handler.rs
pub enum PIILevel {
    High,
    Medium,
    Low,
}

pub struct PIIProcessor {
    encryption_key: Key,
    masking_rules: Vec<MaskingRule>,
}

impl PIIProcessor {
    pub fn process(&self, data: &Data) -> Result<ProcessedData> {
        // Encrypt high-level PII
        // Mask in logs
        // Track access
    }
}
```

### Log Redaction

All PII is automatically redacted from logs:

```json
{
  "user_email": "[REDACTED]",
  "user_id": "user-12345",
  "action": "login",
  "ip": "192.168.x.x"
}
```

## Deletion Workflows

### User Data Deletion Request (GDPR/CCPA)

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ User Request │───▶│   Verify     │───▶│   Queue      │
│  (API/Form)  │    │   Identity   │    │   Deletion   │
└──────────────┘    └──────────────┘    └──────────────┘
                                              │
                                              ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Confirm    │◀───│   Execute    │◀───│   Process    │
│   Deletion   │    │   Deletion   │    │   Queue      │
└──────────────┘    └──────────────┘    └──────────────┘
```

### Deletion API

```http
DELETE /api/v1/users/{user_id}/data
Authorization: Bearer <admin_token>

{
  "reason": "user_request",
  "verification_code": "abc123",
  "include_backups": true
}
```

### Deletion Verification

```sql
-- Verification query for deletion compliance
SELECT
    table_name,
    COUNT(*) as remaining_records
FROM information_schema.tables t
LEFT JOIN (
    SELECT * FROM all_tables WHERE user_id = ?
) r ON t.table_name = r.source_table
WHERE schema_name = 'agentaskit'
GROUP BY table_name;
```

## Legal Hold

### Hold Triggers

| Trigger | Action | Duration |
|---------|--------|----------|
| Litigation | Suspend deletion | Until release |
| Regulatory | Preserve specified data | Per requirement |
| Internal Investigation | Freeze user data | Until closure |

### Hold Implementation

```yaml
# legal_hold configuration
legal_hold:
  enabled: true
  holds:
    - id: HOLD-2025-001
      reason: litigation
      user_ids: [user-123, user-456]
      data_types: [all]
      start_date: 2025-10-01
      end_date: null  # indefinite
```

## Privacy Compliance

### GDPR Compliance

- [ ] Right to access: API endpoint provided
- [ ] Right to rectification: Edit endpoints available
- [ ] Right to erasure: Deletion workflow implemented
- [ ] Right to portability: Export endpoint available
- [ ] Breach notification: Alerting configured

### CCPA Compliance

- [ ] Right to know: Data inventory available
- [ ] Right to delete: Deletion workflow implemented
- [ ] Right to opt-out: Preference settings available
- [ ] Non-discrimination: No service degradation

## Audit Trail

All data access and modifications are logged:

```json
{
  "timestamp": "2025-10-05T12:00:00Z",
  "action": "data_access",
  "user_id": "admin-001",
  "target_user": "user-123",
  "data_type": "user_profile",
  "reason": "support_request",
  "ticket_id": "TICKET-456"
}
```

## Evidence

- Retention configs: `configs/retention.yaml`
- Deletion logs: `operational_audit/deletions/`
- Legal holds: `security/legal_holds/`
- Compliance reports: `docs/compliance/reports/`

## Legal Review Notes

Last legal review: 2025-09-15
Next scheduled review: 2026-03-15
Reviewing counsel: [Internal Legal]

## Related

- [DOC-001](../../.todo) - Documentation
- [SEC-POLICY](../security/) - Security policies
- [AUDIT-RETENTION](../../operational_audit/) - Audit retention
