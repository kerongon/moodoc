# moodoc — Moodle Docker Manager

Tools for creating and managing multiple [Moodle](https://moodle.org/) sites with
[Docker](https://www.docker.com/). This repository contains the Moodle Docker Manager
desktop app for spinning up and managing Moodle instances locally.

## Contents

| Path | Description |
| --- | --- |
| [`moodle-docker-manager/`](./moodle-docker-manager) | A desktop application (Tauri + Vue 3) for creating, starting, stopping, and monitoring Moodle Docker sites through a GUI. |

## Moodle Docker Manager (desktop app)

A desktop application that simplifies managing Moodle sites on Docker. It lets you
create, configure, and run multiple Moodle instances without editing Docker files by hand.

### Download

Prebuilt macOS builds are published on the
[**Releases**](https://github.com/kerongon/moodoc/releases) page.

- Latest: [v0.1.0 (macOS)](https://github.com/kerongon/moodoc/releases/tag/v0.1.0) — download the zip, unzip it, and move **Moodle Docker Manager.app** to your Applications folder.

> The build is unsigned. On first launch, right‑click the app and choose **Open**, or
> allow it under **System Settings → Privacy & Security**. Docker must be installed and running.

### Features

- **Site creation** — create new Moodle sites with a configurable site name, PHP version, database type, and Moodle branch version.
- **Dashboard** — list all configured sites with status indicators and actions (start, stop, remove, edit).
- **Docker status** — detect whether Docker is installed and running.
- **Site logs** — view container logs for each site to monitor and troubleshoot.
- **Settings** — configure user preferences, including the folder where sites are stored.

### Tech stack

- **Frontend:** Vue 3, Vue Router, Pinia, TypeScript, Vite
- **Backend / shell:** [Tauri](https://tauri.app/) (Rust)
- **Bundle identifier:** `com.moodoc.moodle-docker-manager`

The Rust backend exposes commands for `check_docker_installed`, `check_docker_running`,
`create_site`, `list_sites`, `start_site`, `stop_site`, `remove_site`, and `get_docker_logs`.

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed and running
- [Node.js](https://nodejs.org/) and npm
- [Rust toolchain](https://www.rust-lang.org/tools/install) (required by Tauri)

### Getting started

```bash
cd moodle-docker-manager
npm install

# Run the app in development mode
npm run tauri:dev

# Build a distributable app bundle
npm run tauri:build
```

On first launch you are prompted to choose an empty folder where your Moodle sites will
be stored. Use **Create Site** to configure a new instance, then manage it from the dashboard.

## License

This project is licensed under the MIT License.
