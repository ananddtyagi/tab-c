#!/bin/bash
# Development run script

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "MacAutoComplete - Development Mode"
echo "==================================="

# Check if on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "Warning: This script is designed for macOS"
    echo "Running engine only..."
fi

# Build engine in debug mode
echo "Building engine..."
cd "$PROJECT_ROOT/engine"
cargo build

# Create support directory
SUPPORT_DIR="$HOME/Library/Application Support/MacAutoComplete"
mkdir -p "$SUPPORT_DIR"

# Copy engine binary
echo "Installing engine to $SUPPORT_DIR..."
cp "$PROJECT_ROOT/engine/target/debug/mac_autocomplete_engine" "$SUPPORT_DIR/engine"

# Create default config
if [ ! -f "$SUPPORT_DIR/config.json" ]; then
    echo "Creating default config..."
    cat > "$SUPPORT_DIR/config.json" <<EOF
{
  "socket_path": "$SUPPORT_DIR/engine.sock",
  "model_path": "$SUPPORT_DIR/models/tinyllama-1.1b.Q4_K_M.gguf",
  "ctx_size": 1024,
  "threads": 6,
  "metal": true,
  "instant": {
    "enable_trie": true,
    "trie_path": "$SUPPORT_DIR/trie.bin",
    "max_candidates": 5,
    "timeout_ms": 15
  },
  "llm": {
    "max_tokens": 64,
    "temperature": 0.7,
    "top_k": 40,
    "top_p": 0.9,
    "server_url": "http://127.0.0.1:8087",
    "timeout_ms": 300,
    "enable_kv_cache": true
  },
  "privacy": {
    "enable_learning": false,
    "enable_telemetry": false,
    "excluded_apps": ["com.apple.keychainaccess", "1Password"]
  },
  "cache": {
    "max_entries": 10000,
    "enable_persistence": false
  }
}
EOF
fi

# Run engine
echo ""
echo "Starting engine..."
echo "Socket: $SUPPORT_DIR/engine.sock"
echo "Press Ctrl+C to stop"
echo ""

"$SUPPORT_DIR/engine" --log-level debug
