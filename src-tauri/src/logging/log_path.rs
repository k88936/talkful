use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

use crate::shared;

pub fn build_log_path(started_at: DateTime<Utc>) -> Result<PathBuf> {
    let log_dir = shared::get_base_path().join("log");
    std::fs::create_dir_all(&log_dir)
        .with_context(|| format!("failed to create log directory at {}", log_dir.display()))?;

    let file_name = format!("{}.log", started_at.format("%Y%m%dT%H%M%S%.3fZ"));
    Ok(log_dir.join(file_name))
}
