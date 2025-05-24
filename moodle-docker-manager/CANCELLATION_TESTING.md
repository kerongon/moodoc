# Site Creation Cancellation Testing Guide

## Overview
This guide outlines how to test the newly implemented site creation cancellation functionality in the Moodle Docker Manager.

## Implementation Summary

### Backend Changes
1. **Added `RunningCreations` state management** to track active site creation processes
2. **Implemented `cancel_site_creation` Tauri command** with comprehensive cleanup logic:
   - Stops and removes Docker containers
   - Cleans up Docker networks and volumes
   - Removes partial filesystem artifacts
   - Emits progress events during cleanup
3. **Enhanced `create_site_blocking` function** with cancellation checks at key points
4. **Added helper functions** for cancellation detection and state cleanup

### Frontend Changes
1. **Enhanced event bus** with `listenForSiteCreationCancelled` method
2. **Updated cancel button logic** to call backend instead of just logging
3. **Added cancellation event listener** that updates UI and closes dialog
4. **Improved error handling** with fallback behavior

## Test Scenarios

### 1. Basic Cancellation Test
**Objective**: Verify that cancellation works during normal site creation

**Steps**:
1. Open the Moodle Docker Manager application
2. Click "Create New Site"
3. Fill in site details (name, version, etc.)
4. Click "Create Site" to start the creation process
5. **Immediately click the "Cancel" button** while creation is in progress
6. Confirm cancellation in the dialog that appears

**Expected Results**:
- Confirmation dialog appears asking to confirm cancellation
- After confirming, logs show "Cancelling site creation..."
- Status changes to "Cancelling..."
- Backend cleanup runs and emits progress messages
- Status eventually changes to "Creation cancelled"
- Dialog closes automatically after 2 seconds
- No partial Docker containers, networks, or volumes are left behind

### 2. Cancellation During Different Phases
**Objective**: Test cancellation at various stages of site creation

**Phase 1 - During Repository Cloning**:
1. Start site creation
2. Cancel during the "Cloning Moodle repository..." phase
3. Verify cleanup occurs properly

**Phase 2 - During Docker Setup**:
1. Start site creation  
2. Cancel during "Setting up Docker containers..." phase
3. Verify Docker containers are properly stopped and removed

**Phase 3 - During Moodle Installation**:
1. Start site creation
2. Cancel during "Installing Moodle..." phase
3. Verify database and filesystem cleanup occurs

### 3. Multiple Cancellation Test
**Objective**: Ensure only one cancellation can occur per site

**Steps**:
1. Start site creation
2. Click "Cancel" button multiple times rapidly
3. Verify only one cancellation process runs

### 4. Cancellation After Creation Completes
**Objective**: Verify cancel button behavior when creation is finished

**Steps**:
1. Let site creation complete successfully
2. Verify cancel button is no longer visible
3. Verify no cancellation is possible after completion

### 5. Docker Cleanup Verification
**Objective**: Ensure Docker resources are properly cleaned up

**Steps**:
1. Before starting: Run `docker ps -a` to see existing containers
2. Start site creation and cancel immediately
3. After cancellation: Run `docker ps -a` again
4. Verify no new containers remain
5. Check networks: `docker network ls`
6. Check volumes: `docker volume ls`
7. Verify no orphaned resources exist

### 6. Filesystem Cleanup Verification
**Objective**: Ensure partial site directories are removed

**Steps**:
1. Note the sites directory location (usually `~/moodle-sites/`)
2. Start site creation with a unique site name
3. Cancel during creation
4. Verify the site directory is removed from filesystem
5. Verify no partial files remain

### 7. Error Handling Test
**Objective**: Test cancellation when backend cleanup fails

**Steps**:
1. Create a scenario where Docker cleanup might fail (e.g., manually lock a container)
2. Start and cancel site creation
3. Verify error messages are displayed appropriately
4. Verify UI still updates correctly even if cleanup partially fails

## Expected Log Messages

During successful cancellation, you should see logs similar to:
```
[timestamp] Cancelling site creation...
[timestamp] Stopping Docker containers for site: [site-name]
[timestamp] Removing Docker containers for site: [site-name]
[timestamp] Cleaning up Docker networks for site: [site-name]
[timestamp] Cleaning up Docker volumes for site: [site-name]
[timestamp] Removing site directory: [path]
[timestamp] Site creation cancelled successfully
```

## Manual Verification Commands

### Check Docker Resources
```bash
# List all containers (should not show the cancelled site)
docker ps -a | grep [site-name]

# List networks (should not show the cancelled site network)
docker network ls | grep [site-name]

# List volumes (should not show the cancelled site volumes)
docker volume ls | grep [site-name]
```

### Check Filesystem
```bash
# Check if site directory was removed
ls ~/moodle-sites/[site-name]  # Should not exist

# Check for any partial files
find ~/moodle-sites/ -name "*[site-name]*"  # Should return nothing
```

## Troubleshooting

### If cancellation doesn't work:
1. Check browser console for JavaScript errors
2. Check application logs for backend errors
3. Verify the `cancel_site_creation` command is registered in main.rs
4. Ensure event names match between frontend and backend

### If Docker cleanup fails:
1. Manually run cleanup commands:
   ```bash
   docker stop $(docker ps -q --filter "name=[site-name]")
   docker rm $(docker ps -aq --filter "name=[site-name]")
   docker network rm [site-name]_default
   docker volume rm [site-name]_db_data [site-name]_moodle_data
   ```

### If filesystem cleanup fails:
1. Manually remove the site directory:
   ```bash
   rm -rf ~/moodle-sites/[site-name]
   ```

## Success Criteria

The cancellation functionality is working correctly if:
1. ✅ Cancel button appears during site creation
2. ✅ Confirmation dialog appears when cancel is clicked
3. ✅ Backend cleanup runs and removes all Docker resources
4. ✅ Partial site directory is removed from filesystem
5. ✅ UI updates appropriately during cancellation
6. ✅ Dialog closes automatically after cancellation completes
7. ✅ No orphaned Docker containers, networks, or volumes remain
8. ✅ Application remains stable after cancellation
9. ✅ Future site creations work normally after a cancellation

## Performance Expectations

- Cancellation should typically complete within 10-30 seconds
- Docker cleanup should be the longest phase (stopping containers)
- Filesystem cleanup should be nearly instantaneous
- UI should remain responsive during cancellation process
