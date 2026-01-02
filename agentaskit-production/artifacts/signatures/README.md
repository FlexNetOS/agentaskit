# Artifact Signatures

This directory contains cryptographic signatures for AgentAsKit Production artifacts.

## Signature Types

### GPG Signatures (*.sig)

GPG/OpenPGP detached signatures provide strong cryptographic verification using the RSA or EdDSA algorithms.

- **Format**: ASCII-armored PGP signature
- **Extension**: `.sig`
- **Verification**: `gpg --verify <signature> <file>`

### minisign Signatures (*.minisig)

minisign signatures use Ed25519 public-key signatures, providing a modern, lightweight alternative to GPG.

- **Format**: Base64-encoded Ed25519 signature
- **Extension**: `.minisig`
- **Verification**: `minisign -V -p <pubkey> -m <file> -x <signature>`

## Current Status

**Note**: Actual signatures will be generated when GPG or minisign keys are configured.

To configure signing:
1. Follow the instructions in `artifacts/README.md`
2. Set up GitHub Actions secrets (for CI/CD)
3. Run `scripts/sign_artifacts.sh` to generate signatures

## Signature Files

Signature files follow this naming convention:
- Original file: `sbom/agentaskit-core.json`
- GPG signature: `artifacts/signatures/agentaskit-core.json.sig`
- minisign signature: `artifacts/signatures/agentaskit-core.json.minisig`

## Verification

### Automated Verification

```bash
# Verify all signatures
bash scripts/sign_artifacts.sh --verify
```

### Manual Verification

```bash
# GPG
gpg --verify artifacts/signatures/agentaskit-core.json.sig sbom/agentaskit-core.json

# minisign
minisign -V -p artifacts/keys/minisign.pub \
    -m sbom/agentaskit-core.json \
    -x artifacts/signatures/agentaskit-core.json.minisig
```

## Security

- Signatures are generated in CI/CD using GitHub Actions secrets
- Private keys are never committed to the repository
- Public keys are available in `artifacts/keys/` for verification
- All signatures are validated before artifacts are deployed

---

**Task Reference**: SUPPLY-SIGN
**Last Updated**: 2026-01-02
