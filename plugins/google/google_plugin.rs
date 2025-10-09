#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult};

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "google".to_string(),
        name: "Google Search".to_string(),
        description: "Search Google".to_string(),
        prefix: "g".to_string(),
        icon: "🔍".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    if query.is_empty() {
        return PluginSearchResult::Results(vec![]);
    }

    PluginSearchResult::Results(vec![PluginResult {
        id: query.clone(),
        title: format!("Search Google for '{}'", query),
        subtitle: Some("Open in browser".to_string()),
        icon: Some("🔍".to_string()),
        actions: Some(vec![PluginAction {
            id: "search".to_string(),
            label: "Search".to_string(),
            shortcut: Some("Enter".to_string()),
        }]),
    }])
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(result_id: String, action_id: String) -> Result<String, String> {
    match action_id.as_str() {
        "search" => {
            let encoded_query = urlencoding::encode(&result_id);
            let url = format!("https://www.google.com/search?q={}", encoded_query);
            if let Err(e) = opener::open(&url) {
                return Err(format!("Failed to open URL: {}", e));
            }
            Ok("Opened Google search".to_string())
        }
        _ => Err("Unknown action".to_string()),
    }
}