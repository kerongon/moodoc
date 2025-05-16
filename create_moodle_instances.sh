#!/usr/bin/env bash
# Script to create multiple Moodle Docker instances with unique names, web and DB ports.
# Usage: ./create_instances.sh instance1[:webport[:dbport]] [instance2[:webport[:dbport]] ...]

set -euo pipefail
IFS=$' \t\n'

# Base and project directories (use absolute paths)
BASE_DIR="$(pwd)"  # Use current working directory, not script directory
MOODLE_DIR="$BASE_DIR/moodle"
DOCKER_DIR="$BASE_DIR/moodle-docker"
ENV_FILE="$DOCKER_DIR/.env"
LOCAL_YML="$DOCKER_DIR/local.yml"
README_FILE="$BASE_DIR/instances_README.md"

# Show paths for debugging
echo "BASE_DIR: $BASE_DIR"
echo "MOODLE_DIR: $MOODLE_DIR"
echo "DOCKER_DIR: $DOCKER_DIR"
echo "LOCAL_YML: $LOCAL_YML"

# Debug current directory structure
echo "---- Directory structure ----"
ls -la "$DOCKER_DIR" 2>/dev/null || echo "$DOCKER_DIR does not exist yet"
echo "---------------------------"

# Parse Moodle version
VERSION=""
while [[ $# -gt 0 ]]; do
  case $1 in
    -v|--version)
      VERSION="$2"
      shift 2
      ;;
    *)
      break
      ;;
  esac
done

# Default DB type
DB_TYPE="mariadb"
# Parse optional DB type flag
while [[ $# -gt 0 ]]; do
  case $1 in
    -d|--dbtype)
      DB_TYPE="$2"
      # Convert postgres variations to pgsql to match the actual filename
      if [[ "$DB_TYPE" == "postgres" || "$DB_TYPE" == "postgresql" ]]; then
        DB_TYPE="pgsql"
      fi
      shift 2
      ;;
    *)
      break
      ;;
  esac
done

if [[ -z "$VERSION" ]]; then
  echo "Usage: $0 -v VERSION [-d DB_TYPE] instance1[:webport[:dbport]]..."
  exit 1
fi

# Format version branch code (e.g. 4.1 -> 401)
if [[ ${#VERSION} -eq 2 ]]; then
  BRANCH_CODE="${VERSION:0:1}0${VERSION:1:1}"
else
  BRANCH_CODE="$VERSION"
fi
# Clone Moodle code if necessary
if [[ ! -d "$MOODLE_DIR" ]]; then
  echo "Cloning Moodle branch MOODLE_${BRANCH_CODE}_STABLE..."
  git clone -b MOODLE_${BRANCH_CODE}_STABLE git://git.moodle.org/moodle.git "$MOODLE_DIR"
fi

# Clone moodle-docker if necessary
if [[ ! -d "$DOCKER_DIR" ]]; then
  echo "Cloning moodle-docker repository into $DOCKER_DIR..."
  mkdir -p "$(dirname "$DOCKER_DIR")"
  git clone https://github.com/moodlehq/moodle-docker.git "$DOCKER_DIR"

  # Verify the clone worked
  if [[ ! -d "$DOCKER_DIR" ]]; then
    echo "ERROR: Failed to clone moodle-docker to $DOCKER_DIR"
    exit 1
  fi

  # Verify bin directory exists
  if [[ ! -d "$DOCKER_DIR/bin" ]]; then
    echo "ERROR: $DOCKER_DIR/bin not found after clone"
    exit 1
  fi

  # Check for config.docker-template.php
  if [[ ! -f "$DOCKER_DIR/config.docker-template.php" ]]; then
    echo "ERROR: $DOCKER_DIR/config.docker-template.php not found"
    exit 1
  fi

  # Quickstart step: seed the default .env
  if [[ -f "$DOCKER_DIR/.env.example" ]]; then
    cp "$DOCKER_DIR/.env.example" "$DOCKER_DIR/.env"
  else
    echo "Warning: $DOCKER_DIR/.env.example not found; skipping default .env setup"
  fi
  : > "$DOCKER_DIR/.env"
fi

# Create persistent volume config
DB_DATA_PATH="/var/lib/mysql"
if [[ "$DB_TYPE" =~ postgresql|postgres|pgsql ]]; then
  DB_DATA_PATH="/var/lib/postgresql/data"
fi
cat > "$LOCAL_YML" <<EOF
services:
  db:
    volumes:
      - moodledb_data:$DB_DATA_PATH
  mailpit:
    volumes:
      - mailpit_data:/data

volumes:
  moodledb_data:
    driver: local
  mailpit_data:
    driver: local
EOF

# Default starting ports
WEB_BASE=${WEB_BASE:-8000}
DB_BASE=${DB_BASE:-6000}

# Readme output
echo "# Moodle Docker Instances" > "$README_FILE"
echo "" >> "$README_FILE"

# Function to check if port is free
function is_port_free() {
  local port=$1
  if lsof -iTCP:"$port" -sTCP:LISTEN -t >/dev/null; then
    return 1
  fi
  return 0
}

# Loop through arguments
for arg in "$@"; do
  # parse name:web:db
  IFS=':' read -r name wp dp <<< "$arg"
  # assign web port
  if [[ -z "$wp" ]]; then
    while ! is_port_free "$WEB_BASE"; do
      WEB_BASE=$((WEB_BASE+1))
    done
    wp=$WEB_BASE
  fi
  # assign db port
  if [[ -z "$dp" ]]; then
    while ! is_port_free "$DB_BASE"; do
      DB_BASE=$((DB_BASE+1))
    done
    dp=$DB_BASE
  fi

  echo "-> Creating instance '$name' on web port $wp and DB port $dp"

  # Set PHP version based on Moodle version
  case "$VERSION" in
    3.9|39) PHP_VERSION=7.4;; # PHP 7.2 to 7.4
    3.10|310) PHP_VERSION=7.4;; # PHP 7.3 to 7.4
    3.11|311) PHP_VERSION=7.4;; # PHP 7.3 to 8.0
    4.0|40) PHP_VERSION=8.0;; # PHP 7.3 to 8.0
    4.1|41) PHP_VERSION=8.1;; # PHP 7.4 to 8.1
    4.2|42) PHP_VERSION=8.2;; # PHP 8.0 to 8.2
    4.3|43) PHP_VERSION=8.2;; # PHP 8.0 to 8.2
    4.4|44) PHP_VERSION=8.3;; # PHP 8.1 to 8.3
    4.5|45) PHP_VERSION=8.3;; # PHP 8.1 to 8.3
    5.0|50) PHP_VERSION=8.2;; # PHP 8.2 to 8.4
    *)
      echo "WARNING: Unknown Moodle version '$VERSION', defaulting to PHP 8.0"
      PHP_VERSION=8.0;;
  esac
  echo "Using PHP version $PHP_VERSION for Moodle $VERSION"

  # Write environment variables to .env instead of exporting
  cat >> "$DOCKER_DIR/.env" <<EOF
MOODLE_DOCKER_DB=$DB_TYPE
COMPOSE_PROJECT_NAME=$name
MOODLE_DOCKER_WEB_PORT=127.0.0.1:$wp
MOODLE_DOCKER_DB_PORT=127.0.0.1:$dp
MOODLE_DOCKER_WWWROOT=$MOODLE_DIR
MOODLE_DOCKER_PHP_VERSION=$PHP_VERSION
MOODLE_DOCKER_REDIS=true
MOODLE_DOCKER_PREFIX=mdl_
EOF

  # Export all vars from .env so wrapper scripts see them
  set -a
  . "$DOCKER_DIR/.env"
  set +a

  # Now start containers and wait for DB
  "$DOCKER_DIR/bin/moodle-docker-compose" up -d
  "$DOCKER_DIR/bin/moodle-docker-wait-for-db"

  cp "$DOCKER_DIR/config.docker-template.php" "$MOODLE_DIR/config.php"

  # Install Moodle database using CLI
  "${DOCKER_DIR}/bin/moodle-docker-compose" exec webserver php admin/cli/install_database.php \
    --agree-license \
    --fullname="$name" \
    --shortname="$(echo "$name" | tr '[:upper:]' '[:lower:]')" \
    --summary="Moodle site for $name" \
    --adminpass="test" \
    --adminemail="admin@example.com"

  # Check if installation was successful
  if [ $? -eq 0 ]; then
    echo "✅ Moodle database installation completed successfully!"
  else
    echo "❌ Error: Moodle database installation failed!"
  fi

  # Append to readme
  cat >> "$README_FILE" <<EOF
## Instance: $name
- Site URL: http://127.0.0.1:$wp/
- Mailpit URL: http://127.0.0.1:$wp/_/mail
- DB Host: 127.0.0.1
- DB Port: $dp
- DB User: moodle
- DB Password: m@0dl3ing
- Obstack URL: https://webserver.${name}.orb.local/
EOF
  echo "" >> "$README_FILE"

  # bump ports for next
  WEB_BASE=$((wp+1))
  DB_BASE=$((dp+1))
done

# No need to chmod the script file here - should be done externally before running

echo "Done. See $README_FILE for details."

