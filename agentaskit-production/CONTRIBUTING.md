# Contributing to AgentAskit Production

Welcome to AgentAskit Production! This document describes our contribution standards, code review process, and governance practices.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Source of Truth (SoT) System](#source-of-truth-sot-system)
3. [TODO-Driven Development](#todo-driven-development)
4. [Code Review Standards](#code-review-standards)
5. [Quality Gates & CI/CD](#quality-gates--cicd)
6. [Evidence & Verification](#evidence--verification)
7. [Development Workflow](#development-workflow)
8. [Standards & Best Practices](#standards--best-practices)

---

## Quick Start

1. **Read the SoT**: Familiarize yourself with our [Source of Truth document](/core/src/orchestration/sot.md)
2. **Check the TODO**: Review [.todo](/.todo) for current tasks and priorities
3. **Set up pre-push hooks**: Run `./hooks/pre-push` to ensure quality gates
4. **Follow CODEOWNERS**: Changes will be routed to appropriate reviewers via [CODEOWNERS](/CODEOWNERS)

---

## Source of Truth (SoT) System

AgentAskit uses a **single source of truth** approach for governance and tracking:

### Primary SoT Location
- **File**: `/core/src/orchestration/sot.md`
- **Purpose**: Append-only ledger of executed tasks, decisions, and evidence
- **Format**: UTC timestamps, relative paths, SHA-256 hashes

### SoT Principles

1. **Append-Only**: Never modify existing entries; always append new ones
2. **Evidence-Based**: Every change must link to artifacts, transcripts, or hashes
3. **Traceable**: All decisions and risks are recorded with owners and context
4. **Reproducible**: Commands and procedures must be deterministic

### Updating the SoT

When completing a task:

```bash
# 1. Complete your work with deterministic outputs
cargo test --no-color > TEST/my-feature-$(date -u +%Y%m%d-%H%M).log

# 2. Update hash manifest
sha256sum path/to/artifact >> operational_hash/HASHES.txt

# 3. Append entry to SoT
# Edit: core/src/orchestration/sot.md
# Add under "1.1 Executed Tasks (Chronological)"
```

Example SoT entry:
```markdown
- [x] 2026-01-02 14:30 UTC — Implement rate limiting (PERF-RATE)
  - Artifacts:
    - configs/rate_limits.yaml
    - tests/performance/rate_limit/
  - Notes: Global and per-tenant limits implemented; backoff headers added
  - Hashes: operational_hash/HASHES.txt#rate-limits-v1
```

---

## TODO-Driven Development

Our primary task tracking is the `.todo` file at the repository root.

### TODO File Structure

Each task follows this format:
```
- [ ] [Priority] YYYY-MM-DD Task description [metrics] [+tags] [@context]
  [REF: TASK-ID] owner:@team deps:[DEP-1, DEP-2]
  acc:"Acceptance criteria"
  evid:"Evidence paths"
```

### Task Priorities
- `(A)` = Critical - Blockers or production issues
- `(B)` = High - Important features or improvements
- `(C)` = Medium - Nice-to-have or backlog items

### Working with TODO

1. **Find a task**: Review `.todo` for tasks owned by your team
2. **Check dependencies**: Ensure `deps:[]` tasks are completed
3. **Mark in-progress**: Update status when starting work
4. **Complete task**: Mark as done `[x]` and update SoT
5. **CI Sync**: GitHub Issues auto-sync via workflow

### TODO Validation

Pull requests automatically validate TODO entries:
- Required fields: `owner`, `deps`, `acc`, `evid`, `[REF: XXX]`
- CI workflow: `.github/workflows/todo-validate.yml`
- Failures block merge

---

## Code Review Standards

### Review Process

1. **Automatic Routing**: CODEOWNERS file routes PRs to appropriate teams
2. **Multiple Reviewers**: Critical files require approval from multiple teams
3. **Required Checks**: All CI checks must pass before merge
4. **Evidence Required**: Link to test results, benchmarks, or security scans

### Review Checklist

- [ ] **Code Quality**: Follows Rust conventions and project style
- [ ] **Tests**: Unit and integration tests included and passing
- [ ] **Performance**: No regressions (check benchmarks if applicable)
- [ ] **Security**: CodeQL and SCA scans pass with no critical/high issues
- [ ] **Documentation**: Code comments, README updates, or ADR as needed
- [ ] **Evidence**: SoT updated with artifacts and hashes
- [ ] **Dependencies**: Cargo.lock updated if dependencies changed
- [ ] **TODO**: Tasks marked complete, new tasks added if discovered

### Team Responsibilities

Based on CODEOWNERS, these teams review specific areas:

- **@program**: Governance, standards, TODO, SoT, architecture decisions
- **@perf-oncall**: Performance optimizations, benchmarks, rate limiting
- **@sec-oncall**: Security policies, AuthZ, secrets, compliance
- **@sre**: SLOs, SLAs, operational runbooks, incident management
- **@observability**: Metrics, logging, tracing, dashboards, alerts
- **@orchestration**: Workflows, 7-phase system, agent coordination
- **@platform**: CI/CD, deployments, infrastructure, containers
- **@verification-lead**: Triple verification, Model-D integration, QA gates
- **@qa**: Integration tests, test coverage, quality assurance
- **@docs**: Documentation, API specs, architecture diagrams
- **@ml**: AI/ML components, SOP parsing, model selection
- **@devops**: Tooling, automation, deliverable management
- **@process**: Methodologies, 4D framework, process improvements

---

## Quality Gates & CI/CD

### Pre-Commit Checks

Local validation before push:
```bash
# Run via git hook
./hooks/pre-push

# Manual validation
./scripts/validate_sot.sh
cargo test
cargo clippy -- -D warnings
```

### CI Pipeline

`.github/workflows/ci.yml` runs on every PR:

1. **Build**: Matrix build across platforms
2. **Tests**: Unit, integration, and performance tests
3. **Security**: CodeQL and dependency scanning
4. **Linting**: TODO schema validation
5. **Artifacts**: Archive test results and build outputs

### Release Gates

Releases require passing:
- ✅ All CI checks green
- ✅ Triple verification (Gates A/B/C + Model-D acceptance ≥99.99%)
- ✅ Canary deployment metrics pass
- ✅ Security scans: 0 critical/high vulnerabilities
- ✅ Performance: No SLO breaches
- ✅ SBOM generated and signed
- ✅ Provenance attestations attached

### SLO Enforcement

- **SLO Policy**: Defined in `slo/policies.yaml`
- **Burn Rate**: CI workflow `.github/workflows/slo-check.yml` validates
- **Error Budget**: Weekly burn-rate must be <1.0
- **Alerts**: Multi-window burn-rate triggers auto-rollback

---

## Evidence & Verification

### Evidence Requirements

All changes must include:

1. **Test Transcripts**: Stored in `TEST/*.log` with UTC timestamps
2. **SHA-256 Hashes**: Recorded in `operational_hash/HASHES.txt`
3. **Artifact Manifests**: JSON manifests for complex deliverables
4. **Reproducible Commands**: Documented in evidence files or SoT

### Hash Manifest Format

```
# operational_hash/HASHES.txt
<sha256sum>  <relative/path/to/artifact>  # <description>
```

### Triple Verification

For stability-sensitive changes:

1. **Gate A**: Sandbox environment testing
2. **Gate B**: Integration with existing systems
3. **Gate C**: Performance and security validation
4. **Model-D Merge**: Final acceptance threshold ≥99.99%

Failures at any gate block release.

---

## Development Workflow

### 1. Start a Task

```bash
# Create feature branch
git checkout -b feature/TASK-ID-description

# Review task requirements
grep "REF: TASK-ID" .todo

# Check dependencies
# Ensure all deps:[] tasks are completed
```

### 2. Develop & Test

```bash
# Write code following Rust conventions
cargo fmt
cargo clippy

# Run tests
cargo test
cargo test --release  # for performance-sensitive code

# Benchmarks (if applicable)
cargo bench
```

### 3. Create Evidence

```bash
# Run tests with transcript
cargo test --no-color > TEST/task-id-$(date -u +%Y%m%d-%H%M).log

# Generate hashes
sha256sum path/to/artifact >> operational_hash/HASHES.txt

# Update SoT
# Edit: core/src/orchestration/sot.md
```

### 4. Commit & Push

```bash
# Stage changes
git add .

# Commit with reference
git commit -m "feat: Implement TASK-ID description

refs: TASK-ID, SoT entry 2026-01-02 14:30 UTC
evidence: TEST/task-id-20260102-1430.log"

# Pre-push validation runs automatically
git push -u origin feature/TASK-ID-description
```

### 5. Create Pull Request

```bash
# Use GitHub CLI
gh pr create --title "TASK-ID: Description" --body "
## Summary
- Implements TASK-ID
- Acceptance: <acceptance criteria met>

## Evidence
- Tests: TEST/task-id-*.log
- Hashes: operational_hash/HASHES.txt#task-id
- SoT: core/src/orchestration/sot.md

## Test Plan
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Performance benchmarks within targets
- [ ] Security scans clean
"
```

### 6. Address Review Feedback

```bash
# Make changes based on feedback
git add .
git commit -m "fix: Address review feedback for TASK-ID"
git push

# Re-request review after addressing comments
```

### 7. Merge & Update

After approval and CI passes:
```bash
# Merge via GitHub (squash & merge recommended)
# CI automatically syncs TODO with GitHub Issues

# Update TODO manually if needed
# Mark task as [x] complete
# Add any new tasks discovered during implementation
```

---

## Standards & Best Practices

### Code Style

- **Rust**: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Formatting**: Use `cargo fmt` (rustfmt)
- **Linting**: Pass `cargo clippy -- -D warnings`
- **Documentation**: Doc comments for public APIs

### Testing

- **Coverage**: Aim for ≥80% code coverage
- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test component interactions
- **Performance Tests**: Benchmark performance-critical paths
- **Security Tests**: Validate AuthZ, input validation, secrets handling

### Performance

- **Targets**: ≥10k tasks/s, ≥100k msgs/s sustained for 30min
- **Latency**: p99 pipeline ≤50ms, p95 response ≤50ms
- **Error Rate**: ≤0.1% error rate
- **Benchmarks**: Must not regress by >5%

### Security

- **Secrets**: Never commit secrets; use Vault/KMS
- **Dependencies**: Keep Cargo.lock updated; scan with cargo-audit
- **CodeQL**: Zero critical/high findings
- **AuthZ**: Follow capability token schema
- **Rotation**: Tokens rotate ≤24h

### Documentation

- **Code Comments**: Explain "why" not "what"
- **ADRs**: Record architectural decisions in `docs/decisions/adr/`
- **Runbooks**: Operational procedures in `docs/runbooks/`
- **API Docs**: Schemas in `docs/api/`

### Dependencies

- **Cargo.toml**: Document why each dependency is needed
- **Version Pinning**: Pin exact versions for reproducibility
- **SBOM**: Auto-generated via `.github/workflows/sbom.yml`
- **Vulnerability Scanning**: Automated in CI

### Versioning

- **Semantic Versioning**: MAJOR.MINOR.PATCH
- **Changelog**: Auto-generated in `CHANGELOG.md`
- **Tags**: Signed git tags for releases
- **Provenance**: SLSA attestations attached to releases

---

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Issues**: File via GitHub Issues (auto-synced with TODO)
- **Urgent**: Page on-call via documented rotation
- **Documentation**: Start with SoT → TODO → this CONTRIBUTING guide

---

## References

- **Source of Truth**: [/core/src/orchestration/sot.md](/core/src/orchestration/sot.md)
- **TODO List**: [/.todo](/.todo)
- **CODEOWNERS**: [/CODEOWNERS](/CODEOWNERS)
- **Architecture Docs**: [/docs/](/docs/)
- **Runbooks**: [/docs/runbooks/](/docs/runbooks/)
- **CI Workflows**: [/.github/workflows/](/.github/workflows/)

---

**Last Updated**: 2026-01-02
**Owner**: @program
**Reference**: GOV-CODEOWNERS (REF: GOV-CODEOWNERS)
