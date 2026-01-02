# Capacity Plan & Scaling Policy

**REF:** PERF-CAPACITY
**Owner:** @sre
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the capacity planning and autoscaling policies for AgentAskit production, ensuring ≥30% headroom and 90-day forecasting.

## Current Capacity

### Production Environment

| Component | Current Capacity | Peak Usage | Headroom |
|-----------|-----------------|------------|----------|
| API Gateway | 20 replicas | 14 replicas | 30% |
| Orchestrator | 5 replicas | 3 replicas | 40% |
| Worker Pool | 100 replicas | 70 replicas | 30% |
| Agent Manager | 10 replicas | 7 replicas | 30% |
| Message Queue | 50K msg/s | 35K msg/s | 30% |
| Database Connections | 1000 | 700 | 30% |

### Resource Allocation

| Resource | Allocated | Current Usage | Available |
|----------|-----------|---------------|-----------|
| CPU | 200 cores | 140 cores | 60 cores |
| Memory | 400 Gi | 280 Gi | 120 Gi |
| Storage | 10 Ti | 6 Ti | 4 Ti |
| Network | 10 Gbps | 5 Gbps | 5 Gbps |

## Autoscaling Rules

### Horizontal Pod Autoscaling

```yaml
# HPA Configuration
api_gateway:
  min_replicas: 5
  max_replicas: 50
  target_cpu_utilization: 70%
  target_memory_utilization: 75%
  scale_up_stabilization: 0s
  scale_down_stabilization: 300s

worker_pool:
  min_replicas: 10
  max_replicas: 200
  target_cpu_utilization: 70%
  custom_metrics:
    - queue_depth_per_worker: 100
    - pending_tasks: 500
```

### Vertical Pod Autoscaling

```yaml
# VPA Configuration (recommendations only)
enabled: true
mode: "Off"  # Recommendation only, not auto-apply
update_policy:
  min_replicas: 2
```

### Cluster Autoscaling

```yaml
# Node pool configuration
node_pools:
  general:
    min_nodes: 10
    max_nodes: 100
    machine_type: n2-standard-8

  high_memory:
    min_nodes: 5
    max_nodes: 50
    machine_type: n2-highmem-16
```

## 90-Day Forecast

### Growth Projections

| Metric | Current | +30 Days | +60 Days | +90 Days |
|--------|---------|----------|----------|----------|
| Requests/day | 100M | 120M | 145M | 175M |
| Active agents | 500 | 600 | 720 | 860 |
| Storage (Ti) | 6 | 7.2 | 8.6 | 10.3 |
| Peak concurrent | 50K | 60K | 72K | 86K |

### Capacity Actions

| Timeline | Action | Resources Needed |
|----------|--------|-----------------|
| Now | Maintain current | - |
| +30 days | Increase worker pool | +20 replicas |
| +60 days | Add node pool capacity | +10 nodes |
| +90 days | Database scaling | Read replicas |

## Headroom Policy

### Minimum Headroom Requirements

- **CPU:** ≥30% available at peak
- **Memory:** ≥30% available at peak
- **Storage:** ≥40% available (slower to provision)
- **Network:** ≥50% available for burst

### Headroom Alerts

```yaml
# alerts/capacity.yaml
alerts:
  - name: low_cpu_headroom
    condition: cpu_available_percent < 30
    severity: warning
    action: notify_sre

  - name: critical_cpu_headroom
    condition: cpu_available_percent < 15
    severity: critical
    action: page_sre

  - name: low_memory_headroom
    condition: memory_available_percent < 30
    severity: warning
    action: notify_sre
```

## Scaling Runbook

### Manual Scale-Up Procedure

1. Verify capacity alerts in monitoring
2. Check current HPA status: `kubectl get hpa -n agentaskit-production`
3. If HPA maxed, increase max replicas
4. If nodes maxed, trigger cluster autoscaler
5. Update capacity tracking spreadsheet
6. Create incident ticket for review

### Emergency Scale-Up

```bash
# Emergency scale commands
kubectl scale deployment worker-pool --replicas=150 -n agentaskit-production
kubectl scale deployment api-gateway --replicas=30 -n agentaskit-production
```

## Cost Optimization

### Right-Sizing Recommendations

| Component | Current Size | Recommended | Savings |
|-----------|-------------|-------------|---------|
| API Gateway | n2-standard-4 | n2-standard-2 | 40% |
| Workers | n2-standard-8 | n2-highcpu-8 | 20% |

### Spot/Preemptible Usage

- Worker pool: 50% spot instances
- Batch jobs: 100% spot instances
- Critical services: 0% spot (on-demand only)

## Evidence

- Dashboards: `dashboards/capacity/*.json`
- Forecasts: `docs/ops/capacity_forecasts/`
- HPA configs: `deploy/k8s/hpa/`

## Related

- [PERF-001](../../.todo) - Performance optimization
- [PERF-QUOTAS](../../deploy/k8s/limits.yaml) - Resource quotas
- [OBS-DASH-ALERTS](../../dashboards/) - Monitoring dashboards
