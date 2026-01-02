# Access Review Policy

**Version:** 1.0
**Owner:** @sec-oncall
**Last Updated:** 2026-01-02
**Review Cycle:** Quarterly

## Purpose

This policy establishes requirements for periodic review of access rights to ensure principle of least privilege and detect unauthorized or excessive access permissions.

## Scope

This policy covers access reviews for:
- Production system access (infrastructure, databases, services)
- Source code repositories and CI/CD systems
- Capability token grants and scopes
- Administrative and privileged accounts
- Third-party integrations and service accounts
- Data access (PII, secrets, configuration)

## Policy Requirements

### 1. Review Frequency

**Quarterly Access Reviews:**
- All production system access
- Privileged and administrative accounts
- Service account credentials
- Repository and organization permissions

**Monthly Reviews:**
- Capability token grants in production
- Emergency access usage
- Temporary access extensions

**Triggered Reviews:**
- Within 5 business days of personnel departure
- Within 3 business days of role change
- Within 24 hours of security incident
- After failed compliance audit

### 2. Review Process

#### A. Preparation Phase
1. Generate current access inventory from all systems
2. Identify owners for each access category
3. Compile list of recent personnel/role changes
4. Collect context (team structure, projects, on-call rotation)

#### B. Review Phase
1. **Manager Review:** Direct manager validates each team member's access
2. **Owner Review:** System owners validate service accounts and integrations
3. **Security Review:** Security team validates privileged and sensitive access
4. **Cross-check:** Compare against job requirements and project assignments

#### C. Remediation Phase
1. Document all access variances found
2. Prioritize remediation by risk level:
   - **Critical:** Unauthorized privileged access - remediate within 4 hours
   - **High:** Excessive permissions or inactive accounts - remediate within 24 hours
   - **Medium:** Access beyond current role - remediate within 7 days
   - **Low:** Process improvements - remediate within 30 days
3. Create tickets for each remediation action
4. Track to closure with evidence

#### D. Closure Phase
1. Generate review completion report
2. Document findings and actions taken
3. Update access baseline
4. Archive review evidence for audit trail

### 3. Access Classification

**Privileged Access (Enhanced Review):**
- Production infrastructure admin (root, sudo)
- Database admin credentials
- Secrets management access
- CI/CD admin permissions
- Security tool admin access

**Standard Access (Standard Review):**
- Development environment access
- Non-production database read access
- Repository read/write permissions
- Standard capability token grants

**Temporary Access (Expedited Review):**
- Break-glass access
- Incident response escalations
- Time-limited project access

### 4. Documentation Requirements

Each review must produce:
- **Access Inventory:** Complete snapshot of all access at review time
- **Variance Report:** List of variances found with risk classification
- **Remediation Log:** Actions taken, tickets created, resolution status
- **Sign-off Record:** Approvals from managers, owners, and security team
- **Metrics Summary:** Review completion rate, time-to-remediation, variance trends

### 5. Automation and Tooling

- Automated access inventory collection across all systems
- Automated variance detection against baseline
- Automated notifications to reviewers with deadlines
- Automated escalation for overdue reviews
- Dashboard for review status and metrics

### 6. Roles and Responsibilities

**System Owners:**
- Maintain accurate access requirements documentation
- Review and approve access for their systems
- Remediate variances within SLA

**Managers:**
- Review team member access quarterly
- Approve access requests aligned with role
- Notify security team of personnel changes

**Security Team:**
- Coordinate review schedule and process
- Validate privileged access
- Track metrics and compliance
- Report to leadership

**Individual Contributors:**
- Request only necessary access
- Report access no longer needed
- Cooperate with review process

## Compliance

- **Audit Trail:** Review evidence retained for minimum 3 years
- **Metrics Tracking:**
  - Review completion rate (target: 100%)
  - Average time-to-remediation by severity
  - Variance rate and trending
  - Overdue access removals
- **Reporting:**
  - Monthly dashboard to security team
  - Quarterly summary to leadership
  - Annual compliance report

## Enforcement

- Overdue reviews block new access requests for that team/system
- Variances not remediated within SLA escalate to director level
- Repeated policy violations may result in access suspension
- Non-compliance findings reported in security scorecards

## Related Documents

- [Token Rotation Policy](./token_rotation_policy.md)
- [Incident Response Policy](./incident_response_policy.md)
- RBAC Configuration: `/security/rbac/`
- Access Audit Logs: `/operational_audit/access_reviews/`

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2026-01-02 | Initial policy | @sec-oncall |
