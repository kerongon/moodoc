# macOS Code Signing Setup Guide

## Overview
This guide will help you set up code signing and notarization for the Moodle Docker Manager app to prevent quarantine issues on other users' Macs.

## Prerequisites

### 1. Apple Developer Account
- You need a paid Apple Developer account ($99/year)
- Sign up at: https://developer.apple.com/programs/

### 2. Developer ID Certificate
1. Go to Apple Developer Portal > Certificates, Identifiers & Profiles
2. Create a new certificate > Developer ID Application
3. Download and install the certificate in Keychain Access
4. Note the certificate name (e.g., "Developer ID Application: Your Name (TEAM_ID)")

### 3. App-Specific Password (for notarization)
1. Go to appleid.apple.com
2. Sign in with your Apple ID
3. Go to "App-Specific Passwords"
4. Generate a new password for "Tauri Notarization"
5. Save this password securely

## Environment Setup

### For Local Development/Testing
Create a `.env` file in the project root (don't commit this file):

```bash
# Code signing certificate (exact name from Keychain)
APPLE_CERTIFICATE="Developer ID Application: Your Name (TEAM_ID)"

# Notarization credentials
APPLE_ID="your-apple-id@email.com"
APPLE_PASSWORD="abcd-efgh-ijkl-mnop"  # App-specific password
APPLE_TEAM_ID="YOUR_TEAM_ID"  # Found in Apple Developer Portal
```

### For CI/CD (GitHub Actions, etc.)
Set these as repository secrets:
- `APPLE_CERTIFICATE`
- `APPLE_ID` 
- `APPLE_PASSWORD`
- `APPLE_TEAM_ID`

## Building Signed Releases

### Method 1: Using the Build Script (Recommended)
```bash
# Set environment variables (or source .env file)
export APPLE_CERTIFICATE="Developer ID Application: Your Name (TEAM_ID)"
export APPLE_ID="your-apple-id@email.com"
export APPLE_PASSWORD="abcd-efgh-ijkl-mnop"
export APPLE_TEAM_ID="YOUR_TEAM_ID"

# Run the build script
./build_signed.sh
```

### Method 2: Manual Build with Environment Variables
```bash
# Set Tauri-specific environment variables
export TAURI_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAM_ID)"
export TAURI_APPLE_ID="your-apple-id@email.com"
export TAURI_APPLE_PASSWORD="abcd-efgh-ijkl-mnop"
export TAURI_APPLE_TEAM_ID="YOUR_TEAM_ID"

# Build the app
cd moodle-docker-manager
npm run tauri build
```

## Verification

### Check Code Signature
```bash
codesign -v --verbose=2 "src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"
```

### Check Notarization Status
```bash
spctl -a -v "src-tauri/target/release/bundle/macos/Moodle Docker Manager.app"
```

Expected output for properly signed and notarized app:
```
source=Notarized Developer ID
origin=Developer ID Application: Your Name (TEAM_ID)
```

## Distribution

### For Testing
1. Build with signing enabled
2. Copy to another Mac's Applications folder
3. Launch normally - should open without warnings

### For Public Distribution
1. Build signed and notarized app
2. Create a DMG or ZIP file
3. Distribute the signed package
4. Users can download and run without quarantine issues

## Troubleshooting

### "App can't be opened because it's from an unidentified developer"
- App is not signed or signature is invalid
- Check certificate installation and environment variables

### "App is damaged and can't be opened"
- Usually indicates unsigned app or signing failure
- Rebuild with proper signing configuration

### Notarization Failures
- Check Apple ID credentials
- Ensure app-specific password is correct
- Check Apple Developer Portal for account status

### Testing Unsigned Builds
If you need to test an unsigned build locally:
```bash
# Remove quarantine attribute
sudo xattr -r -d com.apple.quarantine "path/to/Moodle Docker Manager.app"
```

## Security Notes

- Never commit signing credentials to version control
- Use app-specific passwords, not your main Apple ID password
- Rotate app-specific passwords periodically
- Keep your Developer ID certificate secure

## Automated CI/CD

For automated builds (GitHub Actions example):
```yaml
- name: Build Tauri App
  env:
    TAURI_SIGNING_IDENTITY: ${{ secrets.APPLE_CERTIFICATE }}
    TAURI_APPLE_ID: ${{ secrets.APPLE_ID }}
    TAURI_APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
    TAURI_APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
  run: npm run tauri build
```
I understand you want to notarize your app but don't want to pay for an Apple Developer account right now. Unfortunately, **Apple requires a paid Developer ID certificate for notarization** - there's no way around this requirement.

However, let me give you some alternatives and explain what you can do:

## 🚫 **Why Notarization Requires Paid Account**

Apple's notarization service is only available to:
- Paid Apple Developer Program members ($99/year)
- Apps signed with Developer ID certificates (which require the paid account)

## ✅ **Alternative Solutions Without Developer Account**

### Option 1: Ad-hoc Signing (Temporary Fix)
This won't eliminate quarantine warnings but makes them easier to bypass: 

Made changes.

### Option 2: Update Your Build Process for Better User Experience 

Made changes.

### Option 3: Add Quarantine Removal to Your App 

Made changes.

## 📋 **Summary of Your Options**

### **Immediate Solutions (No Apple Account Required):**

1. **Use build_adhoc.sh** - Basic signing that reduces some warnings
2. **Use build_unsigned.sh** - Creates distribution with clear user instructions  
3. **Include remove_quarantine.sh** - Helper tool for users

### **What Users Will Experience:**

- **Ad-hoc signed**: Easier to bypass warnings with right-click → Open
- **With instructions**: Clear guidance on how to install safely
- **With removal tool**: One-command fix for quarantine issues

### **Future Options:**

- **Apple Developer Account** ($99/year): Complete elimination of warnings
- **Self-hosted signing service**: More complex but possible
- **Electron alternatives**: Some frameworks have different signing requirements

## 🚀 **Recommended Approach**

For now, I'd suggest using build_unsigned.sh because:
- ✅ Creates professional distribution package
- ✅ Includes clear user instructions  
- ✅ Provides multiple workaround methods
- ✅ Maintains user trust with transparency
- ✅ No ongoing costs

**Would you like me to help you test one of these approaches?**