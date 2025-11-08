#!/bin/bash
# Notarization script for macOS

set -e

APP_PATH="$1"
BUNDLE_ID="com.macautocomplete.IMEApp"
APPLE_ID="your-apple-id@example.com"
TEAM_ID="YOUR_TEAM_ID"

if [ -z "$APP_PATH" ]; then
    echo "Usage: $0 <path-to-app>"
    exit 1
fi

echo "MacAutoComplete - Notarization"
echo "==============================="
echo "App: $APP_PATH"
echo ""

# Check if app exists
if [ ! -d "$APP_PATH" ]; then
    echo "Error: App not found at $APP_PATH"
    exit 1
fi

# Create archive for notarization
echo "Creating archive..."
ARCHIVE_PATH="${APP_PATH}.zip"
ditto -c -k --keepParent "$APP_PATH" "$ARCHIVE_PATH"

# Submit for notarization
echo "Submitting for notarization..."
echo "Note: This requires Apple Developer credentials"
echo ""

xcrun notarytool submit "$ARCHIVE_PATH" \
    --apple-id "$APPLE_ID" \
    --team-id "$TEAM_ID" \
    --wait

# Staple the ticket
echo "Stapling ticket..."
xcrun stapler staple "$APP_PATH"

echo ""
echo "✓ Notarization complete!"
echo ""
echo "The app is now notarized and ready for distribution."
