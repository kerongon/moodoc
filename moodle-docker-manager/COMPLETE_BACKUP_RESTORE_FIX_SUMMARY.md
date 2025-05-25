# Complete Backup and Restore Fix Summary

## Problem
The backup and restore functionality was failing with "Backup script exited with status Some(127)" error. Status 127 typically indicates "command not found" errors, which was occurring because:

1. The backup and restore scripts directly execute `./bin/moodle-docker-compose` commands
2. On modern Docker installations (like OrbStack), only `docker compose` (V2) is available, not the standalone `docker-compose` command
3. While site creation, start/stop, and updates use the Docker Compose wrapper system (`execute_with_docker_compose_wrapper`), the backup and restore functions executed scripts directly without the wrapper

## Root Cause
The backup and restore functions bypassed the Docker Compose wrapper system that handles modern Docker installations. The wrapper creates a temporary `docker-compose` script that forwards calls to `docker compose` (V2), but backup/restore scripts were executing directly with `Command::new("bash")` or `Command::new(script_path)` without access to this wrapper.

## Solution Applied
Modified all backup and restore script executions to use the Docker Compose wrapper environment:

### 1. Backup Function Fix (✅ COMPLETED)
**File**: `/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/src/docker/backup.rs`
**Function**: `create_site_backup`

**Changes**:
- Added `wrapper_dir = super::setup_docker_compose_environment()` before script execution
- Changed from `Command::new("bash").arg(&script_path)` to using the wrapper with `env("PATH", format!("{}:{}", wrapper_dir.display(), current_path))`
- Added `.current_dir(&site.path)` for proper working directory
- Added cleanup `let _ = fs::remove_dir_all(&wrapper_dir);`

### 2. Restore Function Fixes (✅ COMPLETED)
**File**: `/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/src/docker/backup.rs`

#### 2a. `restore_site_backup` function
**Stop Script Execution**:
- Added Docker Compose wrapper setup before executing stop script
- Modified execution to use wrapper environment with proper PATH
- Added cleanup for wrapper directory

**Restore Script Execution**:
- Added Docker Compose wrapper setup before executing restore script
- Modified execution to use wrapper environment with proper PATH
- Added cleanup for wrapper directory

#### 2b. `restore_site_backup_async` function
**Stop Script Execution**:
- Added Docker Compose wrapper setup with progress reporting
- Modified execution to use wrapper environment
- Added error handling with progress emission
- Added cleanup for wrapper directory

**Restore Script Execution**:
- Added Docker Compose wrapper setup before spawning child process
- Modified execution to use wrapper environment with proper PATH
- Added cleanup for wrapper directory in error paths and success path

## Code Pattern Applied
For each script execution, the fix follows this pattern:

```rust
// Setup docker-compose wrapper environment
let wrapper_dir = super::setup_docker_compose_environment()
    .map_err(|e| {
        cleanup_files();
        format!("Failed to setup docker-compose wrapper: {}", e)
    })?;

// Execute script with wrapper environment
let current_path = std::env::var("PATH").unwrap_or_default();
let output = Command::new("bash")
    .arg(&script_path)
    .env("PATH", format!("{}:{}", wrapper_dir.display(), current_path))
    .current_dir(&site.path)
    .output()
    .map_err(|e| {
        cleanup_files();
        let _ = fs::remove_dir_all(&wrapper_dir);
        format!("Failed to execute script: {}", e)
    })?;

// Clean up wrapper directory
let _ = fs::remove_dir_all(&wrapper_dir);
```

## Technical Details

### Wrapper System
The `setup_docker_compose_environment()` function:
1. Creates a temporary directory
2. Creates a `docker-compose` script that forwards to `docker compose` (V2)
3. Makes the script executable
4. Returns the temporary directory path

### Environment Setup
- Prepends the wrapper directory to the PATH environment variable
- Ensures the temporary `docker-compose` script is found before any system installation
- Sets the working directory to the site path for proper context

### Cleanup
- Always cleans up the temporary wrapper directory after script execution
- Handles cleanup in both success and error paths
- Removes temporary script files

## Verification
- Code compiles successfully with `cargo check`
- Only 2 minor warnings about unused functions (non-critical)
- All backup and restore functions now use the wrapper system consistently

## Benefits
1. **Compatibility**: Works with both legacy `docker-compose` and modern `docker compose` installations
2. **Consistency**: All Docker operations now use the same wrapper system
3. **Reliability**: Eliminates the "command not found" status 127 errors
4. **Maintainability**: Consistent pattern across all backup/restore operations

## Testing Needed
1. Test backup functionality in production environment
2. Test restore functionality in production environment
3. Verify operations work correctly with OrbStack and other modern Docker installations
4. Confirm status 127 errors are resolved

## Files Modified
- `/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/src/docker/backup.rs` (all backup and restore functions)

## Impact
This fix ensures that backup and restore operations will work correctly on modern Docker installations where only `docker compose` (V2) is available, resolving the critical status 127 error that was preventing these essential operations from completing successfully.
