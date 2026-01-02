# AgentAskit API Documentation

**REF:** DOC-API
**Owner:** @docs
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document provides comprehensive API documentation for AgentAskit production services.

## API Versioning

All APIs follow semantic versioning and are prefixed with `/api/v{major}`.

| Version | Status | Sunset Date |
|---------|--------|-------------|
| v1 | Current | - |
| v0 | Deprecated | 2025-12-31 |

## Base URLs

| Environment | Base URL |
|-------------|----------|
| Production | `https://api.agentaskit.io` |
| Staging | `https://api.staging.agentaskit.io` |
| Development | `http://localhost:8080` |

## Authentication

All API requests require authentication via Bearer token:

```http
Authorization: Bearer <token>
```

Tokens are obtained via the `/auth/token` endpoint or through OAuth2 flows.

## Core Endpoints

### Tasks API

#### Create Task

```http
POST /api/v1/tasks
Content-Type: application/json

{
  "name": "process-data",
  "priority": "high",
  "payload": {
    "source": "s3://bucket/data.json"
  },
  "timeout_seconds": 300
}
```

**Response:**

```json
{
  "id": "task-12345",
  "status": "pending",
  "created_at": "2025-10-05T12:00:00Z",
  "estimated_completion": "2025-10-05T12:05:00Z"
}
```

#### Get Task Status

```http
GET /api/v1/tasks/{task_id}
```

**Response:**

```json
{
  "id": "task-12345",
  "status": "completed",
  "result": { ... },
  "started_at": "2025-10-05T12:00:01Z",
  "completed_at": "2025-10-05T12:03:45Z"
}
```

### Agents API

#### List Agents

```http
GET /api/v1/agents?status=active&capability=nlp
```

**Response:**

```json
{
  "agents": [
    {
      "id": "agent-001",
      "name": "nlp-processor",
      "status": "active",
      "capabilities": ["nlp", "sentiment"],
      "health": { "status": "healthy", "last_check": "..." }
    }
  ],
  "total": 100,
  "page": 1,
  "page_size": 20
}
```

#### Execute Agent

```http
POST /api/v1/agents/{agent_id}/execute
Content-Type: application/json

{
  "action": "process",
  "input": { ... },
  "options": {
    "timeout": 60,
    "retry_count": 3
  }
}
```

### Workflows API

#### Trigger Workflow

```http
POST /api/v1/workflows/trigger
Content-Type: application/json

{
  "workflow_id": "wf-001",
  "phase": 1,
  "inputs": { ... }
}
```

#### Get Workflow Status

```http
GET /api/v1/workflows/{workflow_id}/status
```

### Health API

#### Liveness

```http
GET /api/v1/health/live
```

#### Readiness

```http
GET /api/v1/health/ready
```

## Error Responses

All errors follow a consistent format:

```json
{
  "error": {
    "code": "TASK_NOT_FOUND",
    "message": "Task with ID 'task-12345' not found",
    "request_id": "req-67890",
    "timestamp": "2025-10-05T12:00:00Z"
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `AUTHENTICATION_REQUIRED` | 401 | Missing or invalid token |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |

## Rate Limits

See [Rate Limits Configuration](../../configs/rate_limits.yaml) for details.

| Tier | Requests/Second | Burst |
|------|-----------------|-------|
| Default | 1000 | 1500 |
| Premium | 5000 | 7500 |
| Enterprise | 10000 | 15000 |

## Pagination

List endpoints support pagination:

```http
GET /api/v1/tasks?page=2&page_size=50
```

Response includes:

```json
{
  "data": [...],
  "pagination": {
    "page": 2,
    "page_size": 50,
    "total": 1234,
    "total_pages": 25
  }
}
```

## SDK References

- [Rust SDK](./rust-sdk.md)
- [Python SDK](./python-sdk.md)
- [TypeScript SDK](./typescript-sdk.md)

## Evidence

- OpenAPI Spec: `core/api/openapi.yaml`
- Schema Validation: `tests/api/`
- Example Requests: `docs/api/examples/`

## Related

- [Authentication Guide](./authentication.md)
- [Error Handling](./errors.md)
- [Rate Limiting](../../configs/rate_limits.yaml)
