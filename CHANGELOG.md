# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of port-viewer
- CLI commands: default view, ps, watch, clean
- Framework detection for Next.js, Vite, Angular, Remix, Astro, Express, Fastify, Nuxt, Django, FastAPI, Flask, Rails, Puma, Go, Cargo
- Docker integration with service detection
- Git branch detection
- Process status indicators (healthy, orphaned, zombie)
- Beautiful Unicode table rendering
- Real-time monitoring with change detection
- Interactive orphan process cleanup
- Detailed port inspection with kill prompt
- CPU and memory usage tracking
- Filtering for dev processes vs all processes

### Performance
- Concurrent subprocess execution
- Single batched calls to ps and lsof
- Runs in ~200ms or less

## [0.1.0] - 2025-04-04

### Added
- Initial project structure
- Core functionality implementation
- README documentation
- MIT License
- Contributing guidelines

[Unreleased]: https://github.com/yourusername/port-viewer/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/port-viewer/releases/tag/v0.1.0
