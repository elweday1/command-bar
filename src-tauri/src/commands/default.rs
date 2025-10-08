use crate::plugins::loader::DynamicPluginLoader;
use crate::plugins::*;
use std::sync::OnceLock;
use tauri::Manager;

static PLUGIN_LOADER: OnceLock<DynamicPluginLoader> = OnceLock::new();

fn get_loader() -> &'static DynamicPluginLoader {
    PLUGIN_LOADER.get_or_init(|| {
        let mut loader = DynamicPluginLoader::new();
        loader.load_all_dynamic_plugins();
        loader
    })
}

#[tauri::command]
pub async fn execute_plugin_action(
    app: tauri::AppHandle,
    plugin_id: String,
    result_id: String,
    action_id: String,
) -> Result<String, String> {
    let result = if let Some(plugin) = get_loader().get_plugin(&plugin_id) {
        plugin.execute_action(&result_id, &action_id)
    } else {
        Err("Plugin not found".to_string())
    };

    // Hide window after action execution
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    result
}

#[tauri::command]
pub fn search_plugin(plugin_id: String, query: String) -> Vec<PluginResult> {
    if let Some(plugin) = get_loader().get_plugin(&plugin_id) {
        plugin.search(&query)
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn get_plugin_info(plugin_id: String) -> Result<Plugin, String> {
    if let Some(plugin) = get_loader().get_plugin(&plugin_id) {
        Ok(plugin.get_info())
    } else {
        Err("Plugin not found".to_string())
    }
}

#[tauri::command]
pub fn list_plugins() -> Vec<Plugin> {
    get_loader().list_plugins()
}

#[tauri::command]
pub fn get_is_window_shown(app: tauri::AppHandle) -> bool {
    if let Some(window) = app.get_webview_window("main") {
        window.is_visible().unwrap_or(false)
    } else {
        false
    }
}

#[tauri::command]
pub fn set_is_window_shown(app: tauri::AppHandle, shown: bool) {
    if let Some(window) = app.get_webview_window("main") {
        if shown {
            let _ = window.show();
            let _ = window.set_focus();
        } else {
            let _ = window.hide();
        }
    }
}
