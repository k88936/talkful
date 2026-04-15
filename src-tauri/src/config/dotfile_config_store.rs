use std::path::Path;
use std::sync::Mutex;

use crate::config::{AppConfig, DotfileConfigStore, IConfigStore};
use crate::shared;
use anyhow::{Context, Result};

impl DotfileConfigStore {
    pub(crate) fn new() -> Result<Self> {
        let path = shared::get_base_path().join("config.yaml");
        let config = load_or_default_from_path(&path)?;
        Ok(Self {
            path,
            config: Mutex::new(config),
        })
    }
}
impl IConfigStore for DotfileConfigStore {
    fn get(&self) -> AppConfig {
        self.config.lock().expect("config state poisoned").clone()
    }

    fn set(&self, new_config: AppConfig) -> Result<AppConfig> {
        save_config_to_path(&self.path, &new_config)?;
        let mut cfg = self.config.lock().expect("config state poisoned");
        *cfg = new_config.clone();
        Ok(new_config)
    }
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config dir at {}", parent.display()))?;
    }
    Ok(())
}

fn load_or_default_from_path(path: &Path) -> Result<AppConfig> {
    ensure_parent_dir(path)?;

    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file at {}", path.display()))?;
    serde_yaml::from_str::<AppConfig>(&raw)
        .with_context(|| format!("failed to parse config YAML at {}", path.display()))
}

fn save_config_to_path(path: &Path, config: &AppConfig) -> Result<()> {
    ensure_parent_dir(path)?;
    let raw = serde_yaml::to_string(config).context("failed to serialize config")?;
    std::fs::write(path, raw)
        .with_context(|| format!("failed to write config file at {}", path.display()))
}
