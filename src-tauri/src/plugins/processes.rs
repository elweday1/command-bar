use super::{Plugin, PluginAction, PluginResult, PluginTrait};
use sysinfo::{ProcessesToUpdate, System};

pub struct ProcessesPlugin;

impl PluginTrait for ProcessesPlugin {
    fn get_info(&self) -> Plugin {
        Plugin {
            id: "processes".to_string(),
            name: "System Processes".to_string(),
            description: "View and manage system processes".to_string(),
            prefix: "ps".to_string(),
            icon: "⚙️".to_string(),
            config: None,
        }
    }

    fn search(&self, query: &str) -> Vec<PluginResult> {
        let mut sys = System::new_all();
        sys.refresh_processes(ProcessesToUpdate::All, true);

        let mut results: Vec<PluginResult> = Vec::new();
        let query_lower = query.to_lowercase();

        for (pid, process) in sys.processes() {
            let process_name = process.name().to_string_lossy().to_string();

            // Filter by query if provided
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
        results
    }

    fn execute_action(&self, result_id: &str, action_id: &str) -> Result<String, String> {
        match action_id {
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
}
