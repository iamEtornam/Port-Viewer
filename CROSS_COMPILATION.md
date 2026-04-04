# Cross-Compilation Guide

This guide explains how to build `port-viewer` for macOS, Linux, and Windows platforms.

## Prerequisites

Install the Rust cross-compilation toolchain:

```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Additional Tools for Cross-Compiling

#### macOS Host (for Linux/Windows)

For Linux targets:
```bash
# Install cross-compilation linker
cargo install cross
```

For Windows targets:
```bash
brew install mingw-w64
```

#### Linux Host (for macOS/Windows)

For macOS targets:
```bash
# Install OSX cross toolchain
git clone https://github.com/tpoechtrager/osxcross
cd osxcross
# Follow the repository instructions
```

For Windows targets:
```bash
sudo apt-get install mingw-w64
```

## Building for All Platforms

### Using cargo (native compilation)

For your current platform:
```bash
cargo build --release
```

### Cross-Compiling from macOS

**Linux (x86_64):**
```bash
cargo build --release --target x86_64-unknown-linux-gnu
# Or use cross for easier setup
cross build --release --target x86_64-unknown-linux-gnu
```

**Windows (x86_64):**
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

**macOS (Intel):**
```bash
cargo build --release --target x86_64-apple-darwin
```

**macOS (Apple Silicon):**
```bash
cargo build --release --target aarch64-apple-darwin
```

### Cross-Compiling from Linux

**macOS (Intel):**
```bash
cargo build --release --target x86_64-apple-darwin
```

**macOS (Apple Silicon):**
```bash
cargo build --release --target aarch64-apple-darwin
```

**Windows (x86_64):**
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

### Cross-Compiling from Windows

**Linux (x86_64):**
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

**macOS (Intel):**
```bash
cargo build --release --target x86_64-apple-darwin
```

## Output Locations

Binaries will be in:
- `target/release/ports` (native)
- `target/x86_64-unknown-linux-gnu/release/ports` (Linux)
- `target/x86_64-pc-windows-gnu/release/ports.exe` (Windows)
- `target/x86_64-apple-darwin/release/ports` (macOS Intel)
- `target/aarch64-apple-darwin/release/ports` (macOS ARM)

## Platform-Specific Notes

### Windows

The Windows version uses:
- `netstat` instead of `lsof` for port detection
- `tasklist` and sysinfo crate for process information
- `taskkill` for process termination

Some features may have different behavior:
- Process tree information might be limited
- CWD detection relies on Windows APIs
- Signal handling uses `taskkill` instead of SIGTERM/SIGKILL

### Linux

The Linux version is fully supported with all features:
- Uses `lsof` for port and CWD detection
- Uses `ps` for process information
- Uses `/proc` filesystem for enhanced process details
- Full signal support (SIGTERM/SIGKILL)

### macOS

The macOS version is the primary development target:
- Full `lsof` and `ps` support
- Native process management
- All features fully supported

## Using GitHub Actions for Releases

The included `.github/workflows/release.yml` automates building for all platforms.
Create a git tag to trigger a multi-platform release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This will build binaries for:
- Linux x86_64
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)
- Windows x86_64 (via cross-compilation or separate runner)

## Troubleshooting

**Linker errors:**
- Ensure you have the appropriate cross-compilation toolchain installed
- Try using `cross` instead of `cargo` for easier setup

**Feature compatibility:**
- Some Unix-specific features may not work identically on Windows
- Docker integration works on all platforms if Docker is installed

**Performance:**
- Cross-compiled binaries may not be as optimized as natively compiled ones
- For production use, compile natively on each target platform when possible
