# Merkle Anchors & Receipts

**REF:** SUPPLY-ANCHOR
**Owner:** @platform
**Status:** Active
**Last Updated:** 2025-10-05

## Overview

This directory contains Merkle tree anchors and timestamping receipts for AgentAskit releases, providing cryptographic proof of artifact existence at a specific point in time.

## Anchor Format

Each release generates an anchor file:

```
anchors/
├── v1.2.3.anchor.txt
├── v1.2.3.receipt.json
├── v1.2.2.anchor.txt
├── v1.2.2.receipt.json
└── ...
```

### Anchor File Format

```
# AgentAskit Release Anchor
# Version: v1.2.3
# Date: 2025-10-05T12:00:00Z

## Artifacts
agentaskit-v1.2.3.tar.gz  sha256:abc123...
agentaskit-v1.2.3.sbom.json  sha256:def456...

## Merkle Root
sha256:789xyz...

## Timestamp
RFC3161: https://timestamp.digicert.com
Receipt: v1.2.3.receipt.json
```

### Receipt Format (RFC 3161)

```json
{
  "version": 1,
  "timestamp": "2025-10-05T12:00:00Z",
  "tsa": "https://timestamp.digicert.com",
  "hash_algorithm": "sha256",
  "merkle_root": "789xyz...",
  "receipt_base64": "MIIx...",
  "verification_url": "https://verify.digicert.com/..."
}
```

## Generating Anchors

```bash
# Generate anchor for release
./scripts/supply-chain/generate-anchor.sh v1.2.3

# Verify anchor
./scripts/supply-chain/verify-anchor.sh v1.2.3
```

### Generation Script

```bash
#!/bin/bash
# scripts/supply-chain/generate-anchor.sh

VERSION=$1
ARTIFACTS_DIR="artifacts"
ANCHORS_DIR="anchors"

# Collect artifact hashes
echo "# AgentAskit Release Anchor" > "$ANCHORS_DIR/$VERSION.anchor.txt"
echo "# Version: $VERSION" >> "$ANCHORS_DIR/$VERSION.anchor.txt"
echo "# Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$ANCHORS_DIR/$VERSION.anchor.txt"
echo "" >> "$ANCHORS_DIR/$VERSION.anchor.txt"
echo "## Artifacts" >> "$ANCHORS_DIR/$VERSION.anchor.txt"

for file in "$ARTIFACTS_DIR"/*"$VERSION"*; do
    hash=$(sha256sum "$file" | cut -d' ' -f1)
    echo "$(basename "$file")  sha256:$hash" >> "$ANCHORS_DIR/$VERSION.anchor.txt"
done

# Calculate Merkle root
MERKLE_ROOT=$(sha256sum "$ANCHORS_DIR/$VERSION.anchor.txt" | cut -d' ' -f1)
echo "" >> "$ANCHORS_DIR/$VERSION.anchor.txt"
echo "## Merkle Root" >> "$ANCHORS_DIR/$VERSION.anchor.txt"
echo "sha256:$MERKLE_ROOT" >> "$ANCHORS_DIR/$VERSION.anchor.txt"

# Get RFC 3161 timestamp
openssl ts -query -data "$ANCHORS_DIR/$VERSION.anchor.txt" -no_nonce -sha256 -out /tmp/ts.req
curl -s -H "Content-Type: application/timestamp-query" \
     --data-binary @/tmp/ts.req \
     https://timestamp.digicert.com > "$ANCHORS_DIR/$VERSION.receipt.der"

echo "Anchor generated: $ANCHORS_DIR/$VERSION.anchor.txt"
```

## Verification

### Verify Anchor Integrity

```bash
# Verify Merkle root
COMPUTED=$(sha256sum anchors/v1.2.3.anchor.txt | cut -d' ' -f1)
STORED=$(grep "Merkle Root" -A1 anchors/v1.2.3.anchor.txt | tail -1 | cut -d: -f2)

if [ "$COMPUTED" = "$STORED" ]; then
    echo "Merkle root verified"
else
    echo "Merkle root mismatch!"
fi
```

### Verify Timestamp Receipt

```bash
# Verify RFC 3161 timestamp
openssl ts -verify \
    -data anchors/v1.2.3.anchor.txt \
    -in anchors/v1.2.3.receipt.der \
    -CAfile /etc/ssl/certs/ca-certificates.crt
```

## CI Integration

```yaml
# .github/workflows/release.yml
- name: Generate anchor
  run: ./scripts/supply-chain/generate-anchor.sh ${{ github.ref_name }}

- name: Upload anchor
  uses: actions/upload-artifact@v4
  with:
    name: release-anchor
    path: anchors/${{ github.ref_name }}.*
```

## Evidence

- Anchor files: `anchors/*.anchor.txt`
- Receipts: `anchors/*.receipt.json`
- Generation logs: `TEST/supply-chain/*.log`

## Related

- [SUPPLY-VERIFY](../.github/workflows/verify.yml) - Verification workflow
- [SUPPLY-SBOM](../sbom/) - SBOM generation
- [REL-VERSIONING](../docs/release/) - Release process
