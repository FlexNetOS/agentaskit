# Operational Hash Manifest System

## Overview

This directory contains the cryptographic integrity manifest system for AgentAsKit Production. The SHA-256 hash manifests provide tamper-evident evidence trails for all production artifacts, supporting:

- **Supply chain security**: Verify artifact integrity before deployment
- **Compliance & audit**: Cryptographic proof for regulatory requirements
- **Change detection**: Identify unauthorized or unexpected modifications
- **Evidence trails**: Link documentation to specific artifact versions

## Files in This Directory

### HASHES.txt
The master SHA-256 integrity manifest containing cryptographic hashes for:
- Root-level key files (Makefile, README, CODEOWNERS, CONTRIBUTING, .todo, .sop)
- Configuration files (*.yaml, *.toml)
- Documentation files (docs/**/*.md, docs/**/*.json)
- Core source code (core/src/**/*.rs)
- Test files (tests/**/*)

**Format**: Standard `sha256sum` output format (hash followed by file path)

**Total entries**: 149 files

### generate_integrity.py
Python script for automated manifest generation. Supports:
- Recursive directory scanning
- Component-based organization
- JSON manifest output
- Overall system hash computation

## Usage

### Verify All Hashes

```bash
cd /home/user/agentaskit/agentaskit-production
sha256sum -c operational_hash/HASHES.txt
```

Expected output: `<filename>: OK` for each verified file

### Verify Specific File

```bash
cd /home/user/agentaskit/agentaskit-production
grep "path/to/file" operational_hash/HASHES.txt | sha256sum -c
```

### Verify Category

```bash
cd /home/user/agentaskit/agentaskit-production
grep -A 50 "CATEGORY: Documentation" operational_hash/HASHES.txt | grep -v "^#" | sha256sum -c
```

### Generate New Manifest

```bash
cd /home/user/agentaskit/agentaskit-production
python3 operational_hash/generate_integrity.py
```

This creates `system_integrity.json` with structured component hashes.

### Detect Changes

```bash
# Check what files have changed since last manifest
cd /home/user/agentaskit/agentaskit-production
sha256sum -c operational_hash/HASHES.txt 2>&1 | grep FAILED
```

## Manifest Structure

### Header Section
- **Metadata**: Generation timestamp, git commit, branch, total file count
- **Verification Instructions**: Commands for hash verification
- **Acceptance Criteria**: Links to DOC-001 task requirements

### Hash Sections
Organized by category:
1. **Root-Level Key Files**: Repository governance files
2. **Configuration Files**: YAML/TOML settings
3. **Documentation Files**: Markdown and JSON documentation
4. **Core Source Code**: Rust implementation files
5. **Test Files**: Test suites and fixtures

### Signature Section
- Completion checklist
- Signer identification
- Task reference (DOC-001)
- Date of manifest generation

## Integration Points

### DOC-001 Task
This manifest system fulfills the DOC-001 acceptance criteria:
- ✅ Every REF links evidence
- ✅ SHA-256 manifests present
- ✅ Checklists signed
- ✅ Evidence locations: docs/, operational_hash/HASHES.txt, TEST/*

### Supply Chain Tasks
Related task dependencies:
- **SUPPLY-SBOM**: Software Bill of Materials generation
- **SUPPLY-SIGN**: Artifact signing with GPG/minisign
- **SUPPLY-VERIFY**: Automated signature verification in CI
- **SUPPLY-SLSA**: Provenance and attestation generation
- **SUPPLY-ANCHOR**: Merkle anchors and receipts

### CI/CD Integration
The manifest integrates with:
- **OPS-HOOKS**: Pre-push hooks verify manifest freshness
- **CI-BUILD**: Build pipeline generates/verifies hashes
- **CD-GATES**: Release gates require hash verification

## Maintenance

### When to Update
Regenerate the manifest when:
- Documentation files are modified
- Configuration files change
- Source code is updated
- Test files are added/modified
- Before creating a release
- After merging significant PRs

### Update Procedure
1. Make your changes to tracked files
2. Run: `python3 operational_hash/generate_integrity.py`
3. Review changes to HASHES.txt
4. Commit both code changes and manifest updates
5. Reference the task/PR in the commit message

### Automation
The manifest can be automated via:
- **Git hooks**: Pre-commit hook to auto-generate hashes
- **CI pipeline**: GitHub Actions to verify and update manifest
- **Release workflow**: Automatic manifest generation on version tags

## Security Considerations

### Hash Algorithm
- **Algorithm**: SHA-256 (FIPS 180-4 compliant)
- **Strength**: 256-bit cryptographic hash function
- **Collision resistance**: Computationally infeasible to find collisions
- **Status**: Approved for use in US federal systems

### Threat Model
This manifest protects against:
- ✅ Unauthorized file modifications
- ✅ Accidental corruption
- ✅ Supply chain tampering (when combined with signing)
- ✅ Insider threats (provides audit trail)

This manifest does NOT protect against:
- ❌ Attacks on the hashing infrastructure itself
- ❌ Compromised keys (requires separate key management)
- ❌ Time-of-check to time-of-use race conditions

### Best Practices
1. **Version control**: Always commit HASHES.txt with code changes
2. **Signing**: Sign the manifest with GPG for non-repudiation
3. **Verification**: Verify hashes before deployment
4. **Automation**: Use CI/CD to enforce hash verification
5. **Monitoring**: Alert on unexpected hash mismatches

## Troubleshooting

### Hash Verification Fails
```bash
# Identify failed files
sha256sum -c operational_hash/HASHES.txt 2>&1 | grep FAILED

# Check if file was legitimately modified
git diff path/to/failed/file

# If legitimate, regenerate manifest
python3 operational_hash/generate_integrity.py
```

### Missing Files
```bash
# Check if file was deleted or moved
sha256sum -c operational_hash/HASHES.txt 2>&1 | grep "No such file"

# Update manifest to reflect current state
python3 operational_hash/generate_integrity.py
```

### Performance Issues
For large repositories:
```bash
# Verify only specific category
grep -A 100 "CATEGORY: Documentation" operational_hash/HASHES.txt | \
  grep -v "^#" | sha256sum -c
```

## References

### Documentation
- [DOC-001 Task](./../.todo): Evidence trails & SHA-256 manifests
- [EVID-HASH Task](./../.todo): Evidence & hash structure
- [Supply Chain Tasks](./../.todo): SUPPLY-SBOM through SUPPLY-ANCHOR

### Standards
- [FIPS 180-4](https://csrc.nist.gov/publications/detail/fips/180/4/final): SHA-256 specification
- [NIST SP 800-107](https://csrc.nist.gov/publications/detail/sp/800-107/rev-1/final): Hash function security
- [SLSA Framework](https://slsa.dev/): Supply chain security levels

### Tools
- `sha256sum`: GNU coreutils hash verification tool
- `openssl dgst -sha256`: Alternative hash computation
- `hashdeep`: Recursive hash computation and verification
- `git hash-object`: Git's internal SHA-256 support (future)

## Support

For questions or issues:
- Task owner: @docs
- Security owner: @sec-oncall
- Platform owner: @platform

Last updated: 2026-01-02
Task: DOC-001 (Evidence trails & SHA-256 manifests)
