#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult};
use app_finder::{AppCommon, AppFinder};
use std::process::Command;
use std::sync::OnceLock;
use tokio::task;

static APPS_CACHE: OnceLock<Vec<app_finder::App>> = OnceLock::new();

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "apps".to_string(),
        name: "Applications".to_string(),
        description: "Launch applications".to_string(),
        prefix: "app".to_string(),
        icon: "🚀".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    let apps = APPS_CACHE.get_or_init(|| AppFinder::list());
    let filtered_apps: Vec<_> = apps
        .iter()
        .filter(|app| {
            if query.is_empty() {
                true
            } else {
                app.name.to_lowercase().contains(&query.to_lowercase())
            }
        })
        .take(20)
        .collect();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let tasks: Vec<_> = filtered_apps
            .into_iter()
            .map(|app| {
                let app_clone = app.clone();
                task::spawn_blocking(move || {
                    let icon = app_clone
                        .get_app_icon_base64(32)
                        .ok()
                        .unwrap_or_else(|| "🚀".to_string());

                    PluginResult {
                        id: app_clone.path.to_string(),
                        title: app_clone.name.clone(),
                        subtitle: Some("Application".to_string()),
                        icon: Some(icon),
                        actions: Some(vec![PluginAction {
                            id: "open".to_string(),
                            label: "Open".to_string(),
                            shortcut: Some("Enter".to_string()),
                        }]),
                    }
                })
            })
            .collect();

        let mut results = Vec::new();
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }
        PluginSearchResult::Results(results)
    })
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(result_id: String, action_id: String) -> Result<String, String> {
    match action_id.as_str() {
        "open" => {
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                Command::new(&result_id)
                    .creation_flags(0x08000000)
                    .spawn()
                    .map_err(|e| format!("Failed to open app: {}", e))?;
            }
            Ok("App launched".to_string())
        }
        _ => Err("Unknown action".to_string()),
    }
}