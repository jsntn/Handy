use log::{debug, info};
use std::env;
use std::path::PathBuf;
use tauri::AppHandle;

/// Determines if the application should run in portable mode.
/// 
/// Portable mode is enabled if:
/// 1. Environment variable HANDY_PORTABLE=1 is set, OR
/// 2. A file named `.portable` exists next to the executable
/// 
/// In portable mode, all app data is stored relative to the executable
/// in a `data/` subdirectory, making the app fully portable.
pub fn is_portable_mode(app: &AppHandle) -> bool {
    // Check environment variable first
    if env::var("HANDY_PORTABLE").unwrap_or_default() == "1" {
        debug!("Portable mode enabled via HANDY_PORTABLE environment variable");
        return true;
    }

    // Check for .portable marker file next to executable
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let marker_path = exe_dir.join(".portable");
            if marker_path.exists() {
                debug!("Portable mode enabled via .portable marker file");
                return true;
            }
        }
    }

    // Check Tauri's resource directory (for development)
    if let Ok(resource_dir) = app.path().resource_dir() {
        let marker_path = resource_dir.join(".portable");
        if marker_path.exists() {
            debug!("Portable mode enabled via .portable marker in resource dir");
            return true;
        }
    }

    false
}

/// Gets the application data directory.
/// 
/// In portable mode: returns `{exe_dir}/data/`
/// In normal mode: returns the platform-specific app data directory
pub fn get_app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    if is_portable_mode(app) {
        // Portable mode: use data directory next to executable
        let exe_path = env::current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;
        
        let exe_dir = exe_path
            .parent()
            .ok_or_else(|| "Failed to get executable directory".to_string())?;
        
        let data_dir = exe_dir.join("data");
        
        // Create data directory if it doesn't exist
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)
                .map_err(|e| format!("Failed to create data directory: {}", e))?;
            info!("Created portable data directory: {:?}", data_dir);
        }
        
        Ok(data_dir)
    } else {
        // Normal mode: use platform-specific app data directory
        app.path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))
    }
}

/// Gets the log directory.
/// 
/// In portable mode: returns `{exe_dir}/data/logs/`
/// In normal mode: returns the platform-specific log directory
pub fn get_log_dir(app: &AppHandle) -> Result<PathBuf, String> {
    if is_portable_mode(app) {
        let data_dir = get_app_data_dir(app)?;
        let log_dir = data_dir.join("logs");
        
        if !log_dir.exists() {
            std::fs::create_dir_all(&log_dir)
                .map_err(|e| format!("Failed to create log directory: {}", e))?;
        }
        
        Ok(log_dir)
    } else {
        app.path()
            .app_log_dir()
            .map_err(|e| format!("Failed to get log directory: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portable_env_var() {
        env::set_var("HANDY_PORTABLE", "1");
        // Note: This test would need a proper AppHandle mock to fully test
        env::remove_var("HANDY_PORTABLE");
    }
}
