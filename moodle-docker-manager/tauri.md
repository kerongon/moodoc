# App Building

## Development Commands

- Compiling and running the app in development mode:  
  `cd /Users/kerongonzales/code/moodoc/moodle-docker-manager && npm run tauri:dev`

- Checking if the code compiles without running:  
  `cd /Users/kerongonzales/code/moodoc/moodle-docker-manager && cargo check --manifest-path=src-tauri/Cargo.toml`

- Building for production:  
  `cd /Users/kerongonzales/code/moodoc/moodle-docker-manager && npm run tauri:build`

## App Features

### Settings Management
- Set default directory for Moodle sites
- Override with custom directory per site
- Settings persist between sessions
- Access via macOS menu or Cmd+,

### Site Management
- Create Moodle sites with custom configurations
- View all sites and their statuses
- Start, stop, and remove sites
- View site logs


cd /Users/kerongonzales/code/moodoc/moodle-docker-manager && npm run