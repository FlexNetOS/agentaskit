#!/usr/bin/env bash
#
# AgentAsKit System Health Monitor
# Monitors system health, performance, and component status
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LOG_DIR="${PROJECT_ROOT}/operational_logs"
MONITOR_LOG="${LOG_DIR}/monitor.log"
HEALTH_REPORT="${LOG_DIR}/health_report.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Thresholds
CPU_WARN_THRESHOLD=80
MEMORY_WARN_THRESHOLD=80
DISK_WARN_THRESHOLD=85

# Initialize log directory
mkdir -p "$LOG_DIR"

# Logging functions
log() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${BLUE}[$timestamp]${NC} $*" | tee -a "$MONITOR_LOG"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $*" | tee -a "$MONITOR_LOG"
}

log_error() {
    echo -e "${RED}[✗]${NC} $*" | tee -a "$MONITOR_LOG"
}

log_warning() {
    echo -e "${YELLOW}[⚠]${NC} $*" | tee -a "$MONITOR_LOG"
}

# System metrics collection
get_cpu_usage() {
    if command -v top &> /dev/null; then
        top -bn1 | grep "Cpu(s)" | awk '{print int($2)}' || echo "0"
    else
        echo "0"
    fi
}

get_memory_usage() {
    if command -v free &> /dev/null; then
        free | grep Mem | awk '{printf("%.0f", ($3/$2) * 100)}'
    else
        echo "0"
    fi
}

get_disk_usage() {
    df "$PROJECT_ROOT" | tail -1 | awk '{print int($5)}'
}

get_load_average() {
    uptime | awk -F'load average:' '{print $2}' | awk '{print $1, $2, $3}'
}

# Component health checks
check_git_status() {
    log "Checking git repository status..."
    if [ -d "$PROJECT_ROOT/.git" ]; then
        cd "$PROJECT_ROOT"
        if git status > /dev/null 2>&1; then
            local uncommitted=$(git status --porcelain | wc -l)
            if [ $uncommitted -gt 0 ]; then
                log_warning "Git: $uncommitted uncommitted changes"
                return 1
            else
                log_success "Git repository clean"
                return 0
            fi
        else
            log_error "Git repository error"
            return 1
        fi
    else
        log_error "Not a git repository"
        return 1
    fi
}

check_config_validity() {
    log "Checking configuration files..."

    local config_errors=0

    # Check critical config files
    for config_file in "$PROJECT_ROOT/configs/tracing.yaml" "$PROJECT_ROOT/configs/rate_limits.yaml"; do
        if [ -f "$config_file" ]; then
            if command -v python3 &> /dev/null; then
                if python3 "$PROJECT_ROOT/configs/tools/validate_config.py" "$config_file" > /dev/null 2>&1; then
                    log_success "Config valid: $(basename $config_file)"
                else
                    log_error "Config invalid: $(basename $config_file)"
                    ((config_errors++))
                fi
            else
                log_warning "Python3 not available for config validation"
            fi
        fi
    done

    return $config_errors
}

check_submodules() {
    log "Checking git submodules..."

    cd "$PROJECT_ROOT"
    local missing_submodules=0

    while IFS= read -r line; do
        if [[ $line =~ ^[+-][0-9a-f] ]]; then
            local submodule=$(echo "$line" | cut -c3-40)
            local status=$(git submodule status | grep "$submodule" | head -c1)

            case "$status" in
                "-")
                    log_warning "Submodule not initialized: $submodule"
                    ((missing_submodules++))
                    ;;
                "+")
                    log_warning "Submodule has uncommitted changes: $submodule"
                    ;;
                "U")
                    log_error "Submodule conflict: $submodule"
                    ((missing_submodules++))
                    ;;
                " ")
                    log_success "Submodule OK: $submodule"
                    ;;
            esac
        fi
    done < <(git config --file .gitmodules --get-regexp path | awk '{print $2}')

    return $missing_submodules
}

check_cargo_workspace() {
    log "Checking Cargo workspace..."

    if [ -f "$PROJECT_ROOT/Cargo.toml" ]; then
        if command -v cargo &> /dev/null; then
            cd "$PROJECT_ROOT"
            if cargo metadata --format-version 1 > /dev/null 2>&1; then
                log_success "Cargo workspace valid"
                return 0
            else
                log_error "Cargo workspace invalid"
                return 1
            fi
        else
            log_warning "Cargo not installed"
            return 0
        fi
    else
        log_error "Cargo.toml not found"
        return 1
    fi
}

check_operational_dirs() {
    log "Checking operational directories..."

    local missing=0
    for dir in operational_audit operational_hash operational_scripts operational_logs; do
        if [ -d "$PROJECT_ROOT/$dir" ]; then
            log_success "Directory OK: $dir"
        else
            log_warning "Directory missing: $dir"
            ((missing++))
        fi
    done

    return $missing
}

# Generate health report
generate_health_report() {
    log "Generating health report..."

    local timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    local cpu=$(get_cpu_usage)
    local memory=$(get_memory_usage)
    local disk=$(get_disk_usage)
    local load=$(get_load_average)

    cat > "$HEALTH_REPORT" << EOF
{
    "timestamp": "$timestamp",
    "system": {
        "cpu_usage_percent": $cpu,
        "memory_usage_percent": $memory,
        "disk_usage_percent": $disk,
        "load_average": "$load"
    },
    "alerts": [],
    "status": "healthy"
}
EOF

    # Add alerts
    if [ $cpu -gt $CPU_WARN_THRESHOLD ]; then
        log_warning "CPU usage high: ${cpu}%"
        local alerts=$(jq ".alerts += [\"High CPU usage: ${cpu}%\"]" "$HEALTH_REPORT" | tee "$HEALTH_REPORT")
    fi

    if [ $memory -gt $MEMORY_WARN_THRESHOLD ]; then
        log_warning "Memory usage high: ${memory}%"
        local alerts=$(jq ".alerts += [\"High memory usage: ${memory}%\"]" "$HEALTH_REPORT" | tee "$HEALTH_REPORT")
    fi

    if [ $disk -gt $DISK_WARN_THRESHOLD ]; then
        log_warning "Disk usage high: ${disk}%"
        local alerts=$(jq ".alerts += [\"High disk usage: ${disk}%\"]" "$HEALTH_REPORT" | tee "$HEALTH_REPORT")
    fi

    log_success "Health report generated: $HEALTH_REPORT"
}

# Main monitoring routine
main() {
    log "========== AgentAsKit System Monitor =========="
    log "Project root: $PROJECT_ROOT"
    log "Timestamp: $(date)"

    # Run all health checks
    log "\n--- Running Health Checks ---"

    check_git_status || log_warning "Git status check failed"
    check_config_validity || log_warning "Config validation failed"
    check_submodules || log_warning "Submodule check failed"
    check_cargo_workspace || log_warning "Cargo workspace check failed"
    check_operational_dirs || log_warning "Operational directories check failed"

    log "\n--- System Metrics ---"
    log "CPU Usage: $(get_cpu_usage)%"
    log "Memory Usage: $(get_memory_usage)%"
    log "Disk Usage: $(get_disk_usage)%"
    log "Load Average: $(get_load_average)"

    # Generate report
    generate_health_report

    log "\n========== Monitor Complete =========="
}

# Run main function
main "$@"

exit 0
