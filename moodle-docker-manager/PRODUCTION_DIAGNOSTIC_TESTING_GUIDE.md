# Production Diagnostic Testing Guide

## Overview
This guide provides step-by-step instructions for testing the diagnostic functionality implemented to resolve production build issues where sites show "error" status.

## Diagnostic Features Implemented

### 1. System-Wide Diagnostics
- **Location**: Main site list → "Diagnostics" button (top-right)
- **Purpose**: Comprehensive system analysis
- **Provides**:
  - Docker version and accessibility status
  - Environment variables analysis
  - Directory permissions verification
  - Port availability checking
  - System user and group information

### 2. Site-Specific Diagnostics
- **Location**: Site with "error" status → "Show Error Details" button
- **Purpose**: Detailed error analysis for specific sites
- **Provides**:
  - Container status information
  - Environment variable validation
  - Path resolution verification
  - Network connectivity checks

## Testing Procedures

### Phase 1: Development Environment Testing

1. **Start Development Server**
   ```bash
   cd /Users/kerongonzales/code/moodoc/moodle-docker-manager
   npm run tauri dev
   ```

2. **Test System Diagnostics**
   - Click "Diagnostics" button in the main interface
   - Verify all diagnostic sections load properly:
     - Docker Status
     - System Environment
     - Directory Status
     - Environment Variables
     - Port Availability
     - Error Log

3. **Create Test Site** (if needed)
   - Use directory: `/tmp/moodle-test-sites`
   - Site name: `diagnostic-test`
   - PHP Version: `8.2`
   - Database: `MySQL`
   - Moodle Branch: `MOODLE_500_STABLE`

4. **Monitor Site Creation**
   - Watch for any "error" status during creation
   - If error occurs, test "Show Error Details" functionality

### Phase 2: Production Environment Testing

1. **Launch Production Build**
   ```bash
   open "/Users/kerongonzales/code/moodoc/moodle-docker-manager/src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"
   ```

2. **Compare Diagnostic Results**
   - Run same diagnostic tests as development
   - Compare outputs for differences
   - Note any permission or path issues

3. **Identify Production-Specific Issues**
   - Look for differences in:
     - Docker socket access permissions
     - Environment variables (especially PATH)
     - Directory write permissions
     - Network port accessibility

### Phase 3: Issue Resolution

Based on diagnostic results, common production issues and solutions:

#### Issue 1: Docker Socket Permission Denied
**Symptoms**: 
- Development works, production shows "Docker not accessible"
- Diagnostic shows socket permission errors

**Solution**:
- Verify Docker Desktop is running
- Check user is in `docker` group (if applicable)
- Restart Docker Desktop

#### Issue 2: Path Resolution Issues
**Symptoms**:
- Sites show "error" status in production
- Diagnostic shows path not found errors

**Solution**:
- Use absolute paths for site directories
- Verify directory permissions
- Check symlink resolution

#### Issue 3: Environment Variable Differences
**Symptoms**:
- Different behavior between dev/prod
- Missing Docker compose project names

**Solution**:
- Set consistent environment variables
- Use absolute paths in configurations
- Verify PATH includes Docker binaries

#### Issue 4: Port Conflicts
**Symptoms**:
- Container startup failures
- Port binding errors

**Solution**:
- Use diagnostic port checker
- Select alternative ports
- Stop conflicting services

## Diagnostic Command Reference

### Backend Commands Added

1. **`run_diagnostics`**
   - Comprehensive system diagnostic
   - Returns `DiagnosticInfo` struct
   - Checks Docker, environment, paths, ports

2. **`check_site_status_detailed`**
   - Site-specific error analysis
   - Enhanced error reporting
   - Container status verification

### Frontend Components Added

1. **`DiagnosticModal.vue`**
   - Real-time diagnostic interface
   - Categorized diagnostic results
   - Error aggregation and display

2. **Enhanced `SiteList.vue`**
   - Diagnostic button integration
   - Error status indicators
   - Detailed error display

## Validation Checklist

### Development Environment
- [ ] Diagnostic modal opens and displays information
- [ ] All diagnostic sections populate correctly
- [ ] Site creation works without errors
- [ ] Error details show for failed sites

### Production Environment
- [ ] Production build launches successfully
- [ ] Diagnostic modal functions identically
- [ ] Site operations work the same as development
- [ ] No additional permission or path errors

### Comparison Testing
- [ ] Diagnostic outputs are consistent between environments
- [ ] Any differences are documented and explained
- [ ] Production-specific issues are identified and resolved

## Expected Outcomes

After implementing and testing the diagnostic functionality:

1. **Immediate Benefits**:
   - Clear identification of production vs development differences
   - Detailed error reporting for troubleshooting
   - Real-time system status monitoring

2. **Long-term Benefits**:
   - Faster issue resolution for users
   - Better understanding of Docker environment requirements
   - Foundation for automated issue detection and resolution

3. **Success Criteria**:
   - Production builds work identically to development
   - "Error" status sites are eliminated or properly diagnosed
   - Users can self-diagnose common Docker issues

## Troubleshooting Tips

1. **If diagnostics don't run**:
   - Check Tauri command registration in `main.rs`
   - Verify frontend is calling correct command names
   - Check browser console for JavaScript errors

2. **If Docker issues persist**:
   - Use diagnostic output to identify specific problems
   - Compare environment variables between dev/prod
   - Check Docker Desktop settings and permissions

3. **If sites still show "error" status**:
   - Use enhanced error details feature
   - Check diagnostic logs for path and permission issues
   - Verify all Docker containers are accessible

## Next Steps

After successful testing:

1. **Performance Optimization**:
   - Cache diagnostic results where appropriate
   - Implement background diagnostic monitoring
   - Add diagnostic result export functionality

2. **User Experience Improvements**:
   - Add guided troubleshooting wizard
   - Implement automated fix suggestions
   - Create diagnostic history tracking

3. **Documentation and Training**:
   - Create user-facing diagnostic guide
   - Document common issues and solutions
   - Provide troubleshooting best practices
