// Site management functionality
// This module contains functions for creating, starting, stopping, and managing Moodle sites

use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tauri::{command, AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use super::{get_docker_path, safe_emit, Site, SiteState, execute_with_docker_compose_wrapper};

// Helper function to check if site creation has been cancelled
fn is_creation_cancelled(app_handle: &AppHandle, name: &str) -> bool {
    if let Some(running_creations) = app_handle.try_state::<super::RunningCreations>() {
        if let Ok(creations) = running_creations.0.lock() {
            return creations.get(name) != Some(&true);
        }
    }
    false
}

// Helper function to clean up running creation state
fn cleanup_running_creation(running_creations: &super::RunningCreations, name: &str) {
    if let Ok(mut creations) = running_creations.0.lock() {
        creations.remove(name);
    }
}

// Helper function to execute moodle-docker commands with docker-compose wrapper
fn execute_moodle_docker_command(command: &str, site_dir: &std::path::Path) -> Result<std::process::Output, String> {
    execute_with_docker_compose_wrapper(|wrapper_dir| {
        // Modify PATH to include our docker-compose wrapper
        let current_path = std::env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", wrapper_dir.display(), current_path);
        // Debug: show wrapper path and PATH environment
        println!("DEBUG: execute_moodle_docker_command wrapper_dir={}", wrapper_dir.display());
        println!("DEBUG: execute_moodle_docker_command PATH={}", new_path);
        
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
           .arg(command)
           .current_dir(site_dir)
           .env("PATH", new_path);
        
        cmd.output().map_err(|e| format!("Failed to execute command: {}", e))
    })
}

#[command]
pub async fn create_site(
    app_handle: AppHandle,
    name: String,
    php_version: String,
    db_type: String,
    moodle_branch: String,
    db_port: Option<u16>,
    web_port: Option<u16>,
    directory: Option<String>,
) -> Result<String, String> {
    // Run the blocking site creation function in a separate thread
    let handle = app_handle.clone();

    let result = tokio::task::spawn_blocking(move || {
        create_site_blocking(
            handle,
            name,
            php_version,
            db_type,
            moodle_branch,
            db_port,
            web_port,
            directory,
        )
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?;

    result
}

// internal synchronous helper
fn create_site_blocking(
    app_handle: AppHandle,
    name: String,
    php_version: String,
    db_type: String,
    moodle_branch: String,
    db_port: Option<u16>,
    web_port: Option<u16>,
    directory: Option<String>,
) -> Result<String, String> {
    // First validate the environment to ensure everything is set up correctly
    super::validate_environment(app_handle.clone())?;

    // Validate site name
    if name.trim().is_empty() {
        return Err("Site name cannot be empty".to_string());
    }

    // REQUIRE directory parameter - this is the key validation
    let directory = directory.ok_or_else(|| {
        "Installation directory is required. Please select a folder to install Moodle.".to_string()
    })?;

    // Validate directory is not empty
    if directory.trim().is_empty() {
        return Err("Installation directory cannot be empty".to_string());
    }

    // Normalize db_type for docker compose file naming (use 'pgsql' for PostgreSQL)
    let db_env = match db_type.to_lowercase().as_str() {
        "postgresql" | "postgres" => "pgsql",
        _ => &db_type.to_lowercase(),
    };

    // Check if the site already exists
    let state = app_handle.state::<SiteState>();
    let running_creations = app_handle.state::<super::RunningCreations>();
    
    // Check if a creation is already running for this site
    {
        let creations = running_creations
            .0
            .lock()
            .map_err(|_| "Failed to lock running creations state".to_string())?;
        if creations.get(&name) == Some(&true) {
            return Err(format!("Site creation is already running for site '{}'", name));
        }
    }
    
    // Mark this site creation as running
    {
        let mut creations = running_creations
            .0
            .lock()
            .map_err(|_| "Failed to lock running creations state".to_string())?;
        creations.insert(name.clone(), true);
    }
    
    // Check if site already exists
    {
        let sites = state
            .0
            .lock()
            .map_err(|_| "Failed to lock sites state".to_string())?;
        if sites.contains_key(&name) {
            cleanup_running_creation(&running_creations, &name);
            return Err(format!("Site '{}' already exists", name));
        }
    }
    
    // Parse directory path
    let site_dir = PathBuf::from(&directory).join(&name);

    // Execute the main creation logic with proper cleanup on error
    let result = create_site_internal(
        &app_handle, 
        &name, 
        &php_version, 
        &db_type, 
        &db_env, 
        &moodle_branch, 
        db_port, 
        web_port, 
        &directory, 
        site_dir.clone()
    );
    
    // Always cleanup running creation state
    cleanup_running_creation(&running_creations, &name);
    
    result
}

// Internal function that performs the actual site creation
fn create_site_internal(
    app_handle: &AppHandle,
    name: &str,
    php_version: &str,
    db_type: &str,
    db_env: &str,
    moodle_branch: &str,
    db_port: Option<u16>,
    web_port: Option<u16>,
    _directory: &str,
    site_dir: PathBuf,
) -> Result<String, String> {
    // Check for cancellation at the start
    if is_creation_cancelled(app_handle, name) {
        return Err("Site creation was cancelled".to_string());
    }

    safe_emit(
        app_handle,
        "site_creation_progress",
        format!("Creating site directory for {}", name),
    );

    // Create site directory
    if !site_dir.exists() {
        create_dir_all(&site_dir).map_err(|e| format!("Failed to create site directory: {}", e))?;
    }

    // Check for cancellation before cloning
    if is_creation_cancelled(app_handle, name) {
        return Err("Site creation was cancelled".to_string());
    }

    // Clone the moodle-docker repository if it doesn't exist
    let moodle_docker_dir = site_dir.join("moodle-docker");
    if !moodle_docker_dir.exists() {
        safe_emit(
            app_handle,
            "site_creation_progress",
            format!("Cloning moodle-docker repository for {}", name),
        );
        
        // Clone the moodle-docker repository
        let output = Command::new("git")
            .current_dir(&site_dir)
            .args(["clone", "https://github.com/moodlehq/moodle-docker.git"])
            .output()
            .map_err(|e| format!("Failed to clone moodle-docker repository: {}", e))?;

        if !output.status.success() {
            let error =
                String::from_utf8(output.stderr).unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!(
                "Failed to clone moodle-docker repository: {}",
                error
            ));
        }
    }

    // Clone the specific Moodle version
    let moodle_dir = moodle_docker_dir.join("moodle");

    // Make sure the parent directories exist
    if !moodle_docker_dir.exists() {
        println!(
            "Creating moodle-docker directory: {}",
            moodle_docker_dir.display()
        );
        create_dir_all(&moodle_docker_dir)
            .map_err(|e| format!("Failed to create moodle-docker directory: {}", e))?;
    }

    // Check for cancellation before cloning Moodle
    if is_creation_cancelled(app_handle, name) {
        return Err("Site creation was cancelled".to_string());
    }

    safe_emit(
        app_handle,
        "site_creation_progress",
        format!("Cloning Moodle {} for {}", moodle_branch, name),
    );

    // Clone Moodle if the directory doesn't exist
    if !moodle_dir.exists() {
        let git_output = Command::new("git")
            .current_dir(&moodle_docker_dir)
            .args([
                "clone",
                "--depth=1",
                "-b",
                &moodle_branch,
                "https://github.com/moodle/moodle.git",
                "moodle",
            ])
            .output()
            .map_err(|e| format!("Failed to clone Moodle repository: {}", e))?;

        if !git_output.status.success() {
            let error = String::from_utf8(git_output.stderr)
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to clone Moodle repository: {}", error));
        }

        // Verify the clone worked
        if !moodle_dir.exists() {
            return Err(format!("Failed to create Moodle directory at: {}. Git clone completed but directory doesn't exist.", moodle_dir.display()));
        }

        println!(
            "Successfully cloned Moodle repository to: {}",
            moodle_dir.display()
        );
    } else {
        println!(
            "Moodle directory already exists at: {}",
            moodle_dir.display()
        );
    }

    // Double check that the directory exists and has content
    if !moodle_dir.exists()
        || fs::read_dir(&moodle_dir).map_or(true, |mut dir| dir.next().is_none())
    {
        return Err(format!("Moodle directory at {} is missing or empty. Please check file permissions and disk space.", moodle_dir.display()));
    }

    // Ensure the moodle directory path can be resolved before proceeding
    let moodle_www_root = moodle_docker_dir.join("moodle");
    if !moodle_www_root.exists() {
        return Err(format!("Moodle directory at {} does not exist after cloning. This indicates a problem with the git clone or file system permissions.", moodle_www_root.display()));
    }

    // Copy the template config file
    let _template_config = moodle_docker_dir.join("config.docker-template.php");
    let config_php = moodle_dir.join("config.php");

    // Copy config.docker-template.php to moodle/config.php
    let output = Command::new("cp")
        .current_dir(&moodle_docker_dir)
        .args([
            "config.docker-template.php",
            config_php.to_str().unwrap_or("./moodle/config.php"),
        ])
        .output()
        .map_err(|e| format!("Failed to copy config template: {}", e))?;

    if !output.status.success() {
        let error =
            String::from_utf8(output.stderr).unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to copy config template: {}", error));
    }

    // Create a script file to set up the environment and start the containers
    let web_port_str = web_port.map_or("8000".to_string(), |p| p.to_string());
    let db_port_str = db_port.map_or("3306".to_string(), |p| p.to_string());
    let moodle_www_root = moodle_docker_dir.join("moodle");
    
    // Ensure we have an absolute path for MOODLE_DOCKER_WWWROOT
    let abs_moodle_path = if moodle_www_root.exists() {
        moodle_www_root
            .canonicalize()
            .unwrap_or_else(|_| moodle_www_root.clone())
    } else {
        // If the directory doesn't exist yet, construct absolute path manually
        site_dir.join("moodle-docker").join("moodle")
            .canonicalize()
            .unwrap_or_else(|_| {
                // Last resort: use absolute site directory path
                let abs_site_dir = site_dir.canonicalize().unwrap_or(site_dir.clone());
                abs_site_dir.join("moodle-docker").join("moodle")
            })
    };
    
    // Ensure we have a valid path for MOODLE_DOCKER_WWWROOT
    let wwwroot_path = abs_moodle_path.to_str()
        .ok_or_else(|| "Failed to convert moodle path to string".to_string())?;
    
    if wwwroot_path.is_empty() {
        return Err(format!("Invalid MOODLE_DOCKER_WWWROOT path for site {}", name));
    }
    let setup_script = format!(
        r#"#!/bin/bash

# This script contains the environment setup and Moodle installation commands
# for the {0} site

# Export environment variables with absolute paths
export COMPOSE_PROJECT_NAME={0}
export MOODLE_DOCKER_WWWROOT={1}
export MOODLE_DOCKER_DB={2}
export MOODLE_DOCKER_WEB_PORT={3}
export MOODLE_DOCKER_PHP_VERSION={4}
export MOODLE_DOCKER_DB_PORT={5}

# Verify environment
echo "Moodle Docker Environment:"
echo "COMPOSE_PROJECT_NAME=$COMPOSE_PROJECT_NAME"
echo "MOODLE_DOCKER_WWWROOT=$MOODLE_DOCKER_WWWROOT"
echo "MOODLE_DOCKER_DB=$MOODLE_DOCKER_DB"
echo "MOODLE_DOCKER_WEB_PORT=$MOODLE_DOCKER_WEB_PORT"
echo "MOODLE_DOCKER_PHP_VERSION=$MOODLE_DOCKER_PHP_VERSION"
echo "MOODLE_DOCKER_DB_PORT=$MOODLE_DOCKER_DB_PORT"

# Verify that MOODLE_DOCKER_WWWROOT exists before starting containers
if [ ! -d "$MOODLE_DOCKER_WWWROOT" ]; then
    echo "Error: MOODLE_DOCKER_WWWROOT directory does not exist: $MOODLE_DOCKER_WWWROOT"
    exit 1
fi

# Start the containers
./bin/moodle-docker-compose up -d

# Wait for database to be ready
./bin/moodle-docker-wait-for-db

# Install Moodle
./bin/moodle-docker-compose exec webserver php admin/cli/install_database.php --agree-license --fullname="Docker Moodle" --shortname="docker_moodle" --summary="Docker Moodle site" --adminpass="test" --adminemail="admin@example.com"

echo "Moodle is now available at http://localhost:{3}"
echo "Admin username: admin"
echo "Admin password: test"
"#,
        name,
        wwwroot_path,
        db_env,
        web_port_str,
        php_version,
        db_port_str
    );

    let setup_script_path = moodle_docker_dir.join("setup.sh");
    let mut file = File::create(&setup_script_path)
        .map_err(|e| format!("Failed to create setup script: {}", e))?;

    file.write_all(setup_script.as_bytes())
        .map_err(|e| format!("Failed to write setup script: {}", e))?;

    // Make the script executable
    let output = Command::new("chmod")
        .current_dir(&moodle_docker_dir)
        .args(["+x", "setup.sh"])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;

    if !output.status.success() {
        let error =
            String::from_utf8(output.stderr).unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to make script executable: {}", error));
    }

    // Create and execute the docker environment script
    safe_emit(
        &app_handle,
        "site_creation_progress",
        format!("Setting up Docker environment for {}", name),
    );

    // Write a dedicated script that explicitly exports all required variables with correct paths
    let docker_script_path = moodle_docker_dir.join("docker-env.sh");
    
    let docker_script_content = format!(
        r#"#!/bin/bash
# This file is automatically generated - do not edit
export COMPOSE_PROJECT_NAME="{}"
export MOODLE_DOCKER_WWWROOT="{}"
export MOODLE_DOCKER_DB="{}"
export MOODLE_DOCKER_WEB_PORT="{}"
export MOODLE_DOCKER_PHP_VERSION="{}"
export MOODLE_DOCKER_DB_PORT="{}"

# Print environment for debugging
echo "Moodle Docker Environment:"
echo "COMPOSE_PROJECT_NAME=$COMPOSE_PROJECT_NAME"
echo "MOODLE_DOCKER_WWWROOT=$MOODLE_DOCKER_WWWROOT"
echo "MOODLE_DOCKER_DB=$MOODLE_DOCKER_DB"
echo "MOODLE_DOCKER_WEB_PORT=$MOODLE_DOCKER_WEB_PORT"
echo "MOODLE_DOCKER_PHP_VERSION=$MOODLE_DOCKER_PHP_VERSION"
echo "MOODLE_DOCKER_DB_PORT=$MOODLE_DOCKER_DB_PORT"
echo ""

# Verify that MOODLE_DOCKER_WWWROOT exists
if [ ! -d "$MOODLE_DOCKER_WWWROOT" ]; then
    echo "Error: MOODLE_DOCKER_WWWROOT directory does not exist: $MOODLE_DOCKER_WWWROOT"
    exit 1
fi
"#,
        name,
        wwwroot_path,
        db_env,
        web_port.map_or("8000".to_string(), |p| p.to_string()),
        php_version,
        db_port.map_or("3306".to_string(), |p| p.to_string())
    );

    // Write the docker environment script
    std::fs::write(&docker_script_path, docker_script_content)
        .map_err(|e| format!("Failed to write docker environment script: {}", e))?;

    // Make the docker environment script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", docker_script_path.to_str().unwrap_or("docker-env.sh")])
        .output()
        .map_err(|e| format!("Failed to make docker environment script executable: {}", e))?;

    if !chmod_output.status.success() {
        return Err(format!(
            "Failed to make docker environment script executable"
        ));
    }

    // NEW: Create local.yml in the moodle-docker directory using the expected schema
    let db_data_path = if db_type.to_lowercase().contains("postgres") {
        "/var/lib/postgresql/data"
    } else {
        "/var/lib/mysql"
    };
    let local_yml_path = moodle_docker_dir.join("local.yml");
    let local_yml_content = format!(r#"services:
  db:
    volumes:
      - moodledb_data:{}
  mailpit:
    volumes:
      - mailpit_data:/data

volumes:
  moodledb_data:
    driver: local
  mailpit_data:
    driver: local
"#, db_data_path);
    std::fs::write(&local_yml_path, local_yml_content)
        .map_err(|e| format!("Failed to create local.yml: {}", e))?;

    // Check for cancellation before starting Docker containers
    if is_creation_cancelled(app_handle, name) {
        return Err("Site creation was cancelled".to_string());
    }

    // Start Docker containers
    safe_emit(
        app_handle,
        "site_creation_progress",
        format!("Starting Docker containers for {}", name),
    );

    let start_cmd = format!(
        r#"source ./docker-env.sh && ./bin/moodle-docker-compose up -d"#
    );

    let output = execute_moodle_docker_command(&start_cmd, &moodle_docker_dir)?;

    println!(
        "Docker containers start command executed for site: {}. Success: {}",
        name,
        output.status.success()
    );
    if !output.status.success() {
        safe_emit(
            &app_handle,
            "site_creation_progress",
            format!("Failed to start Docker containers for {}", name),
        );
        let error =
            String::from_utf8(output.stderr).unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to start Docker containers: {}", error));
    }

    // Wait for the database to be ready
    println!("Attempting to wait for database for site: {}", name);
    safe_emit(
        &app_handle,
        "site_creation_progress",
        format!("Waiting for database to be ready for {}", name),
    );

    let wait_db_cmd = format!(
        r#"source ./docker-env.sh && ./bin/moodle-docker-wait-for-db"#
    );

    println!("Executing command: {}", wait_db_cmd);
    let output = execute_moodle_docker_command(&wait_db_cmd, &moodle_docker_dir)?;

    let std_output = String::from_utf8_lossy(&output.stdout).to_string();
    let error_output = String::from_utf8_lossy(&output.stderr).to_string();

    println!(
        "Wait for database command executed for site: {}. Success: {}",
        name,
        output.status.success()
    );
    println!("STDOUT: {}", std_output);
    println!("STDERR: {}", error_output);

    if !output.status.success() {
        safe_emit(
            &app_handle,
            "site_creation_progress",
            format!("Failed while waiting for database for {}", name),
        );
        // Try to get container logs to see what's happening
        println!("Getting container logs to diagnose the issue...");
        let logs_cmd = format!(
            r#"source ./docker-env.sh && ./bin/moodle-docker-compose logs db"#
        );

        let logs_output = execute_moodle_docker_command(&logs_cmd, &moodle_docker_dir);

        if let Ok(logs) = logs_output {
            let logs_str = String::from_utf8_lossy(&logs.stdout).to_string();
            println!("Database container logs: {}", logs_str);
        }

        return Err(format!(
            "Failed while waiting for database: {}",
            error_output
        ));
    }

    // Verify Docker containers are running properly before proceeding
    println!("Verifying Docker containers before installing Moodle");
    safe_emit(
        &app_handle,
        "site_creation_progress",
        format!("Verifying Docker containers for {}", name),
    );

    // Check webserver container status
    let verify_cmd = format!(
        r#"source ./docker-env.sh && ./bin/moodle-docker-compose ps webserver | grep -q "Up ""#
    );

    let verify_result = execute_moodle_docker_command(&verify_cmd, &moodle_docker_dir);

    let webserver_running = match verify_result {
        Ok(output) => output.status.success(),
        Err(_) => false,
    };

    if !webserver_running {
        println!("Webserver container is not running, checking logs");
        let logs_cmd = format!(
            r#"source ./docker-env.sh && ./bin/moodle-docker-compose logs webserver"#
        );

        let logs_output = execute_moodle_docker_command(&logs_cmd, &moodle_docker_dir);

        if let Ok(logs) = logs_output {
            let logs_str = String::from_utf8_lossy(&logs.stdout).to_string();
            println!("Webserver container logs: {}", logs_str);
        }

        // Try restarting the container
        safe_emit(
            &app_handle,
            "site_creation_progress",
            format!("Restarting webserver container for {}", name),
        );
        let restart_cmd = format!(
            r#"source ./docker-env.sh && ./bin/moodle-docker-compose restart webserver"#
        );

        let _ = execute_moodle_docker_command(&restart_cmd, &moodle_docker_dir);

        // Give it a moment to start up
        std::thread::sleep(Duration::from_secs(5));
    }

    // Check for cancellation before installing Moodle
    if is_creation_cancelled(app_handle, name) {
        return Err("Site creation was cancelled".to_string());
    }

    // Install Moodle
    println!("Attempting to install Moodle for site: {}", name);
    safe_emit(
        &app_handle,
        "site_creation_progress",
        format!(
            "Installing Moodle for {}. This may take several minutes...",
            name
        ),
    );

    let install_cmd = format!(
        r#"source ./docker-env.sh && ./bin/moodle-docker-compose exec -T webserver php admin/cli/install_database.php --agree-license --fullname='Docker Moodle' --shortname='docker_moodle' --summary='Docker Moodle site' --adminpass='test' --adminemail='admin@example.com'"#
    );

    println!("Executing command: {}", install_cmd);
    let output = execute_moodle_docker_command(&install_cmd, &moodle_docker_dir)?;

    println!(
        "Moodle installation command executed for site: {}. Success: {}",
        name,
        output.status.success()
    );

    let output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| "No output".to_string());

    let error_str = String::from_utf8(output.stderr).unwrap_or_else(|_| "".to_string());

    println!("STDOUT: {}", output_str);
    println!("STDERR: {}", error_str);

    if !output.status.success() {
        safe_emit(
            &app_handle,
            "site_creation_progress",
            format!("Failed to install Moodle for {}", name),
        );
        // Try to get webserver logs to diagnose the issue
        println!("Getting webserver logs to diagnose the issue...");
        let logs_cmd = format!(
            r#"cd {} && export COMPOSE_PROJECT_NAME={} && export MOODLE_DOCKER_DB_PORT={} && ./bin/moodle-docker-compose logs webserver"#,
            moodle_docker_dir.to_str().unwrap_or(""),
            name,
            db_port.map_or("3306".to_string(), |p| p.to_string())
        );

        let logs_output = Command::new("sh").arg("-c").arg(&logs_cmd).output();

        if let Ok(logs) = logs_output {
            let logs_str = String::from_utf8_lossy(&logs.stdout).to_string();
            println!("Webserver container logs: {}", logs_str);
        }

        return Err(format!("Failed to install Moodle: {}", error_str));
    }

    // Save site configuration
    safe_emit(
        &app_handle,
        "site_creation_progress",
        format!("Finalizing setup for {}", name),
    );
    let site = Site {
        name: name.to_string(),
        php_version: php_version.to_string(),
        db_type: db_type.to_string(),
        moodle_branch: moodle_branch.to_string(),
        db_port,
        web_port: Some(web_port_str.parse().unwrap_or(8000)),
        status: "running".to_string(),
        path: moodle_docker_dir,
    };

    super::save_site_config_to_store(app_handle.clone(), site)?;

    Ok(format!("Site '{}' created successfully", name))
}

/// Helper blocking version of list_sites to avoid UI lock during status inspection
pub(crate) fn list_sites_blocking(app_handle: AppHandle) -> Result<Vec<Site>, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites_map = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;
    let mut sites_vec: Vec<Site> = Vec::new();

    // Refresh status by inspecting Docker
    for site in sites_map.values_mut() {
        let filter = format!("name={}", site.name);
        let output = Command::new(&get_docker_path())
            .args(&["ps", "-a", "--filter", &filter, "--format", "{{.Status}}"])
            .output();
        if let Ok(o) = output {
            let status_str = String::from_utf8_lossy(&o.stdout);
            site.status = if status_str.contains("Up") { "running".into() } else { "stopped".into() };
        } else {
            site.status = "error".into();
        }
        sites_vec.push(site.clone());
    }
    Ok(sites_vec)
}

#[command]
pub async fn list_sites(app_handle: AppHandle) -> Result<Vec<Site>, String> {
    let handle = app_handle.clone();
    tokio::task::spawn_blocking(move || list_sites_blocking(handle))
        .await
        .map_err(|e| format!("Task execution error: {}", e))?
}

// Helper blocking version of start_site to avoid UI lock during container start
pub(crate) fn start_site_blocking(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Get the absolute path to the moodle directory
    let moodle_www_root = site.path.join("moodle");
    let abs_moodle_path = moodle_www_root.canonicalize().unwrap_or_else(|_| moodle_www_root.clone());
    
    // Emit starting progress (25%)
    super::safe_emit(
        &app_handle,
        "site_start_progress",
        serde_json::json!({
            "site": name,
            "percent": 25,
            "step": "starting_containers"
        })
    );
    
    // Start containers
    let command = format!(
        r#"export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT={}
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_WEB_PORT={}
export MOODLE_DOCKER_PHP_VERSION={}
export MOODLE_DOCKER_DB_PORT={}

# Echo environment for debugging
echo "Starting containers for site: {}"

# Start containers
./bin/moodle-docker-compose up -d"#,
        name,
        abs_moodle_path.to_str().unwrap_or(""),
        site.db_type.to_lowercase(),
        site.web_port.unwrap_or(8000),
        site.php_version,
        site.db_port.unwrap_or(3306),
        name
    );

    let output = execute_moodle_docker_command(&command, &site.path)
        .map_err(|e| format!("Failed to start site '{}': {}", name, e))?;
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Failed to start site '{}': {}", name, error));
    }
    
    // Emit waiting for database progress (50%)
    super::safe_emit(
        &app_handle,
        "site_start_progress",
        serde_json::json!({
            "site": name,
            "percent": 50,
            "step": "waiting_for_database"
        })
    );
    
    // Wait for database to be ready
    let wait_db_cmd = format!(
        r#"export COMPOSE_PROJECT_NAME={} \
&& export MOODLE_DOCKER_WWWROOT={} \
&& export MOODLE_DOCKER_DB={} \
&& echo "Waiting for database to be ready..." \
&& ./bin/moodle-docker-wait-for-db"#,
        name,
        abs_moodle_path.to_str().unwrap_or(""),
        site.db_type.to_lowercase()
    );
    
    let db_output = execute_moodle_docker_command(&wait_db_cmd, &site.path)
        .map_err(|e| format!("Failed waiting for database: {}", e))?;
    if !db_output.status.success() {
        let error = String::from_utf8_lossy(&db_output.stderr).to_string();
        return Err(format!("Database failed to start properly: {}", error));
    }
    
    // Emit final progress (100%) when site is ready
    super::safe_emit(
        &app_handle,
        "site_start_progress",
        serde_json::json!({
            "site": name,
            "percent": 100,
            "step": "ready"
        })
    );
    
    if let Some(site) = sites.get_mut(&name) {
        site.status = "running".to_string();
    }
    Ok(format!("Site '{}' started successfully", name))
}

// Helper blocking version of stop_site to avoid UI lock during container stop
pub(crate) fn stop_site_blocking(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Get the absolute path to the moodle directory
    let moodle_www_root = site.path.join("moodle");
    let abs_moodle_path = moodle_www_root
        .canonicalize()
        .unwrap_or_else(|_| moodle_www_root.clone());

    // Use our docker-compose wrapper to stop containers directly
    let stop_cmd = format!(
        r#"cd {} && export COMPOSE_PROJECT_NAME={} && export MOODLE_DOCKER_WWWROOT={} && export MOODLE_DOCKER_DB={} && export MOODLE_DOCKER_WEB_PORT={} && export MOODLE_DOCKER_PHP_VERSION={} && export MOODLE_DOCKER_DB_PORT={} && ./bin/moodle-docker-compose down"#,
        site.path.to_str().unwrap_or(""),
        name,
        abs_moodle_path.to_str().unwrap_or(""),
        site.db_type.to_lowercase(),
        site.web_port.unwrap_or(8000),
        site.php_version,
        site.db_port.unwrap_or(3306)
    );

    let output = execute_moodle_docker_command(&stop_cmd, &site.path)
        .map_err(|e| format!("Failed to execute stop command: {}", e))?;

    if output.status.success() {
        if let Some(site) = sites.get_mut(&name) {
            site.status = "stopped".to_string();
        }
        Ok(format!("Site '{}' stopped successfully", name))
    } else {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Failed to stop site '{}': {}", name, error))
    }
}

#[command]
pub async fn start_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let handle = app_handle.clone();
    let name_clone = name.clone();
    tokio::task::spawn_blocking(move || start_site_blocking(handle, name_clone))
        .await
        .map_err(|e| format!("Task execution error: {}", e))?
}

#[command]
pub async fn stop_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let handle = app_handle.clone();
    let name_clone = name.clone();
    let result = tokio::task::spawn_blocking(move || stop_site_blocking(handle, name_clone))
        .await
        .map_err(|e| format!("Task execution error: {}", e))?;
    result
}

#[command]
pub fn check_site_status(app_handle: AppHandle, name: String) -> Result<Site, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    match sites.get(&name) {
        Some(site) => Ok(site.clone()),
        None => Err(format!("Site '{}' not found", name)),
    }
}

#[command]
pub async fn remove_site(
    app_handle: AppHandle,
    name: String,
    remove_containers: Option<bool>,
) -> Result<String, String> {
    let handle = app_handle.clone();
    let name_clone = name.clone();
    let remove_clone = remove_containers.clone();
    tokio::task::spawn_blocking(move || remove_site_blocking(handle, name_clone, remove_clone))
        .await
        .map_err(|e| format!("Task execution error: {}", e))?
}

// synchronous removal logic matching docker_original.rs implementation with progress reporting
fn remove_site_blocking(
    app_handle: AppHandle,
    name: String,
    remove_containers: Option<bool>,
) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Default to true if not specified
    let should_remove_containers = remove_containers.unwrap_or(true);

    // Emit initial progress
    super::safe_emit(
        &app_handle,
        "site_removal_progress",
        serde_json::json!({
            "site": name,
            "percent": 0,
            "step": "validating"
        }),
    );

    // Only remove containers if explicitly requested
    if should_remove_containers {
        super::safe_emit(
            &app_handle,
            "site_removal_progress",
            serde_json::json!({
                "site": name,
                "percent": 25,
                "step": "stopping_containers"
            }),
        );

        // Wrapper directory is stored in site.path (moodle-docker root)
        let moodle_docker_dir = site.path.clone();
        let moodle_www = moodle_docker_dir.join("moodle");
        let abs_moodle_path = moodle_www
            .canonicalize()
            .unwrap_or_else(|_| moodle_www.clone());

        // Step 1: Tear down containers + volumes in one go
        super::safe_emit(
            &app_handle,
            "site_removal_progress",
            serde_json::json!({ "site": name, "percent": 35, "step": "removing_containers" }),
        );
        let down_cmd = format!(
            "export COMPOSE_PROJECT_NAME={} && export MOODLE_DOCKER_WWWROOT={} && export MOODLE_DOCKER_DB={} && echo \"Tearing down containers & volumes...\" && ./bin/moodle-docker-compose down -v --remove-orphans",
            name,
            abs_moodle_path.display(),
            site.db_type.to_lowercase(),
        );
        // Debug: print working directory and full command
        println!("Running down command in {}: {}", moodle_docker_dir.display(), down_cmd);
        match execute_moodle_docker_command(&down_cmd, &moodle_docker_dir) {
            Ok(output) => {
                println!("Down command exit status: {}", output.status);
                println!("Down command stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("Down command stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                println!("Error executing down command: {}", e);
            }
        }

        super::safe_emit(
            &app_handle,
            "site_removal_progress",
            serde_json::json!({
                "site": name,
                "percent": 45,
                "step": "cleaning_volumes"
            }),
        );

        // Step 2: Remove any remaining volumes associated with this site
        let volume_cleanup_cmd = format!(
            r#"docker volume ls --filter name={} --format "{{{{.Name}}}}" | xargs -r docker volume rm 2>/dev/null || true"#,
            name
        );
        
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&volume_cleanup_cmd)
            .output();

        super::safe_emit(
            &app_handle,
            "site_removal_progress",
            serde_json::json!({
                "site": name,
                "percent": 65,
                "step": "cleaning_networks"
            }),
        );

        // Step 3: Remove any remaining networks associated with this site
        let network_cleanup_cmd = format!(
            r#"docker network ls --filter name={} --format "{{{{.ID}}}}" | xargs -r docker network rm 2>/dev/null || true"#,
            name
        );
        
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&network_cleanup_cmd)
            .output();

        super::safe_emit(
            &app_handle,
            "site_removal_progress",
            serde_json::json!({
                "site": name,
                "percent": 75,
                "step": "cleaning_images"
            }),
        );

        // Step 4: Remove any dangling images that might be left from this site
        let image_cleanup_cmd = format!(
            r#"docker images --filter label=com.docker.compose.project={} --format "{{{{.ID}}}}" | xargs -r docker rmi 2>/dev/null || true"#,
            name
        );
        
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&image_cleanup_cmd)
            .output();

        // Step 5: General cleanup of dangling resources
        let _ = Command::new("docker")
            .args(&["system", "prune", "-f", "--filter", &format!("label=com.docker.compose.project={}", name)])
            .output();

        println!("Docker cleanup completed for site: {}", name);
    }

    // Even if the script fails, we'll try to remove the directory and site from state
    super::safe_emit(
        &app_handle,
        "site_removal_progress",
        serde_json::json!({
            "site": name,
            "percent": 85,
            "step": "removing_files"
        }),
    );

    // Try to remove the site directory with robust error handling
    println!(
        "Attempting to remove site directory: {}",
        site.path.display()
    );

    // First attempt: standard fs::remove_dir_all
    match fs::remove_dir_all(&site.path) {
        Ok(_) => {
            println!(
                "Successfully removed site directory: {}",
                site.path.display()
            );
        }
        Err(e) => {
            println!(
                "Failed to remove directory with fs::remove_dir_all: {}. Trying system command...",
                e
            );

            // Fallback: use system command for forced removal
            let remove_cmd = if cfg!(target_os = "windows") {
                format!("rmdir /s /q \"{}\"", site.path.display())
            } else {
                format!("rm -rf \"{}\"", site.path.display())
            };

            match Command::new("sh").arg("-c").arg(&remove_cmd).output() {
                Ok(output) => {
                    if output.status.success() {
                        println!("Successfully removed site directory using system command");
                    } else {
                        let error = String::from_utf8(output.stderr)
                            .unwrap_or_else(|_| "Unknown error".to_string());
                        println!(
                            "Warning: Failed to remove site directory even with system command: {}",
                            error
                        );
                        // We continue anyway to remove from state - partial cleanup is better than none
                    }
                }
                Err(e) => {
                    println!("Warning: Failed to execute system remove command: {}", e);
                    // We continue anyway to remove from state
                }
            }
        }
    }

    // Remove from in-memory state and store
    sites.remove(&name);
    drop(sites); // Release the lock early

    super::safe_emit(
        &app_handle,
        "site_removal_progress",
        serde_json::json!({
            "site": name,
            "percent": 95,
            "step": "updating_config"
        }),
    );

    // Remove from the store
    super::remove_site_from_store(app_handle.clone(), name.clone())
        .map_err(|e| format!("Failed to remove site from store: {}", e))?;

    super::safe_emit(
        &app_handle,
        "site_removal_progress",
        serde_json::json!({
            "site": name,
            "percent": 100,
            "step": "completed"
        }),
    );

    super::safe_emit(
        &app_handle,
        "site_removed",
        format!("Site '{}' removed successfully", name),
    );
    Ok(format!("Site '{}' removed successfully", name))
}

#[command]
pub fn cancel_site_creation(app_handle: AppHandle, name: String) -> Result<String, String> {
    let running_creations = app_handle.state::<super::RunningCreations>();

    // Check if a site creation is running for this site
    {
        let creations = running_creations
            .0
            .lock()
            .map_err(|_| "Failed to lock running creations state".to_string())?;
        if creations.get(&name) != Some(&true) {
            return Err(format!(
                "No site creation is currently running for site '{}'",
                name
            ));
        }
    }

    // Cancel the site creation and perform cleanup
    {
        let mut creations = running_creations
            .0
            .lock()
            .map_err(|_| "Failed to lock running creations state".to_string())?;
        creations.remove(&name);
    }

    // Emit cancellation progress update
    super::safe_emit(
        &app_handle,
        "site_creation_progress",
        serde_json::json!({
            "site": name,
            "percent": 0,
            "step": "cancelling"
        }),
    );

    // Perform cleanup operations
    cleanup_cancelled_site_creation(&app_handle, &name)?;

    super::safe_emit(
        &app_handle,
        "site_creation_cancelled",
        format!("Site creation cancelled for '{}'", name),
    );

    Ok(format!("Site creation cancelled for '{}'", name))
}

// Helper function to clean up after cancelled site creation
fn cleanup_cancelled_site_creation(app_handle: &AppHandle, name: &str) -> Result<(), String> {
    super::safe_emit(
        app_handle,
        "site_creation_progress",
        serde_json::json!({
            "site": name,
            "percent": 10,
            "step": "stopping_containers"
        }),
    );

    // Try to stop and remove any containers that might have been created
    let cleanup_commands = vec![
        format!("docker stop {}_db", name),
        format!("docker stop {}_web", name), 
        format!("docker rm {}_db", name),
        format!("docker rm {}_web", name),
    ];

    for cmd_str in cleanup_commands {
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        if parts.len() >= 2 {
            let output = Command::new(parts[0])
                .args(&parts[1..])
                .output();
            // Ignore errors as containers might not exist
            if let Ok(output) = output {
                if !output.status.success() {
                    println!("Cleanup command failed (expected): {}", cmd_str);
                }
            }
        }
    }

    super::safe_emit(
        app_handle,
        "site_creation_progress",
        serde_json::json!({
            "site": name,
            "percent": 50,
            "step": "removing_networks"
        }),
    );

    // Try to remove the Docker network
    let network_cleanup = Command::new(&get_docker_path())
        .args(&["network", "rm", &format!("{}_network", name)])
        .output();
    // Ignore errors as network might not exist
    if let Ok(output) = network_cleanup {
        if !output.status.success() {
            println!("Network cleanup failed (expected): docker network rm {}_network", name);
        }
    }

    super::safe_emit(
        app_handle,
        "site_creation_progress", 
        serde_json::json!({
            "site": name,
            "percent": 75,
            "step": "removing_volumes"
        }),
    );

    // Try to remove Docker volumes
    let volume_cleanup_commands = vec![
        format!("docker volume rm {}_db_data", name),
        format!("docker volume rm {}_moodle_data", name),
    ];

    for cmd_str in volume_cleanup_commands {
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        if parts.len() >= 3 {
            let output = Command::new(parts[0])
                .args(&parts[1..])
                .output();
            // Ignore errors as volumes might not exist
            if let Ok(output) = output {
                if !output.status.success() {
                    println!("Volume cleanup command failed (expected): {}", cmd_str);
                }
            }
        }
    }

    super::safe_emit(
        app_handle,
        "site_creation_progress",
        serde_json::json!({
            "site": name,
            "percent": 90,
            "step": "cleaning_filesystem"
        }),
    );

    // Remove the site from in-memory state if it exists
    let state = app_handle.state::<super::SiteState>();
    {
        let mut sites = state
            .0
            .lock()
            .map_err(|_| "Failed to lock sites state".to_string())?;
        sites.remove(name);
    }

    // Try to remove the site configuration from store if it exists
    if let Ok(store) = app_handle.store("sites.json") {
        let _ = store.delete(name);
        let _ = store.save();
    }

    super::safe_emit(
        app_handle,
        "site_creation_progress",
        serde_json::json!({
            "site": name,
            "percent": 100,
            "step": "cleanup_completed"
        }),
    );

    Ok(())
}
