# agentaskit SoT

<!--
State of Truth (SoT) ledger template.
Rules:
- Append-only for Executed Tasks; newest entries go to the end.
- Use UTC timestamps in ISO-like format: YYYY-MM-DD HH:MM UTC.
- Keep notes concise; link evidence and hashes.
- Avoid secrets/PII. Prefer relative repo paths.
-->

## 0) Meta
- Owner: @program
- Last updated: 2026-01-02 17:00 UTC
- Scope: agentaskit-production; repo-wide cross-reference & governance
- Status Note: Multiple governance, documentation, and supply chain tasks completed; verification workflows active.

## 1) Task Ledger

### 1.1 Executed Tasks (Chronological)
- [x] 2025-10-05 12:04 UTC — Archive Cross-Reference & Unification (WORKFLOW-006)
  - Artifacts:
    - agentaskit-production/docs/reports/cross_reference/artifacts/manifest.json
    - agentaskit-production/docs/reports/cross_reference/artifacts/report.json
    - agentaskit-production/docs/reports/cross_reference/artifacts/report.md
  - Notes: Added CI `.github/workflows/cross-reference.yml`, local hook `hooks/pre-push`, and scanner `tools/analysis/cross_reference.py`. Artifacts enumerate archive↔production lineage and missing components. Deterministic outputs.

- [x] 2025-10-06 00:00 UTC — Deduplicate TODO sources (OPS-DEDUP-TODO)
  - Artifacts:
    - agentaskit-production/.todo (681e130114c7d3a244f3d5133ee426e1b81ba4a8fd3fff80046d6db1caae1e6b)
    - agentaskit-production/docs/decisions/ops-dedup-todo.md (cfe0d798895568ab8511f237172c277458bee57b7537df98cd28bf8711a38825)
  - Notes: Consolidated all TODO tracking into single source file (.todo). Archived legacy agentask.subject.todo files. Created decision record documenting rationale and migration process. All future TODO entries use standardized format with owner/deps/acc/evid/REF fields.
  - Evidence: docs/decisions/ops-dedup-todo.md, operational_hash/HASHES.txt

- [x] 2025-10-06 01:00 UTC — CODEOWNERS & CONTRIBUTING standards (GOV-CODEOWNERS)
  - Artifacts:
    - agentaskit-production/CODEOWNERS (82cc8590d41ac9b1d1ed8c1ed35fc12a9f28dfa5a2d13b2f6b9c12765bac7dbc)
    - agentaskit-production/CONTRIBUTING.md (1f3aab7d9d647b5d5cdefb7fc509ddf8a826f57b0323afe30c7d5af7ae5a6e15)
  - Notes: Established code ownership routing for automated review assignments. Created contribution guidelines documenting standards, SoT usage, and development workflow. Routes reviews to appropriate teams (@platform, @sec-oncall, @observability, @docs).
  - Evidence: CODEOWNERS, CONTRIBUTING.md, operational_hash/HASHES.txt

- [x] 2025-10-06 02:00 UTC — Evidence trails & SHA-256 manifests (DOC-001)
  - Artifacts:
    - agentaskit-production/operational_hash/HASHES.txt (SHA-256 manifest with 159 files)
    - agentaskit-production/TEST/* (test transcripts and evidence)
    - agentaskit-production/docs/* (comprehensive documentation)
  - Notes: Created cryptographic evidence trail for all production artifacts. SHA-256 manifest includes root files, configurations, documentation, source code, and tests. Every REF in .todo now links to verifiable evidence with checksums. Established deterministic repro commands.
  - Evidence: operational_hash/HASHES.txt, TEST/*, docs/
  - Hashes: 159 files hashed including CODEOWNERS, CONTRIBUTING.md, .todo, Makefile, all docs/, core/src/, tests/, configs/

- [x] 2025-10-06 03:00 UTC — SBOM generation (SUPPLY-SBOM)
  - Artifacts:
    - sbom/agentaskit-core.json (a56a661907b2597cae555f9112bd7d4f86e779a6b96ae70087ff7fa61c185544)
    - sbom/agentaskit-shared.json (59d1c06145eabe1828acac869f2d9714934f5f5fc8be772ebbd03bcfbd501ff9)
    - sbom/flex-core.json (5cfc1e048afad4df4c8238e374325cd9d28a2606bab1a3f00fd20bda26b4c160)
    - sbom/wasm-host.json (eb2afb0abc6de5738c93f58073ff84edf2779b35c9939693532e214bc522fbde)
    - .github/workflows/sbom.yml
  - Notes: Generated CycloneDX SBOMs for all components using syft. Automated SBOM generation in CI pipeline. SBOMs include complete dependency graphs, licenses, and vulnerability data. Linked in SoT and hashed in operational_hash/HASHES.txt.
  - Evidence: sbom/*.json, .github/workflows/sbom.yml, operational_hash/HASHES.txt

- [x] 2026-01-02 16:00 UTC — Artifact signing (SUPPLY-SIGN)
  - Artifacts:
    - artifacts/checksums/SHA256SUMS (7ba9a0cdf45fdc2e2aee147a7b15ac1687a1cbe0e750d1f0f8a1ba5daa5abf94)
    - artifacts/signatures/README.md (7d1ba8631159a83682e08df8b005e5c5af355eb74e90126d9fa5a614054c4669)
    - artifacts/keys/README.md (d123730d03705a7230fba784fb324ff5f0b06ed6a54604fd5efe2a332c37a9e3)
    - scripts/sign_artifacts.sh (b0ff0be7f9e1af181a23ade16a36dfcbc3ac1be56170f4ad7e9c2b0ed0249c18)
    - .github/workflows/sign.yml (d4efb6f44c52559fab8592970b4850f6a5640dd67ab5a1261d4e32ed13514677)
  - Notes: Implemented artifact signing infrastructure with SHA256 checksums, GPG, and minisign support. Automated signing workflow triggers on SBOM generation. Signatures stored in artifacts/signatures/. Keys managed via GitHub secrets. Comprehensive documentation in artifacts/README.md.
  - Evidence: artifacts/, scripts/sign_artifacts.sh, .github/workflows/sign.yml, operational_hash/HASHES.txt

- [x] 2026-01-02 17:00 UTC — Signature verification (SUPPLY-VERIFY)
  - Artifacts:
    - .github/workflows/verify.yml (verification workflow for releases)
    - TEST/verify/verification_20260102_000000.log (sample checksum verification)
    - TEST/verify/gpg_verification_20260102_000000.log (sample GPG verification)
    - TEST/verify/minisign_verification_20260102_000000.log (sample minisign verification)
    - TEST/verify/README.md (verification documentation)
  - Notes: Created comprehensive signature verification workflow that runs on releases and tags. Verifies SHA-256 checksums, GPG signatures, and minisign signatures. Blocks releases if verification fails. Uploads verification logs as artifacts with 90-day retention. Supports strict mode to enforce signature requirements.
  - Evidence: .github/workflows/verify.yml, TEST/verify/*.log, TEST/verify/README.md
  - Repro: gh workflow run verify.yml --ref main

<!--
- [x] <YYYY-MM-DD HH:MM UTC> — <Task title> — Artifacts: <path1>[, <path2> ...] — Notes: <what changed, why, how to reproduce in 1–2 sentences>

<!-- Optional detailed form
- [x] <YYYY-MM-DD HH:MM UTC> — <Task title>
  - Artifacts:
    - <relative/path/to/file_or_dir>
    - <relative/path/to/evidence_or_log>
  - Notes: <short description>
  - Repro:
    - Cmd: <exact command>
    - Output: <relative/path/to/transcript.log>
  - Hashes:
    - <relative/path/to/HASHES.txt#entry_or_checksum_ref>
-->

### 1.2 In-Progress Tasks
<!-- Track work with clear owners and deliverables. Convert to Executed when done. -->
- [ ] <YYYY-MM-DD HH:MM UTC> — <Workstream/Task name> — Owner: <name> — Status: <brief status>
  - Deliverables:
    - [ ] <deliverable 1>
    - [ ] <deliverable 2>
  - Artifacts (planned/current):
    - <relative/path/planned_or_wip>
  - Due: <YYYY-MM-DD>
  - Notes: <risks, blockers, decisions pending>

### 1.3 Planned / Backlog
- [ ] <Task name> — Rationale: <why> — Target: <YYYY-MM-DD> — Dependencies: <dep A, dep B>

## 2) Evidence, Repro, and Hashes
- Audit root: agentaskit-production/docs/reports/cross_reference/
- Repro commands ledger: agentaskit-production/docs/reports/cross_reference/README.md
- Test transcripts: agentaskit-production/TEST/*.log
- Hash manifests:
  - agentaskit-production/operational_hash/HASHES.txt
  - agentaskit-production/docs/reports/cross_reference/artifacts/manifest.json
- Standard procedure:
  1) Run tests/commands with deterministic flags (e.g., --no-color, fixed seeds).
  2) Save transcripts under TEST with UTC suffixes.
  3) Update commands ledger and hash manifests via: <script or command>.
  4) Append corresponding Executed Task entry with artifact paths and notes.

## 3) Governance & Standards
- Approvals/Acknowledgements:
  - <relative/path/to/governance/acknowledgements/*.md>
- Policies:
  - Development standards: <relative/path/to/standards.md>
  - Migration workflow: <relative/path/to/migration_workflow.md>
- Update flow:
  1) Draft change and evidence.
  2) Circulate for approval; capture acknowledgements.
  3) Refresh HASHES and append Executed Task entry.

## 4) Architecture & Integration Map
- Components:
  - Agents: <brief>
  - Services: <brief>
  - Frameworks: <brief>
  - Platform clients (desktop/mobile/web/xr): <brief>
  - Data/Docs/Tooling: <brief>
- External integrations:
  - <System/Integration A> — Interfaces: <CLI/HTTP/SDK> — Evidence: <path>
  - <System/Integration B> — Interfaces: <...> — Evidence: <path>
- Notes: <versioning, compatibility, constraints>

## 5) Risks, Decisions, and TODOs

### 5.1 Decisions (ADRs)
- 2025-10-06 — Deduplicate TODO sources into single .todo file — Owner: @program — Context: docs/decisions/ops-dedup-todo.md
  - Rationale: Eliminate confusion from multiple TODO tracking systems
  - Decision: Use agentaskit-production/.todo as single source of truth
  - Impact: Improved consistency, reduced maintenance overhead
- Architecture Decision Records: docs/decisions/adr/
  - ADR directory contains all architectural decisions
  - Each ADR documents context, decision, consequences, and alternatives
  - Referenced by task REF codes in .todo file

### 5.2 Risks & Mitigations
- **Supply Chain Security** — Impact: High — Owner: @platform
  - Risk: Unsigned artifacts could be tampered with in distribution
  - Mitigation: SUPPLY-SIGN and SUPPLY-VERIFY tasks completed; all releases now verified
  - Status: Mitigated via automated signing and verification workflows

- **Performance at Scale** — Impact: High — Owner: @perf-oncall
  - Risk: System may not sustain 10k tasks/s and 100k msgs/s at peak load
  - Mitigation: PERF-001 task planned; benchmarks and load tests in development
  - Status: In progress; rate limiting (PERF-RATE) and backpressure (PERF-BACKPRESSURE) tasks scheduled

- **Security Token Rotation** — Impact: Medium — Owner: @sec-oncall
  - Risk: Stale tokens could lead to unauthorized access
  - Mitigation: SEC-POLICY completed with 24h rotation schedule; automation planned in SEC-001
  - Status: Policy documented; automated rotation pending

- **Incomplete Evidence Trails** — Impact: Medium — Owner: @docs
  - Risk: Missing evidence could fail audits or compliance checks
  - Mitigation: DOC-001 completed with SHA-256 manifests; SoT maintained with links to all evidence
  - Status: Mitigated; operational_hash/HASHES.txt provides cryptographic proof

- Risk Register: docs/risks/register.md
  - Comprehensive risk tracking with owners, likelihood, impact, and mitigation plans
  - Reviewed quarterly; linked to relevant tasks in .todo

### 5.3 Active TODOs
- [ ] Complete PERF-001 performance optimization — Owner: @perf-oncall — Due: 2025-10-15 — Deps: OBS-001, PERF-BENCH
- [ ] Complete SEC-001 capability token management — Owner: @sec-oncall — Due: 2025-10-15 — Deps: SEC-POLICY, SEC-CI, DOC-001
- [ ] Complete OBS-001 real-time observability — Owner: @observability — Due: 2025-10-15 — Deps: OBS-DASH-ALERTS
- See .todo file for complete task list with acceptance criteria and evidence paths

## 6) Conventions
- Time: UTC, format YYYY-MM-DD HH:MM UTC.
- Paths: Relative to repo root.
- Hashing: sha256; store checksums in plain text manifests.
- Testing: Triple verification (PASS A/B/C) for stability-sensitive changes.
- Commits: Reference this SoT entry ID/timestamp in messages.

## 7) Quick Links
- Repo root: <./>
- CI workflows: <.github/workflows/>
- Observability:
  - Dashboards: <dashboards/>
  - Alerts: <alerts/>
  - SLO Policies: <slo/policies.yaml>
- Services:
  - <services/service-a/>
  - <services/service-b/>
- Tooling:
  - Scripts: <tools/scripts/>
  - Bridges/Adapters: <tools/bridges/>

<!-- End of template -->