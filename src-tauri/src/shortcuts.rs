use crate::commands::settings::get_settings;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tauri::{App, AppHandle, Listener, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

static REGISTERED_SHORTCUTS: std::sync::OnceLock<Arc<Mutex<Vec<Shortcut>>>> = std::sync::OnceLock::new();

fn get_shortcuts_from_settings() -> (Shortcut, Shortcut, Shortcut) {
    let settings = get_settings().unwrap_or_else(|_| serde_json::json!({}));
    let default_shortcuts = serde_json::json!({});
    let shortcuts = settings.get("shortcuts").unwrap_or(&default_shortcuts);

    let toggle_key = shortcuts
        .get("toggleWindow")
        .and_then(|v| v.as_str())
        .unwrap_or("Ctrl+R");
    let hide_key = shortcuts
        .get("hideWindow")
        .and_then(|v| v.as_str())
        .unwrap_or("Escape");
    let settings_key = shortcuts
        .get("openSettings")
        .and_then(|v| v.as_str())
        .unwrap_or("Ctrl+Comma");

    let toggle_shortcut = Shortcut::from_str(toggle_key).unwrap_or_else(|_| Shortcut::from_str("Ctrl+R").unwrap());
    let hide_shortcut = Shortcut::from_str(hide_key).unwrap_or_else(|_| Shortcut::from_str("Escape").unwrap());
    let settings_shortcut = Shortcut::from_str(settings_key).unwrap_or_else(|_| Shortcut::from_str("Ctrl+Comma").unwrap());
    
    (toggle_shortcut, hide_shortcut, settings_shortcut)
}

fn register_shortcuts(app: &AppHandle, toggle_shortcut: Shortcut, hide_shortcut: Shortcut, settings_shortcut: Shortcut) -> Result<(), Box<dyn std::error::Error>> {
    let shortcuts_store = REGISTERED_SHORTCUTS.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
    let mut registered = shortcuts_store.lock().unwrap();
    
    for shortcut in registered.drain(..) {
        let _ = app.global_shortcut().unregister(shortcut);
    }
    
    app.global_shortcut().register(toggle_shortcut.clone())?;
    app.global_shortcut().register(hide_shortcut.clone())?;
    app.global_shortcut().register(settings_shortcut.clone())?;
    
    registered.push(toggle_shortcut);
    registered.push(hide_shortcut);
    registered.push(settings_shortcut);
    
    Ok(())
}

pub fn setup_shortcuts(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let (toggle_shortcut, hide_shortcut, settings_shortcut) = get_shortcuts_from_settings();
    let window = app.get_webview_window("main").unwrap();
    let app_handle = app.handle().clone();

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    let (current_toggle, current_hide, current_settings) = get_shortcuts_from_settings();
                    if _shortcut == &current_toggle {
                        let is_visible = window.is_visible().unwrap_or(false);
                        if is_visible {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    } else if _shortcut == &current_hide {
                        let _ = window.hide();
                    } else if _shortcut == &current_settings {
                        let _ = crate::commands::settings::open_settings_window(_app.clone());
                    }
                }
            })
            .build(),
    )?;

    register_shortcuts(&app.handle(), toggle_shortcut, hide_shortcut, settings_shortcut)?;

    app.handle().listen("settings-changed", move |_| {
        let (new_toggle, new_hide, new_settings) = get_shortcuts_from_settings();
        let _ = register_shortcuts(&app_handle, new_toggle, new_hide, new_settings);
    });

    Ok(())
}