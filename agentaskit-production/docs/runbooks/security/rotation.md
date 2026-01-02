# Security Token Rotation Runbook

**Owner:** @sec-oncall
**Last Updated:** 2026-01-02
**Related Policy:** [Token Rotation Policy](/security/policies/token_rotation_policy.md)

## Overview

This runbook provides step-by-step procedures for rotating security tokens, credentials, and keys in the AgentAskit system. Follow these procedures during scheduled rotations or in response to security incidents.

## Pre-Rotation Checklist

Before beginning any rotation:

- [ ] Verify you have appropriate access and permissions
- [ ] Review recent incidents or security alerts
- [ ] Confirm backup/rollback plan is in place
- [ ] Notify relevant teams of maintenance window (if required)
- [ ] Prepare monitoring dashboard for rotation validation
- [ ] Have incident response contacts ready

## Rotation Procedures

### 1. Capability Token Rotation (Automated)

**Frequency:** Every 24 hours (automated)
**Criticality:** HIGH
**Rollback Time:** < 5 minutes

#### Automated Process

The capability token rotation is fully automated via the token rotation service:

```bash
# Check rotation service status
kubectl get pods -n security -l app=token-rotator

# View rotation logs
kubectl logs -n security -l app=token-rotator --tail=100 --follow

# Check last rotation timestamp
kubectl exec -n security deploy/token-rotator -- /app/bin/rotation-status
```

#### Manual Intervention (Emergency Only)

If automated rotation fails:

```bash
# Step 1: Generate new token
cd /security/scripts/
./generate-capability-token.sh --env production --rotation emergency

# Step 2: Update token in secret store
kubectl create secret generic capability-tokens \
  --from-file=tokens.json=./output/tokens-$(date +%Y%m%d-%H%M%S).json \
  --dry-run=client -o yaml | kubectl apply -f -

# Step 3: Restart services to pick up new token
kubectl rollout restart deployment -n agentaskit -l requires=capability-token

# Step 4: Monitor for errors
kubectl logs -n agentaskit -l requires=capability-token --tail=50 --follow

# Step 5: After grace period (24h), revoke old token
./revoke-capability-token.sh --token-id <OLD_TOKEN_ID>
```

#### Validation

```bash
# Verify new tokens are being used
./scripts/validate-token-usage.sh

# Check for any 401/403 errors
kubectl logs -n agentaskit --since=1h | grep -E "401|403" | wc -l

# Confirm old token has no active sessions
./scripts/check-token-sessions.sh --token-id <OLD_TOKEN_ID>
```

### 2. API Key Rotation

**Frequency:** Every 90 days
**Criticality:** MEDIUM-HIGH
**Rollback Time:** < 15 minutes

#### Procedure

```bash
# Step 1: Identify API keys requiring rotation
./scripts/list-api-keys.sh --expiring-in 7d

# Step 2: Generate new API key
export NEW_KEY=$(./scripts/generate-api-key.sh \
  --service agentaskit \
  --scope "read,write" \
  --expires-in 90d)

# Step 3: Update applications with new key (with overlap period)
# For each service using the key:
kubectl set env deployment/<service-name> \
  -n <namespace> \
  API_KEY=$NEW_KEY

# Step 4: Verify services are healthy
kubectl rollout status deployment/<service-name> -n <namespace>

# Step 5: Test API calls with new key
curl -H "Authorization: Bearer $NEW_KEY" \
  https://api.agentaskit.example.com/health

# Step 6: Monitor for 24 hours with both keys active

# Step 7: After validation period, revoke old key
./scripts/revoke-api-key.sh --key-id <OLD_KEY_ID>
```

#### Rollback

```bash
# If issues detected, revert to old key
kubectl set env deployment/<service-name> \
  -n <namespace> \
  API_KEY=<OLD_KEY>

# Verify rollback
kubectl rollout status deployment/<service-name> -n <namespace>
```

### 3. Database Credential Rotation

**Frequency:** Every 180 days
**Criticality:** HIGH
**Rollback Time:** < 10 minutes

#### Procedure

```bash
# Step 1: Create new database user with same permissions
./scripts/db/create-user.sh \
  --username agentaskit_prod_$(date +%Y%m%d) \
  --copy-permissions-from agentaskit_prod

# Step 2: Update application secrets
kubectl create secret generic db-credentials \
  --from-literal=username=agentaskit_prod_$(date +%Y%m%d) \
  --from-literal=password=<GENERATED_PASSWORD> \
  --dry-run=client -o yaml | kubectl apply -f -

# Step 3: Rolling restart of application pods
kubectl rollout restart deployment -n agentaskit -l uses=database

# Step 4: Monitor database connections
# Check for connection errors
kubectl logs -n agentaskit -l uses=database --tail=100 | grep -i "connection"

# Check active database sessions
./scripts/db/list-sessions.sh

# Step 5: After 48h grace period, drop old user
./scripts/db/drop-user.sh --username agentaskit_prod_old
```

#### Validation

```bash
# Verify application can connect with new credentials
kubectl exec -n agentaskit deploy/<app-name> -- \
  /app/bin/db-health-check

# Check for old username in active sessions
./scripts/db/list-sessions.sh | grep agentaskit_prod_old
# (Should return no results after cutover)
```

### 4. Signing Key Rotation

**Frequency:** Every 365 days
**Criticality:** HIGH
**Rollback Time:** < 30 minutes

#### Procedure

```bash
# Step 1: Generate new signing key pair
./scripts/crypto/generate-keypair.sh \
  --algorithm ed25519 \
  --purpose signing \
  --output /security/keys/signing-$(date +%Y).key

# Step 2: Add new public key to keyring (while keeping old key)
./scripts/crypto/add-to-keyring.sh \
  --public-key /security/keys/signing-$(date +%Y).pub \
  --keyring production

# Step 3: Update signing service to use new key for new signatures
kubectl set env deployment/signing-service \
  -n security \
  SIGNING_KEY_PATH=/keys/signing-$(date +%Y).key

# Step 4: Verify new signatures are created with new key
./scripts/crypto/verify-signature.sh \
  --signature <RECENT_SIGNATURE> \
  --key /security/keys/signing-$(date +%Y).pub

# Step 5: Keep old key for verification only (1 year retention)
# Do not remove old public key from keyring for signature verification

# Step 6: After 1 year, archive old private key (offline cold storage)
./scripts/crypto/archive-key.sh \
  --key /security/keys/signing-$(($(date +%Y)-1)).key \
  --destination /secure-archive/
```

### 5. Service Account Token Rotation

**Frequency:** Every 90 days
**Criticality:** MEDIUM

#### Procedure

```bash
# Step 1: List service accounts requiring rotation
./scripts/list-service-accounts.sh --expiring-in 7d

# Step 2: For each service account, create new token
kubectl create token <service-account-name> \
  --duration=2160h \
  -n <namespace> > /tmp/new-token.txt

# Step 3: Update consuming services
# (Application-specific, update configuration or environment)

# Step 4: Validate new token works
./scripts/validate-service-account.sh \
  --token /tmp/new-token.txt \
  --namespace <namespace>

# Step 5: Old token auto-expires at end of validity period
# (No manual revocation needed for time-limited tokens)

# Step 6: Clean up temporary files
shred -u /tmp/new-token.txt
```

## Emergency Rotation Procedures

### Immediate Rotation (Suspected Compromise)

When credentials are suspected to be compromised:

```bash
# Step 1: IMMEDIATELY revoke compromised credential
./scripts/emergency-revoke.sh --credential-id <ID> --reason "suspected_compromise"

# Step 2: Generate new credential
./scripts/emergency-generate.sh --replace <ID>

# Step 3: Update all systems with new credential (parallel execution)
./scripts/emergency-deploy-credential.sh --credential-id <NEW_ID>

# Step 4: Monitor for authentication failures
watch -n 5 './scripts/check-auth-errors.sh'

# Step 5: Create security incident ticket
./scripts/create-incident.sh \
  --severity high \
  --type credential_compromise \
  --credential-id <OLD_ID>

# Step 6: Follow incident response procedures
# See: /security/policies/incident_response_policy.md
```

## Post-Rotation Validation

After any rotation, perform these validation steps:

### 1. Authentication Success Rate

```bash
# Check authentication metrics
./scripts/metrics/auth-success-rate.sh --since 1h

# Expected: >99.9% success rate
# Alert if: <99% success rate
```

### 2. Service Health Checks

```bash
# Run health checks on all services
kubectl get pods -A -o json | \
  jq '.items[] | select(.status.phase != "Running") | .metadata.name'

# Expected: No unhealthy pods
```

### 3. Error Log Review

```bash
# Check for authentication/authorization errors
./scripts/check-recent-errors.sh --type auth --since 1h

# Expected: <10 errors (transient retry errors acceptable)
```

### 4. Dependency Validation

```bash
# Verify all dependent services can authenticate
./scripts/validate-dependencies.sh --credential-type <TYPE>

# Expected: All dependencies pass validation
```

## Monitoring and Alerting

### Key Metrics to Monitor

1. **Rotation Success Rate**
   - Target: 100% for automated rotations
   - Alert: Any failed rotation

2. **Credential Age**
   - Target: All credentials within policy limits
   - Alert: Credentials >90% of max age

3. **Authentication Failures**
   - Baseline: <0.1% of requests
   - Alert: >1% failure rate sustained for >5 minutes

4. **Grace Period Expirations**
   - Alert: Old credentials still in use 24h before expiration

### Dashboard Links

- Rotation Status Dashboard: `/dashboards/security/rotation-status.json`
- Credential Inventory: `/dashboards/security/credential-inventory.json`
- Authentication Metrics: `/dashboards/security/auth-metrics.json`

## Troubleshooting

### Problem: Services can't authenticate after rotation

```bash
# Check if services have new credentials
kubectl get secrets -A | grep <credential-type>

# Check if services restarted to pick up new credentials
kubectl get pods -A -o wide | grep <service-name>

# Check credential validity
./scripts/validate-credential.sh --credential-id <NEW_ID>

# If needed, rollback
./scripts/rollback-rotation.sh --rotation-id <ROTATION_ID>
```

### Problem: Automated rotation failed

```bash
# Check rotation service logs
kubectl logs -n security deploy/token-rotator --tail=200

# Check for common issues:
# - Insufficient permissions
# - Secret store unavailable
# - Network connectivity

# Retry rotation
kubectl exec -n security deploy/token-rotator -- /app/bin/rotate --force

# If still failing, execute manual rotation
# See manual procedures above
```

### Problem: Old credential still showing activity

```bash
# Identify services using old credential
./scripts/trace-credential-usage.sh --credential-id <OLD_ID>

# Force services to refresh credentials
kubectl rollout restart deployment <service-name> -n <namespace>

# If issue persists, may need maintenance window to revoke old credential
```

## Audit Trail

All rotation activities are automatically logged to:
- **Audit Logs:** `/operational_audit/credential_rotations/`
- **Metrics:** Prometheus/Grafana dashboards
- **Incident Tracking:** GitHub Issues with `credential-rotation` label

Manual rotations require additional documentation:
```bash
# Record manual rotation
./scripts/log-rotation.sh \
  --credential-type <TYPE> \
  --old-id <OLD_ID> \
  --new-id <NEW_ID> \
  --operator <USERNAME> \
  --reason <REASON>
```

## Contacts and Escalation

- **Primary:** @sec-oncall (24/7 on-call rotation)
- **Secondary:** @platform-oncall
- **Escalation:** Director of Security
- **Emergency:** Follow incident response policy

## Related Documentation

- [Token Rotation Policy](/security/policies/token_rotation_policy.md)
- [Access Review Policy](/security/policies/access_review_policy.md)
- [Incident Response Policy](/security/policies/incident_response_policy.md)
- [Security Token Schema](/security/token-schema.json)
- [Operational Audit Logs](/operational_audit/)

## Appendix: Credential Inventory

| Credential Type | Location | Rotation Frequency | Automation |
|----------------|----------|-------------------|------------|
| Capability Tokens | Kubernetes Secrets | 24 hours | Automated |
| API Keys | Secret Store | 90 days | Semi-automated |
| Database Credentials | Vault | 180 days | Manual |
| Signing Keys | KMS | 365 days | Manual |
| Service Account Tokens | Kubernetes | 90 days | Semi-automated |
| TLS Certificates | cert-manager | 90 days | Automated |
| SSH Keys | Bastion Config | 180 days | Manual |

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2026-01-02 | Initial runbook | @sec-oncall |
