#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult};

use arboard::Clipboard;

fn parse_torrent_line(line: &str) -> (String, String) {
    // Format: "164/11 - Superman (2025) UHDR+DV cz en.mkv (TC, 5.8 GiB)"
    let parts: Vec<&str> = line.splitn(2, " - ").collect();
    if parts.len() != 2 {
        return (
            line.to_string(),
            "Seeds: Unknown • Peers: Unknown • Size: Unknown".to_string(),
        );
    }

    let seeders_leechers = parts[0];
    let rest = parts[1];

    // Extract seeders/leechers
    let (seeders, leechers) = if let Some(slash_pos) = seeders_leechers.find('/') {
        let seeders = &seeders_leechers[..slash_pos];
        let leechers = &seeders_leechers[slash_pos + 1..];
        (seeders, leechers)
    } else {
        ("Unknown", "Unknown")
    };

    // Extract size from end of line
    let size = if let Some(start) = rest.rfind('(') {
        if let Some(end) = rest.rfind(')') {
            let size_part = &rest[start + 1..end];
            if let Some(comma_pos) = size_part.find(',') {
                size_part[comma_pos + 1..].trim()
            } else {
                "Unknown"
            }
        } else {
            "Unknown"
        }
    } else {
        "Unknown"
    };

    // Extract title (everything before the last parentheses)
    let title = if let Some(paren_pos) = rest.rfind('(') {
        rest[..paren_pos].trim()
    } else {
        rest
    };

    let subtitle = format!("Seeds: {} • Peers: {} • Size: {}", seeders, leechers, size);
    (title.to_string(), subtitle)
}

fn parse_torrent_output(output: &str) -> Vec<PluginResult> {
    let mut results = Vec::new();

    if output.trim().is_empty() {
        return results;
    }

    let lines: Vec<&str> = output.lines().collect();
    let mut current_name: Option<String> = None;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("magnet:") {
            if let Some(name) = current_name.take() {
                let (title, subtitle) = parse_torrent_line(&name);
                results.push(PluginResult {
                    id: line.to_string(),
                    title,
                    subtitle: Some(subtitle),
                    icon: Some("🧲".to_string()),
                    actions: Some(vec![PluginAction {
                        id: "copy_magnet".to_string(),
                        label: "Copy Magnet".to_string(),
                        shortcut: Some("Enter".to_string()),
                    }]),
                });
            }
        } else {
            current_name = Some(line.to_string());
        }
    }

    results
}

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "torrent".to_string(),
        name: "Torrent Search".to_string(),
        description: "Search for torrents using Attractorr".to_string(),
        prefix: "torrent".to_string(),
        icon: "🔍".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    if query.is_empty() {
        return PluginSearchResult::Results(vec![]);
    }

    use std::process::Command;

    let output = Command::new("attractorr").arg(&query).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let results = parse_torrent_output(&stdout);
                PluginSearchResult::Results(results)
            } else {
                PluginSearchResult::Results(vec![])
            }
        }
        Err(_) => PluginSearchResult::Results(vec![]),
    }
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(
    result_id: String,
    action_id: String,
) -> Result<String, String> {
    match action_id.as_str() {
        "copy_magnet" => match Clipboard::new() {
            Ok(mut clipboard) => match clipboard.set_text(&result_id) {
                Ok(_) => Ok("Magnet link copied to clipboard".to_string()),
                Err(e) => Err(format!("Failed to copy magnet link: {}", e)),
            },
            Err(e) => Err(format!("Failed to access clipboard: {}", e)),
        },
        _ => Err("Unknown action".to_string()),
    }
}
