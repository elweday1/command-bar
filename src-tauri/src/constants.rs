use std::path::PathBuf;

pub const APP_NAME: &str = "dossier";

pub fn get_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join(APP_NAME)
}

pub fn get_plugins_dir() -> PathBuf {
    get_config_dir().join("plugins")
}

pub fn get_settings_path() -> PathBuf {
    get_config_dir().join("settings.json")
}
