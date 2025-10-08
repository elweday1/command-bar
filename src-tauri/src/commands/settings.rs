use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{Emitter, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub transparency: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self { transparency: 0.8 }
    }
}

fn get_settings_path() -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|home| home.join(".command-bar-settings.json"))
        .ok_or_else(|| "Could not find home directory".to_string())
}

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    let settings_path = get_settings_path()?;

    if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse settings: {}", e))
    } else {
        Ok(Settings::default())
    }
}

#[tauri::command]
pub fn set_settings(settings: Settings, app: tauri::AppHandle) -> Result<(), String> {
    let settings_path = get_settings_path()?;

    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&settings_path, content)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    // Emit settings changed event
    app.emit("settings-changed", &settings)
        .map_err(|e| format!("Failed to emit settings event: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn open_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        // Create settings window if it doesn't exist
        use tauri::{WebviewUrl, WebviewWindowBuilder};
        let _window =
            WebviewWindowBuilder::new(&app, "settings", WebviewUrl::App("/settings".into()))
                .title("Command Bar Settings")
                .inner_size(400.0, 300.0)
                .resizable(false)
                .center()
                .decorations(true)
                .transparent(false)
                .build()
                .map_err(|e| format!("Failed to create settings window: {}", e))?;
    }
    Ok(())
}
