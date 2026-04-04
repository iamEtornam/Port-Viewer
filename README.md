# 🚢 port-viewer

[![Build Status](https://github.com/yourusername/port-viewer/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/port-viewer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://makeapullrequest.com)

**A beautiful, blazing-fast CLI tool to inspect and manage processes listening on your machine's ports.**

<div align="center">
  <img src="assets/hero.png" alt="port-viewer hero" width="600"/>
</div>

---

## 🚀 Quick Start

### Installation

**From Source (Recommended)**
```bash
cargo install --path .
```

**Binary Usage**
```bash
./target/release/ports --help
```

### Create an Alias
Add this to your `~/.zshrc` or `~/.bashrc` for the ultimate experience:
```bash
alias ports='whoisonport'
alias p='ports'
alias pw='ports watch'
alias pc='ports clean'
```

---


# Or copy to your PATH
sudo cp target/release/ports /usr/local/bin/
```

### Create Handy Aliases

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
alias ports='/path/to/port-viewer/target/release/ports'
alias p='ports'              # Quick access
alias pw='ports watch'       # Watch mode
alias pc='ports clean'       # Cleanup
alias whoisonport='ports'    # Common name

# Reload shell
source ~/.zshrc  # or source ~/.bashrc
```

---

## 📖 Usage Examples

This section provides comprehensive examples of using the port-viewer CLI tool.

### Basic Usage

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

#### View All Ports (Including System Services)

```bash
$ ports --all

# Shows everything, including system ports like :80, :443, etc.
```

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

**Kill a Process:**
Simply answer `y` to the prompt above to gracefully kill the process.

### Process View (`ports ps`)

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

### Real-Time Monitoring (`ports watch`)

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

### Orphan Cleanup (`ports clean`)

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

---

## 🔍 How it Works

### Framework Detection
We scan in priority order:
1. `package.json` dependencies (Next.js, Vite, Express, etc.)
2. Runtime command line (Django, Rails, Flask)
3. Process name fallback (Node.js, Python, Go)

### Docker Integration
If `docker` is available, we automatically run `docker ps` in parallel and map ports. No more guessing which container owns which port!

### Performance
Optimized for zero-lag developer experience:
- **Concurrent Execution**: `lsof`, `ps`, and `docker` run in parallel.
- **Batching**: Single OS calls for all PIDs.
- **Minimal Footprint**: Written in Rust, using ~10MB memory.

---

## 💡 Status Icons
- **● Green (Healthy)** — Process is responsive and has a valid parent.
- **◐ Yellow (Orphaned)** — Process is running but its parent terminal died.
- **✕ Red (Zombie)** — Process is in a non-responsive state.

---

## 🔧 Troubleshooting
- **No listening ports found**: Make sure you have development servers running.
- **Failed to run lsof: Permission denied**: Some system ports require elevated privileges. Try `sudo ports`.
- **Docker info not showing**: Make sure Docker is running (`docker ps`).

---

## 📄 License
This project is licensed under the [MIT License](LICENSE).

---

## 🤝 Contributing
Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for more details on how to get started.
