#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SOCKET_PATH="$HOME/Library/Application Support/MacAutoComplete/engine.sock"

mkdir -p "$(dirname "$SOCKET_PATH")"

(cd "$REPO_ROOT/engine" && cargo run -- --socket "$SOCKET_PATH" "$@")
