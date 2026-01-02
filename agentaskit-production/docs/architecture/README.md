# AgentAskit Architecture Documentation

**REF:** DOC-DIAGRAMS
**Owner:** @docs
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This document provides C4-level architecture diagrams and system design documentation for AgentAskit.

## System Context (C4 Level 1)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              External Systems                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐              │
│  │   Users  │    │ External │    │  Cloud   │    │ Monitoring│              │
│  │  (API)   │    │ Services │    │ Storage  │    │  Systems  │              │
│  └────┬─────┘    └────┬─────┘    └────┬─────┘    └────┬─────┘              │
│       │               │               │               │                     │
│       └───────────────┴───────┬───────┴───────────────┘                     │
│                               │                                             │
│                               ▼                                             │
│                    ┌──────────────────────┐                                 │
│                    │                      │                                 │
│                    │     AgentAskit       │                                 │
│                    │   Production System  │                                 │
│                    │                      │                                 │
│                    └──────────────────────┘                                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Container Diagram (C4 Level 2)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           AgentAskit System                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐                                                           │
│  │ API Gateway │◄────────── External Requests                               │
│  │   (Rust)    │                                                           │
│  └──────┬──────┘                                                           │
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                     │
│  │Orchestrator │───▶│Agent Manager│───▶│ Worker Pool │                     │
│  │   (Rust)    │    │   (Rust)    │    │   (Rust)    │                     │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘                     │
│         │                  │                  │                             │
│         ▼                  ▼                  ▼                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                     │
│  │Message Queue│    │   928-Agent │    │   Task      │                     │
│  │  (NATS/Kafka)│    │   Registry  │    │  Executor   │                     │
│  └──────┬──────┘    └─────────────┘    └──────┬──────┘                     │
│         │                                     │                             │
│         ▼                                     ▼                             │
│  ┌─────────────────────────────────────────────────────┐                   │
│  │              Persistence Layer                       │                   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐             │                   │
│  │  │PostgreSQL│  │  Redis  │  │   S3    │             │                   │
│  │  └─────────┘  └─────────┘  └─────────┘             │                   │
│  └─────────────────────────────────────────────────────┘                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Component Diagram (C4 Level 3)

### Orchestrator Components

```
┌─────────────────────────────────────────────────────────────────┐
│                        Orchestrator                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │ 7-Phase      │    │ Workflow     │    │ Task         │      │
│  │ Controller   │───▶│ Engine       │───▶│ Scheduler    │      │
│  └──────────────┘    └──────────────┘    └──────────────┘      │
│         │                   │                   │               │
│         ▼                   ▼                   ▼               │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │ SOP          │    │ Triple       │    │ Backpressure │      │
│  │ Parser       │    │ Verification │    │ Controller   │      │
│  └──────────────┘    └──────────────┘    └──────────────┘      │
│         │                   │                   │               │
│         └───────────────────┴───────────────────┘               │
│                             │                                   │
│                             ▼                                   │
│                    ┌──────────────┐                            │
│                    │ Event Bus    │                            │
│                    └──────────────┘                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Data Flow Diagram

```
                              Request Flow
                              ============

  Client                API Gateway           Orchestrator          Agents
    │                       │                     │                   │
    │──── HTTP Request ────▶│                     │                   │
    │                       │                     │                   │
    │                       │── Validate Token ──▶│                   │
    │                       │                     │                   │
    │                       │◀── Token Valid ────│                   │
    │                       │                     │                   │
    │                       │── Route Request ──▶│                   │
    │                       │                     │                   │
    │                       │                     │── Match Agent ──▶│
    │                       │                     │                   │
    │                       │                     │◀── Execute ──────│
    │                       │                     │                   │
    │                       │◀── Response ───────│                   │
    │                       │                     │                   │
    │◀── HTTP Response ────│                     │                   │
    │                       │                     │                   │
```

## 7-Phase Workflow Architecture

```
Phase 1          Phase 2          Phase 3          Phase 4
┌─────────┐      ┌─────────┐      ┌─────────┐      ┌─────────┐
│ Define  │─────▶│ Develop │─────▶│ Deliver │─────▶│ Decide  │
│         │      │         │      │         │      │         │
└─────────┘      └─────────┘      └─────────┘      └─────────┘
                                                        │
                                                        ▼
Phase 7          Phase 6          Phase 5
┌─────────┐      ┌─────────┐      ┌─────────┐
│ Deploy  │◀─────│ Docs    │◀─────│ Verify  │
│         │      │         │      │         │
└─────────┘      └─────────┘      └─────────┘
```

## ADR Index

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-0001](./adr/0001-rust-core.md) | Rust as Core Language | Accepted |
| [ADR-0002](./adr/0002-928-agents.md) | 928-Agent Architecture | Accepted |
| [ADR-0003](./adr/0003-7-phase.md) | 7-Phase Workflow | Accepted |
| [ADR-0004](./adr/0004-triple-verify.md) | Triple Verification | Accepted |

## Diagram Generation

Diagrams can be regenerated using:

```bash
# Generate all diagrams
./scripts/docs/generate-diagrams.sh

# Generate specific diagram
./scripts/docs/generate-diagrams.sh --type c4 --output docs/architecture/
```

## Evidence

- Source diagrams: `docs/architecture/diagrams/`
- ADRs: `docs/decisions/adr/`
- Generated outputs: `docs/architecture/generated/`

## Related

- [DOC-001](../../.todo) - Documentation evidence
- [WF-001](../../.todo) - 7-phase workflow
- [VER-001](../../.todo) - Triple verification
