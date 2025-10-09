use crate::constants::get_settings_path;
use serde_json::Value;
use std::fs;
use tauri::{Emitter, Manager};

#[tauri::command]
pub fn get_settings() -> Result<Value, String> {
    let settings_path = get_settings_path();

    if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse settings: {}", e))
    } else {
        Ok(serde_json::json!({}))
    }
}

#[tauri::command]
pub fn set_settings(settings: Value, app: tauri::AppHandle) -> Result<(), String> {
    let settings_path = get_settings_path();

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
pub fn update_shortcuts(app: tauri::AppHandle) -> Result<(), String> {
    app.emit("shortcuts-changed", ())
        .map_err(|e| format!("Failed to emit shortcuts event: {}", e))?;
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
                .inner_size(500.0, 400.0)
                .resizable(false)
                .center()
                .decorations(true)
                .transparent(false)
                .build()
                .map_err(|e| format!("Failed to create settings window: {}", e))?;
    }
    Ok(())
}
