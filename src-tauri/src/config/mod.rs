mod dotfile_config_store;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey_key: String,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkey_key: "F8".into(),
        }
    }
}
pub struct DotfileConfigStore {
    path: PathBuf,
    config: Mutex<AppConfig>,
}

pub trait IConfigStore {
    fn get(&self) -> AppConfig;
    fn set(&self, new_config: AppConfig) -> anyhow::Result<AppConfig>;
}
