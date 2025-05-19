#![cfg_attr(
  all(not(debug_assertions), target_os = "macos"),
  windows_subsystem = "windows"
)]

mod docker;

use tauri::generate_handler;
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .manage(docker::SiteState(Mutex::new(HashMap::new())))
        .invoke_handler(generate_handler![
            docker::check_docker_installed,
            docker::check_docker_running,
            docker::create_site,
            docker::list_sites,
            docker::start_site,
            docker::stop_site,
            docker::remove_site,
            docker::get_docker_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}