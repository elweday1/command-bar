use super::{Plugin, PluginAction, PluginResult, PluginTrait};
use tauri_plugin_opener::open_url;

pub struct GooglePlugin;
impl PluginTrait for GooglePlugin {
    fn get_info(&self) -> Plugin {
        Plugin {
            id: "google".to_string(),
            name: "Google Search".to_string(),
            description: "Search Google".to_string(),
            prefix: "g".to_string(),
            icon: "🔍".to_string(),
            config: None,
        }
    }

    fn search(&self, query: &str) -> Vec<PluginResult> {
        if query.is_empty() {
            return vec![];
        }

        vec![PluginResult {
            id: query.to_string(),
            title: format!("Search Google for '{}'", query),
            subtitle: Some("Open in browser".to_string()),
            icon: Some("🔍".to_string()),
            actions: Some(vec![PluginAction {
                id: "search".to_string(),
                label: "Search".to_string(),
                shortcut: Some("Enter".to_string()),
            }]),
        }]
    }

    fn execute_action(&self, result_id: &str, action_id: &str) -> Result<String, String> {
        match action_id {
            "search" => {
                let encoded_query = urlencoding::encode(result_id);
                let url = format!("https://www.google.com/search?q={}", encoded_query);
                let _ = open_url(url, None::<&str>);
                Ok("Opened Google search".to_string())
            }
            _ => Err("Unknown action".to_string()),
        }
    }
}
