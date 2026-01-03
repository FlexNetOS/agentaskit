# Changelog

All notable changes to AgentAskit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- CodeQL/SCA security scanning workflow (SEC-CI)
- GitHub Issues sync from .todo file (OPS-ISSUE-SYNC)
- CI build and test pipeline (CI-BUILD)
- Release pipeline with verification gates (CD-GATES)
- Container build and registry workflow (CONTAINER-BUILD)
- Kubernetes manifests and Helm charts (DEP-K8S)

### Changed
- Enhanced todo validation workflow
- Improved documentation structure

### Security
- Added cargo-audit for Rust dependency scanning
- Added Trivy for container vulnerability scanning
- Implemented cosign for container image signing

## [0.1.0] - 2025-10-05

### Added
- Initial AgentAskit production framework
- 7-phase workflow system implementation
- 928-agent orchestration framework
- Triple verification protocol (Model A/B/C + Model D merge)
- Performance optimization system targeting 10K+ tasks/sec
- Security enhancement suite with capability tokens
- Comprehensive documentation with evidence trails
- SLO/SLA monitoring and alerting
- SBOM generation and artifact signing
- Pre-push hooks for SoT/HASHES validation

### Infrastructure
- Unified agent hierarchy (6 layers, 928 agents)
- Tri-sandbox execution environment
- Real-time health monitoring
- Centralized logging and distributed tracing

### Documentation
- Architecture documentation with C4 diagrams
- API documentation with schema references
- Runbooks for operational procedures
- ADRs for architectural decisions

[Unreleased]: https://github.com/FlexNetOS/agentaskit/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/FlexNetOS/agentaskit/releases/tag/v0.1.0
