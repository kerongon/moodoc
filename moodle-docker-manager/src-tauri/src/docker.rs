// This file contains Rust functions for managing Docker containers and interacting with the Docker API.

use std::process::Command;
use std::fs::{self, File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tauri::{command, AppHandle, Manager};
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
    web_port: Option<u16>
) -> Result<String, String> {
    // Create site directory structure
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let sites_dir = app_dir.join("sites");
    if !sites_dir.exists() {
        create_dir_all(&sites_dir)
            .map_err(|e| format!("Failed to create sites directory: {}", e))?;
    }
    
    let site_dir = sites_dir.join(&name);
    if !site_dir.exists() {
        create_dir_all(&site_dir)
            .map_err(|e| format!("Failed to create site directory: {}", e))?;
    }
    
    // Generate docker-compose.yml file
    let docker_compose_content = generate_docker_compose(
        &name, 
        &php_version, 
        &db_type, 
        &moodle_branch, 
        db_port, 
        web_port
    );
    
    let docker_compose_path = site_dir.join("docker-compose.yml");
    let mut file = File::create(&docker_compose_path)
        .map_err(|e| format!("Failed to create docker-compose.yml: {}", e))?;
    
    file.write_all(docker_compose_content.as_bytes())
        .map_err(|e| format!("Failed to write docker-compose.yml: {}", e))?;
    
    // Start the containers
    let output = Command::new("docker-compose")
        .current_dir(&site_dir)
        .args(["-f", "docker-compose.yml", "up", "-d"])
        .output()
        .map_err(|e| format!("Failed to execute docker-compose: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to start containers: {}", error));
    }
    
    // Save site configuration
    let site = Site {
        name: name.clone(),
        php_version,
        db_type,
        moodle_branch,
        db_port,
        web_port,
        status: "running".to_string(),
        path: site_dir,
    };
    
    // Store site in state and configuration file
    save_site_config(app_handle, site)?;
    
    Ok(format!("Successfully created site: {}", name))
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
pub fn get_docker_logs(container_name: String) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("logs")
        .arg(&container_name)
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        Ok(output) => Err(String::from_utf8_lossy(&output.stderr).to_string()),
        Err(err) => Err(err.to_string()),
    }
}

fn generate_docker_compose(
    name: &str,
    php_version: &str,
    db_type: &str,
    moodle_branch: &str,
    db_port: Option<u16>,
    web_port: Option<u16>
) -> String {
    let _db_port_str = db_port.map_or_else(|| "".to_string(), |p| p.to_string());
    let _web_port_str = web_port.map_or_else(|| "".to_string(), |p| p.to_string());
    
    let db_image = match db_type.to_lowercase().as_str() {
        "mysql" => "mysql:5.7",
        "postgresql" => "postgres:13",
        _ => "mysql:5.7", // Default to MySQL
    };
    
    let db_environment = match db_type.to_lowercase().as_str() {
        "mysql" => vec![
            "MYSQL_ROOT_PASSWORD=moodle",
            "MYSQL_DATABASE=moodle",
            "MYSQL_USER=moodle",
            "MYSQL_PASSWORD=moodle"
        ],
        "postgresql" => vec![
            "POSTGRES_DB=moodle",
            "POSTGRES_USER=moodle",
            "POSTGRES_PASSWORD=moodle"
        ],
        _ => vec![], // Default empty
    };
    
    let db_port_mapping = if let Some(port) = db_port {
        format!("    ports:\n      - \"{}:3306\"", port)
    } else {
        "".to_string()
    };
    
    let web_port_mapping = if let Some(port) = web_port {
        format!("    ports:\n      - \"{}:80\"", port)
    } else {
        "".to_string()
    };
    
    let db_env_section = db_environment
        .into_iter()
        .map(|e| format!("      - {}", e))
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(
        r#"version: '3'
services:
  db:
    image: {}
    container_name: {}_db
    environment:
{}
{}
    volumes:
      - db_data:/var/lib/mysql
    networks:
      - moodle_network

  moodle:
    image: moodlehq/moodle-php-apache:{}
    container_name: {}_moodle
    depends_on:
      - db
    environment:
      - MOODLE_DB_HOST=db
      - MOODLE_DB_USER=moodle
      - MOODLE_DB_PASSWORD=moodle
      - MOODLE_DB_NAME=moodle
      - MOODLE_BRANCH={}
{}
    volumes:
      - moodle_data:/var/www/html
    networks:
      - moodle_network

networks:
  moodle_network:

volumes:
  db_data:
  moodle_data:
"#,
        db_image,
        name,
        db_env_section,
        db_port_mapping,
        php_version,
        name,
        moodle_branch,
        web_port_mapping
    )
}

fn save_site_config(app_handle: AppHandle, site: Site) -> Result<(), String> {
    // Get sites state
    let state = app_handle.state::<SiteState>();
    let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Add or update site in memory state
    sites.insert(site.name.clone(), site.clone());
    
    // Save to config file
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
    
    Ok(())
}

#[command]
pub fn list_sites(app_handle: AppHandle) -> Result<Vec<Site>, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    let sites_vec: Vec<Site> = sites.values().cloned().collect();
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
    
    // Start the containers
    let output = Command::new("docker-compose")
        .current_dir(&site.path)
        .args(["-f", "docker-compose.yml", "up", "-d"])
        .output()
        .map_err(|e| format!("Failed to execute docker-compose: {}", e))?;
    
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
    
    // Stop the containers
    let output = Command::new("docker-compose")
        .current_dir(&site.path)
        .args(["-f", "docker-compose.yml", "down"])
        .output()
        .map_err(|e| format!("Failed to execute docker-compose: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to stop containers: {}", error));
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
    
    // Stop and remove the containers
    let output = Command::new("docker-compose")
        .current_dir(&site.path)
        .args(["-f", "docker-compose.yml", "down", "-v"])
        .output()
        .map_err(|e| format!("Failed to execute docker-compose: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Failed to remove containers: {}", error));
    }
    
    // Remove the site directory
    fs::remove_dir_all(&site.path)
        .map_err(|e| format!("Failed to remove site directory: {}", e))?;
    
    // Remove from state
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