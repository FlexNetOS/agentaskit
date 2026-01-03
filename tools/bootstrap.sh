#!/usr/bin/env bash
# AgentAskit Development Environment Bootstrap
# REF: ADR-0005 Modern Tooling Strategy
# Cross-platform: Linux, macOS (Windows: use bootstrap.ps1)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TOOLS_DIR="$SCRIPT_DIR/external"
BIN_DIR="$PROJECT_ROOT/tools/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)   OS="linux" ;;
        Darwin*)  OS="macos" ;;
        MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
        *)        OS="unknown" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)  ARCH="amd64" ;;
        aarch64|arm64) ARCH="arm64" ;;
        *)             ARCH="unknown" ;;
    esac

    log_info "Detected: $OS ($ARCH)"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    local missing=()

    command -v git >/dev/null 2>&1 || missing+=("git")
    command -v cargo >/dev/null 2>&1 || missing+=("rust/cargo")
    command -v python3 >/dev/null 2>&1 || missing+=("python3")

    if [[ ${#missing[@]} -gt 0 ]]; then
        log_error "Missing prerequisites: ${missing[*]}"
        log_info "Please install them first."
        exit 1
    fi

    log_success "All prerequisites found"
}

# Install mise (if not present)
install_mise() {
    if command -v mise >/dev/null 2>&1; then
        log_success "mise already installed: $(mise --version)"
        return 0
    fi

    log_info "Installing mise..."

    if [[ "$OS" == "macos" ]]; then
        if command -v brew >/dev/null 2>&1; then
            brew install mise
        else
            curl https://mise.run | sh
        fi
    else
        curl https://mise.run | sh
    fi

    # Add to PATH for this session
    export PATH="$HOME/.local/bin:$PATH"

    if command -v mise >/dev/null 2>&1; then
        log_success "mise installed: $(mise --version)"
    else
        log_warn "mise installed but not in PATH. Add ~/.local/bin to PATH"
    fi
}

# Install direnv (if not present)
install_direnv() {
    if command -v direnv >/dev/null 2>&1; then
        log_success "direnv already installed: $(direnv --version)"
        return 0
    fi

    log_info "Installing direnv..."

    if [[ "$OS" == "macos" ]]; then
        if command -v brew >/dev/null 2>&1; then
            brew install direnv
        fi
    elif [[ "$OS" == "linux" ]]; then
        # Try package managers
        if command -v apt-get >/dev/null 2>&1; then
            sudo apt-get update && sudo apt-get install -y direnv
        elif command -v dnf >/dev/null 2>&1; then
            sudo dnf install -y direnv
        elif command -v pacman >/dev/null 2>&1; then
            sudo pacman -S --noconfirm direnv
        else
            # Fallback: build from source or download binary
            log_warn "Could not install direnv automatically. Please install manually."
        fi
    fi

    if command -v direnv >/dev/null 2>&1; then
        log_success "direnv installed: $(direnv --version)"
    fi
}

# Clone external repositories
clone_repos() {
    log_info "Cloning external tool repositories..."

    mkdir -p "$TOOLS_DIR"
    cd "$TOOLS_DIR"

    # FlexNetOS repos
    local repos=(
        "https://github.com/FlexNetOS/pixi.git"
        "https://github.com/FlexNetOS/nushell.git"
        "https://github.com/FlexNetOS/coreutils.git"
        "https://github.com/FlexNetOS/syn.git"
    )

    # Third-party repos
    repos+=(
        "https://github.com/dan-t/rusty-tags.git"
        "https://github.com/twpayne/chezmoi.git"
    )

    for repo in "${repos[@]}"; do
        local name=$(basename "$repo" .git)
        if [[ -d "$name" ]]; then
            log_info "  $name already cloned"
        else
            log_info "  Cloning $name..."
            git clone --depth 1 "$repo" 2>/dev/null || log_warn "Failed to clone $name"
        fi
    done

    log_success "Repositories ready"
}

# Build tools from source
build_tools() {
    log_info "Building tools from source..."

    local tools=("nushell" "coreutils" "rusty-tags")

    for tool in "${tools[@]}"; do
        if [[ -d "$TOOLS_DIR/$tool" ]]; then
            log_info "  Building $tool..."
            cd "$TOOLS_DIR/$tool"
            if cargo build --release 2>/dev/null; then
                log_success "  $tool built"
            else
                log_warn "  $tool build failed"
            fi
        fi
    done

    # Chezmoi requires Go
    if command -v go >/dev/null 2>&1 && [[ -d "$TOOLS_DIR/chezmoi" ]]; then
        log_info "  Building chezmoi..."
        cd "$TOOLS_DIR/chezmoi"
        git checkout v2.52.0 2>/dev/null || true
        if GOPROXY=direct go build -o chezmoi . 2>/dev/null; then
            log_success "  chezmoi built"
        else
            log_warn "  chezmoi build failed"
        fi
    fi
}

# Create bin directory with symlinks
create_bin_links() {
    log_info "Creating portable bin directory..."

    mkdir -p "$BIN_DIR"

    # Nushell
    if [[ -f "$TOOLS_DIR/nushell/target/release/nu" ]]; then
        ln -sf "$TOOLS_DIR/nushell/target/release/nu" "$BIN_DIR/nu"
        log_success "  nu -> nushell"
    fi

    # Chezmoi
    if [[ -f "$TOOLS_DIR/chezmoi/chezmoi" ]]; then
        ln -sf "$TOOLS_DIR/chezmoi/chezmoi" "$BIN_DIR/chezmoi"
        log_success "  chezmoi -> chezmoi"
    fi

    # Coreutils (creates multiple binaries)
    if [[ -d "$TOOLS_DIR/coreutils/target/release" ]]; then
        for bin in ls cat cp mv rm mkdir; do
            if [[ -f "$TOOLS_DIR/coreutils/target/release/$bin" ]]; then
                ln -sf "$TOOLS_DIR/coreutils/target/release/$bin" "$BIN_DIR/uu-$bin"
            fi
        done
        log_success "  coreutils (prefixed with uu-)"
    fi

    # Rusty-tags
    if [[ -f "$TOOLS_DIR/rusty-tags/target/release/rusty-tags" ]]; then
        ln -sf "$TOOLS_DIR/rusty-tags/target/release/rusty-tags" "$BIN_DIR/rusty-tags"
        log_success "  rusty-tags"
    fi

    log_info "Add to PATH: export PATH=\"$BIN_DIR:\$PATH\""
}

# Install Python tools (aider)
install_python_tools() {
    log_info "Installing Python tools..."

    if command -v aider >/dev/null 2>&1; then
        log_success "aider already installed"
    else
        log_info "  Installing aider..."
        pip3 install --user --upgrade aider-chat 2>/dev/null || log_warn "aider install failed"
    fi
}

# Configure aider for the project
configure_aider() {
    log_info "Configuring aider..."

    cat > "$PROJECT_ROOT/.aider.conf.yml" << 'EOF'
# Aider configuration for AgentAskit
# REF: ADR-0005 Modern Tooling Strategy

# Model settings (override with env vars or CLI)
# model: claude-3-5-sonnet-20241022

# Git settings
auto-commits: true
dirty-commits: false
attribute-author: true
attribute-committer: false

# Editor settings
edit-format: diff
vim: false

# File handling
gitignore: true
aiderignore: .aiderignore

# Output
dark-mode: true
pretty: true
stream: true
EOF

    # Create .aiderignore
    cat > "$PROJECT_ROOT/.aiderignore" << 'EOF'
# Aider ignore patterns
tools/external/
target/
*.lock
.git/
node_modules/
__pycache__/
*.pyc
.env
.env.local
EOF

    log_success "aider configured"
}

# Set up chezmoi dotfile management
setup_chezmoi() {
    log_info "Setting up chezmoi integration..."

    local chezmoi_bin="$BIN_DIR/chezmoi"

    if [[ ! -x "$chezmoi_bin" ]]; then
        log_warn "chezmoi not found, skipping dotfile setup"
        return 0
    fi

    # Create chezmoi config template
    mkdir -p "$PROJECT_ROOT/configs/chezmoi"

    cat > "$PROJECT_ROOT/configs/chezmoi/chezmoi.toml" << EOF
# Chezmoi configuration for AgentAskit
# Copy to ~/.config/chezmoi/chezmoi.toml

[data]
    project = "agentaskit"

[git]
    autoCommit = false
    autoPush = false

[diff]
    command = "delta"
    pager = "delta"
EOF

    log_success "chezmoi config template created at configs/chezmoi/"
}

# Link nushell config
setup_nushell_config() {
    log_info "Setting up nushell configuration..."

    local nu_config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/nushell"
    local project_config="$PROJECT_ROOT/configs/nushell/config.nu"

    if [[ ! -f "$project_config" ]]; then
        log_warn "Nushell config not found"
        return 0
    fi

    mkdir -p "$nu_config_dir"

    # Create a loader that sources the project config
    cat > "$nu_config_dir/agentaskit.nu" << EOF
# AgentAskit nushell configuration loader
# Source this from your main config.nu:
#   source ~/.config/nushell/agentaskit.nu

\$env.AGENTASKIT_ROOT = "$PROJECT_ROOT"
source "$project_config"
EOF

    log_success "Nushell config created at $nu_config_dir/agentaskit.nu"
    log_info "Add to your ~/.config/nushell/config.nu:"
    log_info "  source ~/.config/nushell/agentaskit.nu"
}

# Allow direnv
allow_direnv() {
    log_info "Allowing direnv..."

    cd "$PROJECT_ROOT"
    if command -v direnv >/dev/null 2>&1; then
        direnv allow . 2>/dev/null || true
        log_success "direnv allowed"
    fi
}

# Print summary
print_summary() {
    echo ""
    echo "============================================"
    echo "  AgentAskit Development Environment Setup"
    echo "============================================"
    echo ""
    log_info "Tools directory: $TOOLS_DIR"
    log_info "Bin directory:   $BIN_DIR"
    echo ""
    echo "Next steps:"
    echo "  1. Add to your shell profile:"
    echo "     export PATH=\"$BIN_DIR:\$PATH\""
    echo ""
    echo "  2. For bash/zsh, add direnv hook:"
    echo "     eval \"\$(direnv hook bash)\"  # or zsh"
    echo ""
    echo "  3. For nushell, add to config.nu:"
    echo "     source ~/.config/nushell/agentaskit.nu"
    echo ""
    echo "  4. Configure aider API key:"
    echo "     export ANTHROPIC_API_KEY=sk-ant-..."
    echo ""
}

# Main
main() {
    echo "AgentAskit Bootstrap Script"
    echo "==========================="
    echo ""

    detect_os
    check_prerequisites

    # Parse args
    local skip_build=false
    local skip_clone=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --skip-build) skip_build=true ;;
            --skip-clone) skip_clone=true ;;
            --help|-h)
                echo "Usage: $0 [--skip-build] [--skip-clone]"
                exit 0
                ;;
        esac
        shift
    done

    install_mise
    install_direnv

    if [[ "$skip_clone" != true ]]; then
        clone_repos
    fi

    if [[ "$skip_build" != true ]]; then
        build_tools
    fi

    create_bin_links
    install_python_tools
    configure_aider
    setup_chezmoi
    setup_nushell_config
    allow_direnv

    print_summary

    log_success "Bootstrap complete!"
}

main "$@"
