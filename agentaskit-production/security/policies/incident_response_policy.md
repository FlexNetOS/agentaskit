# Security Incident Response Policy

**Version:** 1.0
**Owner:** @sec-oncall
**Last Updated:** 2026-01-02
**Review Cycle:** Quarterly

## Purpose

This policy defines the process for detecting, responding to, analyzing, and learning from security incidents to minimize impact and prevent recurrence.

## Scope

This policy applies to all security incidents including:
- Unauthorized access or authentication bypass
- Data breaches or exfiltration
- Malware or ransomware
- Denial of service attacks
- Credential compromise or token leakage
- Insider threats
- Supply chain compromises
- Vulnerability exploitation

## Incident Severity Classification

### Severity 1 (Critical)
- **Impact:** Active breach, data exfiltration, or system compromise in production
- **Response Time:** Immediate (< 15 minutes)
- **Escalation:** Security team + Director + Exec on-call
- **Examples:** Active attacker in production, customer data breach, ransomware

### Severity 2 (High)
- **Impact:** Confirmed security compromise in non-production or potential production threat
- **Response Time:** < 1 hour
- **Escalation:** Security team + Engineering manager
- **Examples:** Compromised credentials, vulnerability exploitation in staging, confirmed supply chain issue

### Severity 3 (Medium)
- **Impact:** Security incident contained or low-impact compromise
- **Response Time:** < 4 hours
- **Escalation:** Security team
- **Examples:** Failed attack attempts, suspicious activity, misconfiguration

### Severity 4 (Low)
- **Impact:** Potential security concern requiring investigation
- **Response Time:** < 1 business day
- **Escalation:** Security team (standard process)
- **Examples:** Security alerts, audit findings, compliance gaps

## Incident Response Process

### Phase 1: Detection and Triage (0-30 minutes)

**Actions:**
1. **Receive Alert:** From monitoring, user report, or external notification
2. **Initial Assessment:**
   - What happened? What systems/data affected?
   - Is incident active or contained?
   - What is the severity level?
3. **Declare Incident:** Create incident ticket and assign severity
4. **Notify Responders:** Page on-call security, escalate per severity
5. **Start Incident Log:** Begin timeline documentation

**Deliverables:**
- Incident ticket created with severity classification
- On-call team notified and acknowledged
- War room established (Slack channel, video call as needed)
- Initial incident log started

### Phase 2: Containment (30 minutes - 4 hours)

**Immediate Containment Actions:**
1. **Isolate affected systems:** Network isolation, disable accounts, revoke tokens
2. **Stop the bleeding:** Block attacker IPs, disable compromised services
3. **Preserve evidence:** Take snapshots, collect logs, capture forensic data
4. **Prevent spread:** Check for lateral movement, isolate connected systems

**Short-term Containment:**
1. Apply temporary patches or workarounds
2. Implement enhanced monitoring on related systems
3. Deploy additional security controls
4. Validate containment effectiveness

**Deliverables:**
- Affected systems identified and isolated
- Attack vector contained
- Evidence preserved for investigation
- Containment validated and documented

### Phase 3: Investigation and Eradication (4-24 hours)

**Investigation:**
1. **Root Cause Analysis:** How did attacker gain access?
2. **Scope Determination:** What data/systems were accessed?
3. **Timeline Reconstruction:** When did compromise occur? What actions taken?
4. **Impact Assessment:** What is the damage? What data was exposed?

**Eradication:**
1. Remove attacker access and persistence mechanisms
2. Patch vulnerabilities exploited
3. Rotate all potentially compromised credentials
4. Clean or rebuild compromised systems
5. Validate eradication through testing and monitoring

**Deliverables:**
- Complete incident timeline
- Root cause identified and documented
- All attacker access removed
- Vulnerabilities patched
- System integrity validated

### Phase 4: Recovery (24-72 hours)

**Actions:**
1. Restore systems from clean backups or rebuild
2. Gradually restore service with enhanced monitoring
3. Validate functionality and security posture
4. Monitor for signs of recurring incident
5. Communicate status to stakeholders

**Validation:**
- Run security scans on recovered systems
- Review logs for anomalous activity
- Conduct tabletop exercise of incident
- Verify all containment measures still in place

**Deliverables:**
- Systems restored to normal operation
- Service availability validated
- Enhanced monitoring in place
- Stakeholder communication completed

### Phase 5: Post-Incident Review (72 hours - 1 week)

**Post-Incident Report (PIR):**
1. **Incident Summary:** What happened, when, severity
2. **Timeline:** Detailed chronology of events and responses
3. **Impact Analysis:** Systems affected, data exposed, downtime, costs
4. **Root Cause:** Technical and procedural failures
5. **Response Effectiveness:** What went well, what didn't
6. **Lessons Learned:** Key takeaways
7. **Action Items:** Preventative measures, process improvements

**Follow-up Actions:**
1. Implement preventative controls
2. Update runbooks and playbooks
3. Conduct training if needed
4. Update threat models
5. Review and adjust security policies

**Deliverables:**
- Complete PIR document
- Action items created and assigned
- Stakeholders notified of findings
- PIR archived for future reference

## Communication Plan

### Internal Communication
- **Security Team:** Real-time updates in war room
- **Engineering Teams:** Status updates every 4 hours (Sev 1-2)
- **Leadership:** Initial notification + daily status (Sev 1-2)
- **Company-wide:** Post-resolution summary for significant incidents

### External Communication
- **Customers:** Required for data breaches per SLA and regulations
- **Partners/Vendors:** If their systems involved or affected
- **Regulatory Bodies:** Per legal requirements (e.g., GDPR 72-hour rule)
- **Law Enforcement:** For criminal activity, per legal counsel guidance
- **Public:** Only via approved PR process for public incidents

### Communication Templates
- Initial notification template
- Status update template
- Resolution announcement template
- Customer notification template (legal review required)

## Roles and Responsibilities

### Incident Commander (Security On-Call)
- Overall incident response coordination
- Decision-making authority
- Stakeholder communication
- Resource allocation

### Technical Lead (System Owner)
- Technical investigation and remediation
- System-specific expertise
- Implementation of fixes
- Validation of recovery

### Communications Lead (Manager/Director)
- Internal and external communications
- Stakeholder management
- Legal/compliance coordination
- Media relations if needed

### Scribe (Security Team Member)
- Incident timeline documentation
- Action item tracking
- Evidence collection and organization
- PIR preparation

## Tools and Resources

**Incident Management:**
- Incident tracking: GitHub Issues with `incident` label
- War room: Dedicated Slack channel `#incident-YYYYMMDD-HHmm`
- Video conferencing: For coordination calls
- Runbooks: `/docs/runbooks/security/`

**Forensics and Investigation:**
- Log aggregation: Centralized logging system
- SIEM: Security monitoring dashboards
- Network captures: Packet capture tools
- Memory/disk forensics: Forensic toolkit

**Communication:**
- Status page: For customer-facing incidents
- Email templates: Pre-approved notification templates
- Contact lists: On-call rotations, escalation paths

## Training and Drills

- **Annual Training:** All engineers complete incident response training
- **Quarterly Tabletop:** Simulate incident response for Sev 1-2 scenarios
- **Post-Incident Review:** Team learning session within 2 weeks of each incident
- **Runbook Updates:** Incorporate lessons learned into playbooks

## Compliance and Audit

**Retention:**
- Incident tickets: 7 years
- PIR documents: 7 years
- Forensic evidence: Per legal hold requirements
- Communication records: 3 years

**Metrics:**
- Mean time to detect (MTTD)
- Mean time to respond (MTTR)
- Mean time to recover (MTTR)
- Incident count by severity
- Repeat incidents (same root cause)

**Reporting:**
- Monthly summary to security team
- Quarterly trends to leadership
- Annual compliance report

## Related Documents

- [Token Rotation Policy](./token_rotation_policy.md)
- [Access Review Policy](./access_review_policy.md)
- [Incident Response Runbooks](../../docs/runbooks/security/)
- [Security Token Rotation](../../docs/runbooks/security/rotation.md)
- SLO Policy: `/slo/policies.yaml`
- Incident Management SOP: `/docs/ops/incident_management.md`

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2026-01-02 | Initial policy | @sec-oncall |
