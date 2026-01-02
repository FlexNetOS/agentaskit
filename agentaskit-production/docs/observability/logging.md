# Centralized Logging Pipeline

**REF:** OBS-LOGGING
**Owner:** @platform
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document defines the centralized logging pipeline for AgentAskit production, ensuring structured logs, proper retention, and efficient querying.

## Log Architecture

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Services   │───▶│ Log Shipper │───▶│    OTLP     │───▶│  Log Store  │
│  (stdout)   │    │ (Fluent Bit)│    │  Collector  │    │ (Loki/ELK)  │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                             │
                                             ▼
                                      ┌─────────────┐
                                      │  Dashboards │
                                      │  (Grafana)  │
                                      └─────────────┘
```

## Structured Logging Format

### JSON Log Schema

```json
{
  "timestamp": "2025-10-05T12:34:56.789Z",
  "level": "info",
  "message": "Request processed successfully",
  "service": "api-gateway",
  "version": "1.2.3",
  "trace_id": "abc123def456",
  "span_id": "789ghi",
  "request_id": "req-12345",
  "duration_ms": 45,
  "http": {
    "method": "POST",
    "path": "/api/v1/tasks",
    "status_code": 200
  },
  "user": {
    "id": "user-123",
    "tenant_id": "tenant-456"
  },
  "labels": {
    "environment": "production",
    "region": "us-west-2"
  }
}
```

### Log Levels

| Level | Usage | Retention |
|-------|-------|-----------|
| `error` | Errors requiring attention | 90 days |
| `warn` | Potential issues | 30 days |
| `info` | Normal operations | 14 days |
| `debug` | Detailed debugging | 3 days |
| `trace` | Verbose tracing | 1 day |

## Retention Policy

### By Log Type

| Log Type | Hot Storage | Warm Storage | Archive | Total |
|----------|-------------|--------------|---------|-------|
| Application | 7 days | 23 days | 335 days | 1 year |
| Security/Audit | 30 days | 60 days | 275 days | 1 year |
| Access Logs | 7 days | 23 days | None | 30 days |
| Debug Logs | 3 days | None | None | 3 days |

### Storage Tiers

```yaml
# configs/logging.yaml
retention:
  hot:
    duration: 7d
    storage_class: ssd

  warm:
    duration: 23d
    storage_class: standard

  archive:
    duration: 335d
    storage_class: glacier
    compression: zstd
```

## Query Examples

### Common Queries

```logql
# Errors in last hour
{service="api-gateway"} |= "error" | json | level="error"

# Slow requests (>1s)
{service=~".+"} | json | duration_ms > 1000

# Requests by user
{service="api-gateway"} | json | user_id="user-123"

# Error budget burn
sum(rate({level="error"}[5m])) / sum(rate({level=~".+"}[5m]))
```

### Performance Queries

```logql
# P99 latency by service
quantile_over_time(0.99,
  {service=~".+"} | json | unwrap duration_ms [5m]
) by (service)

# Request rate by endpoint
sum(rate({service="api-gateway"} | json [1m])) by (http_path)
```

## Fluent Bit Configuration

```yaml
# configs/fluent-bit.yaml
[SERVICE]
    Flush         1
    Log_Level     info
    Parsers_File  parsers.conf

[INPUT]
    Name              tail
    Path              /var/log/containers/*.log
    Parser            docker
    Tag               kube.*
    Refresh_Interval  5
    Mem_Buf_Limit     50MB
    Skip_Long_Lines   On

[FILTER]
    Name          kubernetes
    Match         kube.*
    Merge_Log     On
    Keep_Log      Off
    K8S-Logging.Parser On

[OUTPUT]
    Name          opentelemetry
    Match         *
    Host          otel-collector.observability
    Port          4318
    Logs_uri      /v1/logs
    tls           on
    tls.verify    off
```

## Error Budget Metrics

Logs emit the following metrics for SLO tracking:

| Metric | Description |
|--------|-------------|
| `log_messages_total` | Total log messages by level |
| `log_errors_total` | Error count for burn rate |
| `log_latency_seconds` | Log processing latency |
| `log_bytes_total` | Log volume by service |

## Alerting Integration

```yaml
# alerts/logging.yaml
alerts:
  - name: high_error_rate
    expr: |
      sum(rate(log_messages_total{level="error"}[5m]))
      / sum(rate(log_messages_total[5m])) > 0.01
    severity: warning

  - name: log_pipeline_lag
    expr: log_shipper_lag_seconds > 60
    severity: critical
```

## Evidence

- Configuration: `configs/logging.yaml`
- Fluent Bit config: `configs/fluent-bit.yaml`
- Query examples: `docs/observability/queries/`
- Dashboards: `dashboards/logging/*.json`

## Related

- [OBS-001](../../.todo) - Observability system
- [OBS-TRACING](../../configs/tracing.yaml) - Distributed tracing
- [SLO-POLICY](../../slo/policies.yaml) - SLO policies
