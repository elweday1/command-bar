use super::{Plugin, PluginAction, PluginResult, PluginTrait};
use tauri_plugin_opener::open_url;

pub struct YouTubePlugin;
impl PluginTrait for YouTubePlugin {
    fn get_info(&self) -> Plugin {
        Plugin {
            id: "youtube".to_string(),
            name: "YouTube Search".to_string(),
            description: "Search YouTube".to_string(),
            prefix: "yt".to_string(),
            icon: "📺".to_string(),
            config: None,
        }
    }

    fn search(&self, query: &str) -> Vec<PluginResult> {
        if query.is_empty() {
            return vec![];
        }

        vec![PluginResult {
            id: query.to_string(),
            title: format!("Search YouTube for '{}'", query),
            subtitle: Some("Open in browser".to_string()),
            icon: Some("📺".to_string()),
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
                let url = format!("https://www.youtube.com/results?search_query={}", encoded_query);
                let _ = open_url(url, None::<&str>);
                Ok("Opened YouTube search".to_string())
            }
            _ => Err("Unknown action".to_string()),
        }
    }
}