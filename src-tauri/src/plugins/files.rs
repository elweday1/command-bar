use super::{Plugin, PluginAction, PluginResult, PluginTrait};
use std::process::Command;

#[cfg(target_os = "windows")]
#[link(
    name = "O:\\command-bar\\src-tauri\\target\\debug\\Everything64",
    kind = "dylib"
)]
extern "C" {
    fn Everything_SetSearchW(search: *const u16);
    fn Everything_SetMax(max: u32);
    fn Everything_QueryW(wait: i32) -> i32;
    fn Everything_GetNumResults() -> u32;
    fn Everything_GetResultFullPathNameW(index: u32, buf: *mut u16, buf_size: u32) -> u32;
    fn Everything_IsFileResult(index: u32) -> i32;
    fn Everything_GetResultDateModified(index: u32, date_modified: *mut i64) -> bool;
    fn Everything_GetResultSize(index: u32, size: *mut i64) -> bool;
    fn Everything_Reset();
    fn Everything_SetRequestFlags(flags: u32);
}

pub struct SearchResult {
    pub filepath: String,
    pub size: i64,
    pub date_modified: i64, // Stored as a Windows FILETIME i64
}

pub fn search_everything(query: &str) -> Vec<SearchResult> {
    // A vector to store the results
    let mut search_results: Vec<SearchResult> = Vec::new();

    // The FFI calls are inherently unsafe as they interact with C code
    unsafe {
        // 1. Convert the Rust &str (UTF-8) to a null-terminated UTF-16 string for the Windows API.
        let wide_query: Vec<u16> = query.encode_utf16().chain(std::iter::once(0)).collect();

        Everything_SetSearchW(wide_query.as_ptr());
        Everything_SetRequestFlags(0x00000001 | 0x00000002 | 0x00000010 | 0x00000020); // EVERYTHING_REQUEST_FILE_NAME | EVERYTHING_REQUEST_PATH | EVERYTHING_REQUEST_SIZE | EVERYTHING_REQUEST_DATE_MODIFIED
        Everything_SetMax(1000);

        // 3. Execute the query. The parameter '1' means to wait for the results.
        if Everything_QueryW(1) == 0 {
            // 0 indicates failure
            eprintln!("Everything query failed.");
            Everything_Reset(); // Clean up
            return search_results; // Return empty vector
        }

        let num_results = Everything_GetNumResults();

        // 5. Loop through each result to retrieve its details
        for i in 0..num_results {
            let mut size: i64 = 0;
            let mut date_modified: i64 = 0;

            // A buffer for the file path. MAX_PATH (260) is a common size.
            let mut path_buf: Vec<u16> = vec![0; 260];

            // 6. Get file size and modified date - check return values
            let size_ok = Everything_GetResultSize(i, &mut size);
            let date_ok = Everything_GetResultDateModified(i, &mut date_modified);

            // Debug: print the actual values
            if !size_ok {
                size = -1;
            }
            if !date_ok {
                date_modified = -1;
            }

            // 7. Get the full path of the file
            Everything_GetResultFullPathNameW(i, path_buf.as_mut_ptr(), path_buf.len() as u32);

            // 8. Convert the UTF-16 path buffer back to a Rust String.
            // Find the position of the first null terminator.
            let null_pos = path_buf
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(path_buf.len());
            let filepath = String::from_utf16_lossy(&path_buf[..null_pos]);

            // 9. Add the result to our vector
            search_results.push(SearchResult {
                filepath,
                size,
                date_modified,
            });
        }

        // 10. Clean up and reset the Everything state for the next query.
        Everything_Reset();
    }

    search_results
}

pub struct FilesPlugin;

impl PluginTrait for FilesPlugin {
    fn get_info(&self) -> Plugin {
        Plugin {
            id: "files".to_string(),
            name: "Files".to_string(),
            description: "Search files with Everything".to_string(),
            prefix: "f".to_string(),
            icon: "📁".to_string(),
            config: None,
        }
    }

    fn search(&self, query: &str) -> Vec<PluginResult> {
        if query.is_empty() {
            return vec![];
        }
        let results = search_everything(query);

        results
            .iter()
            .filter(|result| result.size > 0)
            .map(|result| {
                let size_str = if result.size >= 0 {
                    if result.size == 0 {
                        "0 B".to_string()
                    } else if result.size > 1024 * 1024 * 1024 {
                        format!("{:.1} GB", result.size as f64 / (1024.0 * 1024.0 * 1024.0))
                    } else if result.size > 1024 * 1024 {
                        format!("{:.1} MB", result.size as f64 / (1024.0 * 1024.0))
                    } else if result.size > 1024 {
                        format!("{:.1} KB", result.size as f64 / 1024.0)
                    } else {
                        format!("{} B", result.size)
                    }
                } else {
                    "Unknown".to_string()
                };

                let filename = std::path::Path::new(&result.filepath)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let date_str = if result.date_modified > 0 {
                    let timestamp = (result.date_modified as u64 - 116444736000000000) / 10000000;
                    let dt = std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
                    let system_time: std::time::SystemTime = dt;
                    match system_time.duration_since(std::time::UNIX_EPOCH) {
                        Ok(duration) => {
                            let secs = duration.as_secs();
                            let days = secs / 86400;
                            let hours = (secs % 86400) / 3600;
                            let minutes = (secs % 3600) / 60;
                            format!("{} days ago", days)
                        }
                        Err(_) => "Unknown".to_string(),
                    }
                } else {
                    "Unknown".to_string()
                };

                let truncated_path = if result.filepath.chars().count() > 80 {
                    let chars: Vec<char> = result.filepath.chars().collect();
                    if chars.len() > 60 {
                        let start: String = chars.iter().take(30).collect();
                        let end: String = chars.iter().skip(chars.len() - 30).collect();
                        format!("{}...{}", start, end)
                    } else {
                        result.filepath.clone()
                    }
                } else {
                    result.filepath.clone()
                };

                PluginResult {
                    id: result.filepath.clone(),
                    title: filename,
                    subtitle: Some(format!("{} • {} • {}", truncated_path, size_str, date_str)),
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
            .collect()
    }

    fn execute_action(&self, result_id: &str, action_id: &str) -> Result<String, String> {
        match action_id {
            "open" => {
                #[cfg(target_os = "windows")]
                {
                    Command::new("cmd")
                        .args(["/c", "start", "", result_id])
                        .spawn()
                        .map_err(|e| format!("Failed to open file: {}", e))?;
                }
                Ok("File opened".to_string())
            }
            "open_folder" => {
                #[cfg(target_os = "windows")]
                {
                    if let Some(parent) = std::path::Path::new(result_id).parent() {
                        Command::new("explorer")
                            .arg("/select,")
                            .arg(result_id)
                            .spawn()
                            .map_err(|e| format!("Failed to open folder: {}", e))?;
                    }
                }
                Ok("Folder opened".to_string())
            }
            _ => Err("Unknown action".to_string()),
        }
    }
}
