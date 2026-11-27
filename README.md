# moodoc — Moodle Docker Manager

Tools for creating and managing multiple [Moodle](https://moodle.org/) sites with
[Docker](https://www.docker.com/). This repository contains a cross‑platform desktop
app and a companion shell script for spinning up Moodle instances locally.

## Contents

| Path | Description |
| --- | --- |
| [`moodle-docker-manager/`](./moodle-docker-manager) | A desktop application (Tauri + Vue 3) for creating, starting, stopping, and monitoring Moodle Docker sites through a GUI. |
| [`create_moodle_instances.sh`](./create_moodle_instances.sh) | A Bash script to create multiple Moodle Docker instances from the command line, each with a unique name and web/DB ports. |

## Moodle Docker Manager (desktop app)

A desktop application that simplifies managing Moodle sites on Docker. It lets you
create, configure, and run multiple Moodle instances without editing Docker files by hand.

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

## create_moodle_instances.sh (CLI)

A Bash script that creates one or more Moodle Docker instances, each with a unique name
and distinct web/DB ports. It builds the required Moodle and `moodle-docker` directories
relative to the current working directory.

```bash
./create_moodle_instances.sh -v VERSION [-d DB_TYPE] instance1[:webport[:dbport]] [instance2 ...]
```

- `-v, --version` — Moodle version branch, e.g. `4.1` (required)
- `-d, --dbtype` — database type: `mariadb` (default), `mysql`, `pgsql`/`postgres`
- `instanceN[:webport[:dbport]]` — instance name with optional web and DB ports

Example:

```bash
./create_moodle_instances.sh -v 4.1 -d pgsql site1:8081 site2:8082
```

## License

This project is licensed under the MIT License.
