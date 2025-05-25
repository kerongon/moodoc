#!/bin/bash

# Production vs Development Diagnostic Validation Script
# This script validates that our diagnostic functionality works correctly

echo "🚀 Moodle Docker Manager Production Diagnostic Validation"
echo "========================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "SUCCESS") echo -e "${GREEN}✅ $message${NC}" ;;
        "ERROR") echo -e "${RED}❌ $message${NC}" ;;
        "WARNING") echo -e "${YELLOW}⚠️  $message${NC}" ;;
        "INFO") echo -e "${BLUE}ℹ️  $message${NC}" ;;
    esac
}

# Test Docker environment
test_docker() {
    echo -e "\n${BLUE}🐳 Testing Docker Environment${NC}"
    echo "--------------------------------"
    
    if command -v docker &> /dev/null; then
        print_status "SUCCESS" "Docker is installed: $(docker --version)"
    else
        print_status "ERROR" "Docker is not installed or not in PATH"
        return 1
    fi
    
    if docker info &> /dev/null; then
        print_status "SUCCESS" "Docker daemon is running"
    else
        print_status "ERROR" "Docker daemon is not running"
        return 1
    fi
    
    # Test Docker socket permissions
    if [ -S "/var/run/docker.sock" ]; then
        print_status "INFO" "Docker socket found at /var/run/docker.sock"
        if [ -r "/var/run/docker.sock" ] && [ -w "/var/run/docker.sock" ]; then
            print_status "SUCCESS" "Docker socket is accessible"
        else
            print_status "WARNING" "Docker socket permission issues detected"
        fi
    else
        print_status "INFO" "Docker socket not found (Docker Desktop mode)"
    fi
}

# Test build artifacts
test_builds() {
    echo -e "\n${BLUE}🏗️  Testing Build Artifacts${NC}"
    echo "-------------------------------"
    
    # Check development build process
    if pgrep -f "moodle-docker-manager" > /dev/null; then
        print_status "SUCCESS" "Development server is running"
    else
        print_status "WARNING" "Development server is not running"
    fi
    
    # Check production build
    PROD_APP="/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"
    if [ -d "$PROD_APP" ]; then
        print_status "SUCCESS" "Production build exists"
        
        # Check if production app is running
        if pgrep -f "Moodle Docker Manager" > /dev/null; then
            print_status "SUCCESS" "Production app is running"
        else
            print_status "INFO" "Production app is not currently running"
        fi
    else
        print_status "ERROR" "Production build not found"
    fi
}

# Test file system permissions
test_permissions() {
    echo -e "\n${BLUE}📁 Testing File System Permissions${NC}"
    echo "------------------------------------"
    
    # Test site directories
    TEST_DIRS=(
        "/tmp/moodle-test-sites"
        "$HOME/Documents"
        "$HOME/Library/Application Support"
    )
    
    for dir in "${TEST_DIRS[@]}"; do
        if [ -d "$dir" ]; then
            if [ -w "$dir" ]; then
                print_status "SUCCESS" "Write access to $dir"
            else
                print_status "ERROR" "No write access to $dir"
            fi
        else
            print_status "WARNING" "Directory does not exist: $dir"
        fi
    done
    
    # Test creating a temporary file in each writable directory
    for dir in "${TEST_DIRS[@]}"; do
        if [ -d "$dir" ] && [ -w "$dir" ]; then
            TEST_FILE="$dir/moodle_test_$$"
            if echo "test" > "$TEST_FILE" 2>/dev/null; then
                rm -f "$TEST_FILE"
                print_status "SUCCESS" "Can create files in $dir"
            else
                print_status "ERROR" "Cannot create files in $dir"
            fi
        fi
    done
}

# Test network ports
test_ports() {
    echo -e "\n${BLUE}🌐 Testing Network Ports${NC}"
    echo "-------------------------"
    
    # Common Moodle ports
    PORTS=(8000 8001 8080 3306 5432)
    
    for port in "${PORTS[@]}"; do
        if nc -z localhost $port 2>/dev/null; then
            print_status "WARNING" "Port $port is in use"
        else
            print_status "SUCCESS" "Port $port is available"
        fi
    done
}

# Test environment variables
test_environment() {
    echo -e "\n${BLUE}🔧 Testing Environment Variables${NC}"
    echo "----------------------------------"
    
    # Important environment variables
    ENV_VARS=(
        "PATH"
        "DOCKER_HOST"
        "DOCKER_CERT_PATH"
        "DOCKER_TLS_VERIFY"
        "HOME"
        "USER"
    )
    
    for var in "${ENV_VARS[@]}"; do
        if [ -n "${!var}" ]; then
            if [ "$var" = "PATH" ]; then
                print_status "SUCCESS" "$var is set (${#!var} characters)"
            else
                print_status "SUCCESS" "$var = ${!var}"
            fi
        else
            print_status "WARNING" "$var is not set"
        fi
    done
}

# Test diagnostic functionality
test_diagnostic_features() {
    echo -e "\n${BLUE}🔍 Testing Diagnostic Features${NC}"
    echo "--------------------------------"
    
    # Check if diagnostic modal component exists
    DIAGNOSTIC_MODAL="/Users/kerongonzales/code/moodoc/moodle-docker-manager/src/components/DiagnosticModal.vue"
    if [ -f "$DIAGNOSTIC_MODAL" ]; then
        print_status "SUCCESS" "DiagnosticModal.vue component exists"
    else
        print_status "ERROR" "DiagnosticModal.vue component missing"
    fi
    
    # Check if diagnostic commands are registered
    MAIN_RS="/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/src/main.rs"
    if grep -q "run_diagnostics" "$MAIN_RS" 2>/dev/null; then
        print_status "SUCCESS" "run_diagnostics command is registered"
    else
        print_status "ERROR" "run_diagnostics command not found in main.rs"
    fi
    
    if grep -q "check_site_status_detailed" "$MAIN_RS" 2>/dev/null; then
        print_status "SUCCESS" "check_site_status_detailed command is registered"
    else
        print_status "ERROR" "check_site_status_detailed command not found in main.rs"
    fi
}

# Generate recommendations
generate_recommendations() {
    echo -e "\n${BLUE}💡 Recommendations${NC}"
    echo "-------------------"
    
    echo "Based on the diagnostic tests above:"
    echo ""
    echo "1. IMMEDIATE ACTIONS:"
    echo "   - Ensure Docker Desktop is running"
    echo "   - Verify all file permissions are correct"
    echo "   - Test both development and production builds"
    echo ""
    echo "2. TESTING WORKFLOW:"
    echo "   a) Start development server: npm run tauri dev"
    echo "   b) Test diagnostic features in development"
    echo "   c) Launch production app and compare results"
    echo "   d) Document any differences found"
    echo ""
    echo "3. TROUBLESHOOTING:"
    echo "   - Use diagnostic modal for real-time analysis"
    echo "   - Check site error details for specific issues"
    echo "   - Compare environment variables between builds"
    echo ""
    echo "4. SUCCESS CRITERIA:"
    echo "   - No 'error' status sites in production"
    echo "   - Diagnostic features work identically in both environments"
    echo "   - All Docker operations succeed"
}

# Main execution
main() {
    test_docker
    test_builds
    test_permissions
    test_ports
    test_environment
    test_diagnostic_features
    generate_recommendations
    
    echo -e "\n${GREEN}🏁 Diagnostic validation completed!${NC}"
    echo -e "Check the output above for any issues that need attention."
}

# Run the main function
main
