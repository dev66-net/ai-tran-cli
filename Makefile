# AI Translation CLI - Multi-platform Build Makefile
# Supports: macOS (Intel/ARM), Linux (AMD64/ARM), Windows (x86/ARM64)

# Project info
BINARY_NAME := ai-tran-cli
VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_TIME := $(shell date +%Y%m%d_%H%M%S)

# Build directories
BUILD_DIR := dist
RELEASE_DIR := target/release
TARGET_DIR := target

# Rust targets
TARGET_MACOS_INTEL := x86_64-apple-darwin
TARGET_MACOS_ARM := aarch64-apple-darwin
TARGET_LINUX_AMD64 := x86_64-unknown-linux-gnu
TARGET_LINUX_ARM := aarch64-unknown-linux-gnu
TARGET_WINDOWS_X86 := i686-pc-windows-gnu
TARGET_WINDOWS_X64 := x86_64-pc-windows-msvc
TARGET_WINDOWS_ARM64 := aarch64-pc-windows-msvc

# Output binary names
BIN_MACOS_INTEL := $(BINARY_NAME)-macos-intel
BIN_MACOS_ARM := $(BINARY_NAME)-macos-arm
BIN_LINUX_AMD64 := $(BINARY_NAME)-linux-amd64
BIN_LINUX_ARM := $(BINARY_NAME)-linux-arm64
BIN_WINDOWS_X86 := $(BINARY_NAME)-windows-x86.exe
BIN_WINDOWS_X64 := $(BINARY_NAME)-windows-x64.exe
BIN_WINDOWS_ARM64 := $(BINARY_NAME)-windows-arm64.exe

# Cargo flags
CARGO_FLAGS := --release
CARGO_BUILD := cargo build $(CARGO_FLAGS)

# Detect current platform
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
    ifeq ($(UNAME_M),arm64)
        CURRENT_TARGET := $(TARGET_MACOS_ARM)
        CURRENT_BIN := $(BIN_MACOS_ARM)
    else
        CURRENT_TARGET := $(TARGET_MACOS_INTEL)
        CURRENT_BIN := $(BIN_MACOS_INTEL)
    endif
else ifeq ($(UNAME_S),Linux)
    ifeq ($(UNAME_M),aarch64)
        CURRENT_TARGET := $(TARGET_LINUX_ARM)
        CURRENT_BIN := $(BIN_LINUX_ARM)
    else
        CURRENT_TARGET := $(TARGET_LINUX_AMD64)
        CURRENT_BIN := $(BIN_LINUX_AMD64)
    endif
endif

# Colors for output
COLOR_RESET := \033[0m
COLOR_BOLD := \033[1m
COLOR_GREEN := \033[32m
COLOR_YELLOW := \033[33m
COLOR_BLUE := \033[34m

# Default target
.PHONY: all
all: info macos-intel macos-arm linux-amd64 linux-arm windows-x86 windows-x64 windows-arm64 checksums
	@echo "$(COLOR_GREEN)$(COLOR_BOLD)✓ All platforms built successfully!$(COLOR_RESET)"
	@echo "$(COLOR_BLUE)Build artifacts in: $(BUILD_DIR)/$(COLOR_RESET)"
	@ls -lh $(BUILD_DIR)/

.PHONY: info
info:
	@echo "$(COLOR_BOLD)═══════════════════════════════════════════════════$(COLOR_RESET)"
	@echo "$(COLOR_BOLD)  AI Translation CLI - Multi-platform Build$(COLOR_RESET)"
	@echo "$(COLOR_BOLD)═══════════════════════════════════════════════════$(COLOR_RESET)"
	@echo "$(COLOR_BLUE)Version:$(COLOR_RESET)        $(VERSION)"
	@echo "$(COLOR_BLUE)Build Time:$(COLOR_RESET)     $(BUILD_TIME)"
	@echo "$(COLOR_BLUE)Current Platform:$(COLOR_RESET) $(UNAME_S) $(UNAME_M)"
	@echo "$(COLOR_BOLD)───────────────────────────────────────────────────$(COLOR_RESET)"

# Install all required Rust targets
.PHONY: install-targets
install-targets:
	@echo "$(COLOR_YELLOW)Installing Rust cross-compilation targets...$(COLOR_RESET)"
	rustup target add $(TARGET_MACOS_INTEL)
	rustup target add $(TARGET_MACOS_ARM)
	rustup target add $(TARGET_LINUX_AMD64)
	rustup target add $(TARGET_LINUX_ARM)
	rustup target add $(TARGET_WINDOWS_X86)
	rustup target add $(TARGET_WINDOWS_X64)
	rustup target add $(TARGET_WINDOWS_ARM64)
	@echo "$(COLOR_GREEN)✓ All targets installed$(COLOR_RESET)"

# Check if targets are installed
.PHONY: check-targets
check-targets:
	@echo "$(COLOR_BLUE)Checking installed targets...$(COLOR_RESET)"
	@rustup target list --installed | grep -E "(darwin|linux|windows)" || echo "$(COLOR_YELLOW)Run 'make install-targets' to install required targets$(COLOR_RESET)"

# Current platform build
.PHONY: build
build: prepare
	@echo "$(COLOR_YELLOW)Building for current platform ($(CURRENT_TARGET))...$(COLOR_RESET)"
	$(CARGO_BUILD)
	@mkdir -p $(BUILD_DIR)
	@cp $(RELEASE_DIR)/$(BINARY_NAME) $(BUILD_DIR)/$(CURRENT_BIN)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(CURRENT_BIN)$(COLOR_RESET)"

# macOS Intel (x86_64)
.PHONY: macos-intel
macos-intel: prepare
	@echo "$(COLOR_YELLOW)Building for macOS Intel...$(COLOR_RESET)"
	$(CARGO_BUILD) --target=$(TARGET_MACOS_INTEL)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(TARGET_MACOS_INTEL)/release/$(BINARY_NAME) $(BUILD_DIR)/$(BIN_MACOS_INTEL)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BIN_MACOS_INTEL)$(COLOR_RESET)"

# macOS ARM (Apple Silicon)
.PHONY: macos-arm
macos-arm: prepare
	@echo "$(COLOR_YELLOW)Building for macOS ARM (Apple Silicon)...$(COLOR_RESET)"
	$(CARGO_BUILD) --target=$(TARGET_MACOS_ARM)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(TARGET_MACOS_ARM)/release/$(BINARY_NAME) $(BUILD_DIR)/$(BIN_MACOS_ARM)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BIN_MACOS_ARM)$(COLOR_RESET)"

# macOS Universal Binary
.PHONY: macos-universal
macos-universal: macos-intel macos-arm
	@echo "$(COLOR_YELLOW)Creating macOS Universal Binary...$(COLOR_RESET)"
	@lipo -create \
		$(BUILD_DIR)/$(BIN_MACOS_INTEL) \
		$(BUILD_DIR)/$(BIN_MACOS_ARM) \
		-output $(BUILD_DIR)/$(BINARY_NAME)-macos-universal
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BINARY_NAME)-macos-universal$(COLOR_RESET)"

# Linux AMD64 (x86_64)
.PHONY: linux-amd64
linux-amd64: prepare
	@echo "$(COLOR_YELLOW)Building for Linux AMD64...$(COLOR_RESET)"
	$(CARGO_BUILD) --target=$(TARGET_LINUX_AMD64)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(TARGET_LINUX_AMD64)/release/$(BINARY_NAME) $(BUILD_DIR)/$(BIN_LINUX_AMD64)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BIN_LINUX_AMD64)$(COLOR_RESET)"

# Linux ARM64 (aarch64)
.PHONY: linux-arm
linux-arm: prepare
	@echo "$(COLOR_YELLOW)Building for Linux ARM64...$(COLOR_RESET)"
	$(CARGO_BUILD) --target=$(TARGET_LINUX_ARM)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(TARGET_LINUX_ARM)/release/$(BINARY_NAME) $(BUILD_DIR)/$(BIN_LINUX_ARM)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BIN_LINUX_ARM)$(COLOR_RESET)"

# Windows x86 (32-bit)
.PHONY: windows-x86
windows-x86: prepare
	@echo "$(COLOR_YELLOW)Building for Windows x86 (32-bit)...$(COLOR_RESET)"
	$(CARGO_BUILD) --target=$(TARGET_WINDOWS_X86)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(TARGET_WINDOWS_X86)/release/$(BINARY_NAME).exe $(BUILD_DIR)/$(BIN_WINDOWS_X86)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BIN_WINDOWS_X86)$(COLOR_RESET)"

# Windows x64 (64-bit)
.PHONY: windows-x64
windows-x64: prepare
	@echo "$(COLOR_YELLOW)Building for Windows x64 (64-bit)...$(COLOR_RESET)"
	$(CARGO_BUILD) --target=$(TARGET_WINDOWS_X64)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(TARGET_WINDOWS_X64)/release/$(BINARY_NAME).exe $(BUILD_DIR)/$(BIN_WINDOWS_X64)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BIN_WINDOWS_X64)$(COLOR_RESET)"

# Windows ARM64
.PHONY: windows-arm64
windows-arm64: prepare
	@echo "$(COLOR_YELLOW)Building for Windows ARM64...$(COLOR_RESET)"
	$(CARGO_BUILD) --target=$(TARGET_WINDOWS_ARM64)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(TARGET_WINDOWS_ARM64)/release/$(BINARY_NAME).exe $(BUILD_DIR)/$(BIN_WINDOWS_ARM64)
	@echo "$(COLOR_GREEN)✓ Built: $(BUILD_DIR)/$(BIN_WINDOWS_ARM64)$(COLOR_RESET)"

# Prepare build directory
.PHONY: prepare
prepare:
	@mkdir -p $(BUILD_DIR)

# Generate checksums
.PHONY: checksums
checksums:
	@echo "$(COLOR_YELLOW)Generating checksums...$(COLOR_RESET)"
	@cd $(BUILD_DIR) && shasum -a 256 $(BINARY_NAME)-* > SHA256SUMS
	@echo "$(COLOR_GREEN)✓ Checksums saved to: $(BUILD_DIR)/SHA256SUMS$(COLOR_RESET)"

# Create release archives
.PHONY: release
release: all
	@echo "$(COLOR_YELLOW)Creating release archives...$(COLOR_RESET)"
	@mkdir -p $(BUILD_DIR)/releases
	@cd $(BUILD_DIR) && tar -czf releases/$(BINARY_NAME)-$(VERSION)-macos-intel.tar.gz $(BIN_MACOS_INTEL) ../README.md ../LICENSE 2>/dev/null || true
	@cd $(BUILD_DIR) && tar -czf releases/$(BINARY_NAME)-$(VERSION)-macos-arm.tar.gz $(BIN_MACOS_ARM) ../README.md ../LICENSE 2>/dev/null || true
	@cd $(BUILD_DIR) && tar -czf releases/$(BINARY_NAME)-$(VERSION)-linux-amd64.tar.gz $(BIN_LINUX_AMD64) ../README.md ../LICENSE 2>/dev/null || true
	@cd $(BUILD_DIR) && tar -czf releases/$(BINARY_NAME)-$(VERSION)-linux-arm64.tar.gz $(BIN_LINUX_ARM) ../README.md ../LICENSE 2>/dev/null || true
	@cd $(BUILD_DIR) && zip -q releases/$(BINARY_NAME)-$(VERSION)-windows-x86.zip $(BIN_WINDOWS_X86) ../README.md ../LICENSE 2>/dev/null || true
	@cd $(BUILD_DIR) && zip -q releases/$(BINARY_NAME)-$(VERSION)-windows-arm64.zip $(BIN_WINDOWS_ARM64) ../README.md ../LICENSE 2>/dev/null || true
	@cd $(BUILD_DIR)/releases && shasum -a 256 * > SHA256SUMS
	@echo "$(COLOR_GREEN)✓ Release archives created in: $(BUILD_DIR)/releases/$(COLOR_RESET)"
	@ls -lh $(BUILD_DIR)/releases/

# Quick builds (current platform only)
.PHONY: quick
quick:
	@echo "$(COLOR_YELLOW)Quick build for current platform...$(COLOR_RESET)"
	cargo build --release
	@echo "$(COLOR_GREEN)✓ Binary: $(RELEASE_DIR)/$(BINARY_NAME)$(COLOR_RESET)"

# Development build (debug mode)
.PHONY: dev
dev:
	@echo "$(COLOR_YELLOW)Development build (debug mode)...$(COLOR_RESET)"
	cargo build
	@echo "$(COLOR_GREEN)✓ Binary: target/debug/$(BINARY_NAME)$(COLOR_RESET)"

# Test build
.PHONY: test
test:
	@echo "$(COLOR_YELLOW)Running tests...$(COLOR_RESET)"
	cargo test --release
	@echo "$(COLOR_GREEN)✓ Tests passed$(COLOR_RESET)"

# Clean build artifacts
.PHONY: clean
clean:
	@echo "$(COLOR_YELLOW)Cleaning build artifacts...$(COLOR_RESET)"
	cargo clean
	rm -rf $(BUILD_DIR)
	@echo "$(COLOR_GREEN)✓ Clean complete$(COLOR_RESET)"

# Show binary sizes
.PHONY: sizes
sizes:
	@echo "$(COLOR_BOLD)Binary Sizes:$(COLOR_RESET)"
	@ls -lh $(BUILD_DIR)/$(BINARY_NAME)-* 2>/dev/null | awk '{printf "  %-35s %8s\n", $$9, $$5}' || echo "  No binaries found. Run 'make all' first."

# Verify binaries
.PHONY: verify
verify:
	@echo "$(COLOR_BOLD)Verifying binaries...$(COLOR_RESET)"
	@for bin in $(BUILD_DIR)/$(BINARY_NAME)-*; do \
		if [ -f "$$bin" ]; then \
			echo "$(COLOR_BLUE)$$bin:$(COLOR_RESET)"; \
			file "$$bin" | sed 's/^/  /'; \
		fi \
	done

# Install to local bin (current platform)
.PHONY: install
install: build
	@echo "$(COLOR_YELLOW)Installing to ~/bin/$(BINARY_NAME)...$(COLOR_RESET)"
	@mkdir -p ~/bin
	@cp $(BUILD_DIR)/$(CURRENT_BIN) ~/bin/$(BINARY_NAME)
	@chmod +x ~/bin/$(BINARY_NAME)
	@echo "$(COLOR_GREEN)✓ Installed to: ~/bin/$(BINARY_NAME)$(COLOR_RESET)"
	@echo "$(COLOR_BLUE)Make sure ~/bin is in your PATH$(COLOR_RESET)"

# Uninstall from local bin
.PHONY: uninstall
uninstall:
	@echo "$(COLOR_YELLOW)Uninstalling from ~/bin/$(BINARY_NAME)...$(COLOR_RESET)"
	@rm -f ~/bin/$(BINARY_NAME)
	@echo "$(COLOR_GREEN)✓ Uninstalled$(COLOR_RESET)"

# Show help
.PHONY: help
help:
	@echo "$(COLOR_BOLD)AI Translation CLI - Build Targets$(COLOR_RESET)"
	@echo ""
	@echo "$(COLOR_BOLD)Building:$(COLOR_RESET)"
	@echo "  make all              - Build for all platforms"
	@echo "  make build            - Build for current platform"
	@echo "  make quick            - Quick build (no dist copy)"
	@echo "  make dev              - Development build (debug mode)"
	@echo ""
	@echo "$(COLOR_BOLD)Platform-specific:$(COLOR_RESET)"
	@echo "  make macos-intel      - Build for macOS Intel"
	@echo "  make macos-arm        - Build for macOS ARM (Apple Silicon)"
	@echo "  make macos-universal  - Build macOS Universal Binary"
	@echo "  make linux-amd64      - Build for Linux AMD64"
	@echo "  make linux-arm        - Build for Linux ARM64"
	@echo "  make windows-x86      - Build for Windows x86 (32-bit)"
	@echo "  make windows-arm64    - Build for Windows ARM64"
	@echo ""
	@echo "$(COLOR_BOLD)Release:$(COLOR_RESET)"
	@echo "  make release          - Create release archives"
	@echo "  make checksums        - Generate SHA256 checksums"
	@echo ""
	@echo "$(COLOR_BOLD)Setup:$(COLOR_RESET)"
	@echo "  make install-targets  - Install all Rust targets"
	@echo "  make check-targets    - Check installed targets"
	@echo ""
	@echo "$(COLOR_BOLD)Utilities:$(COLOR_RESET)"
	@echo "  make test             - Run tests"
	@echo "  make clean            - Clean build artifacts"
	@echo "  make sizes            - Show binary sizes"
	@echo "  make verify           - Verify binaries"
	@echo "  make install          - Install to ~/bin"
	@echo "  make uninstall        - Uninstall from ~/bin"
	@echo ""
	@echo "$(COLOR_BOLD)Info:$(COLOR_RESET)"
	@echo "  Version: $(VERSION)"
	@echo "  Current Platform: $(UNAME_S) $(UNAME_M)"
	@echo ""
