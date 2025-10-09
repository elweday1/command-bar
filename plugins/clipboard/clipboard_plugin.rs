#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult};

use arboard::Clipboard;
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct ClipboardEntry {
    content: String,
    timestamp: DateTime<Local>,
}

static CLIPBOARD_HISTORY: Mutex<VecDeque<ClipboardEntry>> = Mutex::new(VecDeque::new());
const MAX_HISTORY: usize = 100;

fn add_to_history(content: String) {
    if let Ok(mut history) = CLIPBOARD_HISTORY.lock() {
        // Don't add if it's already the most recent entry
        if history.front().map_or(true, |entry| entry.content != content) {
            let entry = ClipboardEntry {
                content,
                timestamp: Local::now(),
            };
            history.push_front(entry);
            if history.len() > MAX_HISTORY {
                history.pop_back();
            }
        }
    }
}

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "clipboard".to_string(),
        name: "Clipboard".to_string(),
        description: "Manage clipboard history and operations".to_string(),
        prefix: "c".to_string(),
        icon: "📋".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    let mut results = Vec::new();
    
    // Auto-add current clipboard to history if not empty
    if let Ok(mut clipboard) = Clipboard::new() {
        if let Ok(current) = clipboard.get_text() {
            if !current.trim().is_empty() {
                add_to_history(current.clone());
            }
        }
    }

    // Add current clipboard content
    if let Ok(mut clipboard) = Clipboard::new() {
        if let Ok(current) = clipboard.get_text() {
            if query.is_empty() || current.to_lowercase().contains(&query.to_lowercase()) {
                results.push(PluginResult {
                    id: current.clone(),
                    title: format!("📋 {}", truncate(&current, 50)),
                    subtitle: Some("Current clipboard".to_string()),
                    icon: None,
                    actions: Some(vec![
                        PluginAction {
                            id: "copy".to_string(),
                            label: "Copy".to_string(),
                            shortcut: Some("Enter".to_string()),
                        },
                        PluginAction {
                            id: "clear".to_string(),
                            label: "Clear".to_string(),
                            shortcut: Some("Ctrl+D".to_string()),
                        },
                    ]),
                });
            }
        }
    }

    // Add history entries
    if let Ok(history) = CLIPBOARD_HISTORY.lock() {
        for entry in history.iter() {
            if query.is_empty() || entry.content.to_lowercase().contains(&query.to_lowercase()) {
                results.push(PluginResult {
                    id: entry.content.clone(),
                    title: format!("🕒 {}", truncate(&entry.content, 50)),
                    subtitle: Some(format!("Copied {}", format_time_ago(&entry.timestamp))),
                    icon: None,
                    actions: Some(vec![PluginAction {
                        id: "copy".to_string(),
                        label: "Copy".to_string(),
                        shortcut: Some("Enter".to_string()),
                    }]),
                });
            }
        }
    }
    results.truncate(20);
    PluginSearchResult::Results(results)
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(
    result_id: String,
    action_id: String,
) -> Result<String, String> {
    match action_id.as_str() {
        "copy" => {
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    match clipboard.set_text(&result_id) {
                        Ok(_) => {
                            add_to_history(result_id);
                            Ok("Copied to clipboard".to_string())
                        }
                        Err(e) => Err(format!("Failed to copy: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to access clipboard: {}", e)),
            }
        }
        "clear" => match Clipboard::new() {
            Ok(mut clipboard) => match clipboard.clear() {
                Ok(_) => Ok("Clipboard cleared".to_string()),
                Err(e) => Err(format!("Failed to clear clipboard: {}", e)),
            },
            Err(e) => Err(format!("Failed to access clipboard: {}", e)),
        },
        _ => Err("Unknown action".to_string()),
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

fn format_time_ago(timestamp: &DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(*timestamp);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} min ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hr ago", duration.num_hours())
    } else {
        format!("{} days ago", duration.num_days())
    }
}
