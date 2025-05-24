// Logging functionality for Moodle Docker Manager
// This module contains functions for retrieving and managing Docker container logs

use std::process::Command;
use tauri::{command, AppHandle, Manager};

use super::{SiteState};

#[command]
pub fn get_docker_logs(app_handle: AppHandle, name: String, service: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let _site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Validate service name
    let valid_services = ["webserver", "db", "mailhog"];
    if !valid_services.contains(&service.as_str()) {
        return Err(format!("Invalid service '{}'. Valid services are: {}", service, valid_services.join(", ")));
    }
    
    // Get logs for the specified service
    let container_name = format!("{}_{}_1", name, service);
    
    let output = Command::new("docker")
        .args([
            "logs", 
            "--tail", "500", // Get last 500 lines
            "--timestamps",  // Include timestamps
            &container_name
        ])
        .output()
        .map_err(|e| format!("Failed to get Docker logs: {}", e))?;
    
    if output.status.success() {
        let logs = String::from_utf8_lossy(&output.stdout);
        Ok(logs.to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        // If the container doesn't exist, provide a helpful message
        if error.contains("No such container") {
            Err(format!("Container '{}' is not running or does not exist", container_name))
        } else {
            Err(format!("Failed to get logs for container '{}': {}", container_name, error))
        }
    }
}

#[command]
pub fn get_docker_logs_follow(app_handle: AppHandle, name: String, service: String, lines: Option<u32>) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let _site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Validate service name
    let valid_services = ["webserver", "db", "mailhog"];
    if !valid_services.contains(&service.as_str()) {
        return Err(format!("Invalid service '{}'. Valid services are: {}", service, valid_services.join(", ")));
    }
    
    let container_name = format!("{}_{}_1", name, service);
    let tail_lines = lines.unwrap_or(100).to_string();
    
    // Get logs with follow option (for real-time streaming)
    let output = Command::new("docker")
        .args([
            "logs", 
            "--tail", &tail_lines,
            "--timestamps",
            "--follow",
            &container_name
        ])
        .output()
        .map_err(|e| format!("Failed to get Docker logs: {}", e))?;
    
    if output.status.success() {
        let logs = String::from_utf8_lossy(&output.stdout);
        Ok(logs.to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        if error.contains("No such container") {
            Err(format!("Container '{}' is not running or does not exist", container_name))
        } else {
            Err(format!("Failed to get logs for container '{}': {}", container_name, error))
        }
    }
}

#[command]
pub fn get_all_service_logs(app_handle: AppHandle, name: String) -> Result<Vec<(String, String)>, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let _site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    let services = ["webserver", "db", "mailhog"];
    let mut all_logs = Vec::new();
    
    for service in &services {
        match get_docker_logs(app_handle.clone(), name.clone(), service.to_string()) {
            Ok(logs) => {
                all_logs.push((service.to_string(), logs));
            }
            Err(_) => {
                // If we can't get logs for a service, include a note about it
                all_logs.push((service.to_string(), format!("No logs available for {} service", service)));
            }
        }
    }
    
    Ok(all_logs)
}

#[command]
pub fn clear_docker_logs(app_handle: AppHandle, name: String, service: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let _site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Validate service name
    let valid_services = ["webserver", "db", "mailhog"];
    if !valid_services.contains(&service.as_str()) {
        return Err(format!("Invalid service '{}'. Valid services are: {}", service, valid_services.join(", ")));
    }
    
    let container_name = format!("{}_{}_1", name, service);
    
    // Note: Docker doesn't have a built-in command to clear logs
    // This is a workaround that restarts the container to clear logs
    let output = Command::new("docker")
        .args(["restart", &container_name])
        .output()
        .map_err(|e| format!("Failed to restart container to clear logs: {}", e))?;
    
    if output.status.success() {
        Ok(format!("Logs cleared for {} service (container restarted)", service))
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to clear logs for {}: {}", service, error))
    }
}

// Helper function to get container status
pub fn get_container_status(name: &str, service: &str) -> Result<String, String> {
    let container_name = format!("{}_{}_1", name, service);
    
    let output = Command::new("docker")
        .args(["inspect", "--format", "{{.State.Status}}", &container_name])
        .output()
        .map_err(|e| format!("Failed to get container status: {}", e))?;
    
    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(status)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        if error.contains("No such object") {
            Ok("not_found".to_string())
        } else {
            Err(format!("Failed to get container status: {}", error))
        }
    }
}

// Helper function to check if container is running
pub fn is_container_running(name: &str, service: &str) -> bool {
    match get_container_status(name, service) {
        Ok(status) => status == "running",
        Err(_) => false,
    }
}
