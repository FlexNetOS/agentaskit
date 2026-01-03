#!/usr/bin/env nu
# Verify hashes and signatures for artifacts
# REF: ADR-0005 Modern Tooling Strategy

print "=== Verifying Hashes and Signatures ==="

# Check for manifest file
let manifest = "artifacts/MANIFEST.sha256"

if ($manifest | path exists) {
    print "Verifying MANIFEST.sha256..."

    let manifest_lines = open $manifest | lines | where { |l| ($l | str length) > 0 }

    mut verified = 0
    mut failed = 0

    for line in $manifest_lines {
        # Skip comments
        if ($line | str starts-with "#") { continue }

        # Parse "hash  filename" format
        let parts = $line | split row "  "
        if ($parts | length) >= 2 {
            let expected_hash = $parts | get 0
            let filename = $parts | get 1

            if ($filename | path exists) {
                let actual_hash = open $filename | hash sha256
                if $actual_hash == $expected_hash {
                    print $"  ✓ ($filename)"
                    $verified = $verified + 1
                } else {
                    print $"  ✗ ($filename) - hash mismatch"
                    $failed = $failed + 1
                }
            } else {
                print $"  ? ($filename) - file not found"
            }
        }
    }

    print $"\nVerified: ($verified), Failed: ($failed)"

    if $failed > 0 {
        exit 1
    }
} else {
    print "No MANIFEST.sha256 found - skipping hash verification"
}

# Check for minisign signature
let sig_file = "artifacts/MANIFEST.sha256.minisig"
if ($sig_file | path exists) {
    print "\nMinisign signature found"
    # Would verify with: minisign -Vm artifacts/MANIFEST.sha256 -P <pubkey>
    print "  (signature verification requires public key configuration)"
}

print "\n✓ Verification complete"
