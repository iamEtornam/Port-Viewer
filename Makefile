.PHONY: help build release install test fmt clippy clean run watch completions

help:
	@echo "Port Viewer - Development Commands"
	@echo "──────────────────────────────────"
	@echo "  make build       - Build debug binary"
	@echo "  make release     - Build optimized release binary"
	@echo "  make install     - Install to ~/.cargo/bin"
	@echo "  make test        - Run all tests"
	@echo "  make fmt         - Format code"
	@echo "  make clippy      - Run clippy linter"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make run         - Run in development mode"
	@echo "  make watch       - Auto-rebuild on file changes"
	@echo "  make completions - Generate shell completions"

build:
	@echo "Building debug binary..."
	cargo build

release:
	@echo "Building release binary..."
	cargo build --release
	@echo "Binary at: target/release/ports"

install:
	@echo "Installing to ~/.cargo/bin..."
	cargo install --path .

test:
	@echo "Running tests..."
	cargo test --verbose

fmt:
	@echo "Formatting code..."
	cargo fmt

clippy:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

run:
	@echo "Running in development mode..."
	cargo run

watch:
	@echo "Watching for changes..."
	cargo watch -x build

completions:
	@echo "Generating shell completions..."
	@mkdir -p completions
	cargo run -- completions bash > completions/ports.bash
	cargo run -- completions zsh > completions/_ports
	cargo run -- completions fish > completions/ports.fish
	@echo "Completions generated in completions/"
	@echo ""
	@echo "To install:"
	@echo "  Bash:  cp completions/ports.bash ~/.local/share/bash-completion/completions/ports"
	@echo "  Zsh:   cp completions/_ports ~/.zsh/completions/"
	@echo "  Fish:  cp completions/ports.fish ~/.config/fish/completions/"
