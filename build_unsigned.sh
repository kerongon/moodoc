#!/bin/bash

# Enhanced build script for unsigned apps with user instructions
# Provides clear guidance for users on how to install unsigned apps

set -e

echo "🔨 Building Moodle Docker Manager (unsigned distribution)..."

# Build the app
echo "📦 Building release bundle..."
cd /Users/kerongonzales/code/moodoc/moodle-docker-manager
npm run tauri build

APP_PATH="src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"

if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully!"
    echo ""
    
    # Create installation instructions
    INSTRUCTIONS_FILE="Installation_Instructions.txt"
    cat > "$INSTRUCTIONS_FILE" << EOF
MOODLE DOCKER MANAGER - Installation Instructions
================================================

This app is not signed with an Apple Developer certificate, so macOS will show security warnings.
This is normal for free/open-source software that doesn't pay for Apple's Developer Program.

INSTALLATION STEPS:
==================

1. Download and extract the app
2. Move "Moodle Docker Manager.app" to your Applications folder
3. DO NOT double-click the app yet!

4. First launch (bypass security warning):
   - Right-click on "Moodle Docker Manager.app" 
   - Select "Open" from the context menu
   - Click "Open" when macOS warns about unidentified developer
   - The app will now launch and be trusted for future use

5. For future launches:
   - You can now double-click the app normally
   - No more security warnings will appear

ALTERNATIVE METHOD:
==================
If the right-click method doesn't work:

1. Open System Preferences → Security & Privacy
2. Try to launch the app normally (it will be blocked)
3. Go to Security & Privacy → General tab
4. Click "Open Anyway" next to the blocked app message
5. Confirm by clicking "Open"

TROUBLESHOOTING:
===============
If you see "App is damaged":
1. Open Terminal
2. Run: sudo xattr -r -d com.apple.quarantine "/Applications/Moodle Docker Manager.app"
3. Enter your password when prompted
4. Try launching the app again

This is safe - you're just removing the quarantine flag that macOS adds to downloaded files.

SECURITY NOTE:
=============
This app is open source and safe to use. The security warnings are due to Apple's 
requirement for paid Developer Program membership (\$99/year) to eliminate these warnings.
Many legitimate open-source applications show these same warnings.

Support: https://github.com/yourusername/moodle-docker-manager
EOF

    echo "📝 Created installation instructions: $INSTRUCTIONS_FILE"
    echo ""
    echo "📍 Files ready for distribution:"
    echo "   - App: $APP_PATH"
    echo "   - Instructions: $INSTRUCTIONS_FILE"
    echo ""
    echo "📦 Creating distribution package..."
    
    # Create a distribution folder
    DIST_DIR="Moodle_Docker_Manager_Distribution"
    rm -rf "$DIST_DIR"
    mkdir -p "$DIST_DIR"
    
    # Copy app and instructions
    cp -R "$APP_PATH" "$DIST_DIR/"
    cp "$INSTRUCTIONS_FILE" "$DIST_DIR/"
    
    # Create a README for the distribution
    cat > "$DIST_DIR/README.txt" << EOF
Moodle Docker Manager
====================

Thank you for downloading Moodle Docker Manager!

IMPORTANT: Please read "Installation_Instructions.txt" before installing.

This will guide you through the simple process of installing an unsigned macOS app.

Contents:
- Moodle Docker Manager.app (the application)
- Installation_Instructions.txt (setup guide)
- README.txt (this file)
EOF

    echo "✅ Distribution package created: $DIST_DIR/"
    echo ""
    echo "🚀 Ready for distribution!"
    echo "   - ZIP the '$DIST_DIR' folder"
    echo "   - Users follow the included instructions"
    echo "   - No quarantine issues when instructions are followed"
    
else
    echo "❌ Build failed!"
    exit 1
fi
