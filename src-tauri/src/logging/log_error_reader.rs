use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct LogEntry {
    level: String,
    message: String,
}

pub fn read_log_error_messages(path: &Path) -> Result<Vec<String>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read log at {}", path.display()))?;
    let mut error_messages = Vec::new();

    for (line_index, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: LogEntry = serde_json::from_str(line)
            .with_context(|| format!("failed to parse log JSON on line {}", line_index + 1))?;
        if entry.level == "ERROR" {
            error_messages.push(entry.message);
        }
    }

    Ok(error_messages)
}
