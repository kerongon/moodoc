# Moodle Docker Manager

Moodle Docker Manager is a desktop application designed for macOS that simplifies the management of Moodle sites using Docker. This application allows users to create, configure, and manage multiple Moodle instances with ease.

## Features

- **Site Creation**: Easily create new Moodle sites with customizable configurations including site name, PHP version, database type, and Moodle branch version.
- **Dashboard**: View a comprehensive dashboard that lists all configured sites along with their status indicators and management actions (start, stop, remove, edit).
- **Docker Status**: Check if Docker is installed and running on your system.
- **Site Logs**: Access logs for each site to monitor operations and troubleshoot issues.
- **User Settings**: Configure user preferences, including the folder for site storage.

## Installation

### For End Users (Signed Release)
1. Download the latest signed release from the releases page
2. Ensure Docker is installed and running on your macOS system
3. Open the downloaded `.dmg` file and drag the app to Applications
4. Launch normally - no quarantine warnings should appear

### For Developers

1. Ensure that Docker is installed and running on your macOS system.
2. Clone the repository:
   ```
   git clone https://github.com/yourusername/moodle-docker-manager.git
   ```
3. Navigate to the project directory:
   ```
   cd moodle-docker-manager
   ```
4. Install the dependencies:
   ```
   npm install
   ```
5. Build the application:
   ```
   npm run build
   ```
6. Start the application:
   ```
   npm run tauri dev
   ```

### Building Signed Releases

To build signed releases that prevent macOS quarantine issues:

1. **Set up Apple Developer credentials** (see `MACOS_SIGNING_SETUP.md` for detailed instructions)
2. **Configure environment variables:**
   ```bash
   export APPLE_CERTIFICATE="Developer ID Application: Your Name (TEAM_ID)"
   export APPLE_ID="your-apple-id@email.com"
   export APPLE_PASSWORD="your-app-specific-password"
   export APPLE_TEAM_ID="YOUR_TEAM_ID"
   ```
3. **Build signed release:**
   ```bash
   ./build_signed.sh
   ```

For detailed signing setup instructions, see [`MACOS_SIGNING_SETUP.md`](../MACOS_SIGNING_SETUP.md).

## macOS Security Notes

- **Unsigned builds** will trigger quarantine warnings on other users' Macs
- **Signed builds** require an Apple Developer account but provide seamless user experience
- **For testing unsigned builds locally:** Use `sudo xattr -r -d com.apple.quarantine "path/to/app"`

## Usage

- On the first launch, you will be prompted to select an empty folder where your Moodle sites will be stored.
- Use the "Create Site" feature to configure new Moodle instances.
- Monitor the status of your sites from the dashboard and manage them as needed.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.