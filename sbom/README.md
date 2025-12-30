# SBOM Directory - Software Bill of Materials

**Purpose:** CycloneDX format inventory of all components, dependencies, and cryptographic hashes.

## Contents

### sbom.cdx.json

**Format:** CycloneDX 1.5  
**Generation:** Automated via `make gen-sbom`  
**Updates:** Regenerated on every release

**Contents:**
- All tracked files with SHA-256 hashes
- Component metadata (version, type, name)
- Dependency relationships
- License information (when available)
- Timestamp and tool information

## Usage

### Generate SBOM

```bash
make gen-sbom
```

### Verify SBOM Integrity

```bash
make verify
```

### View SBOM Contents

```bash
# Pretty print with jq
jq '.' sbom/sbom.cdx.json

# Count components
jq '.components | length' sbom/sbom.cdx.json

# List component names
jq '.components[].name' sbom/sbom.cdx.json
```

### Extract Component Hashes

```bash
# Get all SHA-256 hashes
jq -r '.components[].hashes[] | select(.alg == "SHA-256") | .content' sbom/sbom.cdx.json
```

## SBOM Schema

```json
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "serialNumber": "urn:uuid:...",
  "version": 1,
  "metadata": {
    "timestamp": "2025-01-05T...",
    "tools": [...]
  },
  "components": [
    {
      "type": "file",
      "name": "relative/path/to/file",
      "version": "1.0.0",
      "hashes": [
        {
          "alg": "SHA-256",
          "content": "abc123..."
        }
      ]
    }
  ]
}
```

## Integration

### With Artifact Manifest

SBOM hashes are cross-referenced in `artifacts/MANIFEST.sha256`:

```bash
# Verify SBOM matches manifest
make verify
```

### With Anchoring

SBOM is included in Merkle tree anchoring:

```bash
# Create anchor including SBOM
make anchor
```

### With Signing

SBOM can be signed with minisign:

```bash
# Sign manifest (includes SBOM verification)
make sign
```

## Environment Variables

- `FLEX_MINISIGN_PUB` - Public key path for signature verification
- `FLEX_ENFORCE_SEAL` - Require fs-verity on SBOM file
- `FLEX_ENFORCE_MOUNT_RO` - Require read-only mount for SBOM

## Exclusions

The following are excluded from SBOM generation:
- `anchors/` - Anchor receipts (generated artifacts)
- `.git/` - Git metadata
- Large binary artifacts (configurable)

## Compliance

**Standards:**
- CycloneDX 1.5 specification
- NTIA Minimum Elements for SBOM
- ISO/IEC 5962:2021 SBOM guidelines

**Use Cases:**
- Supply chain security audits
- Vulnerability scanning
- License compliance verification
- Component inventory management

## Automation

### CI/CD Integration

```bash
# Pre-push hook generates SBOM
git commit -m "Release v1.0"
git push  # Triggers SBOM generation
```

### Continuous Verification

```bash
# Cron job to verify SBOM integrity
0 */6 * * * cd /path/to/agentaskit-production && make verify
```

## Troubleshooting

**Problem:** SBOM generation fails
- **Solution:** Check write permissions on `sbom/` directory

**Problem:** Component count doesn't match expectations
- **Solution:** Review exclusion patterns in `unified_tools/sbom_gen.py`

**Problem:** Hashes don't match after update
- **Solution:** Regenerate SBOM with `make gen-sbom` after file modifications

## Related Tools

- **sbom_gen.py** - SBOM generation script
- **signer.py** - Manifest and signature generation
- **verify.py** - Hash and signature verification
- **merkle_anchor.py** - Anchoring with SBOM inclusion

## Related Documentation

- [Agent Task Lifecycle SOP](../agentask.sop)
- [Makefile Targets](../Makefile) - See gen-sbom, sign, verify targets
- [CycloneDX Specification](https://cyclonedx.org/specification/overview/)

---

**Generated:** 2025-01-05 | **Version:** 1.0 | **Status:** Production Ready  
**Last SBOM Generation:** Check `sbom.cdx.json` metadata.timestamp
