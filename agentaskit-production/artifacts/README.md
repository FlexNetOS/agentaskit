# Artifact Signing

This directory contains cryptographic signatures and checksums for all AgentAsKit Production artifacts, ensuring supply chain integrity and authenticity.

## Overview

**Task Reference**: SUPPLY-SIGN
**Owner**: @platform
**Dependencies**: SUPPLY-SBOM (completed)

The artifact signing system provides:

- **SHA-256 Checksums**: Cryptographic hashes for verifying file integrity
- **GPG Signatures**: OpenPGP detached signatures for authenticity verification
- **minisign Signatures**: Lightweight alternative signatures using modern cryptography
- **Automated Workflow**: GitHub Actions integration for continuous signing

## Directory Structure

```
artifacts/
├── README.md               # This file
├── checksums/              # SHA-256 checksums
│   ├── SHA256SUMS          # Master checksums file
│   └── *.sha256            # Individual checksum files
├── signatures/             # Cryptographic signatures
│   ├── *.sig               # GPG signatures
│   └── *.minisig           # minisign signatures
└── keys/                   # Public keys (optional storage)
    ├── minisign.pub        # minisign public key
    └── README.md           # Key management documentation
```

## Quick Start

### Verify Existing Signatures

```bash
# Verify all signatures
bash scripts/sign_artifacts.sh --verify

# Verify checksums only
cd sbom
sha256sum -c ../artifacts/checksums/SHA256SUMS
```

### Generate New Signatures

```bash
# Generate checksums and signatures
bash scripts/sign_artifacts.sh --all

# Generate checksums only
bash scripts/sign_artifacts.sh --checksums

# Sign artifacts only (requires keys)
bash scripts/sign_artifacts.sh --sign
```

## Key Generation and Setup

### Option 1: GPG (GNU Privacy Guard)

GPG is widely used and provides strong OpenPGP-based signatures.

#### Install GPG

```bash
# Debian/Ubuntu
sudo apt-get install gnupg

# macOS
brew install gnupg

# Windows (using WSL or Git Bash)
# GPG is usually pre-installed
```

#### Generate GPG Key

```bash
# Generate a new GPG key pair
gpg --full-generate-key

# Follow the prompts:
# - Key type: (1) RSA and RSA (default)
# - Key size: 4096 bits
# - Expiration: 1 year (recommended for security)
# - Real name: Your name or "AgentAsKit Release Bot"
# - Email: your-email@example.com or noreply@github.com
# - Passphrase: Use a strong password
```

#### Export GPG Keys

```bash
# List your keys
gpg --list-secret-keys --keyid-format LONG

# Export private key (for CI/CD - KEEP SECURE!)
gpg --armor --export-secret-keys YOUR_KEY_ID > private-key.asc

# Export public key (for distribution)
gpg --armor --export YOUR_KEY_ID > public-key.asc

# For GitHub Actions, also export without armor:
gpg --export-secret-keys YOUR_KEY_ID | base64 > private-key-base64.txt
```

#### Configure GPG for GitHub Actions

Add these secrets to your GitHub repository (Settings → Secrets and variables → Actions):

1. **GPG_PRIVATE_KEY**: Contents of `private-key.asc`
2. **GPG_PASSPHRASE**: Your GPG key passphrase

```bash
# Copy private key to clipboard (macOS)
cat private-key.asc | pbcopy

# Copy private key to clipboard (Linux with xclip)
cat private-key.asc | xclip -selection clipboard
```

⚠️ **Security Warning**: Never commit private keys to version control!

### Option 2: minisign (Recommended for CI/CD)

minisign is a lightweight, modern alternative to GPG with simpler key management.

#### Install minisign

```bash
# Debian/Ubuntu
sudo apt-get install minisign

# macOS
brew install minisign

# From source
git clone https://github.com/jedisct1/minisign.git
cd minisign
mkdir build && cd build
cmake .. && make
sudo make install
```

#### Generate minisign Key

```bash
# Generate key pair
minisign -G

# Follow the prompts:
# - Enter a password (can be empty for CI/CD, but not recommended)
# - Keys will be saved as:
#   - Secret key: ~/.minisign/minisign.key
#   - Public key: ~/.minisign/minisign.pub
```

#### Export minisign Keys

```bash
# Copy public key to project (for distribution)
cp ~/.minisign/minisign.pub artifacts/keys/

# For GitHub Actions, export private key
cat ~/.minisign/minisign.key > artifacts/keys/minisign.key.example
# Edit the example file to add instructions, then delete the actual key file
```

#### Configure minisign for GitHub Actions

Add these secrets to your GitHub repository:

1. **MINISIGN_PRIVATE_KEY**: Contents of `~/.minisign/minisign.key`
2. **MINISIGN_PASSWORD**: Your minisign key password (if set)

```bash
# Copy private key content
cat ~/.minisign/minisign.key
```

⚠️ **Security Warning**: Store private keys securely and never commit them!

## Signing Process

### Manual Signing

```bash
# 1. Ensure you have signing keys configured
gpg --list-secret-keys  # For GPG
ls ~/.minisign/         # For minisign

# 2. Run the signing script
cd /path/to/agentaskit-production
bash scripts/sign_artifacts.sh --all

# 3. Verify the signatures
bash scripts/sign_artifacts.sh --verify

# 4. Review generated files
ls -lh artifacts/checksums/
ls -lh artifacts/signatures/
```

### Automated Signing (CI/CD)

The GitHub Actions workflow (`.github/workflows/sign.yml`) automatically:

1. **Triggers** on:
   - SBOM generation completion
   - Push to main/release branches
   - Pull requests affecting artifacts
   - Manual workflow dispatch

2. **Process**:
   - Generates SHA-256 checksums for all SBOM files
   - Signs artifacts with GPG (if configured)
   - Signs artifacts with minisign (if configured)
   - Verifies all signatures
   - Uploads signed artifacts
   - Commits signatures to main branch

3. **Configuration**:
   - Add GPG and/or minisign secrets to GitHub repository
   - Workflow will gracefully handle missing keys (with warnings)

## Verification

### Verify Checksums

```bash
# Verify all SBOM checksums
cd sbom
sha256sum -c ../artifacts/checksums/SHA256SUMS

# Verify individual file
sha256sum agentaskit-core.json
cat ../artifacts/checksums/agentaskit-core.json.sha256
```

### Verify GPG Signatures

```bash
# Import public key (first time only)
gpg --import artifacts/keys/public-key.asc

# Verify a signature
gpg --verify artifacts/signatures/agentaskit-core.json.sig sbom/agentaskit-core.json

# Verify all GPG signatures
for sig in artifacts/signatures/*.sig; do
    file=$(basename "$sig" .sig)
    gpg --verify "$sig" "sbom/$file"
done
```

### Verify minisign Signatures

```bash
# Verify a signature (public key must be available)
minisign -V -p artifacts/keys/minisign.pub \
    -m sbom/agentaskit-core.json \
    -x artifacts/signatures/agentaskit-core.json.minisig

# Verify all minisign signatures
for sig in artifacts/signatures/*.minisig; do
    file=$(basename "$sig" .minisig)
    minisign -V -p artifacts/keys/minisign.pub \
        -m "sbom/$file" \
        -x "$sig"
done
```

## Integration with Operational Hash

All signatures and checksums are referenced in `operational_hash/HASHES.txt`:

```bash
# The HASHES.txt file includes:
# - Original artifact checksums
# - Signature file checksums
# - Verification instructions
# - Task references
```

This provides a complete audit trail linking artifacts to their signatures.

## Best Practices

### Key Management

1. **Generate Strong Keys**:
   - GPG: Use RSA 4096-bit keys
   - minisign: Use default Ed25519 keys

2. **Protect Private Keys**:
   - Store in secure password managers
   - Never commit to version control
   - Use different keys for development vs. production

3. **Key Rotation**:
   - Rotate keys annually
   - Update GitHub secrets when rotating
   - Maintain old public keys for historical verification

4. **Backup Keys**:
   - Export and securely store private keys
   - Document recovery procedures
   - Test key recovery process

### Signature Verification

1. **Always Verify**:
   - Verify signatures before using artifacts
   - Automate verification in deployment pipelines
   - Fail deployments on verification failures

2. **Trust Chain**:
   - Verify public keys through multiple channels
   - Use key fingerprints for verification
   - Document trusted key sources

3. **Audit Trail**:
   - Log all signature verifications
   - Monitor for verification failures
   - Alert on unexpected signature changes

### CI/CD Integration

1. **Secrets Management**:
   - Use GitHub Actions secrets (never hardcode)
   - Rotate secrets regularly
   - Limit secret access to necessary workflows

2. **Workflow Security**:
   - Pin workflow actions to specific versions
   - Review workflow changes carefully
   - Use branch protection rules

3. **Monitoring**:
   - Monitor workflow execution
   - Alert on signing failures
   - Track signature coverage

## Troubleshooting

### GPG Issues

**Problem**: `gpg: signing failed: Inappropriate ioctl for device`

```bash
# Solution: Set GPG_TTY
export GPG_TTY=$(tty)
```

**Problem**: `gpg: signing failed: No secret key`

```bash
# Solution: List and import keys
gpg --list-secret-keys
gpg --import private-key.asc
```

**Problem**: Passphrase prompt in CI/CD

```bash
# Solution: Configure batch mode
echo "allow-loopback-pinentry" >> ~/.gnupg/gpg-agent.conf
echo "pinentry-mode loopback" >> ~/.gnupg/gpg.conf
gpgconf --kill gpg-agent
```

### minisign Issues

**Problem**: `minisign: key not found`

```bash
# Solution: Specify key location
minisign -S -s /path/to/minisign.key -m file.txt
```

**Problem**: Password prompt in CI/CD

```bash
# Solution: Use environment variable
export MINISIGN_PASSWORD="your-password"
echo "$MINISIGN_PASSWORD" | minisign -S -s minisign.key -m file.txt
```

### Verification Failures

**Problem**: Checksum mismatch

```bash
# Causes:
# 1. File was modified after signing
# 2. Corruption during transfer
# 3. Wrong file version

# Solution: Re-download or regenerate artifact
```

**Problem**: Signature invalid

```bash
# Causes:
# 1. Wrong public key
# 2. Signature file corrupted
# 3. File modified after signing

# Solution: Verify key fingerprint and re-sign if necessary
```

## Acceptance Criteria (SUPPLY-SIGN)

✅ **SHA256 + minisign/GPG signatures generated**
- Checksums in `artifacts/checksums/SHA256SUMS`
- GPG signatures in `artifacts/signatures/*.sig`
- minisign signatures in `artifacts/signatures/*.minisig`

✅ **Keys stored securely**
- Private keys only in GitHub Secrets
- Public keys documented in this README
- Key management procedures documented

✅ **Evidence**
- Signatures: `artifacts/*.sig`, `artifacts/*.minisig`
- Checksums: `artifacts/checksums/SHA256SUMS`
- Hash manifest: `operational_hash/HASHES.txt`

✅ **Automation**
- GitHub workflow: `.github/workflows/sign.yml`
- Signing script: `scripts/sign_artifacts.sh`
- Automated verification

## Related Tasks

- **SUPPLY-SBOM**: SBOM generation (dependency, completed)
- **SUPPLY-VERIFY**: Signature verification in CI/CD (next task)
- **SUPPLY-SLSA**: Provenance and attestations
- **DOC-001**: Evidence trails and SHA-256 manifests

## Security Considerations

1. **Private Key Security**:
   - Never commit private keys to version control
   - Use GitHub encrypted secrets for CI/CD
   - Rotate keys annually
   - Revoke compromised keys immediately

2. **Signature Verification**:
   - Always verify signatures before deployment
   - Fail deployments on verification failures
   - Monitor for unexpected signature changes
   - Maintain audit logs

3. **Key Distribution**:
   - Publish public keys through multiple channels
   - Use key fingerprints for verification
   - Document trusted sources
   - Implement key rotation procedures

4. **Access Control**:
   - Limit who can sign artifacts
   - Use separate keys for different environments
   - Implement approval workflows
   - Monitor signing activities

## Support and Resources

### Documentation
- [GPG Manual](https://www.gnupg.org/documentation/)
- [minisign Documentation](https://jedisct1.github.io/minisign/)
- [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)

### Internal Resources
- Task: SUPPLY-SIGN in `.todo`
- Workflow: `.github/workflows/sign.yml`
- Script: `scripts/sign_artifacts.sh`
- Evidence: `operational_hash/HASHES.txt`

### Getting Help
- Open an issue in the repository
- Contact: @platform team
- Review: Security team for key management questions

---

**Last Updated**: 2026-01-02
**Task Reference**: SUPPLY-SIGN
**Owner**: @platform
**Status**: Implementation Complete
