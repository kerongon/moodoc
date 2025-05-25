#!/bin/bash

# Test script for comprehensive Docker engine detection
# This script will test the new Docker path resolution functionality

echo "🔍 Testing Comprehensive Docker Engine Detection"
echo "================================================"

# Check which Docker engines are available on this system
echo ""
echo "📋 Available Docker installations:"

check_docker_path() {
    local path=$1
    local name=$2
    
    if [ -f "$path" ]; then
        echo "✅ $name: $path"
        if [ -x "$path" ]; then
            version=$("$path" --version 2>/dev/null | head -n1)
            echo "   Version: $version"
        else
            echo "   ⚠️  Found but not executable"
        fi
    else
        echo "❌ $name: $path (not found)"
    fi
}

# Test standard Docker Desktop
check_docker_path "/Applications/Docker.app/Contents/Resources/bin/docker" "Docker Desktop"

# Test OrbStack
check_docker_path "/opt/orbstack/bin/docker" "OrbStack (system)"
check_docker_path "$HOME/.orbstack/bin/docker" "OrbStack (user)"

# Test Rancher Desktop
check_docker_path "$HOME/.rd/bin/docker" "Rancher Desktop"

# Test Homebrew installations
check_docker_path "/usr/local/bin/docker" "Homebrew (Intel Mac)"
check_docker_path "/opt/homebrew/bin/docker" "Homebrew (Apple Silicon)"

# Test Podman Desktop
check_docker_path "$HOME/.local/share/containers/podman/machine/bin/docker" "Podman Desktop"

# Test Lima/Colima
check_docker_path "$HOME/.colima/bin/docker" "Colima"
if [ -d "$HOME/.lima" ]; then
    echo "🔍 Checking Lima installations..."
    for lima_dir in "$HOME/.lima"/*/bin; do
        if [ -f "$lima_dir/docker" ]; then
            check_docker_path "$lima_dir/docker" "Lima ($(basename $(dirname $lima_dir)))"
        fi
    done
fi

# Test system installation
check_docker_path "/usr/bin/docker" "System Docker"

echo ""
echo "🧪 Testing which Docker the system would use:"
echo "PATH Docker: $(which docker 2>/dev/null || echo 'not found in PATH')"

if command -v docker >/dev/null 2>&1; then
    echo "Docker version from PATH: $(docker --version)"
else
    echo "❌ Docker not accessible from PATH"
fi

echo ""
echo "🎯 This demonstrates why comprehensive path resolution is needed"
echo "   when launching from Applications folder (no PATH environment)"
