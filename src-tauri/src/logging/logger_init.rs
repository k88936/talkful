use anyhow::{Context, Result};
use chrono::{SecondsFormat, Utc};
use log::LevelFilter;
use serde_json::json;

use crate::logging::log_path::build_log_path;
use crate::logging::LogState;

pub fn init_logger() -> Result<LogState> {
    let started_at = Utc::now();
    let log_path = build_log_path(started_at)?;
    let log_file = fern::log_file(&log_path)
        .with_context(|| format!("failed to open {}", log_path.display()))?;

    fern::Dispatch::new()
        .level(LevelFilter::Info)
        .format(|out, message, record| {
            let entry = json!({
                "ts": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                "level": record.level().to_string(),
                "target": record.target(),
                "message": message.to_string(),
            });
            out.finish(format_args!("{entry}"));
        })
        .chain(log_file)
        .chain(std::io::stdout())
        .apply()
        .context("failed to initialize global logger")?;

    Ok(LogState::new(log_path))
}
