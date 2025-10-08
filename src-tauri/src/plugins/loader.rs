use super::*;
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::fs;
use std::future::Future;
use std::path::Path;

type GetInfoFn = extern "Rust" fn() -> Plugin;
type SearchFn = extern "Rust" fn(String) -> Vec<PluginResult>;
type ExecuteActionFn = extern "Rust" fn(String, String) -> Result<String, String>;

struct DynamicPlugin {
    get_info: GetInfoFn,
    search: SearchFn,
    execute_action: ExecuteActionFn,
    _lib: Library,
}

#[async_trait::async_trait]
impl PluginTrait for DynamicPlugin {
    fn get_info(&self) -> Plugin {
        (self.get_info)()
    }

    async fn search(&self, query: &str) -> Vec<PluginResult> {
        tokio::task::spawn_blocking({
            let search_fn = self.search;
            let query = query.to_string();
            move || {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    search_fn(query)
                })) {
                    Ok(results) => results,
                    Err(_) => vec![]
                }
            }
        }).await.unwrap_or_default()
    }

    fn execute_action(&self, result_id: &str, action_id: &str) -> Result<String, String> {
        (self.execute_action)(result_id.to_string(), action_id.to_string())
    }
}

unsafe impl Send for DynamicPlugin {}
unsafe impl Sync for DynamicPlugin {}

pub struct DynamicPluginLoader {
    libraries: Vec<Library>,
    plugins: HashMap<String, Box<dyn PluginTrait + Send + Sync>>,
}

impl DynamicPluginLoader {
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugins_from_directory<P: AsRef<Path>>(
        &mut self,
        dir: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Scanning directory: {:?}", dir.as_ref());
        if !dir.as_ref().exists() {
            println!("Directory does not exist: {:?}", dir.as_ref());
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("Found file: {:?}", path);

            if path.extension().and_then(|s| s.to_str()) == Some("dll")
                || path.extension().and_then(|s| s.to_str()) == Some("so")
                || path.extension().and_then(|s| s.to_str()) == Some("dylib")
            {
                println!("Attempting to load plugin: {:?}", path);
                if let Err(e) = self.load_plugin_library(&path) {
                    eprintln!("Failed to load plugin {:?}: {}", path, e);
                } else {
                    println!("Successfully loaded plugin: {:?}", path);
                }
            }
        }
        Ok(())
    }

    fn load_plugin_library<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(path.as_ref())?;

            let get_info: Symbol<GetInfoFn> = lib.get(b"get_plugin_info")?;
            let search: Symbol<SearchFn> = lib.get(b"search_plugin")?;
            let execute_action: Symbol<ExecuteActionFn> = lib.get(b"execute_plugin_action")?;

            let plugin = DynamicPlugin {
                get_info: *get_info,
                search: *search,
                execute_action: *execute_action,
                _lib: lib,
            };

            let info = plugin.get_info();
            self.plugins.insert(info.id.clone(), Box::new(plugin));
        }
        Ok(())
    }

    pub fn register_plugin(&mut self, id: String, plugin: Box<dyn PluginTrait + Send + Sync>) {
        self.plugins.insert(id, plugin);
    }

    pub fn get_plugin(&self, id: &str) -> Option<&(dyn PluginTrait + Send + Sync)> {
        self.plugins.get(id).map(|p| p.as_ref())
    }

    pub fn list_plugins(&self) -> Vec<Plugin> {
        self.plugins.values().map(|p| p.get_info()).collect()
    }

    pub fn load_all_dynamic_plugins(&mut self) {
        println!("Starting dynamic plugin loading...");

        if let Some(home_dir) = dirs::home_dir() {
            println!("Home dir: {:?}", home_dir);
            let build_dir = home_dir
                .join(".config")
                .join("command-bar")
                .join("plugins")
                .join(".build");
            println!("Looking for plugins in: {:?}", build_dir);
            println!("Directory exists: {}", build_dir.exists());

            if let Err(e) = self.load_plugins_from_directory(&build_dir) {
                eprintln!("Failed to load plugins from {:?}: {}", build_dir, e);
            }
        } else {
            println!("No home directory found");
        }

        println!("Total plugins loaded: {}", self.list_plugins().len());
        for plugin in self.list_plugins() {
            println!("Plugin: {} ({})", plugin.name, plugin.id);
        }
    }
}
