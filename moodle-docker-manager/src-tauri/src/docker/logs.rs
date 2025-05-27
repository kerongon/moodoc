// Logging functionality for Moodle Docker Manager
// This module contains functions for retrieving Docker container logs using moodle-docker-compose

use std::fs;
use std::process::Command;
use tauri::{command, AppHandle, Manager};

use super::{SiteState, execute_with_docker_compose_wrapper};

// Helper function to normalize database type for docker-compose file naming
fn normalize_db_type(db_type: &str) -> &str {
    match db_type.to_lowercase().as_str() {
        "postgresql" | "postgres" => "pgsql",
        _ => db_type,
    }
}

#[command]
pub fn get_docker_logs(
    app_handle: AppHandle,
    name: String,
    service: String,
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

    // Validate service name
    let valid_services = ["webserver", "db", "mailpit", "selenium", "exttests"];
    if !valid_services.contains(&service.as_str()) {
        return Err(format!(
            "Invalid service '{}'. Valid services are: {}",
            service,
            valid_services.join(", ")
        ));
    }

    let moodle_docker_dir = &site.path;
    let _moodle_docker_dir_str = moodle_docker_dir
        .to_str()
        .ok_or_else(|| "Failed to convert site path to string".to_string())?;

    // MOODLE_DOCKER_WWWROOT should be the absolute path to the moodle subdirectory within moodle_docker_dir
    let moodle_www_root = moodle_docker_dir.join("moodle");
    let moodle_www_root_str = moodle_www_root.to_str().ok_or_else(|| {
        format!(
            "Failed to convert moodle www root path to string. Base path: {:?}, Joined path: {:?}",
            moodle_docker_dir, moodle_www_root
        )
    })?;

    let web_port_str = site.web_port.map_or("8000".to_string(), |p| p.to_string());
    let db_port_str = site.db_port.map_or("3306".to_string(), |p| p.to_string());

    println!("Fetching logs for service '{}' in site '{}'", service, name);

    // Normalize database type for docker-compose consistency
    let normalized_db_type = normalize_db_type(&site.db_type);

    // Create environment script for this operation
    let docker_env_script = format!(
        r#"#!/bin/bash
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT={}
export MOODLE_DOCKER_DB={}
export MOODLE_DOCKER_PHP_VERSION={}
export MOODLE_DOCKER_WEB_PORT={}
export MOODLE_DOCKER_DB_PORT={}
"#,
        name,
        moodle_www_root_str,
        normalized_db_type,
        site.php_version,
        web_port_str,
        db_port_str
    );

    let docker_env_path = site.path.join("docker-env-temp.sh");
    fs::write(&docker_env_path, docker_env_script)
        .map_err(|e| format!("Failed to create temporary environment script: {}", e))?;

    // Make it executable
    Command::new("chmod")
        .args([
            "+x",
            docker_env_path.to_str().unwrap_or("docker-env-temp.sh"),
        ])
        .current_dir(&site.path)
        .output()
        .map_err(|e| format!("Failed to make environment script executable: {}", e))?;

    // Execute logs command with the environment variables
    let logs_cmd = format!(
        r#"source ./docker-env-temp.sh && ./bin/moodle-docker-compose logs --tail=500 --timestamps {}"#,
        service
    );

    let output = execute_with_docker_compose_wrapper(|wrapper_dir| {
        Command::new("sh")
            .arg("-c")
            .arg(&logs_cmd)
            .current_dir(&moodle_docker_dir)
            .env("PATH", format!("{}:{}", wrapper_dir.display(), std::env::var("PATH").unwrap_or_default()))
            .output()
            .map_err(|e| format!("Failed to execute log command: {}", e))
    })?;

    // Clean up the temporary script
    let _ = fs::remove_file(&docker_env_path);

    if output.status.success() {
        let logs = String::from_utf8(output.stdout)
            .map_err(|e| format!("Failed to parse logs from stdout: {}", e))?;

        println!("Successfully fetched logs for {}/{}", name, service);
        Ok(logs)
    } else {
        let stderr_output = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout_output = String::from_utf8_lossy(&output.stdout).to_string();
        let mut error_description = String::new();

        if !stderr_output.is_empty() {
            error_description.push_str(&format!("Stderr: {}", stderr_output));
        }
        if !stdout_output.is_empty() {
            if !error_description.is_empty() {
                error_description.push_str("; ");
            }
            error_description.push_str(&format!("Stdout: {}", stdout_output));
        }

        // If both stderr and stdout are empty, provide a generic message with status
        if error_description.is_empty() {
            error_description = format!(
                "Command failed with status {} but produced no output to stderr or stdout.",
                output.status
            );
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
