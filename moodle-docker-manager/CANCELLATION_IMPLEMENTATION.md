# Site Creation Cancellation Implementation

## Overview
This document describes the technical implementation of the site creation cancellation feature in the Moodle Docker Manager.

## Architecture

### Backend Implementation (Rust/Tauri)

#### State Management
- **`RunningCreations`**: A thread-safe state that tracks active site creation processes
- **Key**: Site name (String)
- **Value**: Boolean indicating if creation is running
- **Thread Safety**: Uses `Arc<Mutex<HashMap<String, bool>>>` for concurrent access

#### Core Functions

##### `cancel_site_creation` Command
```rust
#[tauri::command]
pub async fn cancel_site_creation(
    name: String,
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, RunningCreations>,
) -> Result<String, String>
```

**Responsibilities**:
- Validates that a creation process is actually running for the given site
- Calls `cleanup_cancelled_site()` to perform actual cleanup
- Removes the site from running creations state
- Returns success/error status

##### `cleanup_cancelled_site` Function
```rust
async fn cleanup_cancelled_site(
    app_handle: &tauri::AppHandle,
    site_name: &str,
) -> Result<(), String>
```

**Cleanup Operations**:
1. **Docker Container Cleanup**:
   - Lists all containers with the site name
   - Stops running containers gracefully
   - Removes stopped containers
   - Handles Docker API errors gracefully

2. **Docker Network Cleanup**:
   - Removes the site-specific Docker network
   - Uses site name pattern matching

3. **Docker Volume Cleanup**:
   - Removes database volume (`{site}_db_data`)
   - Removes Moodle data volume (`{site}_moodle_data`)
   - Removes any additional site-specific volumes

4. **Filesystem Cleanup**:
   - Determines site directory path
   - Recursively removes the entire site directory
   - Handles permission and access errors

5. **Progress Reporting**:
   - Emits progress events for each cleanup step
   - Allows frontend to show detailed progress

##### Enhanced `create_site_blocking` Function
**Cancellation Check Points**:
1. Before repository cloning
2. After repository cloning, before Docker setup
3. Before Moodle installation
4. Periodically during long-running operations

**Implementation Pattern**:
```rust
if is_creation_cancelled(app_handle, name) {
    return Err("Site creation was cancelled".to_string());
}
```

#### Helper Functions

##### `is_creation_cancelled`
- Checks if a site creation has been cancelled
- Thread-safe read from `RunningCreations` state
- Returns boolean indicating cancellation status

##### `mark_creation_finished`
- Removes site from running creations state
- Called on both successful completion and cancellation
- Ensures proper cleanup of tracking state

### Frontend Implementation (TypeScript/Vue)

#### Event Bus Enhancement
**File**: `src/utils/event-bus.ts`

##### `cancelSiteCreation` Method
```typescript
async cancelSiteCreation(siteName: string): Promise<void> {
    await invoke('cancel_site_creation', { name: siteName });
    await emit('site_creation_cancelled', { siteName });
}
```

##### `listenForSiteCreationCancelled` Method
```typescript
async listenForSiteCreationCancelled(callback: (message: string) => void): Promise<() => void> {
    return await listen('site_creation_cancelled', (event) => {
        callback(event.payload as string);
    });
}
```

#### UI Component Updates
**File**: `src/components/SiteCreationProgress.vue`

##### Cancel Button Handler
```typescript
const handleCancel = async () => {
    if (isCreating.value) {
        const confirmed = await confirm('Are you sure you want to cancel site creation?');
        if (confirmed) {
            await eventBus.cancelSiteCreation(props.siteName);
        }
    }
};
```

##### Event Listener Setup
```typescript
const cancelListener = await eventBus.listenForSiteCreationCancelled((msg) => {
    logs.value.push({ time: new Date().toLocaleTimeString(), message: msg });
    status.value = 'Creation cancelled';
    isCreating.value = false;
    progress.value = 0;
    // Close dialog after delay
    setTimeout(() => emit('close'), 2000);
});
```

## Data Flow

### Cancellation Initiation
1. User clicks "Cancel" button in `SiteCreationProgress.vue`
2. Confirmation dialog appears
3. If confirmed, `handleCancel()` calls `eventBus.cancelSiteCreation()`
4. Event bus invokes the `cancel_site_creation` Tauri command
5. Backend validates and starts cleanup process

### Cleanup Process
1. Backend checks if creation is actually running
2. Calls `cleanup_cancelled_site()` for comprehensive cleanup
3. Emits progress events during each cleanup step
4. Frontend receives events and updates UI accordingly
5. Creation state is marked as finished
6. Frontend automatically closes dialog after completion

### Progress Communication
- **Backend to Frontend**: Using Tauri's event system
- **Event Name**: `site_creation_cancelled`
- **Payload**: String messages describing cleanup progress
- **UI Updates**: Real-time log updates and status changes

## Error Handling

### Backend Error Scenarios
1. **Site Not Found**: Returns error if no creation is running for the site
2. **Docker Errors**: Gracefully handles Docker API failures
3. **Filesystem Errors**: Handles permission and access issues
4. **Partial Cleanup**: Continues cleanup even if some steps fail

### Frontend Error Handling
1. **Command Failures**: Displays error messages in logs
2. **Event Listener Failures**: Fallback UI updates
3. **Timeout Handling**: Automatic dialog closure even if events fail

## Thread Safety

### Concurrent Access Protection
- `RunningCreations` state uses `Arc<Mutex<HashMap>>` for thread safety
- All state mutations are protected by mutex locks
- Read operations use temporary clones to avoid long locks

### Race Condition Prevention
- Creation process checks for cancellation at multiple points
- State updates are atomic within mutex scopes
- Cleanup operations are idempotent where possible

## Performance Considerations

### Cleanup Optimization
- Docker operations run in parallel where safe
- Filesystem operations use efficient recursive deletion
- Progress events are throttled to avoid UI spam

### Resource Management
- Proper cleanup of event listeners on component unmount
- Docker resources are cleaned up even if process crashes
- Memory-efficient state management with cleanup

## Testing Strategy

### Unit Tests
- Backend functions can be tested with mock Tauri handles
- Frontend event bus methods can be tested in isolation
- Docker cleanup functions can be tested with test containers

### Integration Tests
- End-to-end cancellation flow testing
- Docker resource verification after cancellation
- UI state consistency testing

### Manual Testing
- See `CANCELLATION_TESTING.md` for comprehensive manual test procedures

## Future Enhancements

### Potential Improvements
1. **Partial Cleanup Recovery**: Resume cleanup if application restarts
2. **Cancellation Timeout**: Force cleanup after maximum wait time
3. **Cleanup Progress Bar**: Visual progress indicator for cleanup steps
4. **Cleanup Verification**: Post-cleanup verification of resource removal
5. **Batch Cancellation**: Cancel multiple site creations simultaneously

### Monitoring and Logging
1. **Cleanup Metrics**: Track cleanup success rates and timing
2. **Error Reporting**: Detailed error reporting for failed cleanups
3. **Resource Monitoring**: Monitor for leaked Docker resources

## Dependencies

### Backend Dependencies
- **Docker API**: For container, network, and volume management
- **Tokio**: For async operations and filesystem manipulation
- **Tauri State Management**: For thread-safe state tracking

### Frontend Dependencies
- **Tauri API**: For command invocation and event listening
- **Vue Composition API**: For reactive state management
- **Dialog API**: For user confirmation dialogs

This implementation provides a robust, user-friendly cancellation system that properly cleans up all resources and maintains application stability during and after cancellation operations.
