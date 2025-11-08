#!/bin/bash
# Release build script

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"
VERSION="0.1.0"

echo "MacAutoComplete - Release Build"
echo "==============================="
echo "Version: $VERSION"
echo ""

# Check if on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "Error: Release builds must be done on macOS"
    exit 1
fi

# Clean and create dist directory
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build engine
echo "Building engine (release)..."
cd "$PROJECT_ROOT/engine"
cargo build --release
echo "✓ Engine built"

# Build IME
echo ""
echo "Building IME..."
echo "Note: You must build the Xcode project manually first"
echo "  1. Open mac/IME/IMEApp.xcodeproj"
echo "  2. Select 'Product > Archive'"
echo "  3. Export the app"
echo ""

# Copy artifacts
echo "Copying artifacts to dist/..."
mkdir -p "$DIST_DIR/MacAutoComplete"
cp "$PROJECT_ROOT/engine/target/release/mac_autocomplete_engine" "$DIST_DIR/MacAutoComplete/"

# Copy documentation
cp "$PROJECT_ROOT/README.md" "$DIST_DIR/MacAutoComplete/"
cp "$PROJECT_ROOT/LICENSE" "$DIST_DIR/MacAutoComplete/"

# Create install script
cat > "$DIST_DIR/MacAutoComplete/install.sh" <<'EOF'
#!/bin/bash

set -e

echo "MacAutoComplete Installer"
echo "========================="
echo ""

# Install engine
SUPPORT_DIR="$HOME/Library/Application Support/MacAutoComplete"
mkdir -p "$SUPPORT_DIR"

cp mac_autocomplete_engine "$SUPPORT_DIR/engine"
chmod +x "$SUPPORT_DIR/engine"

echo "✓ Engine installed to $SUPPORT_DIR/engine"

# Create default config if it doesn't exist
if [ ! -f "$SUPPORT_DIR/config.json" ]; then
    echo "Creating default configuration..."
    # (config content would go here)
fi

echo ""
echo "Installation complete!"
echo ""
echo "Next steps:"
echo "  1. Install the IME bundle to ~/Library/Input Methods/"
echo "  2. Open System Settings > Keyboard > Input Sources"
echo "  3. Click '+' and add 'MacAutoComplete'"
echo "  4. Run the MacAutoComplete app to start the engine"
echo ""
EOF

chmod +x "$DIST_DIR/MacAutoComplete/install.sh"

# Create archive
cd "$DIST_DIR"
tar -czf "MacAutoComplete-$VERSION-macos.tar.gz" MacAutoComplete/

echo ""
echo "✓ Release build complete!"
echo "Package: $DIST_DIR/MacAutoComplete-$VERSION-macos.tar.gz"
