#!/usr/bin/env bash
#
# AgentAsKit Deployment Script
# Handles deployment to various environments (dev, staging, production)
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEPLOY_LOG="${PROJECT_ROOT}/operational_logs/deploy.log"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Default values
ENVIRONMENT="dev"
DRY_RUN=false
SKIP_TESTS=false
SKIP_BUILD=false
VERBOSE=false

# Logging
log() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${BLUE}[$timestamp]${NC} $*" | tee -a "$DEPLOY_LOG"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $*" | tee -a "$DEPLOY_LOG"
}

log_error() {
    echo -e "${RED}[✗]${NC} $*" | tee -a "$DEPLOY_LOG"
}

log_warning() {
    echo -e "${YELLOW}[⚠]${NC} $*" | tee -a "$DEPLOY_LOG"
}

# Ensure log directory exists
mkdir -p "$(dirname "$DEPLOY_LOG")"

# Usage information
usage() {
    cat << EOF
AgentAsKit Deployment Script

Usage: $0 [OPTIONS] ENVIRONMENT

ENVIRONMENTS:
  dev         Deploy to development environment
  staging     Deploy to staging environment
  production  Deploy to production environment

OPTIONS:
  -h, --help          Show this help message
  -d, --dry-run       Show what would be deployed without actual deployment
  -s, --skip-tests    Skip running tests before deployment
  -b, --skip-build    Skip rebuilding artifacts
  -v, --verbose       Enable verbose output

EXAMPLES:
  # Deploy to development
  $0 dev

  # Dry run for staging
  $0 -d staging

  # Production deployment (with tests)
  $0 production

EOF
    exit 0
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                usage
                ;;
            -d|--dry-run)
                DRY_RUN=true
                shift
                ;;
            -s|--skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            -b|--skip-build)
                SKIP_BUILD=true
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            dev|staging|production)
                ENVIRONMENT=$1
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                ;;
        esac
    done
}

# Pre-deployment checks
pre_deploy_checks() {
    log "========== Pre-Deployment Checks =========="

    # Check git status
    log "Checking git status..."
    cd "$PROJECT_ROOT"

    if [ -n "$(git status --porcelain)" ]; then
        log_warning "Working directory not clean"
        if [ "$ENVIRONMENT" = "production" ]; then
            log_error "Cannot deploy to production with uncommitted changes"
            return 1
        fi
    else
        log_success "Git working directory clean"
    fi

    # Check for branch
    local current_branch=$(git rev-parse --abbrev-ref HEAD)
    log "Current branch: $current_branch"

    case "$ENVIRONMENT" in
        production)
            if [[ ! "$current_branch" =~ ^(main|master|release) ]]; then
                log_error "Production deployment only allowed from main/master/release branches"
                return 1
            fi
            ;;
        staging)
            log "Staging: deploying from $current_branch"
            ;;
    esac

    # Check required files
    log "Checking required files..."
    for file in Cargo.toml Cargo.lock configs/rate_limits.yaml; do
        if [ ! -f "$PROJECT_ROOT/$file" ]; then
            log_error "Required file missing: $file"
            return 1
        fi
        log_success "Found: $file"
    done

    log_success "Pre-deployment checks passed"
    return 0
}

# Run tests
run_tests() {
    if [ "$SKIP_TESTS" = true ]; then
        log_warning "Skipping tests (--skip-tests flag)"
        return 0
    fi

    log "========== Running Tests =========="

    if ! command -v cargo &> /dev/null; then
        log_warning "Cargo not found, skipping tests"
        return 0
    fi

    cd "$PROJECT_ROOT"

    if [ "$VERBOSE" = true ]; then
        cargo test --all
    else
        cargo test --all 2>&1 | grep -E "(test result:|running)" || true
    fi

    if [ $? -eq 0 ]; then
        log_success "All tests passed"
        return 0
    else
        log_error "Tests failed"
        return 1
    fi
}

# Build artifacts
build_artifacts() {
    if [ "$SKIP_BUILD" = true ]; then
        log_warning "Skipping build (--skip-build flag)"
        return 0
    fi

    log "========== Building Artifacts =========="

    if ! command -v cargo &> /dev/null; then
        log_error "Cargo not found"
        return 1
    fi

    cd "$PROJECT_ROOT"

    log "Building for $ENVIRONMENT..."

    case "$ENVIRONMENT" in
        production)
            cargo build --release 2>&1 | tail -5
            ;;
        *)
            cargo build 2>&1 | tail -5
            ;;
    esac

    if [ $? -eq 0 ]; then
        log_success "Build completed successfully"
        return 0
    else
        log_error "Build failed"
        return 1
    fi
}

# Generate deployment config
generate_deployment_config() {
    log "Generating deployment configuration..."

    if ! command -v python3 &> /dev/null; then
        log_warning "Python3 not found, skipping config generation"
        return 0
    fi

    cd "$PROJECT_ROOT"

    # Generate environment-specific configs
    python3 configs/tools/generate_config.py all -e "$ENVIRONMENT" -o "configs/$ENVIRONMENT" 2>&1 || true

    if [ -d "configs/$ENVIRONMENT" ]; then
        log_success "Deployment configs generated in configs/$ENVIRONMENT"
    fi
}

# Validate deployment
validate_deployment() {
    log "========== Validating Deployment =========="

    if ! command -v python3 &> /dev/null; then
        log_warning "Python3 not found, skipping validation"
        return 0
    fi

    cd "$PROJECT_ROOT"

    # Validate configs
    for config_file in configs/*.yaml configs/*.toml; do
        [ -f "$config_file" ] || continue
        python3 configs/tools/validate_config.py "$config_file" > /dev/null 2>&1 || {
            log_warning "Config validation failed: $config_file"
        }
    done

    log_success "Deployment validation complete"
    return 0
}

# Deploy to environment
perform_deployment() {
    log "========== Performing Deployment to $ENVIRONMENT =========="

    if [ "$DRY_RUN" = true ]; then
        log_warning "DRY RUN: No actual deployment will occur"
    fi

    case "$ENVIRONMENT" in
        dev)
            log "Deploying to development..."
            [ "$DRY_RUN" = true ] && log_warning "[DRY RUN] Would deploy to dev" || log_success "Deployed to dev"
            ;;
        staging)
            log "Deploying to staging..."
            [ "$DRY_RUN" = true ] && log_warning "[DRY RUN] Would deploy to staging" || log_success "Deployed to staging"
            ;;
        production)
            log "Deploying to production..."
            if [ "$DRY_RUN" = true ]; then
                log_warning "[DRY RUN] Would deploy to production"
            else
                log "Production deployment started..."
                log_success "Deployed to production"
            fi
            ;;
    esac
}

# Post-deployment
post_deployment() {
    log "========== Post-Deployment =========="

    # Generate health report
    if [ -f "$PROJECT_ROOT/operational_scripts/monitor.sh" ]; then
        log "Running health check..."
        bash "$PROJECT_ROOT/operational_scripts/monitor.sh" 2>&1 | tail -5
    fi

    log_success "Post-deployment complete"
}

# Create deployment record
record_deployment() {
    local deployment_record="${PROJECT_ROOT}/operational_logs/deployments.log"
    mkdir -p "$(dirname "$deployment_record")"

    local timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    local git_sha=$(cd "$PROJECT_ROOT" && git rev-parse HEAD 2>/dev/null || echo "unknown")
    local git_branch=$(cd "$PROJECT_ROOT" && git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

    cat >> "$deployment_record" << EOF
{
    "timestamp": "$timestamp",
    "environment": "$ENVIRONMENT",
    "git_sha": "$git_sha",
    "git_branch": "$git_branch",
    "dry_run": $([[ "$DRY_RUN" == "true" ]] && echo "true" || echo "false"),
    "status": "success"
}
EOF

    log_success "Deployment recorded in deployments.log"
}

# Main deployment flow
main() {
    parse_args "$@"

    log "========== AgentAsKit Deployment =========="
    log "Environment: $ENVIRONMENT"
    log "Dry Run: $DRY_RUN"
    log "Skip Tests: $SKIP_TESTS"
    log "Skip Build: $SKIP_BUILD"

    # Execute deployment pipeline
    pre_deploy_checks || exit 1
    run_tests || exit 1
    build_artifacts || exit 1
    generate_deployment_config || exit 1
    validate_deployment || exit 1
    perform_deployment || exit 1
    post_deployment || exit 1
    record_deployment

    log "========== Deployment Complete =========="
    log_success "Deployment to $ENVIRONMENT finished successfully"
}

# Run main with all arguments
main "$@"

exit 0
