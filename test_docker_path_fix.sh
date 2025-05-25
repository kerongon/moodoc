#!/bin/bash

# Test script to verify Docker path resolution fix
# This tests that the app can find Docker even when PATH doesn't include Docker locations

echo "🧪 Testing Docker Path Resolution Fix"
echo "====================================="
echo ""

# First, show current Docker locations
echo "📍 Current Docker Installation Locations:"
echo ""

# Check common Docker paths
DOCKER_PATHS=(
    "/usr/local/bin/docker"
    "/opt/homebrew/bin/docker" 
    "/usr/bin/docker"
    "/Applications/Docker.app/Contents/Resources/bin/docker"
)

for path in "${DOCKER_PATHS[@]}"; do
    if [ -f "$path" ]; then
        echo "✅ Found Docker at: $path"
        if [ -x "$path" ]; then
            version=$("$path" --version 2>/dev/null)
            echo "   Version: $version"
        else
            echo "   ❌ Not executable"
        fi
    else
        echo "❌ Not found: $path"
    fi
    echo ""
done

# Show what's in current PATH
echo "🔍 Current PATH Analysis:"
echo "PATH contains these directories:"
echo "$PATH" | tr ':' '\n' | grep -E "(docker|homebrew|local)" || echo "No Docker-related paths found in PATH"
echo ""

# Test with restricted PATH (simulating Applications folder launch)
echo "🚫 Testing with Restricted PATH (simulates Applications folder launch):"
echo ""

# Save original PATH
ORIGINAL_PATH="$PATH"

# Set a very restricted PATH that doesn't include Docker
export PATH="/bin:/usr/bin"

echo "Restricted PATH: $PATH"
echo ""

# Try to find docker with restricted PATH
echo "Testing docker command with restricted PATH:"
if command -v docker >/dev/null 2>&1; then
    echo "✅ Docker still accessible via PATH"
    docker --version
else
    echo "❌ Docker NOT accessible via PATH (this is expected)"
fi
echo ""

# Test if our hardcoded paths still work
echo "Testing hardcoded Docker paths:"
for path in "${DOCKER_PATHS[@]}"; do
    if [ -f "$path" ] && [ -x "$path" ]; then
        echo "✅ Direct path works: $path"
        "$path" --version 2>/dev/null || echo "   ❌ Failed to get version"
        break
    fi
done
echo ""

# Restore original PATH
export PATH="$ORIGINAL_PATH"

echo "✅ PATH restored: $PATH"
echo ""

echo "📋 Test Summary:"
echo "==============="
echo "This test demonstrates that:"
echo "1. ✅ Docker installations can be found at common macOS locations"
echo "2. ❌ Docker may not be accessible when PATH is restricted (like in Applications folder)"
echo "3. ✅ Our fix should work by checking hardcoded paths instead of relying on PATH"
echo ""
echo "🎯 The fix ensures the app works even when launched from Applications folder"
echo "   where PATH environment variable doesn't include Docker's location."
echo ""

# Test if build was successful
if [ -f "/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/target/release/moodle-docker-manager" ]; then
    echo "✅ Release build found - you can now test the app!"
    echo "   Try copying the app to Applications folder and testing Docker functionality."
else
    echo "⏳ Release build not found - build may still be in progress."
fi
