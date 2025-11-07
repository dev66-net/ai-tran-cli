# Build Guide

## Quick Start

### Build for Current Platform

```bash
make build
```

Binary will be in `dist/ai-tran-cli-<platform>`

### Build for All Platforms

```bash
# Install all Rust targets first
make install-targets

# Build for all platforms
make all
```

Binaries will be in `dist/` directory.

### Test the Binary

```bash
echo "Hello World" | dist/ai-tran-cli-macos-arm -q
```

## Complete Workflow

### 1. First-Time Setup

```bash
# Install cross-compilation targets
make install-targets

# Verify installation
make check-targets
```

### 2. Development Build

```bash
# Quick debug build
make dev

# Run tests
make test
```

### 3. Release Build

```bash
# Build all platforms
make all

# Verify binaries
make verify

# Check sizes
make sizes

# Generate checksums
make checksums
```

### 4. Create Release Archives

```bash
# Create .tar.gz and .zip archives
make release
```

Output in `dist/releases/`

### 5. Install Locally

```bash
# Install to ~/bin
make install

# Test
ai-tran-cli --help
```

## Platform-Specific Builds

### macOS

```bash
# Intel
make macos-intel

# Apple Silicon (ARM)
make macos-arm

# Universal Binary (both architectures)
make macos-universal
```

### Linux

```bash
# AMD64 (x86_64)
make linux-amd64

# ARM64 (aarch64)
make linux-arm
```

### Windows

```bash
# x86 (32-bit)
make windows-x86

# ARM64
make windows-arm64
```

## Common Tasks

### Clean Build

```bash
make clean
make all
```

### Quick Rebuild (Current Platform)

```bash
make quick
```

### Update Version

Edit `Cargo.toml`:
```toml
[package]
version = "0.2.0"
```

Then rebuild:
```bash
make clean
make release
```

## Binary Sizes

After building, check sizes:

```bash
make sizes
```

Example output:
```
Binary Sizes:
  ai-tran-cli-macos-intel       1.6M
  ai-tran-cli-macos-arm         1.4M
  ai-tran-cli-linux-amd64       1.8M
  ai-tran-cli-linux-arm64       1.7M
  ai-tran-cli-windows-x86.exe   2.0M
  ai-tran-cli-windows-arm64.exe 1.9M
```

## Checksums

Generate SHA256 checksums:

```bash
make checksums
cat dist/SHA256SUMS
```

Verify:
```bash
cd dist
shasum -c SHA256SUMS
```

## Distribution

### Option 1: Individual Binaries

```bash
# Build all
make all

# Upload dist/ folder
scp -r dist/ server:/path/to/download/
```

### Option 2: Release Archives

```bash
# Create archives
make release

# Upload releases
scp dist/releases/*.tar.gz server:/path/
scp dist/releases/*.zip server:/path/
scp dist/releases/SHA256SUMS server:/path/
```

### Option 3: GitHub Release

```bash
# Tag release
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# Build release
make release

# Create GitHub release and upload:
# - dist/releases/*.tar.gz
# - dist/releases/*.zip
# - dist/releases/SHA256SUMS
```

## Troubleshooting

### "Target not installed"

```bash
make install-targets
```

### "Linker not found" (Linux cross-compilation)

```bash
# Ubuntu/Debian
sudo apt-get install gcc-aarch64-linux-gnu mingw-w64

# macOS
brew install FiloSottile/musl-cross/musl-cross mingw-w64
```

### Build Too Slow

```bash
# Use more CPU cores
CARGO_FLAGS="--release -j8" make all

# Use sccache
brew install sccache
export RUSTC_WRAPPER=sccache
make all
```

### Binary Too Large

Optimizations are already applied in `.cargo/config.toml`. For further reduction:

```bash
# Install UPX
brew install upx

# Compress binaries
upx --best --lzma dist/ai-tran-cli-*
```

## CI/CD Integration

### GitHub Actions

Create `.github/workflows/build.yml`:

```yaml
name: Build

on:
  push:
    branches: [main]
  pull_request:

jobs:
  build:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install targets
        run: make install-targets
      - name: Build
        run: make all
      - name: Test
        run: make test
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: binaries
          path: dist/
```

### GitLab CI

Create `.gitlab-ci.yml`:

```yaml
build:
  image: rust:latest
  script:
    - make install-targets
    - make all
  artifacts:
    paths:
      - dist/
```

## Make Targets Reference

| Command | Description |
|---------|-------------|
| `make help` | Show all available targets |
| `make all` | Build for all platforms |
| `make build` | Build for current platform |
| `make release` | Create release archives |
| `make install-targets` | Install Rust cross-compilation targets |
| `make test` | Run tests |
| `make clean` | Clean build artifacts |
| `make install` | Install to ~/bin |
| `make uninstall` | Uninstall from ~/bin |
| `make sizes` | Show binary sizes |
| `make verify` | Verify binaries |
| `make checksums` | Generate SHA256 checksums |

## Advanced Usage

### Custom Build Flags

```bash
CARGO_FLAGS="--release --features extra" make build
```

### Custom Output Directory

```bash
BUILD_DIR=custom-dist make all
```

### Parallel Builds

```bash
# Build specific platforms in parallel
make macos-intel &
make macos-arm &
make linux-amd64 &
wait
make checksums
```

## See Also

- [Cross-Compilation Guide](doc/cross-compilation.md) - Detailed cross-compilation documentation
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Rust's package manager
- [rustup](https://rust-lang.github.io/rustup/) - Rust toolchain installer
