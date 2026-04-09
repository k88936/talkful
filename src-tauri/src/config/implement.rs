use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::config::{ConfigStore, IConfigStore, TalkfulConfig, WindowConfig};
use anyhow::{anyhow, Context, Result};

impl Default for TalkfulConfig {
    fn default() -> Self {
        Self {
            autostart_enabled: false,
            hotkey_key: "f8".into(),
            window: WindowConfig::default(),
        }
    }
}

impl IConfigStore for ConfigStore {
    fn load() -> Result<Self> {
        let path = resolve_config_path()?;
        let config = load_or_default_from_path(&path)?;
        Ok(Self {
            path,
            config: Mutex::new(config),
        })
    }

    fn get(&self) -> Result<TalkfulConfig> {
        self.config
            .lock()
            .map_err(|_| anyhow!("config state poisoned"))
            .map(|cfg| cfg.clone())
    }

    fn update(&self, new_config: TalkfulConfig) -> Result<TalkfulConfig> {
        save_config_to_path(&self.path, &new_config)?;
        let mut cfg = self
            .config
            .lock()
            .map_err(|_| anyhow!("config state poisoned"))?;
        *cfg = new_config.clone();
        Ok(new_config)
    }

    fn set_autostart(&self, enabled: bool) -> Result<()> {
        let mut cfg = self.get()?;
        cfg.autostart_enabled = enabled;
        self.update(cfg)?;
        Ok(())
    }
}

fn resolve_config_path() -> Result<PathBuf> {
    #[cfg(not(windows))]
    let home = std::env::var("HOME")?;
    #[cfg(windows)]
    let home = std::env::var("USERPROFILE")?;
    Some(PathBuf::from(home).join(".talkful").join("config.yaml")).ok_or_else(|| {
            anyhow!("unable to resolve config path: HOME is not set (and USERPROFILE is not set on Windows)")
        })
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config dir at {}", parent.display()))?;
    }
    Ok(())
}

fn load_or_default_from_path(path: &Path) -> Result<TalkfulConfig> {
    ensure_parent_dir(path)?;

    if !path.exists() {
        return Ok(TalkfulConfig::default());
    }

    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file at {}", path.display()))?;
    serde_yaml::from_str::<TalkfulConfig>(&raw)
        .with_context(|| format!("failed to parse config YAML at {}", path.display()))
}

fn save_config_to_path(path: &Path, config: &TalkfulConfig) -> Result<()> {
    ensure_parent_dir(path)?;
    let raw = serde_yaml::to_string(config).context("failed to serialize config")?;
    std::fs::write(path, raw)
        .with_context(|| format!("failed to write config file at {}", path.display()))
}
