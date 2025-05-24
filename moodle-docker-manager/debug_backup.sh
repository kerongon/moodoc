#!/bin/bash

echo "=== Moodle Docker Manager Backup Debug Helper ==="
echo "This script helps debug common backup issues"
echo ""

# Check if we're in a Moodle site directory
if [ ! -f "./bin/moodle-docker-compose" ]; then
    echo "❌ ERROR: Not in a Moodle site directory (no bin/moodle-docker-compose found)"
    echo "Please run this script from your Moodle site root directory"
    exit 1
fi

echo "✅ Found moodle-docker-compose script"

# Check if moodle directory exists
if [ ! -d "moodle" ]; then
    echo "❌ ERROR: moodle directory not found"
    exit 1
fi

echo "✅ Found moodle directory"

# Check if containers are running
echo ""
echo "Checking Docker containers..."
if ./bin/moodle-docker-compose ps | grep -q "Up"; then
    echo "✅ Docker containers are running:"
    ./bin/moodle-docker-compose ps
else
    echo "⚠️  WARNING: No containers appear to be running"
    echo "Container status:"
    ./bin/moodle-docker-compose ps
    echo ""
    echo "You may need to start the containers with:"
    echo "  ./bin/moodle-docker-compose up -d"
fi

# Test database connection
echo ""
echo "Testing database connection..."
if ./bin/moodle-docker-compose exec -T db mysql -u moodle -pexample -e "SELECT 1;" moodle >/dev/null 2>&1; then
    echo "✅ Database connection successful"
else
    echo "❌ ERROR: Cannot connect to database"
    echo "Check if the database container is running and accessible"
fi

# Test webserver container
echo ""
echo "Testing webserver container..."
if docker ps --format "table {{.Names}}\t{{.Status}}" | grep "_webserver_" | grep -q "Up"; then
    echo "✅ Webserver container is running"
    
    # Test moodledata access
    webserver_container=$(docker ps --format "{{.Names}}" | grep "_webserver_" | head -1)
    if docker exec "$webserver_container" ls /var/www/moodledata >/dev/null 2>&1; then
        echo "✅ Moodledata directory accessible in webserver container"
    else
        echo "❌ ERROR: Cannot access moodledata in webserver container"
    fi
else
    echo "❌ ERROR: Webserver container not running"
fi

# Check disk space
echo ""
echo "Checking disk space..."
df -h . | tail -1 | awk '{print "Available space: " $4 " (" $5 " used)"}'

echo ""
echo "=== Debug Summary ==="
echo "If all checks pass, backup should work."
echo "If you see errors above, fix them before attempting backup."
echo ""
echo "Common fixes:"
echo "1. Start containers: ./bin/moodle-docker-compose up -d"
echo "2. Wait for services: ./bin/moodle-docker-wait-for-db"
echo "3. Check container logs: ./bin/moodle-docker-compose logs"
