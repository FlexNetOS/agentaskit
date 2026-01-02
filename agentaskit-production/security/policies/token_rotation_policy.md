# Token Rotation Policy

**Version:** 1.0
**Owner:** @sec-oncall
**Last Updated:** 2026-01-02
**Review Cycle:** Quarterly

## Purpose

This policy defines the requirements and procedures for rotating capability tokens, API keys, and other authentication credentials used in the AgentAskit system to minimize security risk from credential compromise.

## Scope

This policy applies to all:
- Capability tokens used for agent authorization
- API keys and service credentials
- Signing keys and certificates
- Database credentials
- Third-party integration tokens

## Policy Requirements

### 1. Rotation Schedule

**Mandatory Rotation Intervals:**
- **Capability Tokens:** ≤24 hours (automated)
- **API Keys:** ≤90 days
- **Service Credentials:** ≤90 days
- **Signing Keys:** ≤365 days
- **Database Credentials:** ≤180 days

**Immediate Rotation Triggers:**
- Suspected compromise or exposure
- Personnel changes (departures, role changes)
- Security incident involving credential systems
- Failed audit or compliance finding
- Service termination or decommissioning

### 2. Rotation Process

All credential rotations must follow these steps:

1. **Pre-rotation validation**
   - Verify new credential generation process
   - Confirm rollback capability
   - Check dependent services and systems

2. **Rotation execution**
   - Generate new credential with appropriate scope
   - Deploy new credential to all required systems
   - Validate new credential functionality
   - Monitor for errors during transition period

3. **Deprecation**
   - Mark old credential for deletion
   - Maintain grace period (minimum 24 hours for capability tokens, 7 days for API keys)
   - Monitor for usage of deprecated credential

4. **Deletion**
   - Revoke old credential after grace period
   - Log deletion event
   - Update credential inventory

### 3. Automation Requirements

- Capability token rotation MUST be fully automated
- Automated rotation MUST include failure detection and alerting
- Failed rotation attempts MUST trigger immediate on-call notification
- Rotation events MUST be logged in audit trail

### 4. Documentation Requirements

Each credential type MUST have:
- Current rotation runbook
- Emergency rotation procedure
- Rollback plan
- List of dependent systems
- Testing procedure

### 5. Access Control

- Rotation procedures require least-privilege access
- Automated rotation uses dedicated service accounts
- Manual rotation requires dual authorization for production
- All rotation events logged with operator identity

## Compliance

- **Audit Trail:** All rotations logged for minimum 365 days
- **Metrics:** Track rotation completion rate, failures, and overdue credentials
- **Reporting:** Monthly summary to security team, quarterly to leadership
- **Exceptions:** Require written approval from security team, maximum 30-day extension

## Related Documents

- [Access Review Policy](./access_review_policy.md)
- [Incident Response Policy](./incident_response_policy.md)
- [Token Rotation Runbook](../../docs/runbooks/security/rotation.md)
- Security Token Schema: `/security/token-schema.json`

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2026-01-02 | Initial policy | @sec-oncall |
