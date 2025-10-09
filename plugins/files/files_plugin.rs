#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult};
use std::process::Command;

#[cfg(target_os = "windows")]
#[link(name = "Everything64", kind = "dylib")]
extern "C" {
    fn Everything_SetSearchW(search: *const u16);
    fn Everything_SetMax(max: u32);
    fn Everything_QueryW(wait: i32) -> i32;
    fn Everything_GetNumResults() -> u32;
    fn Everything_GetResultFullPathNameW(index: u32, buf: *mut u16, buf_size: u32) -> u32;
    fn Everything_GetResultSize(index: u32, size: *mut i64) -> bool;
    fn Everything_Reset();
    fn Everything_SetRequestFlags(flags: u32);
}

struct SearchResult {
    filepath: String,
    size: i64,
}

fn search_everything(query: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();

    #[cfg(target_os = "windows")]
    unsafe {
        let wide_query: Vec<u16> = query.encode_utf16().chain(std::iter::once(0)).collect();

        Everything_SetSearchW(wide_query.as_ptr());
        Everything_SetRequestFlags(0x00000001 | 0x00000002 | 0x00000010);
        Everything_SetMax(50);

        if Everything_QueryW(1) == 0 {
            Everything_Reset();
            return results;
        }

        let num_results = Everything_GetNumResults();

        for i in 0..num_results {
            let mut size: i64 = 0;
            let mut path_buf: Vec<u16> = vec![0; 260];

            Everything_GetResultSize(i, &mut size);
            Everything_GetResultFullPathNameW(i, path_buf.as_mut_ptr(), path_buf.len() as u32);

            let null_pos = path_buf
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(path_buf.len());
            let filepath = String::from_utf16_lossy(&path_buf[..null_pos]);

            results.push(SearchResult { filepath, size });
        }

        Everything_Reset();
    }

    results
}

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "files".to_string(),
        name: "Files".to_string(),
        description: "Search files with Everything".to_string(),
        prefix: "f".to_string(),
        icon: "📁".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    if query.is_empty() {
        return PluginSearchResult::Results(vec![]);
    }

    let results = search_everything(&query);

    let plugin_results = results
        .iter()
        .map(|result| {
            let filename = std::path::Path::new(&result.filepath)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let size_str = if result.size > 1024 * 1024 {
                format!("{:.1} MB", result.size as f64 / (1024.0 * 1024.0))
            } else if result.size > 1024 {
                format!("{:.1} KB", result.size as f64 / 1024.0)
            } else {
                format!("{} B", result.size)
            };

            PluginResult {
                id: result.filepath.clone(),
                title: filename,
                subtitle: Some(format!("{} • {}", result.filepath, size_str)),
                icon: Some("📄".to_string()),
                actions: Some(vec![
                    PluginAction {
                        id: "open".to_string(),
                        label: "Open".to_string(),
                        shortcut: Some("Enter".to_string()),
                    },
                    PluginAction {
                        id: "open_folder".to_string(),
                        label: "Open Folder".to_string(),
                        shortcut: Some("Ctrl+O".to_string()),
                    },
                ]),
            }
        })
        .collect();

    PluginSearchResult::Results(plugin_results)
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(
    result_id: String,
    action_id: String,
) -> Result<String, String> {
    match action_id.as_str() {
        "open" => {
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd")
                    .args(["/c", "start", "", &result_id])
                    .spawn()
                    .map_err(|e| format!("Failed to open file: {}", e))?;
            }
            Ok("File opened".to_string())
        }
        "open_folder" => {
            #[cfg(target_os = "windows")]
            {
                Command::new("explorer")
                    .arg("/select,")
                    .arg(&result_id)
                    .spawn()
                    .map_err(|e| format!("Failed to open folder: {}", e))?;
            }
            Ok("Folder opened".to_string())
        }
        _ => Err("Unknown action".to_string()),
    }
}
