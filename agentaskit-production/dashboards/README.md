# AgentAsKit Observability Dashboards

This directory contains Grafana dashboard definitions for monitoring the AgentAsKit multi-agent orchestration system.

## Dashboard Overview

### SLA Dashboard (`sla.json`)
Monitors Service Level Agreements and objectives including:
- **Availability**: 99.9% target with error budget tracking
- **Burn Rate**: Fast (1h) and slow (6h) windows
- **Latency**: P95 (50ms) and P99 (100ms) targets
- **Error Budget**: Remaining budget visualization

**Key Metrics:**
- `http_requests_total` - Total HTTP requests by status
- `http_request_duration_seconds` - Request latency histograms

### Performance Dashboard (`perf/throughput.json`)
Monitors system performance and throughput:
- **Task Throughput**: 10K+ tasks/sec target
- **Message Throughput**: 100K+ messages/sec target
- **Agent Startup Time**: <100ms P95 target
- **Response Time**: <50ms P95 target
- **Active Agents**: Distribution by tier

**Key Metrics:**
- `agentaskit_tasks_processed_total` - Tasks processed counter
- `agentaskit_inter_agent_messages_total` - Inter-agent messages
- `agentaskit_agent_startup_duration_seconds` - Agent startup latency
- `agentaskit_response_duration_seconds` - Response time histograms

### Capacity Dashboard (`capacity/overview.json`)
Monitors resource utilization and capacity planning:
- **Agent Capacity**: 928-agent orchestration pool
- **Queue Depth**: Task backlog by priority
- **Memory Usage**: Per-tier memory consumption
- **CPU Utilization**: System CPU usage
- **Sandbox Pool**: Tri-sandbox availability
- **Network Throughput**: Inter-agent communication

**Key Metrics:**
- `agentaskit_active_agents` - Currently active agents
- `agentaskit_task_queue_depth` - Tasks waiting in queue
- `agentaskit_memory_bytes` - Memory usage by component
- `agentaskit_sandbox_pool_*` - Sandbox pool metrics

## Golden Signals

All dashboards follow the Four Golden Signals methodology:

1. **Latency**: How long requests take (P95, P99 response times)
2. **Traffic**: Volume of requests (tasks/sec, messages/sec)
3. **Errors**: Rate of failing requests (availability, error budget)
4. **Saturation**: Resource utilization (queue depth, CPU, memory)

## Integration

These dashboards integrate with:
- **Prometheus**: Metrics collection and storage
- **Grafana**: Visualization and alerting
- **Alert Rules**: See `../alerts/` directory for alert definitions
- **SLO Policies**: See `../slo/policies.yaml` for targets

## Usage

1. Import dashboard JSON files into Grafana
2. Configure Prometheus data source
3. Ensure metrics are being exported from AgentAsKit
4. Set up alert rules from `../alerts/` directory
5. Monitor SLO compliance via `.github/workflows/slo-check.yml`

## Metrics Requirements

To use these dashboards, the AgentAsKit system must export the following metrics:

- Request/response metrics (HTTP status, duration)
- Task processing metrics (throughput, latency, queue depth)
- Agent lifecycle metrics (startup time, active count)
- Resource metrics (CPU, memory, network)
- Sandbox metrics (pool size, utilization)

## References

- [SLO Policies](../slo/policies.yaml)
- [Alert Rules](../alerts/)
- [CI SLO Check](../../.github/workflows/slo-check.yml)
