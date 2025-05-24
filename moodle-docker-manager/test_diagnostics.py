#!/usr/bin/env python3
"""
Test script to validate diagnostic functionality in Moodle Docker Manager
This script tests both development and production builds to identify production-specific issues
"""

import subprocess
import json
import time
import sys
import os
from pathlib import Path

def run_command(cmd, timeout=30):
    """Run a command and return the result"""
    try:
        result = subprocess.run(
            cmd, 
            shell=True, 
            capture_output=True, 
            text=True, 
            timeout=timeout
        )
        return {
            'success': result.returncode == 0,
            'stdout': result.stdout,
            'stderr': result.stderr,
            'returncode': result.returncode
        }
    except subprocess.TimeoutExpired:
        return {
            'success': False,
            'stdout': '',
            'stderr': f'Command timed out after {timeout}s',
            'returncode': -1
        }
    except Exception as e:
        return {
            'success': False,
            'stdout': '',
            'stderr': str(e),
            'returncode': -1
        }

def test_docker_availability():
    """Test if Docker is available and running"""
    print("🐳 Testing Docker availability...")
    
    # Test Docker installation
    docker_version = run_command("docker --version")
    if not docker_version['success']:
        print("❌ Docker is not installed or not in PATH")
        return False
    
    print(f"✅ Docker version: {docker_version['stdout'].strip()}")
    
    # Test Docker daemon
    docker_info = run_command("docker info")
    if not docker_info['success']:
        print("❌ Docker daemon is not running")
        return False
    
    print("✅ Docker daemon is running")
    return True

def test_production_vs_development():
    """Compare behavior between production and development builds"""
    print("\n🔍 Testing production vs development behavior...")
    
    # Test development build (if running)
    dev_test = run_command("ps aux | grep 'moodle-docker-manager' | grep -v grep")
    if dev_test['success'] and dev_test['stdout']:
        print("✅ Development build is running")
    else:
        print("⚠️  Development build is not running")
    
    # Test production build
    prod_app_path = "/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"
    if os.path.exists(prod_app_path):
        print("✅ Production build exists")
        
        # Check if production app is running
        prod_test = run_command("ps aux | grep 'Moodle Docker Manager' | grep -v grep")
        if prod_test['success'] and prod_test['stdout']:
            print("✅ Production build is running")
        else:
            print("⚠️  Production build is not running")
    else:
        print("❌ Production build not found")

def test_site_directories():
    """Test site directory access and permissions"""
    print("\n📁 Testing site directories...")
    
    # Check common Moodle site locations
    potential_sites = [
        "~/Library/Application Support/moodle-docker-manager/sites",
        "~/Documents/MoodleSites",
        "/tmp/moodle-sites"
    ]
    
    for site_path in potential_sites:
        expanded_path = os.path.expanduser(site_path)
        if os.path.exists(expanded_path):
            print(f"✅ Found site directory: {expanded_path}")
            
            # Test permissions
            try:
                test_file = os.path.join(expanded_path, "test_write.tmp")
                with open(test_file, 'w') as f:
                    f.write("test")
                os.remove(test_file)
                print(f"✅ Write permissions OK for {expanded_path}")
            except Exception as e:
                print(f"❌ Write permission issue for {expanded_path}: {e}")
        else:
            print(f"⚠️  Site directory not found: {expanded_path}")

def test_network_ports():
    """Test network port availability"""
    print("\n🌐 Testing network ports...")
    
    # Test common Moodle ports
    test_ports = [8000, 8001, 8080, 3306, 5432]
    
    for port in test_ports:
        result = run_command(f"nc -z localhost {port}")
        if result['success']:
            print(f"⚠️  Port {port} is in use")
        else:
            print(f"✅ Port {port} is available")

def test_environment_variables():
    """Test environment variables that might affect Docker"""
    print("\n🔧 Testing environment variables...")
    
    important_vars = [
        'DOCKER_HOST',
        'DOCKER_CERT_PATH',
        'DOCKER_TLS_VERIFY',
        'COMPOSE_PROJECT_NAME',
        'PATH'
    ]
    
    for var in important_vars:
        value = os.environ.get(var)
        if value:
            # Truncate long values like PATH
            display_value = value[:100] + "..." if len(value) > 100 else value
            print(f"✅ {var}: {display_value}")
        else:
            print(f"⚠️  {var}: Not set")

def test_file_permissions():
    """Test file permissions that might cause issues"""
    print("\n🔐 Testing file permissions...")
    
    # Test Docker socket
    docker_socket = "/var/run/docker.sock"
    if os.path.exists(docker_socket):
        stat_info = os.stat(docker_socket)
        print(f"✅ Docker socket exists: {docker_socket}")
        print(f"   Permissions: {oct(stat_info.st_mode)[-3:]}")
        
        # Test if current user can access
        try:
            with open(docker_socket, 'r'):
                pass
            print("✅ Docker socket is accessible")
        except PermissionError:
            print("❌ Docker socket permission denied")
        except Exception as e:
            print(f"⚠️  Docker socket test failed: {e}")
    else:
        print("⚠️  Docker socket not found (might be using Docker Desktop)")

def main():
    """Run all diagnostic tests"""
    print("🚀 Moodle Docker Manager Diagnostic Test")
    print("=" * 50)
    
    # Run all tests
    tests = [
        test_docker_availability,
        test_production_vs_development,
        test_site_directories,
        test_network_ports,
        test_environment_variables,
        test_file_permissions
    ]
    
    for test in tests:
        try:
            test()
        except Exception as e:
            print(f"❌ Test {test.__name__} failed: {e}")
    
    print("\n" + "=" * 50)
    print("🏁 Diagnostic test completed")
    
    # Recommendations
    print("\n💡 Recommendations:")
    print("1. If production build shows 'error' status but development works:")
    print("   - Check file permissions for site directories")
    print("   - Verify Docker socket access")
    print("   - Check environment variables in production")
    print("2. Use the diagnostic modal in the application for real-time analysis")
    print("3. Compare logs between development and production builds")

if __name__ == "__main__":
    main()
