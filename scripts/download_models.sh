#!/bin/bash
# Download required models

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
MODELS_DIR="$PROJECT_ROOT/models"

echo "MacAutoComplete - Model Downloader"
echo "==================================="
echo ""

mkdir -p "$MODELS_DIR"

# Model URLs (these are examples - replace with actual URLs)
TINYLLAMA_URL="https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf"

echo "Available models:"
echo "  1. TinyLlama 1.1B Q4_K_M (~0.7 GB) - Recommended"
echo "  2. Qwen2.5-1.5B Q4 (~1.0 GB)"
echo "  3. Skip (use existing model)"
echo ""

read -p "Select model to download [1-3]: " choice

case $choice in
    1)
        echo "Downloading TinyLlama 1.1B Q4_K_M..."
        echo "Note: This requires curl or wget"

        if command -v curl &> /dev/null; then
            curl -L "$TINYLLAMA_URL" -o "$MODELS_DIR/tinyllama-1.1b.Q4_K_M.gguf"
        elif command -v wget &> /dev/null; then
            wget "$TINYLLAMA_URL" -O "$MODELS_DIR/tinyllama-1.1b.Q4_K_M.gguf"
        else
            echo "Error: Neither curl nor wget found"
            echo "Please download manually from: $TINYLLAMA_URL"
            exit 1
        fi

        echo "✓ Model downloaded to $MODELS_DIR/tinyllama-1.1b.Q4_K_M.gguf"
        ;;
    2)
        echo "Note: Download Qwen2.5-1.5B from HuggingFace manually"
        echo "URL: https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF"
        ;;
    3)
        echo "Skipping download"
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

# Copy to support directory on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    SUPPORT_DIR="$HOME/Library/Application Support/MacAutoComplete/models"
    mkdir -p "$SUPPORT_DIR"

    if [ -f "$MODELS_DIR/tinyllama-1.1b.Q4_K_M.gguf" ]; then
        echo "Copying model to $SUPPORT_DIR..."
        cp "$MODELS_DIR/tinyllama-1.1b.Q4_K_M.gguf" "$SUPPORT_DIR/"
        echo "✓ Model installed"
    fi
fi

echo ""
echo "Model setup complete!"
