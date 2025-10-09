use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PluginConfig {
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PluginAction {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
}

#[derive(serde::Serialize, Clone)]
pub struct PluginResult {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub actions: Option<Vec<PluginAction>>,
}

#[derive(serde::Serialize, Clone)]
pub struct PluginHtmlResult {
    pub html: String,
}

#[derive(serde::Serialize, Clone)]
#[serde(untagged)]
pub enum PluginSearchResult {
    Results(Vec<PluginResult>),
    Html(PluginHtmlResult),
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prefix: String,
    pub icon: String,
    pub config: Option<PluginConfig>,
}

pub trait PluginTrait: Send + Sync {
    fn get_info(&self) -> Plugin;
    fn search(&self, query: &str) -> Vec<PluginResult>;
    fn execute_action(&self, result_id: &str, action_id: &str) -> Result<String, String>;
}
