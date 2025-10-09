#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult};
use std::process::Command;

struct Terminal {
    name: &'static str,
    executable: &'static str,
    args: &'static [&'static str],
    icon: &'static str,
}

const TERMINALS: &[Terminal] = &[
    Terminal {
        name: "Command Prompt",
        executable: "cmd",
        args: &["/k"],
        icon: "⚫",
    },
    Terminal {
        name: "PowerShell",
        executable: "powershell",
        args: &["-NoExit", "-Command"],
        icon: "🔵",
    },
    Terminal {
        name: "Windows Terminal",
        executable: "wt",
        args: &["-p", "Command Prompt", "cmd", "/k"],
        icon: "🖥️",
    },
    Terminal {
        name: "Git Bash",
        executable: "C:\\Program Files\\Git\\bin\\bash.exe",
        args: &["-c"],
        icon: "🐙",
    },
];

fn is_terminal_available(executable: &str) -> bool {
    Command::new("where")
        .arg(executable)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "shell".to_string(),
        name: "Shell".to_string(),
        description: "Execute terminal commands".to_string(),
        prefix: ">".to_string(),
        icon: "💻".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    if query.is_empty() {
        return PluginSearchResult::Results(vec![]);
    }

    let results = TERMINALS
        .iter()
        .filter(|terminal| is_terminal_available(terminal.executable))
        .map(|terminal| PluginResult {
            id: format!("{}:{}", terminal.executable, query),
            title: format!("Run in {}", terminal.name),
            subtitle: Some(format!("Execute: {}", query)),
            icon: Some(terminal.icon.to_string()),
            actions: Some(vec![PluginAction {
                id: "execute".to_string(),
                label: "Execute".to_string(),
                shortcut: Some("Enter".to_string()),
            }]),
        })
        .collect();

    PluginSearchResult::Results(results)
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(
    result_id: String,
    _action_id: String,
) -> Result<String, String> {
    let parts: Vec<&str> = result_id.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err("Invalid result ID".to_string());
    }

    let executable = parts[0];
    let command = parts[1];

    let terminal = TERMINALS
        .iter()
        .find(|t| t.executable == executable)
        .ok_or("Terminal not found")?;

    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new(terminal.executable);

        match terminal.name {
            "Command Prompt" => {
                cmd.args(&["/k", command]);
            }
            "PowerShell" => {
                cmd.args(&["-NoExit", "-Command", command]);
            }
            "Windows Terminal" => {
                cmd.args(&["-p", "Command Prompt", "cmd", "/k", command]);
            }
            "Git Bash" => {
                cmd.args(&["-c", &format!("{}; exec bash", command)]);
            }
            _ => {
                cmd.args(terminal.args).arg(command);
            }
        }

        cmd.spawn()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
    }

    Ok(format!("Executed '{}' in {}", command, terminal.name))
}
