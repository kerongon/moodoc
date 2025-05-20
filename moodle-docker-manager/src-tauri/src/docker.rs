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
    web_port: Option<u16>,
    directory: Option<String>
) -> Result<String, String> {
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

// Load site configurations at application startup
pub fn load_site_configs(app: &tauri::App) -> Result<(), String> {
    let app_handle = app.handle();
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
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
        "mysql" => "mysql:8.0", // 8.0 has ARM64 support
        "mariadb" => "mariadb:10.6", // MariaDB has ARM64 support
        "postgresql" => "postgres:13", // PostgreSQL has ARM64 support
        "microsoft sql server" => {
            if cfg!(target_arch = "aarch64") {
                "mcr.microsoft.com/azure-sql-edge:latest" // SQL Server Edge for ARM64
            } else {
                "mcr.microsoft.com/mssql/server:2019-latest" // Regular SQL Server for x86_64
            }
        },
        "oracle" => {
            // Oracle DB doesn't have official arm64 images, so we'll use a custom build that works on ARM
            if cfg!(target_arch = "aarch64") {
                "gvenzl/oracle-xe:21-slim-faststart" // This tag has experimental ARM support
            } else {
                "gvenzl/oracle-xe:21-slim" // Regular Oracle XE for x86_64
            }
        },
        _ => "mysql:8.0", // Default to MySQL with ARM64 support
    };
    
    let db_environment = match db_type.to_lowercase().as_str() {
        "mysql" => vec![
            "MYSQL_ROOT_PASSWORD=moodle",
            "MYSQL_DATABASE=moodle",
            "MYSQL_USER=moodle",
            "MYSQL_PASSWORD=moodle"
        ],
        "mariadb" => vec![
            "MARIADB_ROOT_PASSWORD=moodle",
            "MARIADB_DATABASE=moodle",
            "MARIADB_USER=moodle",
            "MARIADB_PASSWORD=moodle"
        ],
        "postgresql" => vec![
            "POSTGRES_DB=moodle",
            "POSTGRES_USER=moodle",
            "POSTGRES_PASSWORD=moodle"
        ],
        "microsoft sql server" => vec![
            "ACCEPT_EULA=Y",
            "SA_PASSWORD=MoodleP@ssw0rd",
            "MSSQL_PID=Express"
        ],
        "oracle" => vec![
            "ORACLE_PASSWORD=moodle",
            "APP_USER=moodle",
            "APP_USER_PASSWORD=moodle", 
            "ORACLE_DATABASE=XE"
        ],
        _ => vec![], // Default empty
    };
    
    // Define the default container port based on database type
    let db_container_port = match db_type.to_lowercase().as_str() {
        "mysql" => 3306,
        "mariadb" => 3306,
        "postgresql" => 5432,
        "microsoft sql server" => 1433,
        "oracle" => 1521,
        _ => 3306, // Default to MySQL port
    };
    
    let db_port_mapping = if let Some(port) = db_port {
        format!("    ports:\n      - \"{}:{}\"", port, db_container_port)
    } else {
        "".to_string()
    };
    
    let web_port_mapping = if let Some(port) = web_port {
        format!("    ports:\n      - \"{}:80\"", port)
    } else {
        "".to_string()
    };
    
    // Define database volume path based on database type
    let db_volume_path = match db_type.to_lowercase().as_str() {
        "mysql" => "/var/lib/mysql",
        "mariadb" => "/var/lib/mysql",
        "postgresql" => "/var/lib/postgresql/data",
        "microsoft sql server" => "/var/opt/mssql/data",
        "oracle" => "/opt/oracle/oradata",
        _ => "/var/lib/mysql", // Default to MySQL
    };
    
    // Map database type to Moodle DB type environment variable
    let moodle_db_type = match db_type.to_lowercase().as_str() {
        "mysql" => "mysqli",
        "mariadb" => "mariadb",
        "postgresql" => "pgsql",
        "microsoft sql server" => "sqlsrv",
        "oracle" => "oci",
        _ => "mysqli", // Default to MySQL
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
      - db_data:{}
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
      - MOODLE_DB_TYPE={}
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
        db_volume_path,
        php_version,
        name,
        moodle_db_type,
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