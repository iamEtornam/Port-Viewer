# 🚢 port-viewer

[![Build Status](https://github.com/iamEtornam/port-viewer/actions/workflows/ci.yml/badge.svg)](https://github.com/iamEtornam/port-viewer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://makeapullrequest.com)

**A beautiful, blazing-fast cross-platform CLI tool to inspect and manage processes listening on your machine's ports.**

<div align="center">
  <img src="assets/hero.png" alt="port-viewer hero" width="600"/>
</div>

---

## ✨ Features

- 🚀 **Blazing Fast** — Sub-200ms execution with concurrent subprocess calls
- 🎨 **Beautiful Output** — Rich Unicode tables with colors and emojis
- 🔍 **Smart Detection** — Auto-detects frameworks (Next.js, Django, Rails, etc.)
- 🐳 **Docker Aware** — Automatically maps container ports and services
- ⚡ **Real-time Monitoring** — Watch mode with live diff updates
- 🧹 **Process Cleanup** — Interactive orphan/zombie process killer
- 🌍 **Cross-Platform** — Works on macOS, Linux, and Windows
- 🔧 **Zero Config** — Just install and run

---

## 🖥️ Platform Support

`port-viewer` runs natively on all major platforms:

| Platform | Status | Implementation |
|----------|--------|----------------|
| **macOS** (Intel & Apple Silicon) | ✅ Full Support | Native `lsof`/`ps` |
| **Linux** (x86_64) | ✅ Full Support | Native `lsof`/`ps` + `/proc` |
| **Windows** (x86_64) | ✅ Full Support | `netstat`/`sysinfo`/`taskkill` |

> **Note**: Windows implementation uses platform-specific tools. See [WINDOWS_SUPPORT.md](WINDOWS_SUPPORT.md) for details and limitations.

---

## 🚀 Quick Start

### Installation

#### Pre-built Binaries (Recommended)

Download the latest release for your platform from the [Releases](https://github.com/iamEtornam/port-viewer/releases) page:

**macOS:**
```bash
# Intel
curl -L https://github.com/iamEtornam/port-viewer/releases/latest/download/ports-macos-x86_64.tar.gz | tar xz
sudo mv ports /usr/local/bin/

# Apple Silicon
curl -L https://github.com/iamEtornam/port-viewer/releases/latest/download/ports-macos-aarch64.tar.gz | tar xz
sudo mv ports /usr/local/bin/
```

**Linux:**
```bash
curl -L https://github.com/iamEtornam/port-viewer/releases/latest/download/ports-linux-x86_64.tar.gz | tar xz
sudo mv ports /usr/local/bin/
```

**Windows:**
```powershell
# Download ports-windows-x86_64.zip from releases
# Extract and move ports.exe to C:\Windows\System32\
```

#### From Source

```bash
# Clone the repository
git clone https://github.com/iamEtornam/port-viewer
cd port-viewer

# Build release binary
cargo build --release

# Install (Unix)
sudo cp target/release/ports /usr/local/bin/

# Install (Windows - Run as Administrator)
copy target\release\ports.exe C:\Windows\System32\
```

#### Using Install Script (Unix only)

```bash
chmod +x install.sh
./install.sh
```

The install script will:
1. Build from source if `cargo` is available
2. Otherwise, download the latest release binary
3. Install to `/usr/local/bin/ports`
4. Suggest creating handy aliases

### Create Handy Aliases

**Unix (Bash/Zsh):**

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
alias p='ports'              # Quick access
alias pw='ports watch'       # Watch mode
alias pc='ports clean'       # Cleanup orphans
alias whoisonport='ports'    # Alternative name

# Reload shell
source ~/.zshrc  # or source ~/.bashrc
```

**Windows (PowerShell):**

Add to your PowerShell profile (`$PROFILE`):

```powershell
Set-Alias -Name p -Value ports
Set-Alias -Name pw -Value "ports watch"
Set-Alias -Name pc -Value "ports clean"
```

---

## 📖 Usage Examples

### Basic Commands

#### View All Dev Ports (Default)

```bash
$ ports

╭──────┬──────────┬──────┬────────────┬──────────────┬─────────┬────────╮
│ PORT │ PROCESS  │ PID  │ PROJECT    │ FRAMEWORK    │ UPTIME  │ STATUS │
├──────┼──────────┼──────┼────────────┼──────────────┼─────────┼────────┤
│ :3000│ node     │ 1234 │ my-app     │ ⚡ Next.js   │ 2h 30m  │ ●      │
│ :3001│ node     │ 1235 │ api-server │ 🚂 Express   │ 1h 45m  │ ●      │
│ :5432│ postgres │ 5678 │ -          │ 🐳 PostgreSQL│ 5d 3h   │ ●      │
│ :6379│ redis    │ 5679 │ -          │ 🐳 Redis     │ 5d 3h   │ ●      │
│ :8000│ python3  │ 9012 │ backend    │ 🎸 Django    │ 30m 15s │ ●      │
╰──────┴──────────┴──────┴────────────┴──────────────┴─────────┴────────╯

5 ports active · Run ports <number> for details · --all to show everything
```

#### View All Ports (Including System)

```bash
$ ports --all
```

Shows everything, including system services like `:80`, `:443`, etc.

### Inspect a Specific Port

```bash
$ ports 3000

╔═══════════════════════════════════════════════════════════════╗
║ ● Port :3000                                                  ║
╠═══════════════════════════════════════════════════════════════╣
║ Process:        node                                          ║
║ PID:            1234                                          ║
║ Project:        my-app                                        ║
║ Path:           /Users/name/projects/my-app                   ║
║ Framework:      ⚡ Next.js                                    ║
║ Git Branch:     🌿 main                                       ║
║ Uptime:         2h 30m                                        ║
║ Memory:         245.3 MB                                      ║
║ Parent PID:     890                                           ║
╠═══════════════════════════════════════════════════════════════╣
║ Command:                                                      ║
║ /usr/local/bin/node                                           ║
║ /Users/name/projects/my-app/node_modules/.bin/next dev       ║
╚═══════════════════════════════════════════════════════════════╝

Kill this process? (PID 1234) [y/N]: 
```

**Graceful Process Termination:**
- Answer `y` to send SIGTERM (Unix) or graceful taskkill (Windows)
- If the process doesn't exit after 3 seconds, SIGKILL/force kill is used
- Safe and interactive confirmation required

### Process View

#### Show All Dev Processes

```bash
$ ports ps

╭──────┬─────────┬──────┬──────┬────────────┬──────────────┬─────────┬─────────────────╮
│ PID  │ PROCESS │ CPU% │ MEM  │ PROJECT    │ FRAMEWORK    │ UPTIME  │ WHAT            │
├──────┼─────────┼──────┼──────┼────────────┼──────────────┼─────────┼─────────────────┤
│ 1234 │ node    │ 12.5 │ 245M │ my-app     │ ⚡ Next.js   │ 2h 30m  │ next dev        │
│ 1235 │ node    │ 3.2  │ 120M │ api-server │ 🚂 Express   │ 1h 45m  │ node server.js  │
│ 9012 │ python3 │ 8.1  │ 180M │ backend    │ 🎸 Django    │ 30m 15s │ python manage.py│
│ -    │ docker  │ -    │ -    │ -          │ 🐳 Docker · 4│ -       │ Container runtime│
╰──────┴─────────┴──────┴──────┴────────────┴──────────────┴─────────┴─────────────────╯
```

#### Show All Processes (Including System)

```bash
$ ports ps --all
```

### Real-Time Monitoring

```bash
$ ports watch

Starting port monitor (Ctrl+C to exit)...

[12:03:44] ● :3001 started — node / Express / preview-app
[12:05:02] ✕ :3001 stopped

╭──────┬──────────┬──────┬────────────┬──────────────┬─────────┬────────╮
│ PORT │ PROCESS  │ PID  │ PROJECT    │ FRAMEWORK    │ UPTIME  │ STATUS │
├──────┼──────────┼──────┼────────────┼──────────────┼─────────┼────────┤
│ :3000│ node     │ 1234 │ my-app     │ ⚡ Next.js   │ 2h 32m  │ ●      │
│ :5432│ postgres │ 5678 │ -          │ 🐳 PostgreSQL│ 5d 3h   │ ●      │
╰──────┴──────────┴──────┴────────────┴──────────────┴─────────┴────────╯

2 ports active
```

Press **Ctrl+C** to exit.

### Orphan Cleanup

```bash
$ ports clean

Found 3 orphaned processes:

╭──────┬─────────┬──────────┬─────────┬────────╮
│ PID  │ PROCESS │ PROJECT  │ UPTIME  │ STATUS │
├──────┼─────────┼──────────┼─────────┼────────┤
│ 7890 │ node    │ old-app  │ 12h 5m  │ ◐      │
│ 7891 │ python3 │ test-api │ 8h 30m  │ ◐      │
│ 7892 │ ruby    │ legacy   │ 2d 4h   │ ◐      │
╰──────┴─────────┴──────────┴─────────┴────────╯

Kill PID 7890 node [y/N/a(ll)/q(uit)]: a
Killing all orphans...
  ✓ PID 7890
  ✓ PID 7891
  ✓ PID 7892

✓ Cleanup complete.
```

Options:
- `y` — Kill this one process
- `N` — Skip this process (default)
- `a` — Kill all orphaned processes
- `q` — Quit without killing anything

---

## 🔍 How It Works

### Data Collection Pipeline

The tool uses a highly optimized 3-step concurrent pipeline:

1. **Port Detection**
   - Unix: `lsof -iTCP -sTCP:LISTEN -P -n`
   - Windows: `netstat -ano -p TCP`

2. **Process Information** (batched, single call)
   - Unix: `ps -o pid,comm,etime,rss,ppid,stat -p <all_pids>`
   - Windows: `sysinfo` crate + Windows APIs

3. **Working Directory Detection** (batched)
   - Unix: `lsof -d cwd -a -p <all_pids> -Fn`
   - Windows: Windows APIs via `sysinfo`

4. **Docker Integration** (parallel)
   - Runs `docker ps --format json` concurrently
   - Maps host ports → container services

All steps run in parallel using `tokio::join!` for maximum performance.

### Framework Detection

We scan in priority order:

1. **`package.json` dependencies** — Next.js, Vite, Angular, Remix, Astro, Express, Fastify, Nuxt
2. **Command line inspection** — Django, FastAPI, Rails, Flask, Puma, Cargo, Go
3. **Process name fallback** — Node.js, Python, Ruby, Go, Rust

### Docker Service Detection

Automatically detects common Docker services:
- 🐳 PostgreSQL (ports 5432, 5433)
- 🐳 Redis (6379)
- 🐳 MongoDB (27017)
- 🐳 MySQL (3306)
- 🐳 nginx (80, 443, 8080)
- 🐳 LocalStack (4566, 4571)

### Status Icons

- **● Green (Healthy)** — Process is responsive with a valid parent
- **◐ Yellow (Orphaned)** — Parent process died (PPID=1 on Unix, PPID=0 on Windows)
- **✕ Red (Zombie)** — Process is defunct/non-responsive

---

## ⚡ Performance

Optimized for zero-lag developer experience:

- **Sub-200ms Execution** — Typical run completes in ~150ms
- **Concurrent I/O** — All subprocess calls run in parallel
- **Batched Operations** — Single OS call for multiple PIDs
- **Minimal Footprint** — ~10MB memory, zero dependencies at runtime
- **Release Optimizations** — LTO, codegen-units=1, stripped binaries

Benchmark on MacBook Pro M1:
```
ports           ~150ms
ports --all     ~180ms
ports watch     ~1s/update
ports clean     ~200ms
```

---

## 📋 Requirements

### All Platforms
- **Rust 1.70+** (for building from source)
- **Docker** (optional, for container enrichment)
- **Git** (optional, for branch detection)

### Unix (macOS/Linux)
- `lsof` — Pre-installed on most systems
- `ps` — Pre-installed on most systems

### Windows
- `netstat` — Built into Windows
- `tasklist` — Built into Windows
- `taskkill` — Built into Windows

---

## 🔧 Cross-Compilation

Want to build for multiple platforms? See [CROSS_COMPILATION.md](CROSS_COMPILATION.md) for detailed instructions.

**Quick examples:**

```bash
# Add targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-apple-darwin

# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# Build for Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

---

## 🐛 Troubleshooting

### Common Issues

**No ports found:**
- Make sure you have development servers running
- Try `ports --all` to see system ports

**Permission errors:**
- Some system ports require elevated privileges
- Unix: Try `sudo ports`
- Windows: Run as Administrator

**Docker info not showing:**
- Ensure Docker is installed and running
- Check that `docker ps` works in your terminal

**Windows-specific:**
- See [WINDOWS_SUPPORT.md](WINDOWS_SUPPORT.md) for platform-specific limitations
- Some features (like signal handling) work differently on Windows

### Getting Help

1. Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) (if available)
2. Search existing [GitHub Issues](https://github.com/iamEtornam/port-viewer/issues)
3. Open a new issue with:
   - Your OS and version
   - Output of `ports --version`
   - Full error message
   - Steps to reproduce

---

## 🏗️ Project Structure

```
port-viewer/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── collector.rs      # Data collection pipeline
│   ├── platform/         # Platform-specific implementations
│   │   ├── mod.rs        # Platform selection
│   │   ├── unix.rs       # macOS/Linux implementation
│   │   └── windows.rs    # Windows implementation
│   ├── process.rs        # Process data structures
│   ├── framework.rs      # Framework detection
│   ├── renderer.rs       # Table rendering
│   ├── detail.rs         # Port detail view
│   ├── ps_view.rs        # Process view
│   ├── watch.rs          # Real-time monitoring
│   └── clean.rs          # Orphan cleanup
├── Cargo.toml            # Dependencies
├── README.md             # This file
├── CROSS_COMPILATION.md  # Cross-platform build guide
├── WINDOWS_SUPPORT.md    # Windows-specific details
├── CONTRIBUTING.md       # Contribution guidelines
├── LICENSE               # MIT License
└── .github/
    └── workflows/
        ├── ci.yml        # Continuous integration
        └── release.yml   # Multi-platform releases
```

---

## 📚 Documentation

- [QUICKSTART.md](QUICKSTART.md) — Get up and running in 5 minutes
- [CROSS_COMPILATION.md](CROSS_COMPILATION.md) — Build for all platforms
- [WINDOWS_SUPPORT.md](WINDOWS_SUPPORT.md) — Windows implementation details
- [CONTRIBUTING.md](CONTRIBUTING.md) — How to contribute
- [CHANGELOG.md](CHANGELOG.md) — Version history

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code organization and standards
- Testing guidelines
- Pull request process
- Feature requests and bug reports

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🌟 Acknowledgments

- Inspired by the original JavaScript [port-viewer](https://github.com/original/port-viewer) project
- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- Terminal UI powered by [tabled](https://github.com/zhiburt/tabled)
- Cross-platform process info via [sysinfo](https://github.com/GuillaumeGomez/sysinfo)

---

## 🚀 What's Next

See [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) for:
- Detailed feature list
- Architecture overview
- Performance benchmarks
- Future enhancements
- Development workflow

---

**Made with ⚡ and 🦀 by the Etornam**
