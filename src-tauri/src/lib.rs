use tauri::State;
mod anonsurf;
mod iptables;
mod stealth;
mod stats;

use anonsurf::{Anonsurf, AnonsurfState};
use tauri::AppHandle;


#[tauri::command]
fn start_anonsurf(app: AppHandle, state: State<AnonsurfState>) -> Result<String, String> {
    match Anonsurf::start(app) {
        Ok(_) => {
            *state.is_active.lock().unwrap() = true;
            Ok("Anonsurf started successfully".to_string())
        }
        Err(e) => Err(format!("Error starting Anonsurf: {}", e))
    }
}

#[tauri::command]
fn stop_anonsurf(state: State<AnonsurfState>) -> Result<String, String> {
    match Anonsurf::stop() {
        Ok(_) => {
            *state.is_active.lock().unwrap() = false;
            Ok("Anonsurf stopped successfully".to_string())
        }
        Err(e) => Err(format!("Error stopping Anonsurf: {}", e))
    }
}

#[tauri::command]
fn refresh_identity() -> Result<String, String> {
    match Anonsurf::new_identity() {
        Ok(_) => Ok("New identity requested".to_string()),
        Err(e) => Err(format!("Error requesting new identity: {}", e))
    }
}

// --- Stealth Commands ---

#[tauri::command]
fn restore_mac(interface: String) -> Result<String, String> {
    match stealth::restore_mac(&interface) {
        Ok(_) => Ok("MAC Address restored".to_string()),
        Err(e) => Err(format!("Failed to restore MAC: {}", e))
    }
}

#[tauri::command]
fn spoof_mac(interface: String) -> Result<String, String> {
    match stealth::spoof_mac(&interface) {
        Ok(_) => Ok("MAC Address spoofed".to_string()),
        Err(e) => Err(format!("Failed to spoof MAC: {}", e))
    }
}

#[tauri::command]
fn randomize_hostname() -> Result<String, String> {
    match stealth::randomize_hostname() {
        Ok(new_name) => Ok(format!("Hostname changed to {}", new_name)),
        Err(e) => Err(format!("Failed to change hostname: {}", e))
    }
}

#[tauri::command]
fn wipe_ram() -> Result<String, String> {
    match stealth::wipe_ram() {
        Ok(_) => Ok("RAM/Cache wiped".to_string()),
        Err(e) => Err(format!("Failed to wipe RAM: {}", e))
    }
}

#[tauri::command]
fn set_utc() -> Result<String, String> {
    match stealth::set_utc_timezone() {
        Ok(_) => Ok("Timezone set to UTC".to_string()),
        Err(e) => Err(format!("Failed to set UTC: {}", e))
    }
}

#[tauri::command]
fn clean_logs() -> Result<String, String> {
    match stealth::clean_logs() {
        Ok(_) => Ok("System logs truncated".to_string()),
        Err(e) => Err(format!("Failed to clean logs: {}", e))
    }
}

#[derive(serde::Serialize)]
struct SystemIdentity {
    mac: String,
    hostname: String,
}

#[tauri::command]
fn get_system_identity(interface: String) -> SystemIdentity {
    SystemIdentity {
        mac: stealth::get_mac(&interface),
        hostname: stealth::get_hostname(),
    }
}

#[tauri::command]
fn check_status(state: State<AnonsurfState>) -> bool {
    *state.is_active.lock().unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Start traffic monitoring
            stats::start_traffic_monitor(app.handle().clone());
            Ok(())
        })
        .manage(AnonsurfState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_anonsurf,
            stop_anonsurf,
            refresh_identity,
            check_status,
            spoof_mac,
            restore_mac,
            randomize_hostname,
            wipe_ram,
            set_utc,
            clean_logs,
            get_system_identity
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
