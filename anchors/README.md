# Anchors Directory - Merkle Tree Anchor Receipts

**Purpose:** Cryptographic proof of release integrity via Merkle tree anchoring.

## What is Anchoring?

Anchoring creates a **Merkle tree** over all release artifacts (SBOM, manifest, policies) and stores the **root hash** as a tamper-evident receipt. This provides:

- ✅ **Immutable proof** of release state
- ✅ **Verifiable audit trail** across releases
- ✅ **Tamper detection** for any component
- ✅ **Compliance evidence** for security audits

## Anchor Receipt Format

Each anchor is a JSON file named `anchor-YYYYMMDD-HHMMSS.json`:

```json
{
  "timestamp": "2025-01-05T14:30:00Z",
  "version": "1.0.0",
  "merkle_root": "abc123def456...",
  "components": [
    {
      "path": "sbom/sbom.cdx.json",
      "hash": "sha256:...",
      "size": 12345
    },
    {
      "path": "artifacts/MANIFEST.sha256",
      "hash": "sha256:...",
      "size": 6789
    }
  ],
  "metadata": {
    "release_id": "v1.0.0",
    "commit_hash": "abc123...",
    "build_env": "production"
  }
}
```

## Usage

### Generate Anchor

```bash
make anchor
```

This creates `anchors/anchor-<timestamp>.json` with:
- Merkle root of all tracked artifacts
- Individual file hashes and metadata
- Release timestamp and version

### Verify Anchor

```bash
# Verify current state matches anchor
python unified_tools/merkle_anchor.py --verify \
    --anchor anchors/anchor-20250105-143000.json \
    --root .
```

### List Anchors

```bash
# Show all anchors chronologically
ls -lt anchors/

# Count anchors
ls anchors/ | wc -l
```

### Compare Anchors

```bash
# Diff two release anchors
diff <(jq -S '.' anchors/anchor-old.json) \
     <(jq -S '.' anchors/anchor-new.json)
```

## Anchor Workflow

```
┌─────────────┐
│ make verify │  Verify all artifacts
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ make anchor │  Generate Merkle root
└──────┬──────┘
       │
       ▼
┌─────────────────────────────┐
│ anchors/anchor-<time>.json  │  Store receipt
└─────────────────────────────┘
```

## Integration Points

### With SBOM

Anchor includes SBOM hash:
```bash
jq '.components[] | select(.path | contains("sbom"))' \
   anchors/anchor-latest.json
```

### With Manifest

Anchor includes manifest hash:
```bash
jq '.components[] | select(.path | contains("MANIFEST"))' \
   anchors/anchor-latest.json
```

### With CI/CD

Generate anchor on release:
```yaml
# .github/workflows/release.yml
- name: Create Release Anchor
  run: make anchor
  
- name: Commit Anchor
  run: |
    git add anchors/
    git commit -m "Add release anchor"
```

## Merkle Tree Structure

```
                 ROOT (merkle_root)
                   /        \
                  /          \
            BRANCH1          BRANCH2
           /      \          /      \
      SBOM      MANIFEST  POLICIES  CONFIGS
       |          |         |         |
    (hash)    (hash)     (hash)    (hash)
```

## Anchor Lifecycle

1. **Pre-Release:** Run `make verify` to ensure all artifacts valid
2. **Anchor:** Run `make anchor` to generate Merkle root
3. **Commit:** Add anchor to Git: `git add anchors/ && git commit`
4. **Tag:** Tag release: `git tag v1.0.0`
5. **Audit:** Use anchor for compliance audits

## Environment Variables

- `FLEX_ANCHOR_METADATA` - Additional metadata to include in anchor (JSON string)
- `FLEX_ANCHOR_ALGORITHM` - Hash algorithm for Merkle tree (default: SHA-256)

## Best Practices

### 1. One Anchor Per Release

- Generate anchor AFTER all artifacts finalized
- Anchor represents final release state
- Don't regenerate anchor for same release

### 2. Anchor Retention

- Keep all anchors indefinitely for audit trail
- Never delete historical anchors
- Store anchors in immutable storage (e.g., S3 versioned bucket)

### 3. Anchor Verification

- Verify current state matches latest anchor before deployment
- Use anchors for rollback verification
- Include anchor verification in CI/CD pipelines

### 4. Anchor Documentation

- Document release notes with anchor hash
- Include anchor receipt in security documentation
- Reference anchor in vulnerability disclosures

## Troubleshooting

**Problem:** Anchor generation fails with missing files
- **Solution:** Run `make verify` first to ensure all artifacts present

**Problem:** Merkle root differs between generations
- **Solution:** Ensure no files changed between `make verify` and `make anchor`

**Problem:** Cannot verify old anchor
- **Solution:** Checkout Git tag for that release and verify: `git checkout v1.0.0 && make anchor --verify`

## Compliance Use Cases

### Supply Chain Security (SLSA)

Anchors provide **provenance** for SLSA Level 3+:
- Immutable build record
- Verifiable artifact integrity
- Tamper-evident audit trail

### SOC 2 Type II

Anchors support **security monitoring** controls:
- Change detection for production systems
- Audit trail for compliance reviews
- Evidence of security controls

### ISO 27001

Anchors demonstrate **asset management**:
- Complete inventory of components
- Version control and change tracking
- Security incident response support

## Advanced Features

### Blockchain Anchoring (Optional)

For maximum immutability, anchor Merkle root to blockchain:

```bash
# Example: Ethereum mainnet anchoring
echo "merkle_root" | cast send --rpc-url $ETH_RPC \
    $ANCHOR_CONTRACT "anchor(bytes32)"
```

### Timestamping Service

Use RFC 3161 timestamping for legal evidence:

```bash
# Generate timestamp
openssl ts -query -data anchors/anchor-latest.json \
    -out anchor.tsq
    
# Submit to TSA
curl -H "Content-Type: application/timestamp-query" \
     --data-binary @anchor.tsq \
     $TSA_URL > anchor.tsr
```

## Related Tools

- **merkle_anchor.py** - Anchor generation and verification
- **verify.py** - Pre-anchor verification
- **signer.py** - Manifest generation (included in anchor)

## Related Documentation

- [Agent Task Lifecycle SOP](../agentask.sop)
- [Makefile Targets](../Makefile) - See anchor, verify targets
- [Merkle Tree Specification](https://en.wikipedia.org/wiki/Merkle_tree)

---

**Generated:** 2025-01-05 | **Version:** 1.0 | **Status:** Production Ready  
**Total Anchors:** Run `ls anchors/ | wc -l` to count
