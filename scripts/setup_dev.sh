#!/bin/bash
# Development environment setup

set -e

echo "MacAutoComplete - Development Setup"
echo "===================================="
echo ""

# Check OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Platform: macOS"

    # Check Xcode
    if ! command -v xcodebuild &> /dev/null; then
        echo "Error: Xcode Command Line Tools not found"
        echo "Install with: xcode-select --install"
        exit 1
    fi
    echo "✓ Xcode Command Line Tools"

    # Check Homebrew
    if ! command -v brew &> /dev/null; then
        echo "Warning: Homebrew not found"
        echo "Install from: https://brew.sh"
    else
        echo "✓ Homebrew"
    fi
else
    echo "Platform: $OSTYPE (IME development requires macOS)"
fi

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust not found"
    echo "Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo "✓ Rust $(rustc --version)"

# Check required tools
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo ""
    echo "Recommended tools:"

    tools=(cmake ninja pkg-config)
    for tool in "${tools[@]}"; do
        if command -v $tool &> /dev/null; then
            echo "✓ $tool"
        else
            echo "⚠ $tool (install with: brew install $tool)"
        fi
    done
fi

# Create directory structure
echo ""
echo "Setting up directories..."
mkdir -p models
echo "✓ Created models/"

# Create support directory
if [[ "$OSTYPE" == "darwin"* ]]; then
    SUPPORT_DIR="$HOME/Library/Application Support/MacAutoComplete"
    mkdir -p "$SUPPORT_DIR"
    echo "✓ Created $SUPPORT_DIR"
fi

echo ""
echo "Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Download a model: ./scripts/download_models.sh (optional)"
echo "  2. Build the engine: make engine"
echo "  3. Run in dev mode: make dev"
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "  4. Build the IME: open mac/IME/IMEApp.xcodeproj"
fi
