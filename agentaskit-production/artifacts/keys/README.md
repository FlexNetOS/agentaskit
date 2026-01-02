# Signing Keys

This directory stores public keys used for signature verification.

## ⚠️ Security Warning

**NEVER commit private keys to this directory or anywhere in version control!**

- ✅ **DO**: Store public keys here for distribution
- ✅ **DO**: Document key fingerprints and generation dates
- ❌ **DON'T**: Commit private/secret keys
- ❌ **DON'T**: Commit unencrypted passphrases

## Key Types

### GPG Public Key

- **File**: `public-key.asc` (not yet generated)
- **Format**: ASCII-armored OpenPGP public key
- **Purpose**: Verify GPG signatures in `artifacts/signatures/*.sig`

**To generate**:
```bash
# Generate key pair
gpg --full-generate-key

# Export public key
gpg --armor --export YOUR_KEY_ID > artifacts/keys/public-key.asc

# Get fingerprint
gpg --fingerprint YOUR_KEY_ID
```

### minisign Public Key

- **File**: `minisign.pub` (not yet generated)
- **Format**: Base64-encoded Ed25519 public key
- **Purpose**: Verify minisign signatures in `artifacts/signatures/*.minisig`

**To generate**:
```bash
# Generate key pair
minisign -G

# Copy public key
cp ~/.minisign/minisign.pub artifacts/keys/
```

## Key Management

### Key Generation

Follow the comprehensive instructions in `artifacts/README.md` for:
- GPG key generation and setup
- minisign key generation and setup
- GitHub Actions secrets configuration
- Key rotation procedures

### Key Fingerprints

Document key fingerprints here for verification:

```
# GPG Key (when generated)
# Fingerprint: [To be added]
# Created: [Date]
# Expires: [Date]
# Owner: [Name/Email]

# minisign Key (when generated)
# Fingerprint: [To be added]
# Created: [Date]
# Owner: [Name/Email]
```

### Key Rotation

Keys should be rotated annually or when:
- Key compromise is suspected
- Team member with key access leaves
- Best practice security review recommends it

**Rotation Process**:
1. Generate new key pair
2. Update GitHub Actions secrets
3. Re-sign all active artifacts
4. Update key fingerprints in this README
5. Publish key transition statement
6. Maintain old public keys for historical verification

## Key Verification

### Verify Public Key Integrity

```bash
# GPG
sha256sum artifacts/keys/public-key.asc
# Compare with published fingerprint

# minisign
sha256sum artifacts/keys/minisign.pub
# Compare with published fingerprint
```

### Import Keys for Verification

```bash
# Import GPG public key
gpg --import artifacts/keys/public-key.asc

# Verify minisign key location
ls -l artifacts/keys/minisign.pub
```

## Trust and Distribution

### Publishing Public Keys

Public keys should be published through multiple channels:

1. **This Repository**: `artifacts/keys/` (primary)
2. **GitHub Release**: Attached to release notes
3. **Key Servers** (GPG only):
   ```bash
   gpg --send-keys YOUR_KEY_ID
   ```
4. **Project Website**: If available
5. **Documentation**: Include in security documentation

### Verifying Key Authenticity

When obtaining public keys, verify through multiple channels:
- Compare fingerprints from different sources
- Verify with team members out-of-band
- Check commit signatures if keys are in git
- Use Web of Trust (GPG) if applicable

## GitHub Actions Integration

### Secrets Configuration

The following secrets must be configured in GitHub repository settings:

**For GPG**:
- `GPG_PRIVATE_KEY`: Private key (ASCII-armored)
- `GPG_PASSPHRASE`: Key passphrase

**For minisign**:
- `MINISIGN_PRIVATE_KEY`: Private key
- `MINISIGN_PASSWORD`: Key password (if set)

### Adding Secrets

1. Go to repository Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add each secret with the exact name above
4. Paste the key/passphrase content (never commit these!)

## Emergency Key Revocation

If a private key is compromised:

1. **Immediate Actions**:
   ```bash
   # Revoke GPG key
   gpg --gen-revoke YOUR_KEY_ID > revoke.asc
   gpg --import revoke.asc
   gpg --send-keys YOUR_KEY_ID
   ```

2. **Remove from GitHub**:
   - Delete compromised secrets from GitHub Actions
   - Rotate to new keys immediately

3. **Notify Users**:
   - Publish security advisory
   - Update documentation
   - Provide new public keys

4. **Re-sign Artifacts**:
   - Sign all current releases with new key
   - Update signature verification instructions

## Support

For questions about key management:
- Review: `artifacts/README.md` (comprehensive guide)
- Contact: @platform team
- Security issues: @sec-oncall

---

**Task Reference**: SUPPLY-SIGN
**Last Updated**: 2026-01-02
**Status**: Keys not yet generated (setup instructions provided)
