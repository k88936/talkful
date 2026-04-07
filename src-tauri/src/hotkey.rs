use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut};

pub const MIN_HOLD_MS: u64 = 120;
pub const MAX_HOLD_MS: u64 = 30_000;

#[derive(Debug, Default)]
struct HotkeyCycleState {
    is_pressed: bool,
    cycle_id: u64,
    pressed_at: Option<Instant>,
    finalized: bool,
}

#[derive(Default)]
pub struct HotkeyCycle {
    state: Mutex<HotkeyCycleState>,
}

impl HotkeyCycle {
    pub fn on_press(&self) -> Option<u64> {
        let mut cycle = self.state.lock().expect("hotkey state poisoned");
        if cycle.is_pressed {
            return None;
        }

        cycle.is_pressed = true;
        cycle.finalized = false;
        cycle.pressed_at = Some(Instant::now());
        cycle.cycle_id = cycle.cycle_id.wrapping_add(1);
        Some(cycle.cycle_id)
    }

    pub fn on_release(&self) -> Option<(u64, Duration)> {
        let mut cycle = self.state.lock().expect("hotkey state poisoned");
        if !cycle.is_pressed {
            return None;
        }

        cycle.is_pressed = false;
        let elapsed = cycle
            .pressed_at
            .map(|started| started.elapsed())
            .unwrap_or_default();
        Some((cycle.cycle_id, elapsed))
    }

    pub fn finalize_release(&self, cycle_id: u64) -> bool {
        let mut cycle = self.state.lock().expect("hotkey state poisoned");
        if cycle.cycle_id != cycle_id || cycle.finalized {
            return false;
        }

        cycle.finalized = true;
        cycle.is_pressed = false;
        cycle.pressed_at = None;
        true
    }

    pub fn finalize_timeout(&self, cycle_id: u64) -> bool {
        let mut cycle = self.state.lock().expect("hotkey state poisoned");
        if cycle.cycle_id != cycle_id || cycle.finalized || !cycle.is_pressed {
            return false;
        }

        cycle.finalized = true;
        cycle.is_pressed = false;
        cycle.pressed_at = None;
        true
    }
}

pub fn min_hold_duration() -> Duration {
    Duration::from_millis(MIN_HOLD_MS)
}

fn parse_hotkey_key_to_shortcut(normalized: &str) -> Option<Shortcut> {
    let code = match normalized {
        "f1" => Code::F1,
        "f2" => Code::F2,
        "f3" => Code::F3,
        "f4" => Code::F4,
        "f5" => Code::F5,
        "f6" => Code::F6,
        "f7" => Code::F7,
        "f8" => Code::F8,
        "f9" => Code::F9,
        "f10" => Code::F10,
        "f11" => Code::F11,
        "f12" => Code::F12,
        _ => return None,
    };

    Some(Shortcut::new(None, code))
}

pub fn register_runtime_hotkey(app: &tauri::App, configured_key: &str) -> Result<String> {
    let normalized = configured_key.trim().to_ascii_lowercase();
    let shortcut = parse_hotkey_key_to_shortcut(&normalized).ok_or_else(|| {
        anyhow!(
            "unsupported hotkey_key '{}'; expected one of: f1..f12",
            configured_key
        )
    })?;
    app.global_shortcut().register(shortcut)?;
    Ok(normalized)
}
