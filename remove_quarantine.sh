#!/bin/bash

# Helper script for users to remove quarantine attributes
# This can be included in your distribution

echo "🔧 Moodle Docker Manager - Quarantine Removal Tool"
echo ""

APP_PATH="/Applications/Moodle Docker Manager.app"

if [ ! -d "$APP_PATH" ]; then
    echo "❌ App not found at: $APP_PATH"
    echo "   Please install the app to Applications folder first"
    exit 1
fi

echo "🔍 Checking quarantine status..."
if xattr -l "$APP_PATH" | grep -q "com.apple.quarantine"; then
    echo "⚠️  App has quarantine attribute"
    echo "🧹 Removing quarantine attribute..."
    
    if sudo xattr -r -d com.apple.quarantine "$APP_PATH"; then
        echo "✅ Quarantine attribute removed successfully!"
        echo "🚀 You can now launch the app normally"
    else
        echo "❌ Failed to remove quarantine attribute"
        echo "   Make sure you entered your password correctly"
    fi
else
    echo "✅ App has no quarantine attribute"
    echo "🚀 App should launch without issues"
fi

echo ""
echo "💡 You can now launch Moodle Docker Manager from Applications"
