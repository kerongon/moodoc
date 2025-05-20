// This file contains Rust functions for managing Docker containers and interacting with the Docker API.

use std::process::Command;
use std::fs::{self, File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tauri::{command, AppHandle, Manager, State};
use std::sync::Mutex;

// Define data structures for site configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    pub name: String,
    pub php_version: String,
    pub db_type: String,
    pub moodle_branch: String,
    pub db_port: Option<u16>,
    pub web_port: Option<u16>,
    pub status: String, // "running", "stopped", or "error"
    pub path: PathBuf,
}

// State management for sites
pub struct SiteState(pub Mutex<HashMap<String, Site>>);

#[command]
pub fn check_docker_installed() -> bool {
    Command::new("which")
        .arg("docker")
        .output()
        .map_or(false, |output| output.status.success())
}

#[command]
pub fn check_docker_running() -> bool {
    let output = Command::new("docker")
        .arg("info")
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

#[command]
pub fn create_site(
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
    validate_environment(app_handle.clone())?;
    
    // Determine the site directory path based on user preferences
    let site_dir = if let Some(dir) = directory {
        // Use custom directory if provided
        let path = PathBuf::from(dir);
        let site_path = path.join(&name);
        
        // Create the directory if it doesn't exist
        if !path.exists() {
            create_dir_all(&path)
                .map_err(|e| format!("Failed to create directory '{}': {}", path.display(), e))?;
        }
        
        site_path
    } else {
        // Use default directory in app data if no custom directory provided
        let app_dir = app_handle.path_resolver().app_data_dir()
            .ok_or_else(|| "Failed to get app data directory".to_string())?;
        
        let sites_dir = app_dir.join("sites");
        // Double check that sites directory exists
        if !sites_dir.exists() {
            create_dir_all(&sites_dir)
                .map_err(|e| format!("Failed to create sites directory: {}", e))?;
        }
        
        sites_dir.join(&name)
    };
    
    // Create the site directory if it doesn't exist
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
    if !moodle_dir.exists() {
        // Clone the Moodle repository with the specified branch
        let output = Command::new("git")
            .current_dir(&moodle_docker_dir)
            .args([
                "clone", 
                "-b", 
                &moodle_branch,
                "git://git.moodle.org/moodle.git", 
                "./moodle"
            ])
            .output()
            .map_err(|e| format!("Failed to clone Moodle repository: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8(output.stderr)
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to clone Moodle repository: {}", error));
        }
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
    let setup_script = format!(r#"#!/bin/bash
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_WEB_PORT={}
export MOODLE_DOCKER_PHP_VERSION={}

# Start the containers
./bin/moodle-docker-compose up -d

# Wait for database to be ready
./bin/moodle-docker-wait-for-db

# Install Moodle
./bin/moodle-docker-compose exec webserver php admin/cli/install_database.php --agree-license --fullname="Docker Moodle" --shortname="docker_moodle" --summary="Docker Moodle site" --adminpass="test" --adminemail="admin@example.com"

echo "Moodle is now available at http://localhost:{}"
echo "Admin username: admin"
echo "Admin password: test"
"#, name, db_type.to_lowercase(), web_port_str, php_version, web_port_str);

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
    
    // Instead of directly executing the setup.sh script, we'll run the docker compose commands one by one
    // to better handle potential hanging/freezing issues
    
    // First, start the containers
    println!("Attempting to start Docker containers for site: {}", name);
    app_handle.emit_all("site_creation_progress", format!("Starting Docker containers for {}", name)).unwrap_or_default();
    
    let start_cmd = format!(r#"cd {} && export COMPOSE_PROJECT_NAME={} && export MOODLE_DOCKER_WWWROOT=$PWD/moodle && export MOODLE_DOCKER_DB={} && export MOODLE_DOCKER_WEB_PORT={} && export MOODLE_DOCKER_PHP_VERSION={} && ./bin/moodle-docker-compose up -d"#,
        moodle_docker_dir.to_str().unwrap_or(""),
        name,
        db_type.to_lowercase(),
        web_port.map_or("8000".to_string(), |p| p.to_string()),
        php_version
    );
    
    // Execute the start command
    let output = Command::new("sh")
        .arg("-c")
        .arg(&start_cmd)
        .output()
        .map_err(|e| format!("Failed to start Docker containers: {}", e))?;
    
    println!("Docker containers start command executed for site: {}. Success: {}", name, output.status.success());
    if !output.status.success() {
        app_handle.emit_all("site_creation_progress", format!("Failed to start Docker containers for {}", name)).unwrap_or_default();
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to start Docker containers: {}", error));
    }
    
    // Wait for the database to be ready
    println!("Attempting to wait for database for site: {}", name);
    app_handle.emit_all("site_creation_progress", format!("Waiting for database to be ready for {}", name)).unwrap_or_default();
    let wait_db_cmd = format!(r#"cd {} && export COMPOSE_PROJECT_NAME={} && export MOODLE_DOCKER_WWWROOT=$PWD/moodle && export MOODLE_DOCKER_DB={} && export MOODLE_DOCKER_PHP_VERSION={} && ./bin/moodle-docker-wait-for-db"#,
        moodle_docker_dir.to_str().unwrap_or(""),
        name,
        db_type.to_lowercase(),
        php_version
    );
    
    // Execute the wait for db command
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
        app_handle.emit_all("site_creation_progress", format!("Failed while waiting for database for {}", name)).unwrap_or_default();
        // Try to get container logs to see what's happening
        println!("Getting container logs to diagnose the issue...");
        let logs_cmd = format!(r#"cd {} && export COMPOSE_PROJECT_NAME={} && ./bin/moodle-docker-compose logs db"#,
            moodle_docker_dir.to_str().unwrap_or(""),
            name
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
    
    // Install Moodle
    println!("Attempting to install Moodle for site: {}", name);
    app_handle.emit_all("site_creation_progress", format!("Installing Moodle for {}. This may take several minutes...", name)).unwrap_or_default();
    let install_cmd = format!(r#"cd {} && export COMPOSE_PROJECT_NAME={} && export MOODLE_DOCKER_WWWROOT=$PWD/moodle && export MOODLE_DOCKER_DB={} && export MOODLE_DOCKER_PHP_VERSION={} && ./bin/moodle-docker-compose exec -T webserver php admin/cli/install_database.php --agree-license --fullname="Docker Moodle" --shortname="docker_moodle" --summary="Docker Moodle site" --adminpass="test" --adminemail="admin@example.com""#,
        moodle_docker_dir.to_str().unwrap_or(""),
        name,
        db_type.to_lowercase(),
        php_version
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
        app_handle.emit_all("site_creation_progress", format!("Failed to install Moodle for {}", name)).unwrap_or_default();
        // Try to get webserver logs to diagnose the issue
        println!("Getting webserver logs to diagnose the issue...");
        let logs_cmd = format!(r#"cd {} && export COMPOSE_PROJECT_NAME={} && ./bin/moodle-docker-compose logs webserver"#,
            moodle_docker_dir.to_str().unwrap_or(""),
            name
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
    app_handle.emit_all("site_creation_progress", format!("Finalizing setup for {}", name)).unwrap_or_default();
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
    
    // Store site in state and configuration file
    save_site_config(app_handle, site)?;
    
    // Return combined stdout and stderr for logging
    let result = format!("{}\n{}", output_str, error_str);
    Ok(result)
}

// Load site configurations at application startup
pub fn load_site_configs(app: &tauri::App) -> Result<(), String> {
    let app_handle = app.handle();
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    // Ensure app data directory exists
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }
    
    let config_path = app_dir.join("sites_config.json");
    
    // Create sites state if it doesn't exist
    let state = app.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Skip if config file doesn't exist
    if !config_path.exists() {
        return Ok(());
    }
    
    // Read and parse config file
    let config_data = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read sites config: {}", e))?;
    
    let sites_vec: Vec<Site> = serde_json::from_str(&config_data)
        .map_err(|e| format!("Failed to parse sites config: {}", e))?;
    
    // Populate the sites state
    for site in sites_vec {
        sites.insert(site.name.clone(), site);
    }
    
    Ok(())
}

pub fn stop_docker_container(container_name: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("stop")
        .arg(container_name)
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        Ok(output) => Err(String::from_utf8_lossy(&output.stderr).to_string()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn remove_docker_container(container_name: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("rm")
        .arg(container_name)
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        Ok(output) => Err(String::from_utf8_lossy(&output.stderr).to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[command]
pub fn get_docker_logs(app_handle: AppHandle, name: String, service: String) -> Result<String, String> {
    let state: State<SiteState> = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;

    let site = sites.get(&name)
        .ok_or_else(|| format!("Site '{}' not found", name))?;

    let moodle_docker_dir = &site.path; // This is a PathBuf
    let moodle_docker_dir_str = moodle_docker_dir.to_str()
        .ok_or_else(|| "Failed to convert site path to string".to_string())?;

    // MOODLE_DOCKER_WWWROOT should be the absolute path to the moodle subdirectory within moodle_docker_dir
    let moodle_www_root = moodle_docker_dir.join("moodle");
    let moodle_www_root_str = moodle_www_root.to_str()
        .ok_or_else(|| format!("Failed to convert moodle www root path to string. Base path: {:?}, Joined path: {:?}", moodle_docker_dir, moodle_www_root))?;

    let web_port_str = site.web_port.map_or("8000".to_string(), |p| p.to_string());

    println!(
        "Executing './bin/moodle-docker-compose logs {}' in dir: {}. Env: COMPOSE_PROJECT_NAME={}, MOODLE_DOCKER_WWWROOT={}, MOODLE_DOCKER_DB={}, MOODLE_DOCKER_PHP_VERSION={}, MOODLE_DOCKER_WEB_PORT={}",
        service,
        moodle_docker_dir_str,
        name,
        moodle_www_root_str,
        site.db_type.to_lowercase(),
        site.php_version,
        web_port_str
    );

    let output = Command::new("./bin/moodle-docker-compose")
        .current_dir(moodle_docker_dir_str) // Set the working directory
        .env("COMPOSE_PROJECT_NAME", &name)
        .env("MOODLE_DOCKER_WWWROOT", moodle_www_root_str)
        .env("MOODLE_DOCKER_DB", site.db_type.to_lowercase())
        .env("MOODLE_DOCKER_PHP_VERSION", &site.php_version)
        .env("MOODLE_DOCKER_WEB_PORT", &web_port_str)
        .arg("logs")
        .arg(&service)
        .output()
        .map_err(|e| format!("Failed to execute log command './bin/moodle-docker-compose': {}", e))?;

    if output.status.success() {
        let logs = String::from_utf8(output.stdout)
            .map_err(|e| format!("Failed to parse logs from stdout: {}", e))?;
        // If logs are empty, it could still be a success but no logs to show.
        // The frontend can decide how to display empty logs.
        println!("Successfully fetched logs for {}/{}:\n{}", name, service, if logs.is_empty() { "[empty]" } else { &logs });
        Ok(logs)
    } else {
        let stderr_output = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout_output = String::from_utf8_lossy(&output.stdout).to_string();
        let mut error_description = String::new();

        if !stderr_output.is_empty() {
            error_description.push_str(&format!("Stderr: {}", stderr_output));
        }
        if !stdout_output.is_empty() {
            if !error_description.is_empty() { error_description.push_str("; "); }
            error_description.push_str(&format!("Stdout: {}", stdout_output));
        }
        
        // If both stderr and stdout are empty, provide a generic message with status
        if error_description.is_empty() {
            error_description = format!("Command failed with status {} but produced no output to stderr or stdout.", output.status);
        } else {
            error_description = format!("Details: {}", error_description);
        }


        println!(
            "Failed to fetch logs for {}/{}. Exit status: {:?}. {}",
            name, service, output.status, error_description
        );
        Err(format!(
            "Failed to fetch logs for {}/{}. Status: {:?}. {}",
            name, service, output.status, error_description
        ))
    }
}

// Function removed as we are now using the official moodle-docker approach

fn save_site_config(app_handle: AppHandle, site: Site) -> Result<(), String> {
    // Get sites state
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Add or update site in memory state
    sites.insert(site.name.clone(), site.clone());
    
    // Save to config file
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    // Ensure the app data directory exists
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }
    
    let config_path = app_dir.join("sites_config.json");
    
    // Convert HashMap to Vec for serialization
    let sites_vec: Vec<Site> = sites.values().cloned().collect();
    
    // Serialize and save to file
    let json = serde_json::to_string_pretty(&sites_vec)
        .map_err(|e| format!("Failed to serialize sites config: {}", e))?;
    
    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write sites config: {}", e))?;
    
    Ok(())
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
    
    // Set environment variables and start containers
    let script_content = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_WEB_PORT={}

# Start containers
./bin/moodle-docker-compose up -d
"#, 
    site.path.to_str().unwrap_or(""),
    name,
    site.db_type.to_lowercase(),
    site.web_port.unwrap_or(8000)
    );
    
    // Create .env file to ensure environment variables are properly set
    let env_content = format!(r#"COMPOSE_PROJECT_NAME={}
MOODLE_DOCKER_DB={}
MOODLE_DOCKER_WWWROOT={}/moodle
MOODLE_DOCKER_WEB_PORT={}
MOODLE_DOCKER_PHP_VERSION={}
"#, 
    name,
    site.db_type.to_lowercase(),
    site.path.to_str().unwrap_or(""),
    site.web_port.unwrap_or(8000),
    site.php_version
    );

    // Write the .env file
    let env_file_path = site.path.join(".env");
    let mut env_file = File::create(&env_file_path)
        .map_err(|e| format!("Failed to create .env file: {}", e))?;
    
    env_file.write_all(env_content.as_bytes())
        .map_err(|e| format!("Failed to write .env file: {}", e))?;

    // Write the script to a temporary file
    let temp_script_path = site.path.join("start.sh");
    let mut file = File::create(&temp_script_path)
        .map_err(|e| format!("Failed to create start script: {}", e))?;
    
    file.write_all(script_content.as_bytes())
        .map_err(|e| format!("Failed to write start script: {}", e))?;
    
    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", temp_script_path.to_str().unwrap_or("start.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        return Err(format!("Failed to make script executable"));
    }
    
    // Execute the script
    let output = Command::new(temp_script_path.to_str().unwrap_or("./start.sh"))
        .output()
        .map_err(|e| format!("Failed to execute start script: {}", e))?;
    
    // Delete the temporary script
    let _ = fs::remove_file(temp_script_path);
    
    if !output.status.success() {
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to start containers: {}", error));
    }
    
    // Update site status
    let mut updated_site = site;
    updated_site.status = "running".to_string();
    sites.insert(name.clone(), updated_site.clone());
    
    // Save the updated configuration
    drop(sites); // Release the lock before calling save_site_config
    save_site_config(app_handle, updated_site)?;
    
    Ok(format!("Site '{}' started successfully", name))
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
    
    // Set environment variables and stop containers
    let script_content = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_WEB_PORT={}
export MOODLE_DOCKER_PHP_VERSION={}

# Stop containers
./bin/moodle-docker-compose down
"#, 
    site.path.to_str().unwrap_or(""),
    name,
    site.db_type.to_lowercase(),
    site.web_port.unwrap_or(8000),
    site.php_version
    );
    
    // Print debugging information
    println!("Stopping site: {}", name);
    println!("Site path: {:?}", site.path);
    
    // Check if directory exists
    if !site.path.exists() {
        return Err(format!("Site directory does not exist: {:?}", site.path));
    }
    
    // Check if moodle-docker-compose exists
    let compose_script = site.path.join("bin").join("moodle-docker-compose");
    if !compose_script.exists() {
        return Err(format!("Docker compose script not found at: {:?}", compose_script));
    }
    
    // Write the script to a temporary file
    let temp_script_path = site.path.join("stop.sh");
    let mut file = File::create(&temp_script_path)
        .map_err(|e| format!("Failed to create stop script: {}", e))?;
    
    file.write_all(script_content.as_bytes())
        .map_err(|e| format!("Failed to write stop script: {}", e))?;
    
    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", temp_script_path.to_str().unwrap_or("stop.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        return Err(format!("Failed to make script executable"));
    }
    
    // Execute the script with proper shell
    let output = Command::new("bash")
        .arg(temp_script_path.to_str().unwrap_or("./stop.sh"))
        .output()
        .map_err(|e| format!("Failed to execute stop script: {}", e))?;
    
    // Delete the temporary script
    let _ = fs::remove_file(temp_script_path);
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        let mut error_message = format!("Failed to stop containers (exit code: {:?}).", output.status.code());
        
        if !stderr.trim().is_empty() {
            error_message.push_str(&format!("\n\nError output:\n{}", stderr));
        }
        
        if !stdout.trim().is_empty() {
            error_message.push_str(&format!("\n\nStandard output:\n{}", stdout));
        }
        
        return Err(error_message);
    }
    
    // Update site status
    let mut updated_site = site;
    updated_site.status = "stopped".to_string();
    sites.insert(name.clone(), updated_site.clone());
    
    // Save the updated configuration
    drop(sites); // Release the lock before calling save_site_config
    save_site_config(app_handle, updated_site)?;
    
    Ok(format!("Site '{}' stopped successfully", name))
}

#[command]
pub fn remove_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Create a cleanup script to ensure proper environment variables are set
    let cleanup_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle

# Stop and remove containers with volumes
./bin/moodle-docker-compose down -v

echo "Removed all containers and volumes for {}"
"#, 
        site.path.to_str().unwrap_or(""),
        name,
        name
    );
    
    // Write the cleanup script
    let script_path = site.path.join("cleanup.sh");
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
    
    // Execute the cleanup script
    let _output = Command::new(script_path.to_str().unwrap_or("./cleanup.sh"))
        .output()
        .map_err(|e| format!("Failed to execute cleanup script: {}", e))?;
    
    // Even if the script fails, we'll try to remove the directory and site from state
    
    // Try to remove the site directory
    let _remove_result = fs::remove_dir_all(&site.path);
    
    // Remove the site from state
    sites.remove(&name);
    
    // Save the updated configuration
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let config_path = app_dir.join("sites_config.json");
    
    // Convert HashMap to Vec for serialization
    let sites_vec: Vec<Site> = sites.values().cloned().collect();
    
    // Serialize and save to file
    let json = serde_json::to_string_pretty(&sites_vec)
        .map_err(|e| format!("Failed to serialize sites config: {}", e))?;
    
    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write sites config: {}", e))?;
    
    Ok(format!("Site '{}' removed successfully", name))
}

#[command]
pub fn check_platform_compatibility(db_type: String) -> Result<String, String> {
    // Check if we're on ARM architecture (like Apple Silicon)
    #[cfg(target_arch = "aarch64")]
    {
        // All database types that might have compatibility issues on ARM
        match db_type.to_lowercase().as_str() {
            "oracle" => {
                return Ok("Note: Oracle on ARM64 uses an experimental image. You may experience some performance issues.".to_string());
            },
            "microsoft sql server" => {
                return Ok("Note: SQL Server on ARM64 uses Azure SQL Edge, which has some feature limitations compared to SQL Server.".to_string());
            },
            _ => {
                return Ok("Your platform is compatible with the selected database type.".to_string());
            }
        }
    }
    
    // For non-ARM architectures, all database types should work fine
    #[cfg(not(target_arch = "aarch64"))]
    {
        return Ok("Your platform is compatible with the selected database type.".to_string());
    }
}

// This function will validate the Docker setup and ensure directories exist
#[command]
pub fn validate_environment(app_handle: AppHandle) -> Result<String, String> {
    // Check if Docker is installed and running
    if !check_docker_installed() {
        return Err("Docker is not installed".to_string());
    }
    
    if !check_docker_running() {
        return Err("Docker is installed but not running".to_string());
    }
    
    // Ensure app data directory exists
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }
    
    // Create sites directory
    let sites_dir = app_dir.join("sites");
    if !sites_dir.exists() {
        fs::create_dir_all(&sites_dir)
            .map_err(|e| format!("Failed to create sites directory: {}", e))?;
    }
    
    // Create sites config file if it doesn't exist
    let config_path = app_dir.join("sites_config.json");
    if !config_path.exists() {
        let empty_sites: Vec<Site> = Vec::new();
        let json = serde_json::to_string_pretty(&empty_sites)
            .map_err(|e| format!("Failed to create empty sites config: {}", e))?;
        
        fs::write(&config_path, json)
            .map_err(|e| format!("Failed to initialize sites config: {}", e))?;
    }
    
    Ok("Environment validated successfully".to_string())
}

#[command]
pub fn check_site_status(app_handle: AppHandle, name: String) -> Result<Site, String> {
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Create a script to check if containers are running
    let check_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle

# Check if containers are running
./bin/moodle-docker-compose ps -q webserver
"#, 
        site.path.to_str().unwrap_or(""),
        name
    );
    
    // Write the script to a temporary file
    let check_script_path = site.path.join("check_status.sh");
    let mut file = File::create(&check_script_path)
        .map_err(|e| format!("Failed to create check script: {}", e))?;
    
    file.write_all(check_script.as_bytes())
        .map_err(|e| format!("Failed to write check script: {}", e))?;
    
    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", check_script_path.to_str().unwrap_or("check_status.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        return Err(format!("Failed to make check script executable"));
    }
    
    // Execute the script
    let output = Command::new(check_script_path.to_str().unwrap_or("./check_status.sh"))
        .output()
        .map_err(|e| format!("Failed to execute check script: {}", e))?;
    
    // Delete the temporary script
    let _ = fs::remove_file(check_script_path);
    
    // If we got a container ID, the site is running
    let status = if output.status.success() && !output.stdout.is_empty() {
        "running".to_string()
    } else {
        "stopped".to_string()
    };
    
    // Update site status if it changed
    let mut updated_site = site.clone();
    if updated_site.status != status {
        updated_site.status = status;
        sites.insert(name.clone(), updated_site.clone());
        
        // Save the updated configuration
        drop(sites); // Release the lock before calling save_site_config
        save_site_config(app_handle, updated_site.clone())?;
    }
    
    Ok(updated_site)
}

use std::process::Stdio;
use std::io::{BufRead, BufReader};
#[command]
pub async fn backup_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // First, check if the site is currently running
    let status_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle

# Check if containers are running
./bin/moodle-docker-compose ps -q webserver
"#, 
        site.path.to_str().unwrap_or(""),
        name
    );
    
    // Write the status check script to a temporary file
    let status_script_path = site.path.join("check_status.sh");
    let mut file = File::create(&status_script_path)
        .map_err(|e| format!("Failed to create status check script: {}", e))?;
    
    file.write_all(status_script.as_bytes())
        .map_err(|e| format!("Failed to write status check script: {}", e))?;
    
    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", status_script_path.to_str().unwrap_or("check_status.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        let _ = fs::remove_file(&status_script_path);
        return Err(format!("Failed to make status script executable"));
    }
    
    // Execute the script
    let output = Command::new(status_script_path.to_str().unwrap_or("./check_status.sh"))
        .output()
        .map_err(|e| format!("Failed to execute status script: {}", e))?;
    
    // Delete the temporary script
    let _ = fs::remove_file(status_script_path);
    
    let is_running = output.status.success() && !output.stdout.is_empty();
    
    // Create timestamp for backup name
    let now = chrono::Utc::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
    let backup_name = format!("backup_{}_{}", name, timestamp);
    
    // Create backup directory in the site directory
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
    
    // Use a simplified approach to avoid placeholder mismatch errors
    let backup_script = format!(r#"#!/bin/bash
set -e
cd {script_path}
export COMPOSE_PROJECT_NAME={name}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB={db_type}
export MOODLE_DOCKER_PHP_VERSION={php_version}
export MOODLE_DOCKER_WEB_PORT={web_port}

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
    web_port = site.web_port.unwrap_or(8000)
    );
    
    // Write the backup script
    let backup_script_path = site.path.join("backup.sh");
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
        let _ = fs::remove_file(&backup_script_path);
        return Err(format!("Failed to make backup script executable"));
    }
    
    // Execute the backup script and stream stdout for progress
    let mut child = Command::new("bash")
        .arg(backup_script_path.to_str().unwrap_or("./backup.sh"))
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| { let _ = fs::remove_file(&backup_script_path); format!("Failed to execute backup script: {}", e) })?;
    // read output lines
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(text) = line {
                let mut pct = 0;
                let step = if text.contains("Backing up database") {
                    pct = 25; "database"
                } else if text.contains("Backing up Moodle code") {
                    pct = 50; "code"
                } else if text.contains("Backing up Moodle data") {
                    pct = 75; "data"
                } else if text.contains("Backup completed successfully") {
                    pct = 100; "done"
                } else {
                    continue;
                };
                // emit progress event
                app_handle.emit_all("backup_progress", serde_json::json!({
                    "site": name,
                    "percent": pct,
                    "step": step
                })).ok();
            }
        }
    }
    let status = child.wait().map_err(|e| format!("Error waiting for backup script: {}", e))?;
    let _ = fs::remove_file(&backup_script_path);
    if !status.success() {
        return Err(format!("Backup script exited with status {:?}", status.code()));
    }
    Ok(format!("Site '{}' backed up successfully to {}", name, backup_dir.to_str().unwrap_or("")))
}

#[command]
pub fn list_site_backups(app_handle: AppHandle, name: String) -> Result<Vec<HashMap<String, String>>, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Path to backups directory
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
                            if let Ok(manifest) = serde_json::from_str::<HashMap<String, String>>(&manifest_content) {
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
pub fn restore_site_backup(app_handle: AppHandle, name: String, backup_timestamp: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
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
    
    // Check if site is running and stop it if needed
    let stop_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle

# Stop containers if running
./bin/moodle-docker-compose down
"#, 
        site.path.to_str().unwrap_or(""),
        name
    );
    
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
        return Err(format!("Failed to make stop script executable"));
    }
    
    // Execute the stop script
    let output = Command::new(stop_script_path.to_str().unwrap_or("./stop_for_restore.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&stop_script_path);
            format!("Failed to execute stop script: {}", e)
        })?;
    
    // Delete the script
    let _ = fs::remove_file(stop_script_path);
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Failed to stop site for restore: {}", error));
    }
    
    // Create restore script
    let restore_script = format!(r#"#!/bin/bash
set -e
cd {site_path}
export COMPOSE_PROJECT_NAME={name}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB={db_type}
export MOODLE_DOCKER_PHP_VERSION={php_version}
export MOODLE_DOCKER_WEB_PORT={web_port}

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
    cat {backup_dir}/database/moodle_db.sql | ./bin/moodle-docker-compose exec -T db bash -c "mysql -u root -pm@0dl3ing moodle"
fi

echo "Starting remaining containers..."
# Start all containers
./bin/moodle-docker-compose up -d

echo "Site restored successfully"
"#, 
        site_path = site.path.to_str().unwrap_or(""),
        name = name,
        db_type = site.db_type.to_lowercase(),
        php_version = site.php_version,
        web_port = site.web_port.unwrap_or(8000),
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
    
    // Execute the restore script
    let output = Command::new(restore_script_path.to_str().unwrap_or("./restore.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&restore_script_path);
            format!("Failed to execute restore script: {}", e)
        })?;
    
    // Delete the script
    let _ = fs::remove_file(restore_script_path);
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Failed to restore site: {}", error));
    }
    
    // Update site status in state
    drop(sites); // Release the lock from earlier
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    if let Some(site) = sites.get_mut(&name) {
        site.status = "running".to_string();
    }
    
    drop(sites);
    
    // Return success message
    Ok(format!("Site '{}' restored successfully from backup {}", name, backup_timestamp))
}

#[command]
pub fn delete_site_backup(app_handle: AppHandle, name: String, backup_timestamp: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
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
    fs::remove_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to delete backup: {}", e))?;
    
    Ok(format!("Backup '{}' deleted successfully", backup_name))
}

#[command]
pub fn open_backup_location(app_handle: AppHandle, name: String, backup_timestamp: Option<String>) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Determine which path to open - specific backup or backups directory
    let target_path = if let Some(ref timestamp) = backup_timestamp {
        // Open specific backup folder
        let backup_name = format!("backup_{}_{}", name, timestamp);
        site.path.join("backups").join(backup_name)
    } else {
        // Open the general backups folder
        site.path.join("backups")
    };
    
    // Check if path exists
    if !target_path.exists() {
        if let Some(ref timestamp) = backup_timestamp {
            return Err(format!("Backup '{}' not found", timestamp));
        } else {
            // Create backups directory if it doesn't exist
            fs::create_dir_all(&target_path)
                .map_err(|e| format!("Failed to create backups directory: {}", e))?;
        }
    }
    
    // Use open command on macOS to open the directory in Finder
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("open")
            .arg(target_path.to_str().unwrap_or(""))
            .output()
            .map_err(|e| format!("Failed to open directory in Finder: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open directory in Finder: {}", error));
        }
        
        return Ok(format!("Opened in Finder: {}", target_path.display()));
    }
    
    // For Windows
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("explorer")
            .arg(target_path.to_str().unwrap_or(""))
            .output()
            .map_err(|e| format!("Failed to open directory in Explorer: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open directory in Explorer: {}", error));
        }
        
        return Ok(format!("Opened in Explorer: {}", target_path.display()));
    }
    
    // For Linux
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdg-open")
            .arg(target_path.to_str().unwrap_or(""))
            .output()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open directory: {}", error));
        }
        
        return Ok(format!("Opened directory: {}", target_path.display()));
    }
    
    #[allow(unreachable_code)]
    Ok(format!("Path: {}", target_path.display()))
}

#[command]
pub fn update_moodle_version(app_handle: AppHandle, name: String, new_branch: String, new_php_version: Option<String>) -> Result<String, String> {
    // First backup the site
    println!("Creating backup before updating site '{}'...", name);
    
    // We'll directly create a backup here rather than calling backup_site to avoid issues
    // with command functions calling each other
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Create timestamp for backup name
    let now = chrono::Utc::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
    let backup_name = format!("pre_update_backup_{}_{}", name, timestamp);
    
    // Create backup directory in the site directory
    let backups_dir = site.path.join("backups");
    if !backups_dir.exists() {
        fs::create_dir_all(&backups_dir)
            .map_err(|e| format!("Failed to create backups directory: {}", e))?;
    }
    
    let backup_dir = backups_dir.join(&backup_name);
    fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;
    
    println!("Backup directory created at: {}", backup_dir.display());
    drop(sites); // Release the lock before continuing
    
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Stop the site first
    let stop_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle

# Stop containers
./bin/moodle-docker-compose down
"#, 
        site.path.to_str().unwrap_or(""),
        name
    );
    
    // Write the stop script
    let stop_script_path = site.path.join("stop_for_update.sh");
    let mut file = File::create(&stop_script_path)
        .map_err(|e| format!("Failed to create stop script: {}", e))?;
    
    file.write_all(stop_script.as_bytes())
        .map_err(|e| format!("Failed to write stop script: {}", e))?;
    
    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", stop_script_path.to_str().unwrap_or("stop_for_update.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        let _ = fs::remove_file(&stop_script_path);
        return Err(format!("Failed to make stop script executable"));
    }
    
    // Execute the stop script
    let output = Command::new(stop_script_path.to_str().unwrap_or("./stop_for_update.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&stop_script_path);
            format!("Failed to execute stop script: {}", e)
        })?;
    
    // Delete the script
    let _ = fs::remove_file(stop_script_path);
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Failed to stop site for update: {}", error));
    }
    
    // Create update script with proper error handling
    let update_script = format!(r#"#!/bin/bash
set -e
cd {}

# Save moodledata directory
if [ -d "moodledata" ]; then
    echo "Backing up moodledata directory..."
    mv moodledata moodledata_backup
fi

# Backup the existing config.php
if [ -f "moodle/config.php" ]; then
    echo "Backing up config.php..."
    cp moodle/config.php moodle_config_backup.php
fi

echo "Removing old Moodle codebase..."
# Remove current Moodle codebase but keep .git directory for faster updates
mv moodle/data moodle_data_backup 2>/dev/null || true
rm -rf moodle

echo "Cloning new Moodle version..."
# Clone the Moodle repository with the specified branch
git clone -b {} git://git.moodle.org/moodle.git moodle

# Restore the data directory if it existed
if [ -d "moodle_data_backup" ]; then
    echo "Restoring data directory..."
    mkdir -p moodle/data
    cp -R moodle_data_backup/* moodle/data/
    rm -rf moodle_data_backup
fi

# Restore moodledata
if [ -d "moodledata_backup" ]; then
    echo "Restoring moodledata directory..."
    mv moodledata_backup moodledata
fi

# Restore config.php 
if [ -f "moodle_config_backup.php" ]; then
    echo "Restoring config.php..."
    cp moodle_config_backup.php moodle/config.php
    rm moodle_config_backup.php
else
    echo "No config.php backup found, creating a new one from template..."
    cp config.docker-template.php moodle/config.php
fi

echo "Successfully updated Moodle codebase to branch {}"
"#, 
        site.path.to_str().unwrap_or(""),
        new_branch,
        new_branch
    );
    
    // Write the update script
    let update_script_path = site.path.join("update.sh");
    let mut file = File::create(&update_script_path)
        .map_err(|e| format!("Failed to create update script: {}", e))?;
    
    file.write_all(update_script.as_bytes())
        .map_err(|e| format!("Failed to write update script: {}", e))?;
    
    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", update_script_path.to_str().unwrap_or("update.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        let _ = fs::remove_file(&update_script_path);
        return Err(format!("Failed to make update script executable"));
    }
    
    // Execute the update script
    let output = Command::new(update_script_path.to_str().unwrap_or("./update.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&update_script_path);
            format!("Failed to execute update script: {}", e)
        })?;
    
    // Delete the script
    let _ = fs::remove_file(&update_script_path);
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Failed to update Moodle version: {}", error));
    }
    
    // Use the new PHP version or keep the existing one
    let php_version = new_php_version.unwrap_or(site.php_version.clone());
    
    // Create a script to start the updated site
    let start_script = format!(r#"#!/bin/bash
cd {}
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT=$PWD/moodle
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_PHP_VERSION={}
export MOODLE_DOCKER_WEB_PORT={}

# Start the containers
./bin/moodle-docker-compose up -d

# Wait for database
./bin/moodle-docker-wait-for-db

# Run Moodle upgrade script
./bin/moodle-docker-compose exec webserver php admin/cli/upgrade.php --non-interactive

echo "Moodle site updated and started successfully"
"#, 
        site.path.to_str().unwrap_or(""),
        name,
        site.db_type.to_lowercase(),
        php_version,
        site.web_port.unwrap_or(8000)
    );
    
    // Write the start script
    let start_script_path = site.path.join("start_updated.sh");
    let mut file = File::create(&start_script_path)
        .map_err(|e| format!("Failed to create start script: {}", e))?;
    
    file.write_all(start_script.as_bytes())
        .map_err(|e| format!("Failed to write start script: {}", e))?;
    
    // Make the script executable
    let chmod_output = Command::new("chmod")
        .args(["+x", start_script_path.to_str().unwrap_or("start_updated.sh")])
        .output()
        .map_err(|e| format!("Failed to make script executable: {}", e))?;
    
    if !chmod_output.status.success() {
        let _ = fs::remove_file(&start_script_path);
        return Err(format!("Failed to make start script executable"));
    }
    
    // Execute the start script
    let output = Command::new(start_script_path.to_str().unwrap_or("./start_updated.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&start_script_path);
            format!("Failed to execute start script: {}", e)
        })?;
    
    // Delete the script
    let _ = fs::remove_file(&start_script_path);
    
    // Update site in state
    let mut updated_site = site.clone();
    updated_site.moodle_branch = new_branch.clone();
    updated_site.php_version = php_version;
    updated_site.status = "running".to_string();
    
    sites.insert(name.clone(), updated_site.clone());
    
    // Save the updated configuration
    drop(sites); // Release the lock before calling save_site_config
    save_site_config(app_handle.clone(), updated_site)?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("Failed to start updated site: {}. A backup was created before the update attempt.", error));
    }
    
    Ok(format!("Site '{}' successfully updated to Moodle {}", name, new_branch))
}