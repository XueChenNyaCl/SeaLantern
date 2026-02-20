use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct m_PluginInfo {
    pub m_id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub file_name: String,
    pub file_size: u64,
    pub enabled: bool,
    pub main_class: String,
    pub has_config_folder: bool,
    pub config_files: Vec<m_PluginConfigFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct m_PluginConfig {
    pub m_plugin_id: String,
    config_file: String,
    pub config_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct m_PluginConfigFile {
    pub file_name: String,
    pub content: String,
    pub file_type: String,
    pub file_path: String,
}
