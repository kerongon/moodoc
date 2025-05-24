// Site management functionality
// This module contains functions for creating, starting, stopping, and managing Moodle sites

use std::fs::{self, File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
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
    
    // Rest of the setup process continues...
    // This is a simplified version for the initial refactor
    
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
pub fn remove_site(app_handle: AppHandle, name: String, remove_containers: Option<bool>) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    let should_remove_containers = remove_containers.unwrap_or(true);
    
    // Stop the site first if it's running
    if site.status == "running" {
        if let Err(e) = stop_site(app_handle.clone(), name.clone()) {
            return Err(format!("Failed to stop site before removal: {}", e));
        }
    }
    
    // Remove Docker containers and volumes if requested
    if should_remove_containers {
        let moodle_www_root = site.path.join("moodle");
        let abs_moodle_path = moodle_www_root.canonicalize()
            .unwrap_or_else(|_| moodle_www_root.clone());
        
        // Create removal script
        let removal_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT={}
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_WEB_PORT={}
export MOODLE_DOCKER_PHP_VERSION={}
export MOODLE_DOCKER_DB_PORT={}

# Stop and remove containers
./bin/moodle-docker-compose down

# Remove volumes (this will delete the database)
docker volume rm ${{COMPOSE_PROJECT_NAME}}_mariadb_data 2>/dev/null || true
docker volume rm ${{COMPOSE_PROJECT_NAME}}_mysql_data 2>/dev/null || true
docker volume rm ${{COMPOSE_PROJECT_NAME}}_postgres_data 2>/dev/null || true

# Remove any orphaned containers
docker container prune -f --filter label=com.docker.compose.project=${{COMPOSE_PROJECT_NAME}}

echo "Docker containers and volumes removed"
"#, 
        site.path.to_str().unwrap_or(""),
        name,
        abs_moodle_path.to_str().unwrap_or(""),
        site.db_type.to_lowercase(),
        site.web_port.unwrap_or(8000),
        site.php_version,
        site.db_port.unwrap_or(3306)
        );
        
        let script_path = site.path.join("remove_temp.sh");
        fs::write(&script_path, removal_script)
            .map_err(|e| format!("Failed to create removal script: {}", e))?;
        
        // Make script executable
        Command::new("chmod")
            .args(["+x", script_path.to_str().unwrap_or("remove_temp.sh")])
            .output()
            .map_err(|e| format!("Failed to make removal script executable: {}", e))?;
        
        // Execute removal script
        let output = Command::new(script_path.to_str().unwrap_or("./remove_temp.sh"))
            .output()
            .map_err(|e| {
                let _ = fs::remove_file(&script_path);
                format!("Failed to execute removal script: {}", e)
            })?;
        
        // Clean up script
        let _ = fs::remove_file(script_path);
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to remove Docker containers: {}", error));
        }
    }
    
    // Remove site directory if it exists
    if site.path.exists() {
        fs::remove_dir_all(&site.path)
            .map_err(|e| format!("Failed to remove site directory: {}", e))?;
    }
    
    // Remove site configuration file
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let config_file = app_dir.join("sites").join(format!("{}.json", name));
    if config_file.exists() {
        fs::remove_file(&config_file)
            .map_err(|e| format!("Failed to remove site configuration: {}", e))?;
    }
    
    // Remove from in-memory state
    sites.remove(&name);
    
    super::safe_emit(&app_handle, "site_removed", format!("Site '{}' removed successfully", name));
    Ok(format!("Site '{}' removed successfully", name))
}
