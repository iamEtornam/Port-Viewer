# Contributing to port-viewer

Thank you for your interest in contributing to port-viewer! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.75+ (2021 edition)
- `lsof` and `ps` commands (standard on macOS and Linux)
- Docker (optional, for Docker integration)
- Git (optional, for branch detection)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/iamEtornam/port-viewer.git
cd port-viewer

# Build debug version
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Install locally
cargo install --path .
```

## Code Organization

The project follows a modular structure:

- `main.rs` — CLI entrypoint and argument parsing
- `collector.rs` — Async data collection pipeline
- `process.rs` — Core data structures
- `framework.rs` — Framework detection logic
- `renderer.rs` — Table rendering and output formatting
- `detail.rs` — Single-port detail view
- `ps_view.rs` — Process listing view
- `watch.rs` — Real-time monitoring
- `clean.rs` — Orphan process cleanup

## Coding Standards

### Style Guide

- Follow Rust standard formatting (`cargo fmt`)
- Run clippy before submitting (`cargo clippy`)
- Keep functions under 50 lines when possible
- Add documentation comments for public APIs
- Use meaningful variable names

### Error Handling

- Always use `Result` for operations that can fail
- Never use `.unwrap()` in production code
- Provide context with `.context()` from anyhow
- Handle all error cases gracefully

### Performance

- Maintain ~200ms or less execution time
- Use concurrent execution with tokio::join!
- Minimize allocations and string processing
- Batch subprocess calls when possible

## Testing

### Unit Tests

```bash
cargo test
```

Write unit tests for:
- Framework detection logic
- String parsing functions
- Data structure methods

### Integration Tests

Test the full CLI workflow:
- Port listing
- Process detail view
- Docker integration
- Git branch detection

### Manual Testing

Test on real systems with various scenarios:
- Multiple ports
- Docker containers
- Orphaned processes
- Framework detection

## Pull Request Process

1. **Fork the repository** and create your branch from `main`
2. **Write tests** for new features
3. **Update documentation** (README, code comments)
4. **Run the test suite** and ensure all tests pass
5. **Run clippy** and fix all warnings
6. **Format your code** with `cargo fmt`
7. **Write a clear PR description** explaining the changes
8. **Link any related issues** in the PR description

### PR Title Format

Use conventional commit format:

- `feat: Add JSON output flag`
- `fix: Handle missing lsof command`
- `docs: Update installation instructions`
- `perf: Optimize process info collection`
- `refactor: Extract Docker detection logic`
- `test: Add framework detection tests`

## Feature Requests

Have an idea for a new feature? Here's how to propose it:

1. **Check existing issues** to avoid duplicates
2. **Open an issue** with the `feature` label
3. **Describe the use case** and why it's valuable
4. **Provide examples** of how it would work
5. **Consider alternatives** you've thought of

## Bug Reports

Found a bug? Help us fix it:

1. **Check existing issues** to avoid duplicates
2. **Open an issue** with the `bug` label
3. **Include version info** (`ports --version`)
4. **Describe the bug** clearly and concisely
5. **Provide steps to reproduce** the issue
6. **Include expected vs actual behavior**
7. **Add any relevant logs or screenshots**

### Bug Report Template

```markdown
**Version:** ports v0.1.0
**OS:** macOS 14.2.1
**Shell:** zsh 5.9

**Description:**
[Clear description of the bug]

**Steps to Reproduce:**
1. Run `ports`
2. ...

**Expected Behavior:**
[What you expected to happen]

**Actual Behavior:**
[What actually happened]

**Additional Context:**
[Any other relevant information]
```

## Architecture Decisions

### Why Tokio?

- Concurrent subprocess execution
- Async-friendly for future network features
- Industry standard for async Rust

### Why tabled for Tables?

- Beautiful Unicode output
- Easy customization
- Good performance

### Why Separate Modules?

- Clear separation of concerns
- Easy to test individually
- Maintainable codebase

## Adding New Framework Detection

To add support for a new framework:

1. Add the framework variant to `Framework` enum in `framework.rs`
2. Implement `display_name()` and `emoji()` methods
3. Add detection logic in `detect_from_package_json()`, `detect_from_cmdline()`, or `detect_from_process_name()`
4. Add tests for the new framework

Example:

```rust
// In framework.rs
pub enum Framework {
    // ...
    Svelte,
}

impl Framework {
    pub fn display_name(&self) -> &str {
        match self {
            // ...
            Framework::Svelte => "SvelteKit",
        }
    }
    
    pub fn emoji(&self) -> &str {
        match self {
            // ...
            Framework::Svelte => "🔥",
        }
    }
}

// In detect_from_package_json()
if all_deps.iter().any(|d| d == "@sveltejs/kit") {
    return Ok(Framework::Svelte);
}
```

## Documentation

### Code Comments

- Use `///` for public API documentation
- Use `//` for inline implementation notes
- Explain *why*, not *what* (code shows what)
- Keep comments up to date

### README Updates

Update the README when:
- Adding new features
- Changing CLI interface
- Modifying requirements
- Adding new framework support

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.2.0`
4. Push tag: `git push origin v0.2.0`
5. GitHub Actions will build and publish release

## Questions?

Feel free to:
- Open a discussion on GitHub
- Ask in the issue tracker
- Submit a draft PR for early feedback

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers learn
- Assume good intentions

Thank you for contributing to port-viewer! 🚀
