use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowConfig {
    #[serde(default)]
    theme: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalkfulConfig {
    #[serde(default)]
    pub autostart_enabled: bool,
    #[serde(default)]
    window: WindowConfig,
}

impl Default for TalkfulConfig {
    fn default() -> Self {
        Self {
            autostart_enabled: false,
            window: WindowConfig::default(),
        }
    }
}

pub struct ConfigStore {
    path: PathBuf,
    config: Mutex<TalkfulConfig>,
}

impl ConfigStore {
    pub fn load() -> Result<Self> {
        let path = resolve_config_path()?;
        let config = load_or_default_from_path(&path)?;
        Ok(Self {
            path,
            config: Mutex::new(config),
        })
    }

    pub fn get(&self) -> Result<TalkfulConfig> {
        self.config
            .lock()
            .map_err(|_| anyhow!("config state poisoned"))
            .map(|cfg| cfg.clone())
    }

    pub fn update(&self, new_config: TalkfulConfig) -> Result<TalkfulConfig> {
        save_config_to_path(&self.path, &new_config)?;
        let mut cfg = self
            .config
            .lock()
            .map_err(|_| anyhow!("config state poisoned"))?;
        *cfg = new_config.clone();
        Ok(new_config)
    }

    pub fn set_autostart(&self, enabled: bool) -> Result<()> {
        let mut cfg = self.get()?;
        cfg.autostart_enabled = enabled;
        self.update(cfg)?;
        Ok(())
    }
}

#[cfg(windows)]
fn resolve_config_path_from_env(home: Option<&str>, userprofile: Option<&str>) -> Option<PathBuf> {
    let base = home.or(userprofile)?;
    Some(PathBuf::from(base).join(".talkful").join("config.yaml"))
}

#[cfg(not(windows))]
fn resolve_config_path_from_env(home: Option<&str>, _userprofile: Option<&str>) -> Option<PathBuf> {
    let base = home?;
    Some(PathBuf::from(base).join(".talkful").join("config.yaml"))
}

fn resolve_config_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").ok();
    let userprofile = std::env::var("USERPROFILE").ok();

    resolve_config_path_from_env(home.as_deref(), userprofile.as_deref()).ok_or_else(|| {
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

#[cfg(test)]
mod tests {
    use super::{
        load_or_default_from_path, resolve_config_path_from_env, save_config_to_path, TalkfulConfig,
    };

    #[cfg(not(windows))]
    #[test]
    fn resolve_path() {
        let linux_path = resolve_config_path_from_env(Some("/home/a"), None).unwrap();
        assert_eq!(
            linux_path,
            std::path::PathBuf::from("/home/a/.talkful/config.yaml")
        );
    }

    #[cfg(windows)]
    #[test]
    fn resolve_path() {
        let win_path = resolve_config_path_from_env(None, Some("C:/Users/a")).unwrap();
        assert_eq!(
            win_path,
            std::path::PathBuf::from("C:/Users/a")
                .join(".talkful")
                .join("config.yaml")
        );
    }

    #[test]
    fn load_default_when_config_missing_and_create_parent() {
        let tmp = tempfile::tempdir().unwrap();
        let config_path = tmp.path().join("nested").join("config.yaml");

        let cfg = load_or_default_from_path(&config_path).unwrap();
        assert!(!cfg.autostart_enabled);
        assert!(config_path.parent().unwrap().exists());
    }

    #[test]
    fn save_then_load_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let config_path = tmp.path().join("config.yaml");

        let cfg = TalkfulConfig {
            autostart_enabled: true,
            ..TalkfulConfig::default()
        };

        save_config_to_path(&config_path, &cfg).unwrap();
        let loaded = load_or_default_from_path(&config_path).unwrap();

        assert!(loaded.autostart_enabled);
    }

    #[test]
    fn invalid_yaml_returns_error() {
        let tmp = tempfile::tempdir().unwrap();
        let config_path = tmp.path().join("config.yaml");
        std::fs::write(&config_path, ": bad: yaml:").unwrap();

        let result = load_or_default_from_path(&config_path);
        assert!(result.is_err());
    }
}
