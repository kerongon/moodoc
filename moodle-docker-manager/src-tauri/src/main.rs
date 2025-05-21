#![cfg_attr(
  all(not(debug_assertions), target_os = "macos"),
  windows_subsystem = "windows"
)]

mod docker;

use tauri::{generate_handler, CustomMenuItem, Menu, MenuItem, Submenu};
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    // Create the application menu (macOS style)
    let app_menu = Submenu::new("Moodle Docker Manager", Menu::new()
        .add_native_item(MenuItem::About(
            "Moodle Docker Manager".to_string(),
            tauri::AboutMetadata::default()
        ))
        .add_item(CustomMenuItem::new("preferences".to_string(), "Preferences...").accelerator("Cmd+,"))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit)
    );

    // Create edit menu with standard actions
    let edit_menu = Submenu::new("Edit", Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll)
    );

    // Combine all menus
    let menu = Menu::new()
        .add_submenu(app_menu)
        .add_submenu(edit_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "preferences" => {
                    let window = event.window();
                    // Emit event to show the settings modal
                    window.emit("open-settings", {}).unwrap();
                }
                _ => {}
            }
        })
        .manage(docker::SiteState(Mutex::new(HashMap::new())))
        .manage(docker::RunningUpdates::default())
        .setup(|app| {
            // Load any existing site configurations
            if let Err(err) = docker::load_site_configs(app) {
                eprintln!("Failed to load site configurations: {}", err);
            }
            Ok(())
        })
        .invoke_handler(generate_handler![
            docker::check_docker_installed,
            docker::check_docker_running,
            docker::check_platform_compatibility,
            docker::validate_environment,
            docker::create_site,
            docker::list_sites,
            docker::start_site,
            docker::stop_site,
            docker::remove_site,
            docker::get_docker_logs,
            docker::check_site_status,
            docker::backup_site,
            docker::list_site_backups,
            docker::restore_site_backup,
            docker::delete_site_backup,
            docker::open_backup_location,
            docker::update_moodle_version,
            docker::open_site_folder,
            docker::cancel_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}