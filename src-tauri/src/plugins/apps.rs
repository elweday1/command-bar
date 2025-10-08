use super::{Plugin, PluginAction, PluginResult, PluginTrait};
use app_finder::{AppCommon, AppFinder};
use std::process::Command;
use std::sync::OnceLock;
use tokio::task;

static APPS_CACHE: OnceLock<Vec<app_finder::App>> = OnceLock::new();

pub struct AppsPlugin;

impl PluginTrait for AppsPlugin {
    fn get_info(&self) -> Plugin {
        Plugin {
            id: "apps".to_string(),
            name: "Applications".to_string(),
            description: "Launch applications".to_string(),
            prefix: "app".to_string(),
            icon: "🚀".to_string(),
            config: None,
        }
    }

    fn search(&self, query: &str) -> Vec<PluginResult> {
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
            results
        })
    }

    fn execute_action(&self, result_id: &str, action_id: &str) -> Result<String, String> {
        match action_id {
            "open" => {
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    Command::new(result_id)
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW
                        .spawn()
                        .map_err(|e| format!("Failed to open app: {}", e))?;
                }
                Ok("App launched".to_string())
            }
            _ => Err("Unknown action".to_string()),
        }
    }
}
