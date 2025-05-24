// Site management functionality
// This module contains functions for creating, starting, stopping, and managing Moodle sites

use std::fs::{self, File, create_dir_all};
use std::io::{Write, BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use tauri::{command, AppHandle, Manager};

use super::{Site, SiteState, safe_emit};

#[command]
pub async fn create_site(
    app_handle: AppHandle,
    name: String, 
    php_version: String, 
    db_type: String, 
    moodle_branch: String, 
    db_port: Option<u16>, 
    web_port: Option<u16>,
    directory: Option<String>
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
            directory
        )
    }).await.map_err(|e| format!("Task execution error: {}", e))?;
    
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
    directory: Option<String>
) -> Result<String, String> {
    // First validate the environment to ensure everything is set up correctly
    super::validate_environment(app_handle.clone())?;
    
    // Validate site name
    if name.trim().is_empty() {
        return Err("Site name cannot be empty".to_string());
    }
    
    // REQUIRE directory parameter - this is the key validation
    let directory = directory.ok_or_else(|| 
        "Installation directory is required. Please select a folder to install Moodle.".to_string())?;
    
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
    {
        let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
        if sites.contains_key(&name) {
            return Err(format!("Site '{}' already exists", name));
        }
    }
    
    // Parse directory path
    let site_dir = PathBuf::from(&directory).join(&name);
    
    safe_emit(&app_handle, "site_creation_progress", format!("Creating site directory for {}", name));
    
    // Create site directory
    if !site_dir.exists() {
        create_dir_all(&site_dir)
            .map_err(|e| format!("Failed to create site directory: {}", e))?;
    }
    
    // Clone the moodle-docker repository if it doesn't exist
    let moodle_docker_dir = site_dir.join("moodle-docker");
    if !moodle_docker_dir.exists() {
        // Clone the moodle-docker repository
        let output = Command::new("git")
            .current_dir(&site_dir)
            .args(["clone", "https://github.com/moodlehq/moodle-docker.git"])
            .output()
            .map_err(|e| format!("Failed to clone moodle-docker repository: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8(output.stderr)
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to clone moodle-docker repository: {}", error));
        }
    }
    
    // Clone the specific Moodle version
    let moodle_dir = moodle_docker_dir.join("moodle");
    
    // Make sure the parent directories exist
    if !moodle_docker_dir.exists() {
        println!("Creating moodle-docker directory: {}", moodle_docker_dir.display());
        create_dir_all(&moodle_docker_dir)
            .map_err(|e| format!("Failed to create moodle-docker directory: {}", e))?;
    }
    
    safe_emit(&app_handle, "site_creation_progress", format!("Cloning Moodle {} for {}", moodle_branch, name));
    
    // Clone Moodle if the directory doesn't exist
    if !moodle_dir.exists() {
        let git_output = Command::new("git")
            .current_dir(&moodle_docker_dir)
            .args([
                "clone", 
                "-b", &moodle_branch,
                "https://github.com/moodle/moodle.git",
                "moodle"
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
        
        println!("Successfully cloned Moodle repository to: {}", moodle_dir.display());
    } else {
        println!("Moodle directory already exists at: {}", moodle_dir.display());
    }
    
    // Double check that the directory exists and has content
    if !moodle_dir.exists() || fs::read_dir(&moodle_dir).map_or(true, |mut dir| dir.next().is_none()) {
        return Err(format!("Moodle directory at {} is missing or empty. Please check file permissions and disk space.", moodle_dir.display()));
    }
    
    // Copy the template config file
    let _template_config = moodle_docker_dir.join("config.docker-template.php");
    let config_php = moodle_dir.join("config.php");
    
    // Copy config.docker-template.php to moodle/config.php
    let output = Command::new("cp")
        .current_dir(&moodle_docker_dir)
        .args([
            "config.docker-template.php",
            config_php.to_str().unwrap_or("./moodle/config.php")
        ])
        .output()
        .map_err(|e| format!("Failed to copy config template: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to copy config template: {}", error));
    }
    
    // Create a script file to set up the environment and start the containers
    let web_port_str = web_port.map_or("8000".to_string(), |p| p.to_string());
    let db_port_str = db_port.map_or("3306".to_string(), |p| p.to_string());
    let moodle_www_root = moodle_docker_dir.join("moodle");
    let abs_moodle_path = moodle_www_root.canonicalize()
        .unwrap_or_else(|_| moodle_www_root.clone());
    let setup_script = format!(r#"#!/bin/bash

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

# Start the containers
./bin/moodle-docker-compose up -d

# Wait for database to be ready
./bin/moodle-docker-wait-for-db

# Install Moodle
./bin/moodle-docker-compose exec webserver php admin/cli/install_database.php --agree-license --fullname="Docker Moodle" --shortname="docker_moodle" --summary="Docker Moodle site" --adminpass="test" --adminemail="admin@example.com"

echo "Moodle is now available at http://localhost:{3}"
echo "Admin username: admin"
echo "Admin password: test"
"#, name, abs_moodle_path.to_str().unwrap_or(""), db_env, web_port_str, php_version, db_port_str);

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
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to make script executable: {}", error));
    }
    
    // Create and execute the docker environment script
    safe_emit(&app_handle, "site_creation_progress", format!("Setting up Docker environment for {}", name));
    
    // Write a dedicated script that explicitly exports all required variables with correct paths
    let docker_script_path = moodle_docker_dir.join("docker-env.sh");
    let docker_script_content = format!(r#"#!/bin/bash
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
"#,
        name,
        abs_moodle_path.to_str().unwrap_or(""),
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
        return Err(format!("Failed to make docker environment script executable"));
    }

    // Start Docker containers
    safe_emit(&app_handle, "site_creation_progress", format!("Starting Docker containers for {}", name));
    
    let start_cmd = format!(r#"cd {} && source ./docker-env.sh && ./bin/moodle-docker-compose up -d"#,
        moodle_docker_dir.to_str().unwrap_or("")
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&start_cmd)
        .output()
        .map_err(|e| format!("Failed to start Docker containers: {}", e))?;

    println!("Docker containers start command executed for site: {}. Success: {}", name, output.status.success());
    if !output.status.success() {
        safe_emit(&app_handle, "site_creation_progress", format!("Failed to start Docker containers for {}", name));
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to start Docker containers: {}", error));
    }

    // Wait for the database to be ready
    println!("Attempting to wait for database for site: {}", name);
    safe_emit(&app_handle, "site_creation_progress", format!("Waiting for database to be ready for {}", name));
    
    let wait_db_cmd = format!(r#"cd {} && source ./docker-env.sh && ./bin/moodle-docker-wait-for-db"#,
        moodle_docker_dir.to_str().unwrap_or("")
    );

    println!("Executing command: {}", wait_db_cmd);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&wait_db_cmd)
        .output()
        .map_err(|e| format!("Failed while waiting for database: {}", e))?;

    let std_output = String::from_utf8_lossy(&output.stdout).to_string();
    let error_output = String::from_utf8_lossy(&output.stderr).to_string();

    println!("Wait for database command executed for site: {}. Success: {}", name, output.status.success());
    println!("STDOUT: {}", std_output);
    println!("STDERR: {}", error_output);

    if !output.status.success() {
        safe_emit(&app_handle, "site_creation_progress", format!("Failed while waiting for database for {}", name));
        // Try to get container logs to see what's happening
        println!("Getting container logs to diagnose the issue...");
        let logs_cmd = format!(r#"cd {} && source ./docker-env.sh && ./bin/moodle-docker-compose logs db"#,
            moodle_docker_dir.to_str().unwrap_or("")
        );
        
        let logs_output = Command::new("sh")
            .arg("-c")
            .arg(&logs_cmd)
            .output();
            
        if let Ok(logs) = logs_output {
            let logs_str = String::from_utf8_lossy(&logs.stdout).to_string();
            println!("Database container logs: {}", logs_str);
        }
        
        return Err(format!("Failed while waiting for database: {}", error_output));
    }

    // Verify Docker containers are running properly before proceeding
    println!("Verifying Docker containers before installing Moodle");
    safe_emit(&app_handle, "site_creation_progress", format!("Verifying Docker containers for {}", name));
    
    // Check webserver container status
    let verify_cmd = format!(r#"cd {} && source ./docker-env.sh && ./bin/moodle-docker-compose ps webserver | grep -q "Up ""#,
        moodle_docker_dir.to_str().unwrap_or("")
    );
    
    let verify_result = Command::new("sh")
        .arg("-c")
        .arg(&verify_cmd)
        .output();
    
    let webserver_running = match verify_result {
        Ok(output) => output.status.success(),
        Err(_) => false
    };
    
    if !webserver_running {
        println!("Webserver container is not running, checking logs");
        let logs_cmd = format!(r#"cd {} && source ./docker-env.sh && ./bin/moodle-docker-compose logs webserver"#,
            moodle_docker_dir.to_str().unwrap_or("")
        );
        
        let logs_output = Command::new("sh")
            .arg("-c")
            .arg(&logs_cmd)
            .output();
            
        if let Ok(logs) = logs_output {
            let logs_str = String::from_utf8_lossy(&logs.stdout).to_string();
            println!("Webserver container logs: {}", logs_str);
        }
        
        // Try restarting the container
        safe_emit(&app_handle, "site_creation_progress", format!("Restarting webserver container for {}", name));
        let restart_cmd = format!(r#"cd {} && source ./docker-env.sh && ./bin/moodle-docker-compose restart webserver"#,
            moodle_docker_dir.to_str().unwrap_or("")
        );
        
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&restart_cmd)
            .output();
        
        // Give it a moment to start up
        std::thread::sleep(Duration::from_secs(5));
    }

    // Install Moodle
    println!("Attempting to install Moodle for site: {}", name);
    safe_emit(&app_handle, "site_creation_progress", format!("Installing Moodle for {}. This may take several minutes...", name));
    
    let install_cmd = format!(r#"cd {} && source ./docker-env.sh && ./bin/moodle-docker-compose exec -T webserver php admin/cli/install_database.php --agree-license --fullname='Docker Moodle' --shortname='docker_moodle' --summary='Docker Moodle site' --adminpass='test' --adminemail='admin@example.com'"#,
        moodle_docker_dir.to_str().unwrap_or("")
    );
    
    println!("Executing command: {}", install_cmd);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&install_cmd)
        .output()
        .map_err(|e| format!("Failed to install Moodle: {}", e))?;
    
    println!("Moodle installation command executed for site: {}. Success: {}", name, output.status.success());
    
    let output_str = String::from_utf8(output.stdout)
        .unwrap_or_else(|_| "No output".to_string());
    
    let error_str = String::from_utf8(output.stderr)
        .unwrap_or_else(|_| "".to_string());
    
    println!("STDOUT: {}", output_str);
    println!("STDERR: {}", error_str);
    
    if !output.status.success() {
        safe_emit(&app_handle, "site_creation_progress", format!("Failed to install Moodle for {}", name));
        // Try to get webserver logs to diagnose the issue
        println!("Getting webserver logs to diagnose the issue...");
        let logs_cmd = format!(r#"cd {} && export COMPOSE_PROJECT_NAME={} && export MOODLE_DOCKER_DB_PORT={} && ./bin/moodle-docker-compose logs webserver"#,
            moodle_docker_dir.to_str().unwrap_or(""),
            name,
            db_port.map_or("3306".to_string(), |p| p.to_string())
        );
        
        let logs_output = Command::new("sh")
            .arg("-c")
            .arg(&logs_cmd)
            .output();
            
        if let Ok(logs) = logs_output {
            let logs_str = String::from_utf8_lossy(&logs.stdout).to_string();
            println!("Webserver container logs: {}", logs_str);
        }
        
        return Err(format!("Failed to install Moodle: {}", error_str));
    }

    // Save site configuration
    safe_emit(&app_handle, "site_creation_progress", format!("Finalizing setup for {}", name));
    let site = Site {
        name: name.clone(),
        php_version,
        db_type,
        moodle_branch,
        db_port,
        web_port: Some(web_port_str.parse().unwrap_or(8000)),
        status: "running".to_string(),
        path: moodle_docker_dir,
    };
    
    super::save_site_config(app_handle, site)?;
    
    Ok(format!("Site '{}' created successfully", name))
}

#[command]
pub fn list_sites(app_handle: AppHandle) -> Result<Vec<Site>, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites_map = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    let mut sites_vec: Vec<Site> = Vec::new();
    
    // Refresh status by inspecting Docker
    for site in sites_map.values_mut() {
        // Get container status for this site
        let filter = format!("name={}", site.name);
        let output = Command::new("docker")
            .args(&["ps", "-a", "--filter", &filter, "--format", "{{.Status}}"])
            .output();
        if let Ok(o) = output {
            let status_str = String::from_utf8_lossy(&o.stdout);
            if status_str.contains("Up") {
                site.status = "running".into();
            } else {
                site.status = "stopped".into();
            }
        } else {
            site.status = "error".into();
        }
        sites_vec.push(site.clone());
    }
    Ok(sites_vec)
}

#[command]
pub fn start_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Get the absolute path to the moodle directory
    let moodle_www_root = site.path.join("moodle");
    let abs_moodle_path = moodle_www_root.canonicalize()
        .unwrap_or_else(|_| moodle_www_root.clone());
        
    // Set environment variables and start containers
    let script_content = format!(r#"#!/bin/bash
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

# Start containers
./bin/moodle-docker-compose up -d
"#, 
    site.path.to_str().unwrap_or(""),
    name,
    abs_moodle_path.to_str().unwrap_or(""),
    site.db_type.to_lowercase(),
    site.web_port.unwrap_or(8000),
    site.php_version,
    site.db_port.unwrap_or(3306)
    );
    
    let script_path = site.path.join("start.sh");
    let mut file = File::create(&script_path)
        .map_err(|e| format!("Failed to create start script: {}", e))?;
    
    file.write_all(script_content.as_bytes())
        .map_err(|e| format!("Failed to write start script: {}", e))?;
    
    // Make script executable and run it
    let chmod_output = Command::new("chmod")
        .args(["+x", script_path.to_str().unwrap_or("start.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        let _ = fs::remove_file(&script_path);
        return Err(format!("Failed to make script executable"));
    }
    
    // Execute the script
    let output = Command::new(script_path.to_str().unwrap_or("./start.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&script_path);
            format!("Failed to execute start script: {}", e)
        })?;
    
    // Delete the script
    let _ = fs::remove_file(script_path);
    
    if output.status.success() {
        // Update site status
        if let Some(site) = sites.get_mut(&name) {
            site.status = "running".to_string();
        }
        Ok(format!("Site '{}' started successfully", name))
    } else {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Failed to start site '{}': {}", name, error))
    }
}

#[command]
pub fn stop_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Get the absolute path to the moodle directory
    let moodle_www_root = site.path.join("moodle");
    let abs_moodle_path = moodle_www_root.canonicalize()
        .unwrap_or_else(|_| moodle_www_root.clone());
        
    // Set environment variables and stop containers
    let script_content = format!(r#"#!/bin/bash
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

# Stop containers
./bin/moodle-docker-compose down
"#, 
    site.path.to_str().unwrap_or(""),
    name,
    abs_moodle_path.to_str().unwrap_or(""),
    site.db_type.to_lowercase(),
    site.web_port.unwrap_or(8000),
    site.php_version,
    site.db_port.unwrap_or(3306)
    );
    
    let script_path = site.path.join("stop.sh");
    let mut file = File::create(&script_path)
        .map_err(|e| format!("Failed to create stop script: {}", e))?;
    
    file.write_all(script_content.as_bytes())
        .map_err(|e| format!("Failed to write stop script: {}", e))?;
    
    // Make script executable and run it
    let chmod_output = Command::new("chmod")
        .args(["+x", script_path.to_str().unwrap_or("stop.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        let _ = fs::remove_file(&script_path);
        return Err(format!("Failed to make script executable"));
    }
    
    // Execute the script
    let output = Command::new(script_path.to_str().unwrap_or("./stop.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&script_path);
            format!("Failed to execute stop script: {}", e)
        })?;
    
    // Delete the script
    let _ = fs::remove_file(script_path);
    
    if output.status.success() {
        // Update site status
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
pub fn check_site_status(app_handle: AppHandle, name: String) -> Result<Site, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
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
    remove_site_blocking(app_handle, name, remove_containers)
}

// synchronous removal logic matching docker_original.rs implementation with progress reporting
fn remove_site_blocking(
    app_handle: AppHandle,
    name: String,
    remove_containers: Option<bool>,
) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Default to true if not specified
    let should_remove_containers = remove_containers.unwrap_or(true);
    
    // Emit initial progress
    super::safe_emit(&app_handle, "site_removal_progress", serde_json::json!({
        "site": name,
        "percent": 0,
        "step": "validating"
    }));
    
    // Create a cleanup script to ensure proper environment variables are set
    let cleanup_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle

echo "Stopping containers..."
# Stop and remove containers with volumes
./bin/moodle-docker-compose down -v

echo "Removing Docker networks..."
# Clean up any remaining networks
docker network ls --filter name={} --format "{{{{.ID}}}}" | xargs -r docker network rm || true

echo "Cleanup completed for {}"
"#, 
        site.path.to_str().unwrap_or(""),
        name,
        name,
        name
    );
    
    // Only remove containers if explicitly requested
    if should_remove_containers {
        super::safe_emit(&app_handle, "site_removal_progress", serde_json::json!({
            "site": name,
            "percent": 25,
            "step": "stopping_containers"
        }));
        
        // Use a cleanup script with guaranteed cleanup and progress reporting
        let script_path = site.path.join("cleanup.sh");
        
        let cleanup_result = (|| -> Result<(), String> {
            let mut file = File::create(&script_path)
                .map_err(|e| format!("Failed to create cleanup script: {}", e))?;
            
            file.write_all(cleanup_script.as_bytes())
                .map_err(|e| format!("Failed to write cleanup script: {}", e))?;
            
            // Make the script executable
            let chmod_output = Command::new("chmod")
                .args(["+x", script_path.to_str().unwrap_or("cleanup.sh")])
                .output()
                .map_err(|e| format!("Failed to make cleanup script executable: {}", e))?;
            
            if !chmod_output.status.success() {
                return Err("Failed to make cleanup script executable".to_string());
            }
            
            // Execute the cleanup script with progress tracking
            let mut child = Command::new("bash")
                .arg(script_path.to_str().unwrap_or("./cleanup.sh"))
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to execute cleanup script: {}", e))?;
            
            // Read output lines for progress tracking
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(text) = line {
                        let (pct, step) = if text.contains("Stopping containers") {
                            (40, "stopping_containers")
                        } else if text.contains("Removing Docker networks") {
                            (60, "cleaning_networks")
                        } else if text.contains("Cleanup completed") {
                            (75, "containers_removed")
                        } else {
                            continue;
                        };
                        // Emit progress event
                        super::safe_emit(&app_handle, "site_removal_progress", serde_json::json!({
                            "site": name,
                            "percent": pct,
                            "step": step
                        }));
                    }
                }
            }
            
            let status = child.wait().map_err(|e| format!("Error waiting for cleanup script: {}", e))?;
            if !status.success() {
                return Err(format!("Cleanup script exited with status {:?}", status.code()));
            }
                
            Ok(())
        })();
        
        // Always clean up the script
        if let Err(e) = fs::remove_file(&script_path) {
            println!("Warning: Failed to remove cleanup script {}: {}", script_path.display(), e);
        }
        
        // Only return error if we couldn't execute the cleanup and the error is worth reporting
        if let Err(e) = cleanup_result {
            println!("Warning during container cleanup: {}", e);
            // We don't return the error since we still want to remove the site from state
        }
    }
    
    // Even if the script fails, we'll try to remove the directory and site from state
    super::safe_emit(&app_handle, "site_removal_progress", serde_json::json!({
        "site": name,
        "percent": 85,
        "step": "removing_files"
    }));
    
    // Try to remove the site directory with robust error handling
    println!("Attempting to remove site directory: {}", site.path.display());
    
    // First attempt: standard fs::remove_dir_all
    match fs::remove_dir_all(&site.path) {
        Ok(_) => {
            println!("Successfully removed site directory: {}", site.path.display());
        }
        Err(e) => {
            println!("Failed to remove directory with fs::remove_dir_all: {}. Trying system command...", e);
            
            // Fallback: use system command for forced removal
            let remove_cmd = if cfg!(target_os = "windows") {
                format!("rmdir /s /q \"{}\"", site.path.display())
            } else {
                format!("rm -rf \"{}\"", site.path.display())
            };
            
            match Command::new("sh")
                .arg("-c")
                .arg(&remove_cmd)
                .output()
            {
                Ok(output) => {
                    if output.status.success() {
                        println!("Successfully removed site directory using system command");
                    } else {
                        let error = String::from_utf8(output.stderr)
                            .unwrap_or_else(|_| "Unknown error".to_string());
                        println!("Warning: Failed to remove site directory even with system command: {}", error);
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
    
    // Remove from in-memory state
    sites.remove(&name);
    drop(sites); // Release the lock early
    
    super::safe_emit(&app_handle, "site_removal_progress", serde_json::json!({
        "site": name,
        "percent": 95,
        "step": "updating_config"
    }));
    
    // Remove the individual site configuration file
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    let sites_dir = app_dir.join("sites");
    let site_config_file = sites_dir.join(format!("{}.json", name));
    
    // Remove the individual site configuration file if it exists
    if site_config_file.exists() {
        fs::remove_file(&site_config_file)
            .map_err(|e| format!("Failed to remove site config file: {}", e))?;
    }
    
    super::safe_emit(&app_handle, "site_removal_progress", serde_json::json!({
        "site": name,
        "percent": 100,
        "step": "completed"
    }));
    
    super::safe_emit(&app_handle, "site_removed", format!("Site '{}' removed successfully", name));
    Ok(format!("Site '{}' removed successfully", name))
}
