# Disaster Recovery & Backup Plan

**REF:** OPS-DR
**Owner:** @sre
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the disaster recovery strategy, backup procedures, and multi-region failover plan for AgentAskit production.

## Recovery Objectives

| Metric | Target | Current |
|--------|--------|---------|
| RPO (Recovery Point Objective) | 1 hour | 15 min |
| RTO (Recovery Time Objective) | 4 hours | 2 hours |
| MTTR (Mean Time to Recovery) | 2 hours | 1 hour |

## Backup Strategy

### Database Backups

```yaml
# configs/backup.yaml
database:
  type: postgresql
  schedule:
    full: "0 2 * * *"      # Daily at 2 AM
    incremental: "0 * * * *"  # Hourly
    wal_archive: continuous

  retention:
    full: 30d
    incremental: 7d
    wal: 24h

  storage:
    primary: s3://agentaskit-backups-primary
    replica: s3://agentaskit-backups-replica
    encryption: AES-256-GCM
```

### Application State

| Component | Backup Method | Frequency | Retention |
|-----------|---------------|-----------|-----------|
| PostgreSQL | pg_dump + WAL | Continuous | 30 days |
| Redis | RDB + AOF | Hourly | 7 days |
| S3 Objects | Cross-region replication | Real-time | 90 days |
| Configs | Git + S3 | On change | 365 days |
| Secrets | Vault snapshots | Daily | 30 days |

### Backup Verification

```bash
# Weekly backup verification
./scripts/dr/verify-backup.sh

# Monthly restore test
./scripts/dr/test-restore.sh --target=staging
```

## Multi-Region Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Primary Region                           │
│                        (us-west-2)                               │
├─────────────────────────────────────────────────────────────────┤
│  ┌───────────┐    ┌───────────┐    ┌───────────┐               │
│  │    K8s    │    │  Database │    │    S3     │               │
│  │  Cluster  │    │  Primary  │    │  Primary  │               │
│  └─────┬─────┘    └─────┬─────┘    └─────┬─────┘               │
│        │                │                │                      │
└────────┼────────────────┼────────────────┼──────────────────────┘
         │                │                │
         │    Replication │                │ Cross-region
         │                │                │ replication
         │                │                │
┌────────┼────────────────┼────────────────┼──────────────────────┐
│        │                │                │                      │
│  ┌─────▼─────┐    ┌─────▼─────┐    ┌─────▼─────┐               │
│  │    K8s    │    │  Database │    │    S3     │               │
│  │  Standby  │    │  Replica  │    │  Replica  │               │
│  └───────────┘    └───────────┘    └───────────┘               │
│                                                                 │
│                       Secondary Region                          │
│                        (us-east-1)                              │
└─────────────────────────────────────────────────────────────────┘
```

## Failover Procedures

### Automated Failover

Failover triggers automatically when:
- Primary region health check fails for >5 minutes
- Database primary unreachable for >2 minutes
- Network partition detected

### Manual Failover

```bash
# 1. Verify primary is truly down
./scripts/dr/check-primary.sh

# 2. Promote secondary database
kubectl exec -n agentaskit-production pg-replica-0 -- \
  pg_ctl promote

# 3. Update DNS to secondary
./scripts/dr/update-dns.sh --target=secondary

# 4. Scale up secondary cluster
kubectl scale deployment --all --replicas=+50% -n agentaskit-production

# 5. Verify services
./scripts/dr/verify-failover.sh
```

### Failback Procedure

```bash
# 1. Verify primary is recovered
./scripts/dr/check-primary.sh --recovered

# 2. Sync data to primary
./scripts/dr/sync-primary.sh

# 3. Verify data integrity
./scripts/dr/verify-data-integrity.sh

# 4. Gradual traffic shift
./scripts/dr/shift-traffic.sh --target=primary --percent=10
./scripts/dr/shift-traffic.sh --target=primary --percent=50
./scripts/dr/shift-traffic.sh --target=primary --percent=100

# 5. Restore secondary to standby
./scripts/dr/restore-standby.sh
```

## Testing Schedule

| Test Type | Frequency | Last Test | Next Test |
|-----------|-----------|-----------|-----------|
| Backup verification | Weekly | 2025-10-01 | 2025-10-08 |
| Restore test | Monthly | 2025-09-15 | 2025-10-15 |
| Failover drill | Quarterly | 2025-07-15 | 2025-10-15 |
| Full DR exercise | Annually | 2025-03-01 | 2026-03-01 |

## Runbook Checklist

### Pre-Failover

- [ ] Confirm primary failure is not transient
- [ ] Notify stakeholders
- [ ] Create incident ticket
- [ ] Verify secondary health

### During Failover

- [ ] Execute failover procedure
- [ ] Monitor replication lag
- [ ] Verify service health
- [ ] Update status page

### Post-Failover

- [ ] Conduct PIR
- [ ] Update documentation
- [ ] Plan failback
- [ ] Review automation

## Evidence

- Backup logs: `TEST/dr/*.log`
- Failover tests: `TEST/dr/failover/`
- DR drills: `operational_audit/dr_drills/`

## Related

- [OPS-RUNBOOK](./runbooks/) - Service runbooks
- [OPS-INCIDENTS](./incident_management.md) - Incident management
- [PERF-CAPACITY](./capacity_plan.md) - Capacity planning
