# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`port-viewer` is a cross-platform Rust CLI that inspects processes listening on TCP ports and enriches them with project/framework context. The crate is named `port-viewer` but the binary is `ports` (see `[[bin]]` in `Cargo.toml`). Most commands are run as `cargo run -- <args>` in dev, or `ports <args>` once installed.

## Commands

The `Makefile` wraps the common cargo invocations:

- `make build` / `make release` — debug vs. optimized build (release profile uses `lto`, `codegen-units=1`, `strip`)
- `make test` — `cargo test --verbose`
- `make fmt` — `cargo fmt`
- `make clippy` — `cargo clippy -- -D warnings` (CI treats warnings as errors)
- `make run` — `cargo run` (default port-listing view)
- `make completions` — generates shell completions into `completions/`

CI (`.github/workflows/ci.yml`) runs `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test --verbose`, and a release build on Ubuntu, macOS, and Windows. **Before pushing, run `make fmt` and `make clippy` — a formatting diff or any clippy warning fails CI.**

Run a single test: `cargo test <test_name>` (or `cargo test <module>::<test_name>`). There is currently no test suite, so new tests establish the conventions.

Exercise subcommands in dev with `cargo run -- ps`, `cargo run -- watch`, `cargo run -- clean`, `cargo run -- 3000` (port detail), or `cargo run -- --all`.

## Architecture

### Command dispatch
`src/main.rs` defines the clap CLI. A bare invocation lists listening ports (or shows detail for a `PORT` positional arg); subcommands are `Ps`, `Watch`, `Clean`. Each subcommand lives in its own module (`ps_view.rs`, `watch.rs`, `clean.rs`, `detail.rs`) and is `async` — the whole program runs under `#[tokio::main]`.

### The platform abstraction (most important to understand)
`src/platform/mod.rs` re-exports either `unix.rs` or `windows.rs` via `#[cfg(unix)]` / `#[cfg(windows)]`, so both files **must expose the same public function set** (`collect_listening_ports`, `collect_process_info_batch`, `collect_process_cwds`, `get_process_cmdline`, `parse_process_status`, `parse_uptime`, `kill_process_signal`, `check_process_alive`, `get_cpu_sample`) and the same `ProcessInfoData` struct. Adding a platform-facing capability means implementing it in **both** files, or CI's Windows/Unix builds break.

- **Unix** shells out to `lsof`, `ps`, `kill`, and reads `/proc/<pid>/cmdline` on Linux. Data is parsed from text output by column index.
- **Windows** uses `netstat -ano` for ports plus the `sysinfo` crate for process metadata (declared only under `[target.'cfg(windows)'.dependencies]`).

When editing parsing logic, the brittleness lives here: column offsets, `lines().skip(N)`, and `rsplit_once(':')` on addresses. Malformed lines are skipped rather than erroring.

### Data flow
`src/collector.rs::collect_all_data(show_all)` is the orchestration hub used by nearly every command. It:
1. Concurrently collects listening ports and `docker ps` output (`tokio::join!`).
2. Concurrently collects process info and CWDs for the discovered PIDs.
3. Per process, concurrently (`futures::future::join_all`) resolves cmdline, then enriches with `framework::detect_framework`, git branch (`git -C <cwd> branch --show-current`), and Docker service mapping.
4. Filters via `should_show_process` unless `show_all` — i.e. `is_dev_process() && !is_system_process()`.

The performance characteristic of this tool (sub-200ms) depends on these collection steps staying concurrent. Don't serialize the joins.

### Domain types — `src/process.rs`
`PortEntry` wraps a `ProcessInfo`. The dev-vs-system classification lives here as hardcoded lists: `is_dev_process` matches a `DEV_RUNTIMES` allowlist (node/python/ruby/cargo/…), `is_system_process` matches a `SYSTEM_APPS` blocklist (Spotify/Chrome/sshd/…). `ProcessStatus` (Healthy/Orphaned/Zombie) carries its own terminal symbol and color. To change what counts as a "dev process," edit these lists rather than the call sites.

### Framework detection — `src/framework.rs`
`detect_framework` tries three strategies in priority order: parse `package.json` deps/devDeps, then scan the command line for keywords (django/uvicorn/rails/…), then fall back to the process-name runtime. The `Framework` enum owns its own `display_name()` and `emoji()`. Docker-served ports override the framework with `Framework::Unknown("Docker · <Service>")` where the service is sniffed from the image name in `DockerContainer::detect_service`.

### Rendering
All table output uses the `tabled` crate with `Style::rounded()`, and `colored` for ANSI styling. Each view defines a private `#[derive(Tabled)]` row struct mapping domain data to display strings (`renderer.rs`, `ps_view.rs`, `clean.rs`). Keep presentation in these row structs; keep domain logic out of them.

## Releases & distribution

Pushing to `main` triggers the CI release pipeline: it auto-computes the next semver tag **from the latest commit message** (`feat!`/`BREAKING CHANGE` → major, `feat:` → minor, else patch), creates the tag and GitHub release, and uploads per-platform binaries. Commit messages on `main` directly drive versioning — follow Conventional Commits.

Homebrew (`homebrew-tap.yml`) and Chocolatey (`chocolatey.yml`) packaging is rendered by the scripts in `scripts/` from templates. Maintainer setup is documented in `HOMEBREW.md` and `CHOCOLATEY.md`; Windows specifics and limitations in `WINDOWS_SUPPORT.md`.
