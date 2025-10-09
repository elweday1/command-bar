#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult};
use sysinfo::{ProcessesToUpdate, System};

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "processes".to_string(),
        name: "System Processes".to_string(),
        description: "View and manage system processes".to_string(),
        prefix: "ps".to_string(),
        icon: "⚙️".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut results: Vec<PluginResult> = Vec::new();
    let query_lower = query.to_lowercase();

    for (pid, process) in sys.processes() {
        let process_name = process.name().to_string_lossy().to_string();

        if !query.is_empty() && !process_name.to_lowercase().contains(&query_lower) {
            continue;
        }

        results.push(PluginResult {
            id: pid.to_string(),
            title: process_name,
            subtitle: Some(format!(
                "PID: {} • CPU: {:.1}% • Memory: {} KB",
                pid,
                process.cpu_usage(),
                process.memory() / 1024
            )),
            icon: None,
            actions: Some(vec![PluginAction {
                id: "kill".to_string(),
                label: "Kill Process".to_string(),
                shortcut: Some("Ctrl+K".to_string()),
            }]),
        });
    }
    PluginSearchResult::Results(results)
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(result_id: String, action_id: String) -> Result<String, String> {
    match action_id.as_str() {
        "kill" => {
            if let Ok(pid) = result_id.parse::<u32>() {
                let mut sys = System::new();
                sys.refresh_processes(ProcessesToUpdate::All, true);
                if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
                    if process.kill() {
                        Ok("Process killed successfully".to_string())
                    } else {
                        Err("Failed to kill process".to_string())
                    }
                } else {
                    Err("Process not found".to_string())
                }
            } else {
                Err("Invalid process ID".to_string())
            }
        }
        _ => Err("Unknown action".to_string()),
    }
}