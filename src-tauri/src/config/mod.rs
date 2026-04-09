mod implement;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowConfig {
    #[serde(default)]
    theme: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalkfulConfig {
    pub autostart_enabled: bool,
    pub hotkey_key: String,
    window: WindowConfig,
}
pub struct ConfigStore {
    path: PathBuf,
    config: Mutex<TalkfulConfig>,
}

impl ConfigStore {
    pub fn load() -> anyhow::Result<Self> {
        <Self as IConfigStore>::load()
    }

    pub fn get(&self) -> anyhow::Result<TalkfulConfig> {
        <Self as IConfigStore>::get(self)
    }

    pub fn update(&self, new_config: TalkfulConfig) -> anyhow::Result<TalkfulConfig> {
        <Self as IConfigStore>::update(self, new_config)
    }

    pub fn set_autostart(&self, enabled: bool) -> anyhow::Result<()> {
        <Self as IConfigStore>::set_autostart(self, enabled)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub trait IConfigStore {
    fn load() -> anyhow::Result<Self>
    where
        Self: Sized;
    fn get(&self) -> anyhow::Result<TalkfulConfig>;
    fn update(&self, new_config: TalkfulConfig) -> anyhow::Result<TalkfulConfig>;
    fn set_autostart(&self, enabled: bool) -> anyhow::Result<()>;
}
