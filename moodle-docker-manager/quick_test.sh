#!/bin/bash

echo "🔍 Quick Diagnostic Test"
echo "======================"

# Check current state
echo "📊 Current System State:"
echo "- Docker: $(docker --version 2>/dev/null || echo 'Not accessible')"
echo "- Running processes: $(ps aux | grep -E '(moodle-docker-manager|tauri)' | grep -v grep | wc -l | tr -d ' ') Moodle processes"
echo "- Production app: $(ps aux | grep 'Moodle Docker Manager.app' | grep -v grep | wc -l | tr -d ' ') instances"

echo ""
echo "🎯 Test Instructions:"
echo "====================
1. The production build should be running already
2. Open the application from the Dock or Applications folder
3. Click the 'Diagnostics' button in the main interface
4. Review all diagnostic sections:
   - Docker Status ✓
   - System Environment ✓  
   - Directory Status ✓
   - Environment Variables ✓
   - Port Availability ✓
   - Error Log ✓

5. Compare this with the development version if needed

6. Try creating a test site to verify functionality"

echo ""
echo "📱 Opening production app now..."
open "/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"

sleep 2

echo ""
echo "✅ Production app should now be open!"
echo "   Please test the diagnostic functionality manually."
