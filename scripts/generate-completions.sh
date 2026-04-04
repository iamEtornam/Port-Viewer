#!/bin/bash
# Generate shell completion scripts

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
COMPLETIONS_DIR="$PROJECT_ROOT/completions"

mkdir -p "$COMPLETIONS_DIR"

echo "Generating shell completions..."

# Build the binary first
cd "$PROJECT_ROOT"
cargo build --release

# Generate completions using clap_complete
# This would require adding completion generation to the CLI
# For now, we'll create basic templates

# Bash completion
cat > "$COMPLETIONS_DIR/ports.bash" << 'EOF'
_ports_completions() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    opts="--all --help --version ps watch clean"
    
    if [[ ${cur} == -* ]] ; then
        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
        return 0
    fi
    
    case "${prev}" in
        ps|watch|clean)
            COMPREPLY=( $(compgen -W "--all --help" -- ${cur}) )
            return 0
            ;;
    esac
    
    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
}

complete -F _ports_completions ports
complete -F _ports_completions whoisonport
EOF

# Zsh completion
cat > "$COMPLETIONS_DIR/_ports" << 'EOF'
#compdef ports whoisonport

_ports() {
    local -a commands
    commands=(
        'ps:Show all running dev processes'
        'watch:Real-time monitoring (poll every 1s)'
        'clean:Find and interactively kill orphaned processes'
    )
    
    _arguments \
        '(- *)'{-h,--help}'[Show help information]' \
        '(- *)'{-V,--version}'[Show version information]' \
        '--all[Show all ports including system services]' \
        '1: :_ports_commands' \
        '*::arg:->args'
    
    case $state in
        args)
            case $words[1] in
                ps)
                    _arguments '--all[Show all processes]'
                    ;;
                watch|clean)
                    _arguments '--help[Show help]'
                    ;;
            esac
            ;;
    esac
}

_ports_commands() {
    local -a commands
    commands=(
        'ps:Show all running dev processes'
        'watch:Real-time monitoring'
        'clean:Clean orphaned processes'
    )
    _describe 'command' commands
}

_ports "$@"
EOF

# Fish completion
cat > "$COMPLETIONS_DIR/ports.fish" << 'EOF'
# Fish shell completions for ports

# Options
complete -c ports -l all -d "Show all ports including system services"
complete -c ports -s h -l help -d "Show help information"
complete -c ports -s V -l version -d "Show version information"

# Commands
complete -c ports -f -n "__fish_use_subcommand" -a "ps" -d "Show all running dev processes"
complete -c ports -f -n "__fish_use_subcommand" -a "watch" -d "Real-time monitoring"
complete -c ports -f -n "__fish_use_subcommand" -a "clean" -d "Clean orphaned processes"

# ps command options
complete -c ports -f -n "__fish_seen_subcommand_from ps" -l all -d "Show all processes"

# Alias for whoisonport
complete -c whoisonport -w ports
EOF

echo "✓ Completions generated successfully!"
echo ""
echo "Install instructions:"
echo "  Bash:  cp $COMPLETIONS_DIR/ports.bash ~/.local/share/bash-completion/completions/ports"
echo "  Zsh:   cp $COMPLETIONS_DIR/_ports \$HOME/.zsh/completions/ (add to fpath)"
echo "  Fish:  cp $COMPLETIONS_DIR/ports.fish ~/.config/fish/completions/"
