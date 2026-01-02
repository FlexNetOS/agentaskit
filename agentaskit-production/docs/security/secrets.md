# Secrets Management & Key Rotation

**REF:** SEC-SECRETS
**Owner:** @sec-oncall
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the secrets management strategy for AgentAskit production, ensuring zero secrets in repository and secure external secrets management.

## External Secrets Integration

### Supported Backends

| Backend | Environment | Use Case |
|---------|-------------|----------|
| HashiCorp Vault | Production | Primary secrets store |
| AWS Secrets Manager | AWS deployments | Cloud-native integration |
| Azure Key Vault | Azure deployments | Cloud-native integration |
| Kubernetes Secrets | K8s clusters | Runtime injection |

### Configuration

```yaml
# configs/secrets/vault.yaml
vault:
  address: ${VAULT_ADDR}
  auth:
    method: kubernetes
    role: agentaskit-production
  secrets:
    - path: secret/data/agentaskit/api-keys
      key: api_key
    - path: secret/data/agentaskit/db-credentials
      key: db_password
    - path: secret/data/agentaskit/jwt-signing
      key: jwt_secret
```

## Key Rotation Policy

### Rotation Schedule

| Secret Type | Rotation Frequency | Grace Period |
|-------------|-------------------|--------------|
| API Keys | 90 days | 7 days |
| Database Credentials | 30 days | 24 hours |
| JWT Signing Keys | 7 days | 1 hour |
| Service Tokens | 24 hours | 30 minutes |
| Encryption Keys | 365 days | 30 days |

### Automated Rotation

```bash
# Rotation script: scripts/rotate_secrets.sh
#!/bin/bash
set -euo pipefail

rotate_api_keys() {
    vault write -f secret/data/agentaskit/api-keys/rotate
    kubectl rollout restart deployment/agentaskit-api
}

rotate_db_credentials() {
    vault write -f database/rotate-role/agentaskit-db
    kubectl rollout restart deployment/agentaskit-workers
}
```

## Zero Secrets in Repository

### Pre-commit Hooks

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/Yelp/detect-secrets
    rev: v1.4.0
    hooks:
      - id: detect-secrets
        args: ['--baseline', '.secrets.baseline']
```

### CI Validation

The `SEC-CI` workflow validates:
- No hardcoded secrets in code
- No secrets in configuration files
- Environment variables properly templated

## Access Control

### RBAC for Secrets

```yaml
# Vault policy: policies/agentaskit-secrets.hcl
path "secret/data/agentaskit/*" {
  capabilities = ["read"]
}

path "secret/metadata/agentaskit/*" {
  capabilities = ["list"]
}
```

### Service Account Mapping

| Service | Vault Role | Permissions |
|---------|------------|-------------|
| API Gateway | api-gateway-role | Read API keys |
| Workers | worker-role | Read DB creds, API keys |
| Orchestrator | orchestrator-role | Read all secrets |

## Monitoring & Audit

- All secret access logged to audit trail
- Alerts on unauthorized access attempts
- Rotation failure alerts with escalation

## Evidence

- Configuration: `configs/secrets/`
- Rotation logs: `operational_audit/secrets/`
- Vault policies: `security/policies/vault/`

## Related

- [SEC-POLICY](../runbooks/security/rotation.md)
- [SEC-ACCESS-REVIEW](../../operational_audit/access_reviews/)
