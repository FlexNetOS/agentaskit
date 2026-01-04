#!/usr/bin/env bash
#
# AgentAsKit System Backup Script
# Performs comprehensive backup of critical system components
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="${PROJECT_ROOT}/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="agentaskit_backup_${TIMESTAMP}"
FULL_BACKUP_PATH="${BACKUP_DIR}/${BACKUP_NAME}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RETENTION_DAYS=30
BACKUP_COMPRESSED=true
BACKUP_LEVEL="full"  # full, incremental
EXCLUDE_PATTERNS=(
    ".git"
    "target/"
    ".cargo"
    "node_modules/"
    ".env"
    "secrets/"
    "*.log"
)

# Logging
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $*"
}

log_error() {
    echo -e "${RED}[✗]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[⚠]${NC} $*"
}

# Create backup directory
mkdir -p "$BACKUP_DIR"

log "Starting AgentAsKit backup: $BACKUP_NAME"
log "Backup location: $FULL_BACKUP_PATH"

# Create backup directory structure
mkdir -p "$FULL_BACKUP_PATH"/{configs,data,artifacts,operational}

# Backup configurations
log "Backing up configurations..."
cp -r "$PROJECT_ROOT/configs" "$FULL_BACKUP_PATH/" 2>/dev/null || log_warning "Failed to backup configs"
log_success "Configurations backed up"

# Backup operational artifacts
log "Backing up operational artifacts..."
if [ -d "$PROJECT_ROOT/operational_audit" ]; then
    cp -r "$PROJECT_ROOT/operational_audit" "$FULL_BACKUP_PATH/operational/" 2>/dev/null
    log_success "Audit logs backed up"
fi

if [ -d "$PROJECT_ROOT/operational_hash" ]; then
    cp -r "$PROJECT_ROOT/operational_hash" "$FULL_BACKUP_PATH/operational/" 2>/dev/null
    log_success "Hash data backed up"
fi

if [ -d "$PROJECT_ROOT/operational_logs" ]; then
    cp -r "$PROJECT_ROOT/operational_logs" "$FULL_BACKUP_PATH/operational/" 2>/dev/null
    log_success "Operational logs backed up"
fi

# Backup system integrity snapshot
log "Creating system integrity snapshot..."
if [ -f "$PROJECT_ROOT/system_integrity.json" ]; then
    cp "$PROJECT_ROOT/system_integrity.json" "$FULL_BACKUP_PATH/system_integrity.json"
    log_success "System integrity snapshot backed up"
fi

# Backup Cargo.lock
log "Backing up dependency locks..."
if [ -f "$PROJECT_ROOT/Cargo.lock" ]; then
    cp "$PROJECT_ROOT/Cargo.lock" "$FULL_BACKUP_PATH/Cargo.lock"
    log_success "Cargo.lock backed up"
fi

# Create backup manifest
log "Creating backup manifest..."
cat > "$FULL_BACKUP_PATH/MANIFEST.json" << EOF
{
    "backup_name": "$BACKUP_NAME",
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "backup_level": "$BACKUP_LEVEL",
    "project_root": "$PROJECT_ROOT",
    "contents": [
        "configs/",
        "operational/",
        "system_integrity.json",
        "Cargo.lock",
        "MANIFEST.json"
    ],
    "retention_days": $RETENTION_DAYS
}
EOF
log_success "Manifest created"

# Compress backup if enabled
if [ "$BACKUP_COMPRESSED" = true ]; then
    log "Compressing backup..."
    tar -czf "${FULL_BACKUP_PATH}.tar.gz" -C "$BACKUP_DIR" "$BACKUP_NAME"

    # Remove uncompressed directory
    rm -rf "$FULL_BACKUP_PATH"

    BACKUP_SIZE=$(du -sh "${FULL_BACKUP_PATH}.tar.gz" | cut -f1)
    log_success "Backup compressed to ${FULL_BACKUP_PATH}.tar.gz (${BACKUP_SIZE})"
fi

# Cleanup old backups
log "Cleaning up old backups (retention: ${RETENTION_DAYS} days)..."
find "$BACKUP_DIR" -type f -name "agentaskit_backup_*.tar.gz" -mtime +$RETENTION_DAYS -delete
find "$BACKUP_DIR" -type d -name "agentaskit_backup_*" -mtime +$RETENTION_DAYS -delete
log_success "Old backups removed"

# Generate backup summary
log "\n========== BACKUP SUMMARY =========="
log "Name: $BACKUP_NAME"
log "Location: $([ "$BACKUP_COMPRESSED" = true ] && echo "${FULL_BACKUP_PATH}.tar.gz" || echo "$FULL_BACKUP_PATH")"
log "Size: $(du -sh $([ "$BACKUP_COMPRESSED" = true ] && echo "${FULL_BACKUP_PATH}.tar.gz" || echo "$FULL_BACKUP_PATH") | cut -f1)"
log "Date: $(date)"
log "===================================="

log_success "Backup completed successfully!"

# Create recovery instructions
cat > "$BACKUP_DIR/RECOVERY_INSTRUCTIONS.md" << 'EOF'
# AgentAsKit Backup Recovery

## Restoring from Backup

### 1. Decompress (if compressed)
```bash
tar -xzf agentaskit_backup_YYYYMMDD_HHMMSS.tar.gz
```

### 2. Restore Configurations
```bash
cp -r agentaskit_backup_YYYYMMDD_HHMMSS/configs/* /path/to/agentaskit/configs/
```

### 3. Restore Operational Data
```bash
cp -r agentaskit_backup_YYYYMMDD_HHMMSS/operational/* /path/to/agentaskit/
```

### 4. Restore Dependency Lock
```bash
cp agentaskit_backup_YYYYMMDD_HHMMSS/Cargo.lock /path/to/agentaskit/
```

### 5. Verify Restoration
```bash
python3 /path/to/agentaskit/operational_hash/generate_integrity.py
cat /path/to/agentaskit/system_integrity.json
```

## Backup Retention Policy
- Backups are retained for 30 days by default
- Older backups are automatically removed
- Update RETENTION_DAYS in backup.sh to change

## Emergency Recovery
For manual recovery, contact operations team with backup manifest.
EOF

exit 0
