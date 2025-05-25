#!/bin/bash

# Production Diagnostic Testing Script
# Tests the diagnostic functionality to identify production vs development differences

echo "🔍 Moodle Docker Manager - Production Diagnostic Test"
echo "=================================================="
echo ""

# Test 1: Verify both builds exist
echo "📋 Test 1: Verifying Build Artifacts"
echo "------------------------------------"

DEV_RUNNING=$(pgrep -f "moodle-docker-manager" | head -1)
PROD_APP="/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"

if [ -n "$DEV_RUNNING" ]; then
    echo "✅ Development server is running (PID: $DEV_RUNNING)"
else
    echo "❌ Development server is not running"
    echo "   Starting development server..."
    cd /Users/kerongonzales/code/moodoc/moodle-docker-manager
    npm run tauri dev &
    DEV_PID=$!
    echo "   Started with PID: $DEV_PID"
    echo "   Waiting 10 seconds for startup..."
    sleep 10
fi

if [ -d "$PROD_APP" ]; then
    echo "✅ Production build exists at: $PROD_APP"
else
    echo "❌ Production build not found"
    exit 1
fi

echo ""

# Test 2: Check Docker environment
echo "📋 Test 2: Docker Environment Verification"
echo "-----------------------------------------"

# Check if Docker is installed
if command -v docker &> /dev/null; then
    echo "✅ Docker CLI is installed"
    DOCKER_VERSION=$(docker --version)
    echo "   Version: $DOCKER_VERSION"
else
    echo "❌ Docker CLI not found in PATH"
fi

# Check if Docker is running
if docker info &> /dev/null; then
    echo "✅ Docker daemon is running"
    echo "   Container count: $(docker ps -a --format 'table {{.Names}}' | tail -n +2 | wc -l | tr -d ' ')"
else
    echo "❌ Docker daemon is not accessible"
    echo "   This could be a production-specific issue"
fi

# Check Docker socket permissions
if [ -S "/var/run/docker.sock" ]; then
    DOCKER_SOCK_PERMS=$(ls -la /var/run/docker.sock)
    echo "✅ Docker socket exists: $DOCKER_SOCK_PERMS"
else
    echo "ℹ️  Docker socket not at standard location (normal on macOS)"
fi

echo ""

# Test 3: Environment Variables
echo "📋 Test 3: Environment Variables Analysis"
echo "----------------------------------------"

echo "PATH directories containing 'docker':"
echo "$PATH" | tr ':' '\n' | grep -i docker || echo "   No Docker-specific paths found"

echo ""
echo "Docker-related environment variables:"
env | grep -i docker || echo "   No Docker-specific variables found"

echo ""

# Test 4: Create test directory structure
echo "📋 Test 4: Test Directory Setup"
echo "------------------------------"

TEST_DIR="/tmp/moodle-diagnostic-test"
if [ -d "$TEST_DIR" ]; then
    echo "🧹 Cleaning existing test directory..."
    rm -rf "$TEST_DIR"
fi

mkdir -p "$TEST_DIR"
echo "✅ Created test directory: $TEST_DIR"

# Test write permissions
TEST_FILE="$TEST_DIR/test-write.txt"
if echo "test" > "$TEST_FILE" 2>/dev/null; then
    echo "✅ Write permissions verified"
    rm "$TEST_FILE"
else
    echo "❌ Write permission denied"
fi

echo ""

# Test 5: Port availability
echo "📋 Test 5: Port Availability Check"
echo "---------------------------------"

for port in 8000 8001 3306 3307 5432; do
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null; then
        echo "⚠️  Port $port is in use"
    else
        echo "✅ Port $port is available"
    fi
done

echo ""

# Test 6: Network connectivity
echo "📋 Test 6: Network Connectivity"
echo "------------------------------"

# Test GitHub connectivity (for Moodle/Docker repo cloning)
if ping -c 1 github.com &> /dev/null; then
    echo "✅ GitHub is reachable"
else
    echo "❌ GitHub is not reachable"
fi

# Test Docker Hub connectivity
if ping -c 1 registry-1.docker.io &> /dev/null; then
    echo "✅ Docker Hub is reachable"
else
    echo "❌ Docker Hub is not reachable"
fi

echo ""

# Test 7: File system checks
echo "📋 Test 7: File System Analysis"
echo "------------------------------"

# Check available disk space
DISK_USAGE=$(df -h /tmp | tail -1 | awk '{print $4}')
echo "Available space in /tmp: $DISK_USAGE"

# Check if tmp is writable and executable
if [ -w /tmp ] && [ -x /tmp ]; then
    echo "✅ /tmp is writable and executable"
else
    echo "❌ /tmp permissions issue"
fi

echo ""

# Test 8: Production app launch test
echo "📋 Test 8: Production App Launch Test"
echo "------------------------------------"

echo "🚀 Launching production app..."
echo "   (This will open the production build)"
echo "   Please manually test the diagnostic functionality in the production app"
echo "   and compare with the development version"

# Open production app
open "$PROD_APP"

echo ""
echo "📝 Manual Testing Instructions:"
echo "------------------------------"
echo "1. In BOTH the development AND production versions:"
echo "   a. Click the 'Diagnostics' button in the main interface"
echo "   b. Review all diagnostic sections for differences"
echo "   c. Note any error messages or missing information"
echo ""
echo "2. Try creating a test site in BOTH versions:"
echo "   a. Site name: diagnostic-test-$(date +%s)"
echo "   b. Directory: $TEST_DIR"
echo "   c. Monitor for any 'error' status"
echo ""
echo "3. Compare diagnostic outputs and note differences"
echo ""

# Test 9: Automated diagnostic data collection
echo "📋 Test 9: Collecting Diagnostic Data"
echo "------------------------------------"

DIAGNOSTIC_LOG="/tmp/moodle-diagnostic-$(date +%Y%m%d-%H%M%S).log"

echo "Collecting diagnostic information to: $DIAGNOSTIC_LOG"

{
    echo "=== Moodle Docker Manager Diagnostic Report ==="
    echo "Generated: $(date)"
    echo "System: $(uname -a)"
    echo ""
    
    echo "=== Docker Information ==="
    docker --version 2>&1 || echo "Docker CLI not available"
    docker info 2>&1 || echo "Docker daemon not accessible"
    echo ""
    
    echo "=== Environment ==="
    echo "USER: $USER"
    echo "HOME: $HOME"
    echo "PWD: $PWD"
    echo "PATH: $PATH"
    echo ""
    
    echo "=== Process Information ==="
    ps aux | grep -E "(docker|moodle)" | grep -v grep || echo "No relevant processes"
    echo ""
    
    echo "=== Network Ports ==="
    lsof -i :8000-8010 || echo "No processes on ports 8000-8010"
    lsof -i :3306-3310 || echo "No processes on ports 3306-3310"
    echo ""
    
    echo "=== File Permissions ==="
    ls -la "$TEST_DIR"
    echo ""
    
} > "$DIAGNOSTIC_LOG"

echo "✅ Diagnostic data saved to: $DIAGNOSTIC_LOG"

echo ""
echo "🎯 Next Steps:"
echo "============="
echo "1. Review the diagnostic outputs in both development and production"
echo "2. Identify any differences or error conditions"
echo "3. If issues are found, use the diagnostic information to troubleshoot"
echo "4. Document any production-specific fixes needed"
echo ""
echo "📊 Test Summary:"
echo "==============="
echo "✅ Build verification completed"
echo "✅ Docker environment checked"
echo "✅ Environment variables analyzed"
echo "✅ Test directories created"
echo "✅ Port availability verified"
echo "✅ Network connectivity tested"
echo "✅ File system analyzed"
echo "✅ Production app launched"
echo "✅ Diagnostic data collected"
echo ""
echo "Manual testing in progress - please compare diagnostic outputs between dev and prod versions."
