# 🚀 Quick Start Guide

Get up and running with `port-viewer` in 5 minutes!

## Installation

### Option 1: Homebrew (Recommended on macOS and Linux)

```bash
brew install iamEtornam/tap/port-viewer
ports --version
```

Package name: `port-viewer`  
Command name: `ports`

### Option 2: Download Pre-built Binary

#### macOS

**Intel Mac:**
```bash
curl -L https://github.com/iamEtornam/port-viewer/releases/latest/download/ports-macos-x86_64.tar.gz | tar xz
sudo mv ports /usr/local/bin/
```

**Apple Silicon (M1/M2/M3):**
```bash
curl -L https://github.com/iamEtornam/port-viewer/releases/latest/download/ports-macos-arm64.tar.gz | tar xz
sudo mv ports /usr/local/bin/
```

#### Linux

```bash
curl -L https://github.com/iamEtornam/port-viewer/releases/latest/download/ports-linux-x86_64.tar.gz | tar xz
sudo mv ports /usr/local/bin/
```

#### Windows

If the Chocolatey package has been published:

```powershell
choco install port-viewer -y
ports --version
```

1. Download `ports-windows-x86_64.zip` from [Releases](https://github.com/iamEtornam/port-viewer/releases/latest)
2. Extract the ZIP file
3. Move `ports.exe` to `C:\Windows\System32\` (requires Administrator privileges)

### Option 3: Build from Source

**Prerequisites:** Rust 1.70+ ([install from rust-lang.org](https://www.rust-lang.org/tools/install))

```bash
git clone https://github.com/iamEtornam/port-viewer
cd port-viewer
cargo build --release

# Unix (macOS/Linux)
sudo cp target/release/ports /usr/local/bin/

# Windows (Run as Administrator)
copy target\release\ports.exe C:\Windows\System32\
```

### Option 4: Install Script (Unix only)

```bash
chmod +x install.sh
./install.sh
```

---

## Verify Installation

```bash
# Check version
ports --version

# Show help
ports --help
```

---

## Basic Usage

### 1. View Active Dev Ports

```bash
ports
```

Shows all development server ports with framework detection:

```
╭──────┬──────────┬──────┬────────────┬──────────────┬─────────┬────────╮
│ PORT │ PROCESS  │ PID  │ PROJECT    │ FRAMEWORK    │ UPTIME  │ STATUS │
├──────┼──────────┼──────┼────────────┼──────────────┼─────────┼────────┤
│ :3000│ node     │ 1234 │ my-app     │ ⚡ Next.js   │ 2h 30m  │ ●      │
│ :8000│ python3  │ 9012 │ backend    │ 🎸 Django    │ 30m 15s │ ●      │
╰──────┴──────────┴──────┴────────────┴──────────────┴─────────┴────────╯
```

### 2. Inspect a Specific Port

```bash
ports 3000
```

Shows detailed information including:
- Full command line
- Project path and Git branch
- Memory usage
- Interactive kill prompt

### 3. View All Processes

```bash
ports ps
```

Shows all dev processes (not just port-bound ones) with CPU and memory usage.

### 4. Real-time Monitoring

```bash
ports watch
```

Live updates every second with change notifications. Press **Ctrl+C** to exit.

### 5. Clean Up Orphaned Processes

```bash
ports clean
```

Finds and interactively kills orphaned/zombie dev processes.

---

## Optional: Create Aliases

### Unix (Bash/Zsh)

Add to `~/.zshrc` or `~/.bashrc`:

```bash
alias p='ports'              # Quick access
alias pw='ports watch'       # Watch mode
alias pc='ports clean'       # Cleanup
alias whoisonport='ports'    # Alternative name

# Reload
source ~/.zshrc
```

### Windows (PowerShell)

Add to your PowerShell profile:

```powershell
# Open profile
notepad $PROFILE

# Add these lines:
Set-Alias -Name p -Value ports

# Save and reload
. $PROFILE
```

---

## Common Workflows

### Starting a New Dev Session

```bash
# Check what's already running
ports

# Start your dev servers
# (they'll auto-appear in ports)

# Watch for changes
ports watch
```

### Debugging Port Conflicts

```bash
# Find what's using port 3000
ports 3000

# Kill it if needed (interactive)
# Answer 'y' to the prompt
```

### End of Day Cleanup

```bash
# Find orphaned processes
ports clean

# Kill all orphans
# Answer 'a' to kill all
```

---

## Platform-Specific Notes

### macOS
- All features fully supported
- May need to run with `sudo` for system ports
- Docker Desktop required for Docker integration

### Linux
- Full `/proc` filesystem support for enhanced process details
- Works on most distributions (Ubuntu, Debian, Fedora, Arch, etc.)
- Requires `lsof` and `ps` (usually pre-installed)

### Windows
- Uses `netstat` instead of `lsof`
- Process termination via `taskkill`
- Some Unix-specific features work differently (see [WINDOWS_SUPPORT.md](WINDOWS_SUPPORT.md))
- Requires Administrator privileges for system process inspection

---

## Next Steps

1. **Read the full README** — [README.md](README.md)
2. **See more examples** — [examples/usage.md](examples/usage.md)
3. **Learn about features** — [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)
4. **Build for other platforms** — [CROSS_COMPILATION.md](CROSS_COMPILATION.md)
5. **Contribute** — [CONTRIBUTING.md](CONTRIBUTING.md)

---

## Getting Help

- 📖 Read the [full documentation](README.md)
- 🐛 [Report bugs](https://github.com/iamEtornam/port-viewer/issues/new)
- 💡 [Request features](https://github.com/iamEtornam/port-viewer/issues/new)
- 💬 [Ask questions](https://github.com/iamEtornam/port-viewer/discussions)

---

**Happy port viewing! 🚢**
