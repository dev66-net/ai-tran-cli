#!/bin/bash
# Check CI status for GitHub Actions
# Usage: ./scripts/check-ci.sh [workflow-name]

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Get repository info
REPO=$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')
BRANCH=$(git branch --show-current)

if [ -z "$REPO" ]; then
    echo -e "${RED}Error: Not a GitHub repository${NC}"
    exit 1
fi

# Workflow name (default to all)
WORKFLOW=${1:-""}

echo -e "${BLUE}Checking CI status for ${REPO}${NC}"
echo -e "${BLUE}Branch: ${BRANCH}${NC}"
echo

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${YELLOW}GitHub CLI (gh) not found. Install with:${NC}"
    echo "  brew install gh    # macOS"
    echo "  Or visit: https://cli.github.com/"
    echo
    echo -e "${BLUE}Manual check:${NC}"
    echo "  https://github.com/${REPO}/actions"
    exit 1
fi

# List recent workflow runs
if [ -z "$WORKFLOW" ]; then
    echo -e "${BLUE}Recent workflow runs:${NC}"
    gh run list --limit 10
else
    echo -e "${BLUE}Recent runs for ${WORKFLOW}:${NC}"
    gh run list --workflow "$WORKFLOW" --limit 10
fi

echo
echo -e "${BLUE}View in browser:${NC}"
echo "  https://github.com/${REPO}/actions"
