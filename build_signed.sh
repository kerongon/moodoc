#!/bin/bash

# Build script for signed macOS Tauri app
# This script builds, signs, and optionally notarizes the app

set -e  # Exit on any error

echo "🔨 Building Moodle Docker Manager with code signing..."

# Load environment variables from .env if it exists
if [ -f ".env" ]; then
    echo "📄 Loading environment from .env file..."
    export $(cat .env | grep -v '^#' | xargs)
fi

# Check for required environment variables
if [ -z "$APPLE_CERTIFICATE" ]; then
    echo "❌ Error: APPLE_CERTIFICATE not set."
    echo "📋 Available certificates:"
    security find-identity -v -p codesigning
    echo ""
    echo "🛠️  Setup steps:"
    echo "   1. Get Apple Developer account ($99/year)"
    echo "   2. Create Developer ID Application certificate"
    echo "   3. Install certificate in Keychain Access"
    echo "   4. Copy .env.example to .env and configure"
    echo "   5. Set APPLE_CERTIFICATE to exact certificate name"
    exit 1
fi

# Verify certificate exists
echo "🔍 Verifying certificate: $APPLE_CERTIFICATE"
if ! security find-identity -v -p codesigning | grep -q "$APPLE_CERTIFICATE"; then
    echo "❌ Error: Certificate '$APPLE_CERTIFICATE' not found in keychain"
    echo "📋 Available certificates:"
    security find-identity -v -p codesigning
    exit 1
fi

if [ -z "$APPLE_ID" ] || [ -z "$APPLE_PASSWORD" ] || [ -z "$APPLE_TEAM_ID" ]; then
    echo "⚠️  Warning: Apple ID credentials not set. Skipping notarization."
    echo "   For distribution, set:"
    echo "   - APPLE_ID (your Apple ID email)"
    echo "   - APPLE_PASSWORD (app-specific password)"
    echo "   - APPLE_TEAM_ID (your team ID)"
    ENABLE_NOTARIZATION="false"
fi

# Set signing identity in environment
export TAURI_SIGNING_IDENTITY="$APPLE_CERTIFICATE"

if [ "$ENABLE_NOTARIZATION" = "true" ] && [ -n "$APPLE_ID" ]; then
    export TAURI_APPLE_ID="$APPLE_ID"
    export TAURI_APPLE_PASSWORD="$APPLE_PASSWORD"
    export TAURI_APPLE_TEAM_ID="$APPLE_TEAM_ID"
    echo "🍎 Notarization enabled"
else
    echo "⚠️  Notarization disabled"
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cd /Users/kerongonzales/code/moodoc/moodle-docker-manager
rm -rf src-tauri/target/release/bundle/

# Build the app
echo "📦 Building release bundle..."
npm run tauri build

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully!"
    echo ""
    
    APP_PATH="src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"
    echo "📍 Built app location: $APP_PATH"
    echo ""
    
    # Verify the signature
    echo "🔍 Verifying code signature..."
    if codesign -v --verbose=2 "$APP_PATH"; then
        echo "✅ Code signature verified successfully!"
        
        # Get signing info
        echo "📋 Signing information:"
        codesign -dv --verbose=4 "$APP_PATH" 2>&1 | grep -E "(Authority|Identifier|TeamIdentifier)"
        
        if [ "$ENABLE_NOTARIZATION" = "true" ] && [ -n "$APPLE_ID" ]; then
            echo "📨 Checking notarization status..."
            if spctl -a -v "$APP_PATH" 2>&1 | grep -q "source=Notarized Developer ID"; then
                echo "✅ App is properly notarized!"
            else
                echo "⚠️  App signature verified but notarization status unclear"
                echo "    This may still work, but full notarization is recommended for distribution"
            fi
        fi
        
        echo ""
        echo "🎉 SUCCESS: App should work on other Macs without quarantine warnings!"
        
    else
        echo "❌ Code signature verification failed!"
        echo "   App may still have quarantine issues"
    fi
    
    echo ""
    echo "🚀 Next steps:"
    echo "   1. Test locally: cp -r '$APP_PATH' /Applications/"
    echo "   2. Test on another Mac to verify no quarantine warnings"
    echo "   3. For distribution: Create DMG or ZIP of signed app"
    
else
    echo "❌ Build failed!"
    exit 1
fi
