# TEST Directory - Evidence & Verification Logs

## Overview

This directory contains test logs, verification transcripts, and evidence artifacts that demonstrate compliance with production readiness criteria. The TEST/ directory serves as a central repository for operational evidence referenced by tasks in the `.todo` file.

## Directory Structure

```
TEST/
├── README.md                          # This file
├── docs/                              # Documentation evidence
│   └── evidence_trail_example.md      # Example evidence trail for DOC-001
├── integration/                       # Integration test transcripts
│   └── doc001_completion_transcript.log # DOC-001 completion verification
├── perf/                              # Performance test logs
│   ├── sample.log                     # General performance logs
│   ├── overload/                      # Overload testing
│   │   └── sample.log
│   └── rate/                          # Rate limiting tests
│       └── sample.log
├── slo/                               # SLO verification logs
│   └── sample.log                     # SLO compliance logs
└── verification/                      # Hash & integrity verification
    └── hash_verification.log          # SHA-256 manifest verification
```

## Purpose by Directory

### docs/
Evidence documentation and examples of evidence trails. Shows how tasks link to artifacts and how compliance is demonstrated.

**Referenced by**: DOC-001, GOV-SOT-EXEC, EVID-HASH

### integration/
End-to-end integration test transcripts demonstrating that all components work together correctly.

**Referenced by**: WORKFLOW-005, COMPREHENSIVE-7PHASE-001, DOC-001

### perf/
Performance test results, benchmarks, and load testing logs.

**Subdirectories**:
- `overload/`: Overload and stress testing (PERF-QUOTAS)
- `rate/`: Rate limiting validation (PERF-RATE)

**Referenced by**: PERF-001, SLA-001, PERF-RATE, PERF-BACKPRESSURE, PERF-QUOTAS

### slo/
Service Level Objective (SLO) verification logs showing compliance with targets.

**Referenced by**: SLA-001, SLO-POLICY, OBS-DASH-ALERTS

### verification/
Cryptographic verification logs, hash checks, and integrity validation.

**Referenced by**: DOC-001, EVID-HASH, SUPPLY-VERIFY

## Usage

### Linking from Tasks
In the `.todo` file, tasks reference evidence using the `evid:` field:

```
evid:"docs/, operational_hash/HASHES.txt, TEST/*"
```

This creates a bidirectional link:
- **Task → Evidence**: Where to find proof of completion
- **Evidence → Task**: What requirement this evidence satisfies

### Adding New Logs
When adding test logs or verification artifacts:

1. Choose the appropriate subdirectory
2. Use descriptive filenames with timestamps
3. Include task reference (REF) in the log header
4. Update the corresponding task's `evid:` field
5. Link from documentation if applicable

### Verification
To verify evidence completeness:

```bash
# Check all evidence locations for DOC-001
cd /home/user/agentaskit/agentaskit-production
ls -l docs/ operational_hash/HASHES.txt TEST/

# Verify specific test category
ls -R TEST/verification/

# Search for task references in logs
grep -r "DOC-001" TEST/
```

## Log Format Standards

### Header Template
All log files should include:

```
# ═══════════════════════════════════════════════════════════════
# <Log Title>
# ═══════════════════════════════════════════════════════════════
# Task: <REF-ID> (<Task Description>)
# Date: YYYY-MM-DD
# Purpose: <Brief description>
# ═══════════════════════════════════════════════════════════════
```

### Timestamp Format
Use ISO 8601 with millisecond precision:
```
[2026-01-02 00:00:00.123] LEVEL: Message
```

### Result Indicators
- `✓ PASS` - Check passed
- `✗ FAIL` - Check failed
- `⚠ WARN` - Warning condition
- `ℹ INFO` - Informational message

## Task Evidence Map

| Task | Evidence Location | Type |
|------|------------------|------|
| DOC-001 | TEST/verification/, TEST/docs/ | Hash verification, evidence trails |
| EVID-HASH | TEST/verification/ | Hash manifests, integrity checks |
| PERF-001 | TEST/perf/ | Performance benchmarks |
| PERF-RATE | TEST/perf/rate/ | Rate limiting tests |
| PERF-QUOTAS | TEST/perf/overload/ | Overload/quota tests |
| SLA-001 | TEST/slo/ | SLO compliance logs |
| WORKFLOW-005 | TEST/integration/ | Integration test results |
| SUPPLY-VERIFY | TEST/verification/ | Signature verification logs |

## Retention Policy

- **Active development**: All logs retained
- **Post-release**: Critical logs archived, debug logs pruned
- **Compliance logs**: Retained per AUDIT-RETENTION requirements (≥30 days)
- **Performance logs**: Retained for trend analysis (90 days)

## Related Documentation

- [operational_hash/README.md](../operational_hash/README.md) - Hash manifest system
- [docs/PRODUCTION_READINESS_CERTIFICATION.md](../docs/PRODUCTION_READINESS_CERTIFICATION.md) - Production readiness criteria
- [.todo](../.todo) - Task list with evidence references

## Adding New Test Categories

To add a new test category:

1. Create subdirectory: `mkdir TEST/<category>/`
2. Add README in subdirectory explaining the category
3. Update this README's directory structure section
4. Update the task evidence map table
5. Reference in relevant `.todo` task `evid:` fields

## Support

For questions about test logs or evidence:
- Task owner: See specific task in `.todo`
- QA owner: @qa
- Documentation owner: @docs

---
*Last updated: 2026-01-02*
*Related task: DOC-001 (Evidence trails & SHA-256 manifests)*
