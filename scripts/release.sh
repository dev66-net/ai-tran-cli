#!/bin/bash
# Release automation script for ai-tran-cli
# Usage: ./scripts/release.sh <version>

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

warn() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Check arguments
if [ -z "$1" ]; then
    error "Usage: $0 <version>\nExample: $0 1.0.0"
fi

VERSION=$1
TAG="v$VERSION"

# Validate version format (semantic versioning)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
    error "Invalid version format. Use semantic versioning (e.g., 1.0.0, 1.0.0-beta.1)"
fi

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    error "Cargo.toml not found. Run this script from the project root."
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    warn "Working directory is not clean. Uncommitted changes:"
    git status --short
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
    warn "You are on branch '$CURRENT_BRANCH', not 'main' or 'master'"
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    error "Tag $TAG already exists. Delete it first if you want to recreate:\n  git tag -d $TAG\n  git push origin :refs/tags/$TAG"
fi

# Confirm release
echo
info "Release Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Version: $VERSION"
echo "  Tag:     $TAG"
echo "  Branch:  $CURRENT_BRANCH"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo
read -p "Create release $TAG? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    info "Release cancelled"
    exit 0
fi

# Update Cargo.toml version
info "Updating Cargo.toml version..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
else
    # Linux
    sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
fi
success "Updated Cargo.toml to version $VERSION"

# Verify Cargo.lock updates
info "Updating Cargo.lock..."
cargo check --quiet
success "Cargo.lock updated"

# Run tests
info "Running tests..."
if cargo test --quiet --release; then
    success "All tests passed"
else
    error "Tests failed. Fix tests before releasing."
fi

# Build locally to verify
info "Building release binary..."
if cargo build --release --quiet; then
    success "Release build successful"
else
    error "Build failed. Fix build errors before releasing."
fi

# Update CHANGELOG (if exists)
if [ -f "CHANGELOG.md" ]; then
    info "Please update CHANGELOG.md before proceeding"
    read -p "Open CHANGELOG.md in editor? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ${EDITOR:-vim} CHANGELOG.md
    fi
fi

# Commit version bump
info "Committing version bump..."
git add Cargo.toml Cargo.lock
if [ -f "CHANGELOG.md" ]; then
    git add CHANGELOG.md
fi
git commit -m "Bump version to $VERSION"
success "Version bump committed"

# Create annotated tag
info "Creating git tag $TAG..."
git tag -a "$TAG" -m "Release $TAG"
success "Tag $TAG created"

# Show what will be pushed
echo
info "Ready to push:"
echo "  Commits: $(git rev-parse --short HEAD)"
echo "  Tag:     $TAG"
echo

# Push to remote
read -p "Push to origin? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    warn "Skipping push. You can push manually with:"
    echo "  git push origin $CURRENT_BRANCH"
    echo "  git push origin $TAG"
    exit 0
fi

info "Pushing to origin..."
git push origin "$CURRENT_BRANCH"
success "Pushed commits to origin/$CURRENT_BRANCH"

git push origin "$TAG"
success "Pushed tag $TAG to origin"

# Success message
echo
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ Release $TAG created successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo
info "Next steps:"
echo "  1. Monitor GitHub Actions: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"
echo "  2. Wait for builds to complete (~10-15 minutes)"
echo "  3. Check release: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/releases/tag/$TAG"
echo
