use crate::plugins::apps::AppsPlugin;
use crate::plugins::files::FilesPlugin;
use crate::plugins::google::GooglePlugin;
use crate::plugins::processes::ProcessesPlugin;
use crate::plugins::youtube::YouTubePlugin;
use crate::plugins::*;
use tauri::Manager;

fn get_plugin(plugin_id: &str) -> Option<Box<dyn PluginTrait>> {
    match plugin_id {
        "processes" => Some(Box::new(ProcessesPlugin)),
        "apps" => Some(Box::new(AppsPlugin)),
        "files" => Some(Box::new(FilesPlugin)),
        "youtube" => Some(Box::new(YouTubePlugin)),
        "google" => Some(Box::new(GooglePlugin)),
        _ => None,
    }
}

#[tauri::command]
pub fn get_all_processes() -> Vec<PluginResult> {
    if let Some(plugin) = get_plugin("processes") {
        plugin.search("")
    } else {
        vec![]
    }
}

#[tauri::command]
pub async fn execute_plugin_action(
    app: tauri::AppHandle,
    plugin_id: String,
    result_id: String,
    action_id: String,
) -> Result<String, String> {
    let result = if let Some(plugin) = get_plugin(&plugin_id) {
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
    if let Some(plugin) = get_plugin(&plugin_id) {
        plugin.search(&query)
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn get_plugin_info(plugin_id: String) -> Result<Plugin, String> {
    if let Some(plugin) = get_plugin(&plugin_id) {
        Ok(plugin.get_info())
    } else {
        Err("Plugin not found".to_string())
    }
}

#[tauri::command]
pub fn toggle_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}
