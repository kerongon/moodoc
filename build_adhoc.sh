#!/bin/bash

# Build script for ad-hoc signed macOS Tauri app
# This provides basic signing without Apple Developer account
# Note: Users will still see warnings but can bypass them more easily

set -e

echo "🔨 Building Moodle Docker Manager with ad-hoc signing..."

# Build the app
echo "📦 Building release bundle..."
cd /Users/kerongonzales/code/moodoc/moodle-docker-manager
npm run tauri build

APP_PATH="src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully!"
    echo ""
    echo "📍 Built app location: $APP_PATH"
    echo ""
    
    # Ad-hoc sign the app
    echo "🔐 Applying ad-hoc signature..."
    codesign --force --deep --sign - "$APP_PATH"
    
    # Verify the signature
    echo "🔍 Verifying ad-hoc signature..."
    if codesign -v "$APP_PATH"; then
        echo "✅ Ad-hoc signature applied successfully!"
        
        echo ""
        echo "⚠️  Note: This is an ad-hoc signature"
        echo "   - Reduces some security warnings"
        echo "   - Users can bypass warnings more easily"
        echo "   - Not suitable for wide distribution"
        echo "   - Still shows 'unidentified developer' warning"
        
    else
        echo "❌ Ad-hoc signing failed!"
    fi
    
    echo ""
    echo "🚀 Distribution instructions for users:"
    echo "   1. Download the app"
    echo "   2. Right-click → Open (don't double-click)"
    echo "   3. Click 'Open' when warned about unidentified developer"
    echo "   4. App will run and be trusted for future launches"
    
else
    echo "❌ Build failed!"
    exit 1
fi
