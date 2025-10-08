mod commands;
mod plugins;
use commands::default::{
    execute_plugin_action, get_is_window_shown, get_plugin_info, list_plugins, search_plugin, set_is_window_shown,
};
use commands::settings::{get_settings, open_settings_window, set_settings};
use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    App, Manager,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn setup_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = MenuBuilder::new(app)
        .items(&[&show, &settings, &quit])
        .build()?;
    let _tray = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .title("Command Bar")
        .tooltip("Command Bar")
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "settings" => {
                let _ = open_settings_window(app.app_handle().clone());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click {
                    button: tauri::tray::MouseButton::Left,
                    ..
                } => {
                    if let Some(app) = tray.app_handle().get_webview_window("main") {
                        if app.is_visible().unwrap_or(false) {
                            let _ = app.hide();
                        } else {
                            let _ = app.show();
                            let _ = app.set_focus();
                        }
                    }
                }
                TrayIconEvent::Click {
                    button: tauri::tray::MouseButton::Right,
                    ..
                } => {
                    if let Some(tray) = tray.app_handle().tray_by_id("tray") {
                        // todo
                    }
                }
                _ => {}
            }
        })
        .build(app)?;
    Ok(())
}

fn setup_debug(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )?;
    }
    Ok(())
}

fn setup_global_shortcuts(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let ctrl_r_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyR);
    let escape_shortcut = Shortcut::new(None, Code::Escape);

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    if shortcut == &ctrl_r_shortcut {
                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    } else if shortcut == &escape_shortcut {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                }
            })
            .build(),
    )?;

    app.global_shortcut()
        .register(Shortcut::new(Some(Modifiers::CONTROL), Code::KeyR))?;
    app.global_shortcut()
        .register(Shortcut::new(None, Code::Escape))?;
    Ok(())
}

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            setup_tray(&*app)?;
            setup_debug(&*app)?;
            setup_global_shortcuts(&*app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search_plugin,
            get_plugin_info,
            list_plugins,
            execute_plugin_action,
            get_is_window_shown,
            set_is_window_shown,
            get_settings,
            set_settings,
            open_settings_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
