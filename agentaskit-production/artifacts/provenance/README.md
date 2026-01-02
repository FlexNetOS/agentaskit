# SLSA Provenance & Attestations

**REF:** SUPPLY-SLSA
**Owner:** @platform
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This directory contains SLSA (Supply-chain Levels for Software Artifacts) provenance documents and attestations for AgentAskit releases.

## SLSA Level

Current target: **SLSA Level 3**

| Requirement | Status |
|-------------|--------|
| Source - Version controlled | ✅ |
| Build - Scripted build | ✅ |
| Build - Build service | ✅ |
| Build - Ephemeral environment | ✅ |
| Build - Isolated | ✅ |
| Provenance - Available | ✅ |
| Provenance - Authenticated | ✅ |
| Provenance - Service generated | ✅ |
| Provenance - Non-falsifiable | ✅ |

## Provenance Format

Provenance documents follow the SLSA v1.0 specification:

```json
{
  "_type": "https://in-toto.io/Statement/v1",
  "subject": [
    {
      "name": "agentaskit-v1.2.3.tar.gz",
      "digest": {
        "sha256": "abc123..."
      }
    }
  ],
  "predicateType": "https://slsa.dev/provenance/v1",
  "predicate": {
    "buildDefinition": {
      "buildType": "https://github.com/slsa-framework/slsa-github-generator",
      "externalParameters": {
        "workflow": ".github/workflows/release.yml",
        "ref": "refs/tags/v1.2.3"
      }
    },
    "runDetails": {
      "builder": {
        "id": "https://github.com/slsa-framework/slsa-github-generator/.github/workflows/generator_generic_slsa3.yml@refs/tags/v1.9.0"
      },
      "metadata": {
        "invocationId": "https://github.com/FlexNetOS/agentaskit/actions/runs/12345"
      }
    }
  }
}
```

## Generating Provenance

Provenance is automatically generated during the release workflow:

```yaml
# .github/workflows/release.yml
jobs:
  build:
    outputs:
      hashes: ${{ steps.hash.outputs.hashes }}

  provenance:
    needs: build
    uses: slsa-framework/slsa-github-generator/.github/workflows/generator_generic_slsa3.yml@v1.9.0
    with:
      base64-subjects: "${{ needs.build.outputs.hashes }}"
```

## Verifying Provenance

```bash
# Install slsa-verifier
go install github.com/slsa-framework/slsa-verifier/v2/cli/slsa-verifier@latest

# Verify artifact
slsa-verifier verify-artifact \
  agentaskit-v1.2.3.tar.gz \
  --provenance-path agentaskit-v1.2.3.intoto.jsonl \
  --source-uri github.com/FlexNetOS/agentaskit \
  --source-tag v1.2.3
```

## Attestation Storage

| Type | Location |
|------|----------|
| Provenance | `artifacts/provenance/v{version}.intoto.jsonl` |
| SBOM attestation | `sbom/{version}.sbom.att` |
| Signature | `artifacts/{version}.sig` |

## Evidence

- Provenance files: `artifacts/provenance/`
- Verification logs: `TEST/verify/*.log`
- CI workflow: `.github/workflows/release.yml`

## Related

- [SUPPLY-SBOM](../../sbom/) - SBOM generation
- [SUPPLY-SIGN](../../artifacts/) - Artifact signing
- [SUPPLY-VERIFY](../../.github/workflows/verify.yml) - Verification workflow
