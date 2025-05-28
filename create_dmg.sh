#!/bin/bash

# Distribution script for Moodle Docker Manager
# Creates a signed DMG for distribution

set -e

echo "📦 Creating distribution package for Moodle Docker Manager..."

# Build signed app first
if [ ! -f "build_signed.sh" ]; then
    echo "❌ build_signed.sh not found. Run from project root."
    exit 1
fi

echo "🔨 Building signed app..."
./build_signed.sh

APP_PATH="moodle-docker-manager/src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"
DMG_NAME="Moodle-Docker-Manager-$(date +%Y%m%d).dmg"

if [ ! -d "$APP_PATH" ]; then
    echo "❌ App not found at $APP_PATH"
    exit 1
fi

# Create temporary DMG directory
echo "📁 Creating DMG structure..."
TMP_DIR=$(mktemp -d)
mkdir -p "$TMP_DIR/Moodle Docker Manager"
cp -R "$APP_PATH" "$TMP_DIR/Moodle Docker Manager/"

# Create symbolic link to Applications
ln -s /Applications "$TMP_DIR/Moodle Docker Manager/Applications"

# Create DMG
echo "💿 Creating DMG..."
hdiutil create -volname "Moodle Docker Manager" \
    -srcfolder "$TMP_DIR/Moodle Docker Manager" \
    -ov -format UDZO \
    "$DMG_NAME"

# Clean up
rm -rf "$TMP_DIR"

# Sign the DMG if we have signing set up
if [ -n "$APPLE_CERTIFICATE" ] || [ -f ".env" ]; then
    if [ -f ".env" ]; then
        export $(cat .env | grep -v '^#' | xargs)
    fi
    
    if [ -n "$APPLE_CERTIFICATE" ]; then
        echo "🔐 Signing DMG..."
        codesign --sign "$APPLE_CERTIFICATE" "$DMG_NAME"
        
        echo "🔍 Verifying DMG signature..."
        codesign -v "$DMG_NAME"
    fi
fi

echo ""
echo "✅ Distribution package created: $DMG_NAME"
echo "📊 Package size: $(du -h "$DMG_NAME" | cut -f1)"
echo ""
echo "🚀 Ready for distribution!"
echo "   Users can download and install without quarantine warnings"
