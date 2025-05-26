// Backup management functionality
// This module contains functions for backing up, restoring, and managing Moodle site backups

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tauri::{command, AppHandle, Manager};

use super::{safe_emit, SiteState, execute_with_docker_compose_wrapper};

// Helper function to execute moodle-docker commands with docker-compose wrapper for backup operations
// Currently unused but kept for potential future backup functionality
#[allow(dead_code)]
fn execute_backup_command(command: &str, site_dir: &std::path::Path) -> Result<std::process::Output, String> {
    execute_with_docker_compose_wrapper(|wrapper_dir| {
        // Modify PATH to include our docker-compose wrapper
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
           .arg(command)
           .current_dir(site_dir)
           .env("PATH", format!("{}:{}", wrapper_dir.display(), std::env::var("PATH").unwrap_or_default()));
        
        cmd.output().map_err(|e| format!("Failed to execute backup command: {}", e))
    })
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupInfo {
    pub id: String,
    pub name: String,
    pub site_name: String,
    pub created_at: DateTime<Utc>,
    pub size: u64,
    pub path: PathBuf,
    pub backup_type: String, // "full", "files_only", "database_only"
}

#[command]
pub async fn backup_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Validate site directory and required files
    if !site.path.exists() {
        return Err(format!(
            "Site directory does not exist: {}",
            site.path.display()
        ));
    }

    let docker_compose_script = site.path.join("bin/moodle-docker-compose");
    if !docker_compose_script.exists() {
        return Err(format!(
            "moodle-docker-compose script not found at: {}",
            docker_compose_script.display()
        ));
    }

    let moodle_dir = site.path.join("moodle");
    if !moodle_dir.exists() {
        return Err(format!(
            "Moodle directory not found at: {}",
            moodle_dir.display()
        ));
    }

    safe_emit(
        &app_handle,
        "backup_progress",
        serde_json::json!({
            "site": name,
            "percent": 0,
            "step": "validating"
        }),
    );

    // First, check if the site is currently running
    let status_script = format!(
        r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB_PORT={}

# Check if containers are running
./bin/moodle-docker-compose ps -q webserver
"#,
        site.path.to_str().unwrap_or(""),
        name,
        site.db_port.unwrap_or(3306)
    );

    // Write the status check script to a temporary file
    let status_script_path = site.path.join("check_status.sh");
    let mut file = File::create(&status_script_path)
        .map_err(|e| format!("Failed to create status check script: {}", e))?;

    file.write_all(status_script.as_bytes())
        .map_err(|e| format!("Failed to write status check script: {}", e))?;

    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args([
            "+x",
            status_script_path.to_str().unwrap_or("check_status.sh"),
        ])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;

    if !chmod_output.status.success() {
        let _ = fs::remove_file(&status_script_path);
        return Err(format!("Failed to make status script executable"));
    }

    // Setup docker-compose wrapper environment before executing status script
    let wrapper_dir = super::setup_docker_compose_environment()
        .map_err(|e| {
            let _ = fs::remove_file(&status_script_path);
            format!("Failed to setup docker-compose wrapper: {}", e)
        })?;

    // Execute the status script with the wrapper environment
    let current_path = std::env::var("PATH").unwrap_or_default();
    let output = Command::new("bash")
        .arg(&status_script_path)
        .env("PATH", format!("{}:{}", wrapper_dir.display(), current_path))
        .current_dir(&site.path)
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&status_script_path);
            let _ = fs::remove_dir_all(&wrapper_dir);
            format!("Failed to execute status script: {}", e)
        })?;

    // Clean up wrapper directory and delete the temporary script
    let _ = fs::remove_dir_all(&wrapper_dir);
    let _ = fs::remove_file(status_script_path);

    let is_running = output.status.success() && !output.stdout.is_empty();

    // Create timestamp for backup name
    let now = chrono::Utc::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
    let backup_name = format!("backup_{}_{}", name, timestamp);

    // Create backup directory in the site directory (not app data dir)
    let backups_dir = site.path.join("backups");
    if !backups_dir.exists() {
        fs::create_dir_all(&backups_dir)
            .map_err(|e| format!("Failed to create backups directory: {}", e))?;
    }

    let backup_dir = backups_dir.join(&backup_name);
    fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    // Create the backup script with support for both MySQL/MariaDB and PostgreSQL
    let db_type = site.db_type.to_lowercase();
    let script_path = site.path.to_str().unwrap_or("");
    let backup_path = backup_dir.to_str().unwrap_or("");
    let run_status = if is_running { "running" } else { "stopped" };

    let backup_script = format!(
        r#"#!/bin/bash
set -e
cd {script_path}
export COMPOSE_PROJECT_NAME={name}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB={db_type}
export MOODLE_DOCKER_PHP_VERSION={php_version}
export MOODLE_DOCKER_WEB_PORT={web_port}
export MOODLE_DOCKER_DB_PORT={db_port}

# Create backup directory structure
mkdir -p {backup_path}/database
mkdir -p {backup_path}/moodledata

echo "Backing up database..."
# Export database based on type
if [ "{run_status}" == "running" ]; then
    # If running, use docker-compose to export the database
    if [ "{db_type}" == "pgsql" ] || [ "{db_type}" == "postgres" ] || [ "{db_type}" == "postgresql" ]; then
        echo "Using PostgreSQL backup command..."
        ./bin/moodle-docker-compose exec -T db bash -c "PGPASSWORD=moodle pg_dumpall -U postgres" > {backup_path}/database/moodle_db.sql
    else
        echo "Using MySQL/MariaDB backup command..."
        ./bin/moodle-docker-compose exec -T db bash -c "mysqldump -u root -pm@0dl3ing moodle" > {backup_path}/database/moodle_db.sql
    fi
else
    echo "Site is not running. Starting containers temporarily for backup..."
    ./bin/moodle-docker-compose up -d db
    echo "Waiting for database to be ready..."
    sleep 20  # Increased wait time for the database to start completely
    
    if [ "{db_type}" == "pgsql" ] || [ "{db_type}" == "postgres" ] || [ "{db_type}" == "postgresql" ]; then
        echo "Using PostgreSQL backup command..."
        ./bin/moodle-docker-compose exec -T db bash -c "PGPASSWORD=moodle pg_dumpall -U postgres" > {backup_path}/database/moodle_db.sql
    else
        echo "Using MySQL/MariaDB backup command..."
        ./bin/moodle-docker-compose exec -T db bash -c "mysqldump -u root -pm@0dl3ing moodle" > {backup_path}/database/moodle_db.sql
    fi
    
    # Only stop the containers if they weren't running before
    ./bin/moodle-docker-compose down
fi

echo "Backing up Moodle code..."
# Copy Moodle codebase (excluding git directories to save space)
rsync -a --exclude='.git' $PWD/moodle/ {backup_path}/moodle/

echo "Backing up Moodle data..."
# Copy moodledata directory if it exists
if [ -d "$PWD/moodledata" ]; then
    rsync -a $PWD/moodledata/ {backup_path}/moodledata/
fi

echo "Creating backup manifest..."
# Create a manifest file with backup information
cat << EOF > {backup_path}/manifest.json
{{
    "name": "{name}",
    "timestamp": "{timestamp}",
    "php_version": "{php_version}",
    "db_type": "{db_type_original}",
    "moodle_branch": "{moodle_branch}"
}}
EOF

echo "Backup completed successfully"
"#,
        script_path = script_path,
        name = name,
        db_type = db_type,
        backup_path = backup_path,
        run_status = run_status,
        php_version = site.php_version,
        db_type_original = site.db_type,
        moodle_branch = site.moodle_branch,
        timestamp = timestamp,
        web_port = site.web_port.unwrap_or(8000),
        db_port = site.db_port.unwrap_or(3306)
    );

    // Write the backup script
    let backup_script_path = site.path.join("backup.sh");

    // Setup docker-compose wrapper environment
    let wrapper_dir = super::setup_docker_compose_environment()
        .map_err(|e| format!("Failed to setup docker-compose environment: {}", e))?;

    // Create and execute within a block to ensure cleanup
    let backup_result = (|| -> Result<String, String> {
        // First prepare the script
        let mut file = File::create(&backup_script_path)
            .map_err(|e| format!("Failed to create backup script: {}", e))?;

        file.write_all(backup_script.as_bytes())
            .map_err(|e| format!("Failed to write backup script: {}", e))?;

        // Make the script executable
        let chmod_output = Command::new("chmod")
            .args(["+x", backup_script_path.to_str().unwrap_or("backup.sh")])
            .output()
            .map_err(|e| format!("Failed to make backup script executable: {}", e))?;

        if !chmod_output.status.success() {
            return Err(format!("Failed to make backup script executable"));
        }

        // Execute the backup script with docker-compose wrapper and stream stdout for progress
        let mut child = Command::new("bash")
            .arg(backup_script_path.to_str().unwrap_or("./backup.sh"))
            .current_dir(&site.path)
            .env("PATH", format!("{}:{}", wrapper_dir.display(), std::env::var("PATH").unwrap_or_default()))
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to execute backup script: {}", e))?;

        // read output lines
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(text) = line {
                    let (pct, step) = if text.contains("Backing up database") {
                        (25, "database")
                    } else if text.contains("Backing up Moodle code") {
                        (50, "code")
                    } else if text.contains("Backing up Moodle data") {
                        (75, "data")
                    } else if text.contains("Backup completed successfully") {
                        (100, "done")
                    } else {
                        continue;
                    };
                    // emit progress event
                    safe_emit(
                        &app_handle,
                        "backup_progress",
                        serde_json::json!({
                            "site": name,
                            "percent": pct,
                            "step": step
                        }),
                    );
                }
            }
        }
        let status = child
            .wait()
            .map_err(|e| format!("Error waiting for backup script: {}", e))?;
        if !status.success() {
            return Err(format!(
                "Backup script exited with status {:?}",
                status.code()
            ));
        }

        Ok(format!(
            "Site '{}' backed up successfully to {}",
            name,
            backup_dir.to_str().unwrap_or("")
        ))
    })();

    // Always clean up the temporary script and wrapper directory
    let _ = fs::remove_file(&backup_script_path);
    let _ = fs::remove_dir_all(&wrapper_dir);

    backup_result
}

#[command]
pub fn list_site_backups(
    app_handle: AppHandle,
    name: String,
) -> Result<Vec<HashMap<String, String>>, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Path to backups directory in the site directory (not app data dir)
    let backups_dir = site.path.join("backups");
    if !backups_dir.exists() {
        return Ok(Vec::new()); // No backups yet
    }

    // List all backup directories and read their manifests
    let mut backups = Vec::new();

    if let Ok(entries) = fs::read_dir(&backups_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Read manifest file if it exists
                    let manifest_path = path.join("manifest.json");
                    if manifest_path.exists() {
                        if let Ok(manifest_content) = fs::read_to_string(&manifest_path) {
                            if let Ok(manifest) =
                                serde_json::from_str::<HashMap<String, String>>(&manifest_content)
                            {
                                backups.push(manifest);
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort backups by timestamp (most recent first)
    backups.sort_by(|a, b| {
        let empty_string = "".to_string();
        let a_time = a.get("timestamp").unwrap_or(&empty_string);
        let b_time = b.get("timestamp").unwrap_or(&empty_string);
        b_time.cmp(a_time)
    });

    Ok(backups)
}

#[command]
pub fn restore_site_backup(
    app_handle: AppHandle,
    name: String,
    backup_timestamp: String,
) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Find the backup directory
    let backup_name = format!("backup_{}_{}", name, backup_timestamp);
    let backups_dir = site.path.join("backups");
    let backup_dir = backups_dir.join(&backup_name);

    if !backup_dir.exists() {
        return Err(format!("Backup '{}' not found", backup_name));
    }

    // Get the absolute path to the moodle directory
    let moodle_www_root = site.path.join("moodle");
    let abs_moodle_path = moodle_www_root
        .canonicalize()
        .unwrap_or_else(|_| moodle_www_root.clone());

    // Check if site is running and stop it if needed
    let stop_script = format!(
        r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT={}
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_WEB_PORT={}
export MOODLE_DOCKER_PHP_VERSION={}
export MOODLE_DOCKER_DB_PORT={}

# Echo environment for debugging
echo "COMPOSE_PROJECT_NAME=$COMPOSE_PROJECT_NAME"
echo "MOODLE_DOCKER_WWWROOT=$MOODLE_DOCKER_WWWROOT"
echo "MOODLE_DOCKER_DB=$MOODLE_DOCKER_DB"
echo "MOODLE_DOCKER_WEB_PORT=$MOODLE_DOCKER_WEB_PORT"
echo "MOODLE_DOCKER_DB_PORT=$MOODLE_DOCKER_DB_PORT"

# Stop containers if running
./bin/moodle-docker-compose down
"#,
        site.path.to_str().unwrap_or(""),
        name,
        abs_moodle_path.to_str().unwrap_or(""),
        site.db_type.to_lowercase(),
        site.web_port.unwrap_or(8000),
        site.php_version,
        site.db_port.unwrap_or(3306),
    );

    // Write the stop script
    let stop_script_path = site.path.join("stop_for_restore.sh");
    let mut file = File::create(&stop_script_path)
        .map_err(|e| format!("Failed to create stop script: {}", e))?;

    file.write_all(stop_script.as_bytes())
        .map_err(|e| format!("Failed to write stop script: {}", e))?;

    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args([
            "+x",
            stop_script_path.to_str().unwrap_or("stop_for_restore.sh"),
        ])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;

    if !chmod_output.status.success() {
        let _ = fs::remove_file(&stop_script_path);
        return Err(format!("Failed to make stop script executable"));
    }

    // Check if directory exists
    if !site.path.exists() {
        let _ = fs::remove_file(&stop_script_path);
        return Err(format!("Site directory does not exist: {:?}", site.path));
    }

    // Check if moodle-docker-compose exists
    let compose_script = site.path.join("bin").join("moodle-docker-compose");
    if !compose_script.exists() {
        let _ = fs::remove_file(&stop_script_path);
        return Err(format!(
            "Docker compose script not found at: {:?}",
            compose_script
        ));
    }

    // Setup docker-compose wrapper environment before executing stop script
    let wrapper_dir = super::setup_docker_compose_environment()
        .map_err(|e| {
            let _ = fs::remove_file(&stop_script_path);
            format!("Failed to setup docker-compose wrapper: {}", e)
        })?;

    // Execute the stop script with the wrapper environment
    let current_path = std::env::var("PATH").unwrap_or_default();
    let output = Command::new("bash")
        .arg(&stop_script_path)
        .env("PATH", format!("{}:{}", wrapper_dir.display(), current_path))
        .current_dir(&site.path)
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&stop_script_path);
            let _ = fs::remove_dir_all(&wrapper_dir);
            format!("Failed to execute stop script: {}", e)
        })?;

    // Clean up wrapper directory
    let _ = fs::remove_dir_all(&wrapper_dir);

    // Delete the script
    let _ = fs::remove_file(stop_script_path);

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        println!("Stop script stdout: {}", stdout);
        println!("Stop script stderr: {}", stderr);
        return Err(format!("Failed to stop site for restore: {}", stderr));
    }

    // Create restore script
    let restore_script = format!(
        r#"#!/bin/bash
set -e
cd {site_path}
export COMPOSE_PROJECT_NAME={name}
export MOODLE_DOCKER_WWWROOT={abs_moodle_path}
export MOODLE_DOCKER_DB={db_type}
export MOODLE_DOCKER_PHP_VERSION={php_version}
export MOODLE_DOCKER_WEB_PORT={web_port}
export MOODLE_DOCKER_DB_PORT={db_port}

echo "Restoring Moodle code..."
# Backup current code before overwriting
if [ -d "$PWD/moodle" ]; then
    mv $PWD/moodle $PWD/moodle.bak.$(date +%s)
fi

# Copy Moodle codebase from backup
rsync -a {backup_dir}/moodle/ $PWD/moodle/

echo "Restoring Moodle data..."
# Copy moodledata directory if it exists in the backup
if [ -d "{backup_dir}/moodledata" ]; then
    # Backup current moodledata before overwriting
    if [ -d "$PWD/moodledata" ]; then
        mv $PWD/moodledata $PWD/moodledata.bak.$(date +%s)
    fi
    mkdir -p $PWD/moodledata
    rsync -a {backup_dir}/moodledata/ $PWD/moodledata/
fi

echo "Starting database container..."
# Start DB container only
./bin/moodle-docker-compose up -d db

echo "Waiting for database to be ready..."
sleep 20

echo "Restoring database..."
# Restore database based on type
if [ "{db_type}" == "pgsql" ] || [ "{db_type}" == "postgres" ] || [ "{db_type}" == "postgresql" ]; then
    echo "Using PostgreSQL restore command..."
    cat {backup_dir}/database/moodle_db.sql | ./bin/moodle-docker-compose exec -T db bash -c "PGPASSWORD=moodle psql -U postgres"
else
    echo "Using MySQL/MariaDB restore command..."
    # First check if the SQL file contains non-SQL content at the beginning
    echo "Checking SQL file for valid content..."
    
    # Create a temp file for cleaned SQL
    CLEAN_SQL_FILE="{backup_dir}/database/clean_moodle_db.sql"
    
    # Filter out any non-SQL content from the backup file
    grep -v "^Including" {backup_dir}/database/moodle_db.sql | grep -v "^Warning" > "$CLEAN_SQL_FILE"
    
    # Use the cleaned file with the -f flag to force execution even if minor errors occur
    cat "$CLEAN_SQL_FILE" | ./bin/moodle-docker-compose exec -T db bash -c "mysql -f -u root -pm@0dl3ing moodle"
    
    # Remove temporary file
    rm -f "$CLEAN_SQL_FILE"
fi

echo "Starting remaining containers..."
# Start all containers
./bin/moodle-docker-compose up -d

echo "Site restored successfully"
"#,
        site_path = site.path.to_str().unwrap_or(""),
        name = name,
        abs_moodle_path = abs_moodle_path.to_str().unwrap_or(""),
        db_type = site.db_type.to_lowercase(),
        php_version = site.php_version,
        web_port = site.web_port.unwrap_or(8000),
        db_port = site.db_port.unwrap_or(3306),
        backup_dir = backup_dir.to_str().unwrap_or("")
    );

    // Write the restore script
    let restore_script_path = site.path.join("restore.sh");
    let mut file = File::create(&restore_script_path)
        .map_err(|e| format!("Failed to create restore script: {}", e))?;

    file.write_all(restore_script.as_bytes())
        .map_err(|e| format!("Failed to write restore script: {}", e))?;

    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", restore_script_path.to_str().unwrap_or("restore.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;

    if !chmod_output.status.success() {
        let _ = fs::remove_file(&restore_script_path);
        return Err(format!("Failed to make restore script executable"));
    }

    // Setup docker-compose wrapper environment before executing restore script
    let wrapper_dir = super::setup_docker_compose_environment()
        .map_err(|e| {
            let _ = fs::remove_file(&restore_script_path);
            format!("Failed to setup docker-compose wrapper: {}", e)
        })?;

    // Execute the restore script with the wrapper environment
    let current_path = std::env::var("PATH").unwrap_or_default();
    let output = Command::new("bash")
        .arg(&restore_script_path)
        .env("PATH", format!("{}:{}", wrapper_dir.display(), current_path))
        .current_dir(&site.path)
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&restore_script_path);
            let _ = fs::remove_dir_all(&wrapper_dir);
            format!("Failed to execute restore script: {}", e)
        })?;

    // Clean up wrapper directory and script
    let _ = fs::remove_dir_all(&wrapper_dir);

    // Delete the script
    let _ = fs::remove_file(restore_script_path);

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Failed to restore site: {}", error));
    }

    // Update site status in state
    drop(sites); // Release the lock from earlier
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    if let Some(site) = sites.get_mut(&name) {
        site.status = "running".to_string();
    }

    drop(sites);

    // Return success message
    Ok(format!(
        "Site '{}' restored successfully from backup {}",
        name, backup_timestamp
    ))
}

#[command]
pub async fn restore_site_backup_async(
    app_handle: AppHandle,
    name: String,
    backup_timestamp: String,
) -> Result<String, String> {
    // Create copies of parameters for the spawned thread
    let app = app_handle.clone();
    let name_clone = name.clone();
    let timestamp_clone = backup_timestamp.clone();

    // Run the restore process in a separate blocking thread
    let res = tokio::task::spawn_blocking(move || {
        // Create a helper function to emit progress events
        let emit_progress = |status: &str, message: &str, progress: f32| {
            safe_emit(&app, "restore_progress", serde_json::json!({
                "siteName": name_clone,
                "status": status,
                "message": message,
                "progress": progress,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }));
        };
        
        // Start the restore process with progress reporting
        emit_progress("starting", "Beginning backup restoration...", 0.0);
        
        let state = app.state::<SiteState>();
        let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
        
        // Check if site exists
        let site = match sites.get(&name_clone) {
            Some(site) => site.clone(),
            None => return Err(format!("Site '{}' not found", name_clone)),
        };
        
        // Find the backup directory
        let backup_name = format!("backup_{}_{}", name_clone, timestamp_clone);
        let backups_dir = site.path.join("backups");
        let backup_dir = backups_dir.join(&backup_name);
        
        if !backup_dir.exists() {
            return Err(format!("Backup '{}' not found", backup_name));
        }
        
        emit_progress("preparing", "Preparing for restoration...", 0.1);
        
        // Get the absolute path to the moodle directory
        let moodle_www_root = site.path.join("moodle");
        let abs_moodle_path = moodle_www_root.canonicalize()
            .unwrap_or_else(|_| moodle_www_root.clone());
            
        // Check if site is running and stop it if needed
        let stop_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT={}
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_WEB_PORT={}
export MOODLE_DOCKER_PHP_VERSION={}
export MOODLE_DOCKER_DB_PORT={}

# Echo environment for debugging
echo "COMPOSE_PROJECT_NAME=$COMPOSE_PROJECT_NAME"
echo "MOODLE_DOCKER_WWWROOT=$MOODLE_DOCKER_WWWROOT"
echo "MOODLE_DOCKER_DB=$MOODLE_DOCKER_DB"
echo "MOODLE_DOCKER_WEB_PORT=$MOODLE_DOCKER_WEB_PORT"
echo "MOODLE_DOCKER_DB_PORT=$MOODLE_DOCKER_DB_PORT"

# Stop containers if running
./bin/moodle-docker-compose down
"#, 
            site.path.to_str().unwrap_or(""),
            name_clone,
            abs_moodle_path.to_str().unwrap_or(""),
            site.db_type.to_lowercase(),
            site.web_port.unwrap_or(8000),
            site.php_version,
            site.db_port.unwrap_or(3306),
        );
        
        emit_progress("stopping", "Stopping site containers...", 0.2);
        
        // Write the stop script
        let stop_script_path = site.path.join("stop_for_restore.sh");
        let mut file = File::create(&stop_script_path)
            .map_err(|e| format!("Failed to create stop script: {}", e))?;
        
        file.write_all(stop_script.as_bytes())
            .map_err(|e| format!("Failed to write stop script: {}", e))?;
        
        // Make the script executable
        let chmod_output = Command::new("chmod")
            .args(["+x", stop_script_path.to_str().unwrap_or("stop_for_restore.sh")])
            .output()
            .map_err(|e| format!("Failed to make script executable: {}", e))?;
        
        if !chmod_output.status.success() {
            let _ = fs::remove_file(&stop_script_path);
            emit_progress("error", "Failed to make stop script executable", 0.0);
            return Err(format!("Failed to make stop script executable"));
        }
        
        // Check if directory exists
        if !site.path.exists() {
            let _ = fs::remove_file(&stop_script_path);
            emit_progress("error", &format!("Site directory does not exist: {:?}", site.path), 0.0);
            return Err(format!("Site directory does not exist: {:?}", site.path));
        }
        
        // Check if moodle-docker-compose exists
        let compose_script = site.path.join("bin").join("moodle-docker-compose");
        if !compose_script.exists() {
            let _ = fs::remove_file(&stop_script_path);
            emit_progress("error", &format!("Docker compose script not found at: {:?}", compose_script), 0.0);
            return Err(format!("Docker compose script not found at: {:?}", compose_script));
        }
        
        // Setup docker-compose wrapper environment before executing stop script
        let wrapper_dir = super::setup_docker_compose_environment()
            .map_err(|e| {
                let _ = fs::remove_file(&stop_script_path);
                emit_progress("error", &format!("Failed to setup docker-compose wrapper: {}", e), 0.0);
                format!("Failed to setup docker-compose wrapper: {}", e)
            })?;

        // Execute the stop script with the wrapper environment
        let current_path = std::env::var("PATH").unwrap_or_default();
        let output = Command::new("bash")
            .arg(&stop_script_path)
            .env("PATH", format!("{}:{}", wrapper_dir.display(), current_path))
            .current_dir(&site.path)
            .output()
            .map_err(|e| {
                let _ = fs::remove_file(&stop_script_path);
                let _ = fs::remove_dir_all(&wrapper_dir);
                emit_progress("error", &format!("Failed to execute stop script: {}", e), 0.0);
                format!("Failed to execute stop script: {}", e)
            })?;

        // Clean up wrapper directory
        let _ = fs::remove_dir_all(&wrapper_dir);
        
        // Delete the script
        let _ = fs::remove_file(stop_script_path);
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            emit_progress("error", &format!("Failed to stop site for restore: {}", stderr), 0.0);
            return Err(format!("Failed to stop site for restore: {}", stderr));
        }
        
        emit_progress("restoring_files", "Restoring Moodle code and data files...", 0.3);
        
        // Create restore script
        let restore_script = format!(r#"#!/bin/bash
set -e
cd {site_path}
export COMPOSE_PROJECT_NAME={name}
export MOODLE_DOCKER_WWWROOT={abs_moodle_path}
export MOODLE_DOCKER_DB={db_type}
export MOODLE_DOCKER_PHP_VERSION={php_version}
export MOODLE_DOCKER_WEB_PORT={web_port}
export MOODLE_DOCKER_DB_PORT={db_port}

echo "Restoring Moodle code..."
# Backup current code before overwriting
if [ -d "$PWD/moodle" ]; then
    mv $PWD/moodle $PWD/moodle.bak.$(date +%s)
fi

# Copy Moodle codebase from backup
rsync -a {backup_dir}/moodle/ $PWD/moodle/

echo "Restoring Moodle data..."
# Copy moodledata directory if it exists in the backup
if [ -d "{backup_dir}/moodledata" ]; then
    # Backup current moodledata before overwriting
    if [ -d "$PWD/moodledata" ]; then
        mv $PWD/moodledata $PWD/moodledata.bak.$(date +%s)
    fi
    mkdir -p $PWD/moodledata
    rsync -a {backup_dir}/moodledata/ $PWD/moodledata/
fi

echo "Starting database container..."
# Start DB container only
./bin/moodle-docker-compose up -d db

echo "Waiting for database to be ready..."
sleep 20

echo "Restoring database..."
# Restore database based on type
if [ "{db_type}" == "pgsql" ] || [ "{db_type}" == "postgres" ] || [ "{db_type}" == "postgresql" ]; then
    echo "Using PostgreSQL restore command..."
    cat {backup_dir}/database/moodle_db.sql | ./bin/moodle-docker-compose exec -T db bash -c "PGPASSWORD=moodle psql -U postgres"
else
    echo "Using MySQL/MariaDB restore command..."
    # First check if the SQL file contains non-SQL content at the beginning
    echo "Checking SQL file for valid content..."
    
    # Create a temp file for cleaned SQL
    CLEAN_SQL_FILE="{backup_dir}/database/clean_moodle_db.sql"
    
    # Filter out any non-SQL content from the backup file
    grep -v "^Including" {backup_dir}/database/moodle_db.sql | grep -v "^Warning" > "$CLEAN_SQL_FILE"
    
    # Use the cleaned file with the -f flag to force execution even if minor errors occur
    cat "$CLEAN_SQL_FILE" | ./bin/moodle-docker-compose exec -T db bash -c "mysql -f -u root -pm@0dl3ing moodle"
    
    # Remove temporary file
    rm -f "$CLEAN_SQL_FILE"
fi

echo "Starting remaining containers..."
# Start all containers
./bin/moodle-docker-compose up -d

echo "Site restored successfully"
"#, 
            site_path = site.path.to_str().unwrap_or(""),
            name = name_clone,
            abs_moodle_path = abs_moodle_path.to_str().unwrap_or(""),
            db_type = site.db_type.to_lowercase(),
            php_version = site.php_version,
            web_port = site.web_port.unwrap_or(8000),
            db_port = site.db_port.unwrap_or(3306),
            backup_dir = backup_dir.to_str().unwrap_or("")
        );
        
        // Write the restore script
        let restore_script_path = site.path.join("restore.sh");
        let mut file = File::create(&restore_script_path)
            .map_err(|e| format!("Failed to create restore script: {}", e))?;
        
        file.write_all(restore_script.as_bytes())
            .map_err(|e| format!("Failed to write restore script: {}", e))?;
        
        // Make the script executable
        let chmod_output = Command::new("chmod")
            .args(["+x", restore_script_path.to_str().unwrap_or("restore.sh")])
            .output()
            .map_err(|e| format!("Failed to make script executable: {}", e))?;
        
        if !chmod_output.status.success() {
            let _ = fs::remove_file(&restore_script_path);
            emit_progress("error", "Failed to make restore script executable", 0.0);
            return Err(format!("Failed to make restore script executable"));
        }
        
        emit_progress("restoring_database", "Restoring database... This may take a few minutes", 0.5);
        
        // Setup docker-compose wrapper environment before executing restore script
        let wrapper_dir = super::setup_docker_compose_environment()
            .map_err(|e| {
                let _ = fs::remove_file(&restore_script_path);
                emit_progress("error", &format!("Failed to setup docker-compose wrapper: {}", e), 0.0);
                format!("Failed to setup docker-compose wrapper: {}", e)
            })?;

        // Execute the restore script with the wrapper environment
        let current_path = std::env::var("PATH").unwrap_or_default();
        let output = Command::new("bash")
            .arg(&restore_script_path)
            .env("PATH", format!("{}:{}", wrapper_dir.display(), current_path))
            .current_dir(&site.path)
            .output()
            .map_err(|e| {
                let _ = fs::remove_file(&restore_script_path);
                let _ = fs::remove_dir_all(&wrapper_dir);
                emit_progress("error", &format!("Failed to execute restore script: {}", e), 0.0);
                format!("Failed to execute restore script: {}", e)
            })?;
        
        // Clean up wrapper directory and script
        let _ = fs::remove_dir_all(&wrapper_dir);
        
        // Delete the script
        let _ = fs::remove_file(restore_script_path);
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            emit_progress("error", &format!("Failed to restore site: {}", error), 0.0);
            return Err(format!("Failed to restore site: {}", error));
        }
        
        emit_progress("finalizing", "Finalizing restoration...", 0.9);
        
        // Update site status in state
        drop(sites); // Release the lock from earlier
        let state = app.state::<SiteState>();
        let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
        
        if let Some(site) = sites.get_mut(&name_clone) {
            site.status = "running".to_string();
        }
        
        drop(sites);
        
        emit_progress("completed", "Site restoration completed successfully", 1.0);
        
        // Return success message
        Ok(format!("Site '{}' restored successfully from backup {}", name_clone, timestamp_clone))
    })
    .await
    .map_err(|e| format!("Thread panicked during site restoration: {}", e))?;

    res
}

#[command]
pub fn delete_site_backup(
    app_handle: AppHandle,
    name: String,
    backup_timestamp: String,
) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Find the backup directory
    let backup_name = format!("backup_{}_{}", name, backup_timestamp);
    let backups_dir = site.path.join("backups");
    let backup_dir = backups_dir.join(&backup_name);

    if !backup_dir.exists() {
        return Err(format!("Backup '{}' not found", backup_name));
    }

    // Delete the backup directory
    fs::remove_dir_all(&backup_dir).map_err(|e| format!("Failed to delete backup: {}", e))?;

    Ok(format!("Backup '{}' deleted successfully", backup_name))
}
