# Moodle Docker Manager

Moodle Docker Manager is a desktop application designed for macOS that simplifies the management of Moodle sites using Docker. This application allows users to create, configure, and manage multiple Moodle instances with ease.

## Features

- **Site Creation**: Easily create new Moodle sites with customizable configurations including site name, PHP version, database type, and Moodle branch version.
- **Dashboard**: View a comprehensive dashboard that lists all configured sites along with their status indicators and management actions (start, stop, remove, edit).
- **Docker Status**: Check if Docker is installed and running on your system.
- **Site Logs**: Access logs for each site to monitor operations and troubleshoot issues.
- **User Settings**: Configure user preferences, including the folder for site storage.

## Installation

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

## Usage

- On the first launch, you will be prompted to select an empty folder where your Moodle sites will be stored.
- Use the "Create Site" feature to configure new Moodle instances.
- Monitor the status of your sites from the dashboard and manage them as needed.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.