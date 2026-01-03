#!/bin/bash
# Integration Test Runner for AgentasKit
# Runs comprehensive tests for all integrations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
SKIPPED=0

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     AgentasKit Integration Test Suite                      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo

# Function to run a test and track results
run_test() {
    local name="$1"
    local command="$2"

    echo -n "  Testing: $name... "

    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}✗ FAILED${NC}"
        ((FAILED++))
        return 1
    fi
}

# Function to check if a file exists
check_file() {
    local file="$1"
    [ -f "$file" ]
}

# Function to check if a directory exists
check_dir() {
    local dir="$1"
    [ -d "$dir" ]
}

echo -e "${YELLOW}[1/5] Checking Submodules${NC}"
echo "────────────────────────────────────────"
run_test "agentgateway submodule exists" "check_dir agentgateway"
run_test "agentgateway is git repo" "check_file agentgateway/.git || check_dir agentgateway/.git"
run_test "wiki-rs submodule exists" "check_dir wiki-rs"
run_test "wiki-rs is git repo" "check_file wiki-rs/.git || check_dir wiki-rs/.git"
run_test ".gitmodules configured" "check_file .gitmodules && grep -q agentgateway .gitmodules && grep -q wiki-rs .gitmodules"
echo

echo -e "${YELLOW}[2/5] Checking Integration Modules${NC}"
echo "────────────────────────────────────────"

# Agentgateway integration
run_test "agentgateway/Cargo.toml exists" "check_file integrations/agentgateway/Cargo.toml"
run_test "agentgateway/src/lib.rs exists" "check_file integrations/agentgateway/src/lib.rs"
run_test "agentgateway/src/config.rs exists" "check_file integrations/agentgateway/src/config.rs"
run_test "agentgateway/src/gateway.rs exists" "check_file integrations/agentgateway/src/gateway.rs"
run_test "agentgateway/src/mcp.rs exists" "check_file integrations/agentgateway/src/mcp.rs"
run_test "agentgateway/src/a2a.rs exists" "check_file integrations/agentgateway/src/a2a.rs"
run_test "agentgateway/src/auth.rs exists" "check_file integrations/agentgateway/src/auth.rs"
run_test "agentgateway/src/routing.rs exists" "check_file integrations/agentgateway/src/routing.rs"
run_test "agentgateway/src/ratelimit.rs exists" "check_file integrations/agentgateway/src/ratelimit.rs"
run_test "agentgateway/src/observability.rs exists" "check_file integrations/agentgateway/src/observability.rs"
run_test "agentgateway/src/xds.rs exists" "check_file integrations/agentgateway/src/xds.rs"

# Wiki-rs integration
run_test "wiki-rs/Cargo.toml exists" "check_file integrations/wiki-rs/Cargo.toml"
run_test "wiki-rs/src/lib.rs exists" "check_file integrations/wiki-rs/src/lib.rs"
run_test "wiki-rs/src/config.rs exists" "check_file integrations/wiki-rs/src/config.rs"
run_test "wiki-rs/src/generator.rs exists" "check_file integrations/wiki-rs/src/generator.rs"
run_test "wiki-rs/src/llm.rs exists" "check_file integrations/wiki-rs/src/llm.rs"
run_test "wiki-rs/src/output.rs exists" "check_file integrations/wiki-rs/src/output.rs"
echo

echo -e "${YELLOW}[3/5] Checking Configuration Files${NC}"
echo "────────────────────────────────────────"
run_test "agentgateway/local.yaml exists" "check_file configs/agentgateway/local.yaml"
run_test "agentgateway/production.yaml exists" "check_file configs/agentgateway/production.yaml"
run_test "wiki/local.toml exists" "check_file configs/wiki/local.toml"
run_test "wiki/production.toml exists" "check_file configs/wiki/production.toml"
echo

echo -e "${YELLOW}[4/5] Checking CI/CD Workflows${NC}"
echo "────────────────────────────────────────"
run_test "agentgateway-build.yml exists" "check_file .github/workflows/agentgateway-build.yml"
run_test "agentgateway-build.yml has jobs" "grep -q 'jobs:' .github/workflows/agentgateway-build.yml"
run_test "wiki-build.yml exists" "check_file .github/workflows/wiki-build.yml"
run_test "wiki-build.yml has jobs" "grep -q 'jobs:' .github/workflows/wiki-build.yml"
run_test "mdBook action configured" "grep -q 'peaceiris/actions-mdbook' .github/workflows/agentgateway-build.yml || grep -q 'peaceiris/actions-mdbook' .github/workflows/wiki-build.yml"
echo

echo -e "${YELLOW}[5/5] Checking Documentation${NC}"
echo "────────────────────────────────────────"
run_test "agentgateway README exists" "check_file integrations/agentgateway/README.md"
run_test "agentgateway README has content" "[ \$(wc -l < integrations/agentgateway/README.md) -gt 50 ]"
run_test "agentgateway DEPENDENCY_ANALYSIS exists" "check_file integrations/agentgateway/DEPENDENCY_ANALYSIS.md"
run_test "wiki-rs README exists" "check_file integrations/wiki-rs/README.md"
run_test "wiki-rs README has content" "[ \$(wc -l < integrations/wiki-rs/README.md) -gt 50 ]"
echo

# Summary
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}                     TEST SUMMARY                           ${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo
TOTAL=$((PASSED + FAILED + SKIPPED))
echo -e "  Total Tests:  $TOTAL"
echo -e "  ${GREEN}Passed:       $PASSED${NC}"
echo -e "  ${RED}Failed:       $FAILED${NC}"
echo -e "  ${YELLOW}Skipped:      $SKIPPED${NC}"
echo

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║              ALL TESTS PASSED! ✓                           ║${NC}"
    echo -e "${GREEN}║         Integration health: 100% HEALTHY                   ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${RED}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║              SOME TESTS FAILED! ✗                          ║${NC}"
    echo -e "${RED}║         Please review the failures above                   ║${NC}"
    echo -e "${RED}╚════════════════════════════════════════════════════════════╝${NC}"
    exit 1
fi
