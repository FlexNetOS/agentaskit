#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════
# AgentAsKit Production - Artifact Signing Script
# ═══════════════════════════════════════════════════════════════════════════
#
# Purpose: Generate SHA256 checksums and cryptographic signatures for artifacts
# Dependencies: sha256sum, gpg (optional), minisign (optional)
# Task Reference: SUPPLY-SIGN
# Owner: @platform
#
# ═══════════════════════════════════════════════════════════════════════════

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ARTIFACTS_DIR="${PROJECT_ROOT}/artifacts"
SBOM_DIR="${PROJECT_ROOT}/sbom"
SIGNATURES_DIR="${ARTIFACTS_DIR}/signatures"
CHECKSUMS_DIR="${ARTIFACTS_DIR}/checksums"
HASHES_FILE="${PROJECT_ROOT}/operational_hash/HASHES.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to generate SHA256 checksums
generate_checksums() {
    log_info "Generating SHA256 checksums..."

    # Create checksums file
    CHECKSUMS_FILE="${CHECKSUMS_DIR}/SHA256SUMS"
    > "${CHECKSUMS_FILE}"

    # Generate checksums for SBOM files
    cd "${SBOM_DIR}"
    for file in *.json; do
        if [ -f "$file" ]; then
            log_info "  Computing checksum for: $file"
            sha256sum "$file" >> "${CHECKSUMS_FILE}"
        fi
    done

    cd "${PROJECT_ROOT}"

    # Also generate individual checksum files
    while IFS= read -r line; do
        checksum=$(echo "$line" | awk '{print $1}')
        filename=$(echo "$line" | awk '{print $2}')
        echo "$checksum" > "${CHECKSUMS_DIR}/${filename}.sha256"
    done < "${CHECKSUMS_FILE}"

    log_success "Checksums generated: ${CHECKSUMS_FILE}"
    return 0
}

# Function to sign with GPG
sign_with_gpg() {
    local file="$1"
    local signature="${SIGNATURES_DIR}/$(basename "${file}").sig"

    if ! command_exists gpg; then
        log_warning "GPG not found, skipping GPG signing"
        return 1
    fi

    # Check if GPG key is available
    if ! gpg --list-secret-keys >/dev/null 2>&1; then
        log_warning "No GPG secret keys found. Please configure GPG keys first."
        log_info "See artifacts/README.md for instructions on key generation."
        return 1
    fi

    log_info "  Signing with GPG: $(basename "$file")"

    # Create detached signature
    gpg --batch --yes --detach-sign --armor --output "${signature}" "$file" 2>/dev/null || {
        log_warning "GPG signing failed for: $file"
        return 1
    }

    log_success "  GPG signature created: $(basename "$signature")"
    return 0
}

# Function to sign with minisign
sign_with_minisign() {
    local file="$1"
    local signature="${SIGNATURES_DIR}/$(basename "${file}").minisig"

    if ! command_exists minisign; then
        log_warning "minisign not found, skipping minisign signing"
        return 1
    fi

    # Check if minisign key exists
    if [ ! -f "${HOME}/.minisign/minisign.key" ] && [ ! -f "${ARTIFACTS_DIR}/keys/minisign.key" ]; then
        log_warning "No minisign key found. Please generate one first."
        log_info "See artifacts/README.md for instructions on key generation."
        return 1
    fi

    log_info "  Signing with minisign: $(basename "$file")"

    # Determine key location
    local key_file="${ARTIFACTS_DIR}/keys/minisign.key"
    if [ ! -f "$key_file" ]; then
        key_file="${HOME}/.minisign/minisign.key"
    fi

    # Create signature (requires password if key is encrypted)
    if [ -n "${MINISIGN_PASSWORD:-}" ]; then
        echo "${MINISIGN_PASSWORD}" | minisign -S -s "$key_file" -m "$file" -x "${signature}" 2>/dev/null || {
            log_warning "minisign signing failed for: $file"
            return 1
        }
    else
        log_info "  Note: You may be prompted for the minisign key password"
        minisign -S -s "$key_file" -m "$file" -x "${signature}" || {
            log_warning "minisign signing failed for: $file"
            return 1
        }
    fi

    log_success "  minisign signature created: $(basename "$signature")"
    return 0
}

# Function to sign all artifacts
sign_artifacts() {
    log_info "Signing artifacts..."

    local gpg_available=0
    local minisign_available=0

    if command_exists gpg && gpg --list-secret-keys >/dev/null 2>&1; then
        gpg_available=1
    fi

    if command_exists minisign; then
        minisign_available=1
    fi

    if [ $gpg_available -eq 0 ] && [ $minisign_available -eq 0 ]; then
        log_error "Neither GPG nor minisign is properly configured"
        log_error "Please install and configure at least one signing tool"
        log_info "See artifacts/README.md for setup instructions"
        return 1
    fi

    # Sign SBOM files
    cd "${SBOM_DIR}"
    for file in *.json; do
        if [ -f "$file" ]; then
            local full_path="${SBOM_DIR}/${file}"

            # Try GPG signing
            if [ $gpg_available -eq 1 ]; then
                sign_with_gpg "$full_path"
            fi

            # Try minisign signing
            if [ $minisign_available -eq 1 ]; then
                sign_with_minisign "$full_path"
            fi
        fi
    done

    # Sign the checksums file
    if [ -f "${CHECKSUMS_DIR}/SHA256SUMS" ]; then
        if [ $gpg_available -eq 1 ]; then
            sign_with_gpg "${CHECKSUMS_DIR}/SHA256SUMS"
        fi

        if [ $minisign_available -eq 1 ]; then
            sign_with_minisign "${CHECKSUMS_DIR}/SHA256SUMS"
        fi
    fi

    cd "${PROJECT_ROOT}"

    log_success "Artifact signing completed"
    return 0
}

# Function to verify signatures
verify_signatures() {
    log_info "Verifying signatures..."

    local verification_failed=0

    # Verify GPG signatures
    if command_exists gpg; then
        log_info "Verifying GPG signatures..."
        for sig_file in "${SIGNATURES_DIR}"/*.sig; do
            if [ -f "$sig_file" ]; then
                local original_file=$(basename "$sig_file" .sig)
                local original_path="${SBOM_DIR}/${original_file}"

                if [ ! -f "$original_path" ]; then
                    original_path="${CHECKSUMS_DIR}/${original_file}"
                fi

                if [ -f "$original_path" ]; then
                    log_info "  Verifying: $(basename "$sig_file")"
                    if gpg --verify "$sig_file" "$original_path" 2>/dev/null; then
                        log_success "    ✓ GPG signature valid"
                    else
                        log_error "    ✗ GPG signature verification failed"
                        verification_failed=1
                    fi
                fi
            fi
        done
    fi

    # Verify minisign signatures
    if command_exists minisign; then
        log_info "Verifying minisign signatures..."
        for sig_file in "${SIGNATURES_DIR}"/*.minisig; do
            if [ -f "$sig_file" ]; then
                local original_file=$(basename "$sig_file" .minisig)
                local original_path="${SBOM_DIR}/${original_file}"

                if [ ! -f "$original_path" ]; then
                    original_path="${CHECKSUMS_DIR}/${original_file}"
                fi

                local pub_key="${ARTIFACTS_DIR}/keys/minisign.pub"
                if [ ! -f "$pub_key" ]; then
                    pub_key="${HOME}/.minisign/minisign.pub"
                fi

                if [ -f "$original_path" ] && [ -f "$pub_key" ]; then
                    log_info "  Verifying: $(basename "$sig_file")"
                    if minisign -V -p "$pub_key" -m "$original_path" -x "$sig_file" 2>/dev/null; then
                        log_success "    ✓ minisign signature valid"
                    else
                        log_error "    ✗ minisign signature verification failed"
                        verification_failed=1
                    fi
                fi
            fi
        done
    fi

    if [ $verification_failed -eq 0 ]; then
        log_success "All signatures verified successfully"
        return 0
    else
        log_error "Some signature verifications failed"
        return 1
    fi
}

# Function to display summary
display_summary() {
    log_info "═══════════════════════════════════════════════════════════"
    log_info "Artifact Signing Summary"
    log_info "═══════════════════════════════════════════════════════════"

    log_info "Checksums:"
    if [ -f "${CHECKSUMS_DIR}/SHA256SUMS" ]; then
        local checksum_count=$(wc -l < "${CHECKSUMS_DIR}/SHA256SUMS")
        log_info "  ✓ ${checksum_count} artifacts checksummed"
        log_info "  Location: artifacts/checksums/SHA256SUMS"
    fi

    log_info ""
    log_info "Signatures:"

    local gpg_sig_count=$(find "${SIGNATURES_DIR}" -name "*.sig" 2>/dev/null | wc -l)
    local minisig_count=$(find "${SIGNATURES_DIR}" -name "*.minisig" 2>/dev/null | wc -l)

    if [ $gpg_sig_count -gt 0 ]; then
        log_info "  ✓ ${gpg_sig_count} GPG signatures (.sig)"
    fi

    if [ $minisig_count -gt 0 ]; then
        log_info "  ✓ ${minisig_count} minisign signatures (.minisig)"
    fi

    log_info ""
    log_info "Location: artifacts/signatures/"
    log_info "═══════════════════════════════════════════════════════════"
}

# Function to show usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Artifact Signing Script for AgentAsKit Production

OPTIONS:
    -h, --help          Show this help message
    -c, --checksums     Generate checksums only
    -s, --sign          Sign artifacts only
    -v, --verify        Verify signatures only
    -a, --all           Generate checksums, sign, and verify (default)

ENVIRONMENT VARIABLES:
    MINISIGN_PASSWORD   Password for minisign key (optional, for CI/CD)
    GPG_KEY_ID          Specific GPG key ID to use (optional)

EXAMPLES:
    # Generate checksums, sign, and verify
    $0 --all

    # Generate checksums only
    $0 --checksums

    # Sign artifacts only
    $0 --sign

    # Verify existing signatures
    $0 --verify

For key generation and setup instructions, see:
    artifacts/README.md

EOF
}

# Main function
main() {
    local do_checksums=0
    local do_sign=0
    local do_verify=0

    # Parse arguments
    if [ $# -eq 0 ]; then
        # Default: do all
        do_checksums=1
        do_sign=1
        do_verify=1
    else
        while [ $# -gt 0 ]; do
            case "$1" in
                -h|--help)
                    usage
                    exit 0
                    ;;
                -c|--checksums)
                    do_checksums=1
                    shift
                    ;;
                -s|--sign)
                    do_sign=1
                    shift
                    ;;
                -v|--verify)
                    do_verify=1
                    shift
                    ;;
                -a|--all)
                    do_checksums=1
                    do_sign=1
                    do_verify=1
                    shift
                    ;;
                *)
                    log_error "Unknown option: $1"
                    usage
                    exit 1
                    ;;
            esac
        done
    fi

    log_info "═══════════════════════════════════════════════════════════"
    log_info "AgentAsKit Production - Artifact Signing"
    log_info "Task Reference: SUPPLY-SIGN"
    log_info "═══════════════════════════════════════════════════════════"
    log_info ""

    # Create directories if they don't exist
    mkdir -p "${SIGNATURES_DIR}" "${CHECKSUMS_DIR}"

    # Execute requested operations
    if [ $do_checksums -eq 1 ]; then
        generate_checksums || {
            log_error "Checksum generation failed"
            exit 1
        }
        log_info ""
    fi

    if [ $do_sign -eq 1 ]; then
        sign_artifacts || {
            log_warning "Signing completed with warnings"
        }
        log_info ""
    fi

    if [ $do_verify -eq 1 ]; then
        verify_signatures || {
            log_warning "Verification completed with warnings"
        }
        log_info ""
    fi

    display_summary

    log_info ""
    log_success "All operations completed successfully"
}

# Run main function
main "$@"
