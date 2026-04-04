# Windows Support

## Overview

`port-viewer` now supports Windows through platform-specific implementations. While the core functionality remains the same across all platforms, the Windows version uses native Windows tools and APIs where Unix tools like `lsof` are unavailable.

## Platform Abstraction Layer

The codebase uses a platform abstraction layer in `src/platform/` to provide unified APIs across different operating systems:

```
src/platform/
├── mod.rs       # Platform selection (unix vs windows)
├── unix.rs      # Unix implementation (macOS, Linux)
└── windows.rs   # Windows implementation
```

### Conditional Compilation

The project uses Rust's conditional compilation features:

```rust
#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;
```

## Implementation Details

### Port Detection

**Unix (lsof):**
```bash
lsof -iTCP -sTCP:LISTEN -P -n
```

**Windows (netstat):**
```bash
netstat -ano -p TCP
```

The Windows implementation parses `netstat` output to find listening TCP ports and their associated Process IDs.

### Process Information

**Unix (ps):**
```bash
ps -o pid,comm,etime,rss,ppid,stat -p <pids>
```

**Windows (sysinfo crate):**
- Uses the `sysinfo` crate which provides cross-platform process information
- Accesses Windows APIs under the hood for process details
- Provides: PID, name, CPU usage, memory (RSS), parent PID, status

### Process Termination

**Unix (kill command):**
```bash
kill -TERM <pid>  # Graceful termination
kill -KILL <pid>  # Force kill
```

**Windows (taskkill):**
```bash
taskkill /PID <pid>        # Graceful termination
taskkill /F /PID <pid>     # Force kill
```

### Current Working Directory (CWD)

**Unix (lsof):**
```bash
lsof -d cwd -a -p <pids> -Fn
```

**Windows (sysinfo crate):**
- Uses `sysinfo::Process::cwd()` method
- Accesses Windows process APIs to retrieve working directory

## Dependencies

### Platform-Agnostic
- `tokio` - Async runtime
- `clap` - CLI parsing
- `serde`/`serde_json` - JSON serialization
- `anyhow` - Error handling
- `colored` - Terminal colors
- `tabled` - Table rendering

### Cross-Platform (works on all systems)
- `sysinfo` - Process and system information
- `which` - Command detection
- `chrono` - Time handling

### Windows-Specific
- `windows` crate (v0.58) - Windows APIs
  - `Win32_Foundation`
  - `Win32_System_Threading`
  - `Win32_NetworkManagement_IpHelper`
  - `Win32_System_ProcessStatus`

## Feature Parity

| Feature | Unix | Windows | Notes |
|---------|------|---------|-------|
| Port listing | ✅ | ✅ | Uses netstat on Windows |
| Process info | ✅ | ✅ | Via sysinfo crate |
| Process CWD | ✅ | ✅ | Via sysinfo crate |
| Framework detection | ✅ | ✅ | Reads package.json, parses cmdline |
| Git branch detection | ✅ | ✅ | Requires Git installed |
| Docker integration | ✅ | ✅ | Requires Docker Desktop |
| Process termination | ✅ | ✅ | Uses taskkill on Windows |
| Graceful SIGTERM | ✅ | ⚠️ | Windows uses taskkill (not a signal) |
| Force SIGKILL | ✅ | ✅ | Works on both |
| Real-time monitoring | ✅ | ✅ | Full support |
| Orphan detection | ✅ | ⚠️ | Different semantics (PPID=0 vs PPID=1) |
| Zombie detection | ✅ | ⚠️ | Limited on Windows |
| Process tree | ✅ | ✅ | Parent PID available |
| CPU % measurement | ✅ | ✅ | Via sysinfo |

## Known Limitations on Windows

1. **Signal Handling**: Windows doesn't have POSIX signals. `taskkill` is used instead, which is not as graceful as SIGTERM.

2. **Zombie Processes**: The concept of zombie processes (defunct but not reaped) is Unix-specific. Windows handles terminated processes differently.

3. **Orphaned Processes**: On Unix, orphaned processes have PPID=1 (init). On Windows, PPID=0 or PPID=4 (System) is used as the heuristic.

4. **Process Status**: Unix `ps` provides detailed status codes (R, S, Z, T, etc.). Windows status is simplified to Running, Sleeping, Stopped, Zombie, or Unknown.

5. **Performance**: Some operations may be slightly slower on Windows due to API overhead, but overall performance is comparable.

## Testing on Windows

To test on Windows:

1. Build the project:
   ```powershell
   cargo build --release
   ```

2. Run the binary:
   ```powershell
   .\target\release\ports.exe
   ```

3. Test specific features:
   ```powershell
   .\target\release\ports.exe --all
   .\target\release\ports.exe 3000
   .\target\release\ports.exe ps
   .\target\release\ports.exe watch
   .\target\release\ports.exe clean
   ```

## Building for Windows from Unix

See [CROSS_COMPILATION.md](CROSS_COMPILATION.md) for detailed cross-compilation instructions.

Quick example from macOS/Linux:
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Troubleshooting

**"netstat command not found":**
- `netstat` is built into Windows. Ensure you're running in a proper command prompt or PowerShell.

**Permission errors:**
- Run PowerShell or Command Prompt as Administrator when inspecting or killing system processes.

**Docker not detected:**
- Ensure Docker Desktop is installed and running.
- Check that `docker` command is available in PATH.

**Git branch not showing:**
- Install Git for Windows: https://git-scm.com/download/win
- Ensure `git` is in your PATH.

## Future Improvements

- [ ] Use Windows-specific APIs directly for better performance
- [ ] Implement proper Windows Service detection
- [ ] Add Windows Task Scheduler integration
- [ ] Better handling of Windows-specific process states
- [ ] Support for Windows containers (if applicable)
