use std::sync::Mutex;

mod config;

use config::{ConfigStore, TalkfulConfig};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tauri::menu::{CheckMenuItem, Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, State, WindowEvent, Wry};
use tauri_plugin_autostart::ManagerExt as AutostartExt;

const MENU_ID_STATUS: &str = "runtime_status";
const MENU_ID_ACTION: &str = "runtime_action";
const MENU_ID_AUTOSTART: &str = "runtime_autostart";
const MENU_ID_QUIT: &str = "runtime_quit";
const TRAY_ID: &str = "talkful-runtime-tray";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RuntimeState {
    Idle,
    Recording,
    Processing,
    Error,
}

impl RuntimeState {
    fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Recording => "Recording",
            Self::Processing => "Processing",
            Self::Error => "Error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RuntimeEvent {
    HotkeyToggle,
    AudioStarted,
    AudioStopped,
    AsrStarted,
    AsrCompleted,
    AsrFailed,
    InjectionCompleted,
    RuntimeFailure,
    ResetError,
}

#[derive(Debug, Clone, Serialize)]
struct RuntimeSnapshot {
    state: RuntimeState,
    autostart_enabled: bool,
}

#[derive(Debug, Clone, Copy)]
struct TransitionError {
    from: RuntimeState,
    event: RuntimeEvent,
}

fn transition(from: RuntimeState, event: RuntimeEvent) -> Result<RuntimeState, TransitionError> {
    match (from, event) {
        (RuntimeState::Idle, RuntimeEvent::HotkeyToggle)
        | (RuntimeState::Idle, RuntimeEvent::AudioStarted) => Ok(RuntimeState::Recording),
        (RuntimeState::Recording, RuntimeEvent::HotkeyToggle)
        | (RuntimeState::Recording, RuntimeEvent::AudioStopped)
        | (RuntimeState::Recording, RuntimeEvent::AsrStarted) => Ok(RuntimeState::Processing),
        (RuntimeState::Processing, RuntimeEvent::AsrCompleted)
        | (RuntimeState::Processing, RuntimeEvent::InjectionCompleted) => Ok(RuntimeState::Idle),
        (_, RuntimeEvent::AsrFailed) | (_, RuntimeEvent::RuntimeFailure) => Ok(RuntimeState::Error),
        (RuntimeState::Error, RuntimeEvent::ResetError)
        | (RuntimeState::Error, RuntimeEvent::HotkeyToggle) => Ok(RuntimeState::Idle),
        _ => Err(TransitionError { from, event }),
    }
}

#[derive(Clone)]
struct TrayHandles {
    status_item: MenuItem<Wry>,
    action_item: MenuItem<Wry>,
    autostart_item: CheckMenuItem<Wry>,
}

struct RuntimeModel {
    state: RuntimeState,
    autostart_enabled: bool,
}

impl Default for RuntimeModel {
    fn default() -> Self {
        Self {
            state: RuntimeState::Idle,
            autostart_enabled: false,
        }
    }
}

#[derive(Default)]
struct RuntimeController {
    model: Mutex<RuntimeModel>,
    tray: Mutex<Option<TrayHandles>>,
}

impl RuntimeController {
    fn snapshot(&self) -> RuntimeSnapshot {
        let model = self.model.lock().expect("runtime model poisoned");
        RuntimeSnapshot {
            state: model.state,
            autostart_enabled: model.autostart_enabled,
        }
    }

    fn register_tray(&self, handles: TrayHandles) {
        let mut tray = self.tray.lock().expect("tray state poisoned");
        *tray = Some(handles);
    }

    fn set_autostart_initial_value(&self, enabled: bool) {
        let mut model = self.model.lock().expect("runtime model poisoned");
        model.autostart_enabled = enabled;
    }

    fn apply_event(&self, app: &tauri::AppHandle, event: RuntimeEvent, reason: String) {
        let (old_state, new_state_or_err) = {
            let mut model = self.model.lock().expect("runtime model poisoned");
            let old_state = model.state;
            let transitioned = transition(old_state, event);
            if let Ok(new_state) = transitioned {
                model.state = new_state;
            }
            (old_state, transitioned)
        };

        match new_state_or_err {
            Ok(new_state) => {
                info!(
                    "runtime transition: {} --({:?}, reason: {})--> {}",
                    old_state.label(),
                    event,
                    reason,
                    new_state.label()
                );
                self.sync_tray(app);
            }
            Err(err) => {
                warn!(
                    "blocked invalid transition: {} --({:?}, reason: {})--> <invalid>",
                    err.from.label(),
                    err.event,
                    reason
                );
            }
        }
    }

    fn set_autostart(
        &self,
        app: &tauri::AppHandle,
        enabled: bool,
        reason: &str,
    ) -> Result<(), String> {
        let manager = app.autolaunch();
        let op_result = if enabled {
            manager.enable()
        } else {
            manager.disable()
        };
        op_result.map_err(|e| e.to_string())?;

        {
            let mut model = self.model.lock().expect("runtime model poisoned");
            model.autostart_enabled = enabled;
        }
        info!("autostart updated to {} (reason: {})", enabled, reason);
        self.sync_tray(app);
        Ok(())
    }

    fn sync_tray(&self, app: &tauri::AppHandle) {
        let snapshot = self.snapshot();
        let tray = self.tray.lock().expect("tray state poisoned");
        let Some(handles) = tray.as_ref() else {
            return;
        };

        if let Err(err) = handles
            .status_item
            .set_text(format!("Status: {}", snapshot.state.label()))
        {
            error!("failed to set tray status label: {err}");
        }

        let (action_text, action_enabled) = match snapshot.state {
            RuntimeState::Idle => ("Start Recording", true),
            RuntimeState::Recording => ("Stop Recording", true),
            RuntimeState::Processing => ("Processing...", false),
            RuntimeState::Error => ("Reset To Idle", true),
        };
        if let Err(err) = handles.action_item.set_text(action_text) {
            error!("failed to set tray action text: {err}");
        }
        if let Err(err) = handles.action_item.set_enabled(action_enabled) {
            error!("failed to set tray action enabled state: {err}");
        }
        if let Err(err) = handles.autostart_item.set_checked(snapshot.autostart_enabled) {
            error!("failed to set tray autostart check state: {err}");
        }

        if let Some(tray_icon) = app.tray_by_id(TRAY_ID) {
            if let Err(err) = tray_icon.set_tooltip(Some(format!("talkful: {}", snapshot.state.label()))) {
                error!("failed to set tray tooltip: {err}");
            }
        }
    }
}

fn persist_autostart_preference(app: &tauri::AppHandle, enabled: bool, reason: &str) {
    let config_store = app.state::<ConfigStore>();
    if let Err(err) = config_store.set_autostart(enabled) {
        error!(
            "failed to persist autostart preference {} (reason: {}): {}",
            enabled,
            reason,
            format!("{err:#}")
        );
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn runtime_bus_emit(
    app: tauri::AppHandle,
    controller: State<'_, RuntimeController>,
    event: RuntimeEvent,
    reason: Option<String>,
) {
    let reason = reason.unwrap_or_else(|| "runtime_bus_emit".to_string());
    controller.apply_event(&app, event, reason);
}

#[tauri::command]
fn runtime_snapshot(controller: State<'_, RuntimeController>) -> RuntimeSnapshot {
    controller.snapshot()
}

#[tauri::command]
fn set_autostart(
    app: tauri::AppHandle,
    controller: State<'_, RuntimeController>,
    enabled: bool,
) -> Result<(), String> {
    controller.set_autostart(&app, enabled, "set_autostart command")?;
    persist_autostart_preference(&app, enabled, "set_autostart command");
    Ok(())
}

#[tauri::command]
fn get_config(config_store: State<'_, ConfigStore>) -> Result<TalkfulConfig, String> {
    config_store.get().map_err(|e| format!("{e:#}"))
}

#[tauri::command]
fn update_config(
    app: tauri::AppHandle,
    controller: State<'_, RuntimeController>,
    config_store: State<'_, ConfigStore>,
    config: TalkfulConfig,
) -> Result<TalkfulConfig, String> {
    controller.set_autostart(&app, config.autostart_enabled, "update_config command")?;
    config_store.update(config).map_err(|e| format!("{e:#}"))
}

fn build_tray(app: &tauri::AppHandle) -> tauri::Result<TrayHandles> {
    let status_item = MenuItem::with_id(app, MENU_ID_STATUS, "Status: Idle", false, None::<&str>)?;
    let action_item = MenuItem::with_id(app, MENU_ID_ACTION, "Start Recording", true, None::<&str>)?;
    let autostart_item =
        CheckMenuItem::with_id(app, MENU_ID_AUTOSTART, "Start At Login", true, false, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, MENU_ID_QUIT, "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[&status_item, &action_item, &autostart_item, &quit_item],
    )?;

    TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .tooltip("talkful: Idle")
        .on_menu_event(|app, event| {
            let runtime = app.state::<RuntimeController>();
            match event.id.as_ref() {
                MENU_ID_ACTION => {
                    let state = runtime.snapshot().state;
                    match state {
                        RuntimeState::Idle => runtime.apply_event(
                            app,
                            RuntimeEvent::HotkeyToggle,
                            "tray action: start recording".to_string(),
                        ),
                        RuntimeState::Recording => runtime.apply_event(
                            app,
                            RuntimeEvent::HotkeyToggle,
                            "tray action: stop recording".to_string(),
                        ),
                        RuntimeState::Error => runtime.apply_event(
                            app,
                            RuntimeEvent::ResetError,
                            "tray action: reset error".to_string(),
                        ),
                        RuntimeState::Processing => {
                            warn!("ignored tray action in Processing state");
                        }
                    }
                }
                MENU_ID_AUTOSTART => {
                    let enabled = !runtime.snapshot().autostart_enabled;
                    match runtime.set_autostart(app, enabled, "tray toggle") {
                        Ok(()) => persist_autostart_preference(app, enabled, "tray toggle"),
                        Err(err) => error!("failed to toggle autostart from tray: {err}"),
                    }
                }
                MENU_ID_QUIT => {
                    info!("shutdown requested from tray");
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(TrayHandles {
        status_item,
        action_item,
        autostart_item,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            info!("secondary instance launch intercepted");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .manage(RuntimeController::default())
        .setup(|app| {
            let config_store = ConfigStore::load().map_err(tauri::Error::from)?;
            let config = config_store.get().map_err(tauri::Error::from)?;
            app.manage(config_store);

            let controller = app.state::<RuntimeController>();
            if let Err(err) = controller.set_autostart(app.handle(), config.autostart_enabled, "startup config") {
                warn!("failed to apply startup autostart preference: {err}");
                let plugin_enabled = app.autolaunch().is_enabled().unwrap_or(false);
                controller.set_autostart_initial_value(plugin_enabled);
            }

            let tray_handles = build_tray(app.handle())?;
            controller.register_tray(tray_handles);
            controller.sync_tray(app.handle());

            if let Some(window) = app.get_webview_window("main") {
                if let Err(err) = window.hide() {
                    error!("failed to hide main window on startup: {err}");
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                if let Err(err) = window.hide() {
                    error!("failed to hide window on close request: {err}");
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            runtime_bus_emit,
            runtime_snapshot,
            set_autostart,
            get_config,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::{transition, RuntimeEvent, RuntimeState};

    #[test]
    fn valid_flow_idle_recording_processing_idle() {
        let s1 = transition(RuntimeState::Idle, RuntimeEvent::HotkeyToggle).unwrap();
        let s2 = transition(s1, RuntimeEvent::AudioStopped).unwrap();
        let s3 = transition(s2, RuntimeEvent::AsrCompleted).unwrap();
        assert_eq!(s3, RuntimeState::Idle);
    }

    #[test]
    fn invalid_transition_is_rejected() {
        let result = transition(RuntimeState::Idle, RuntimeEvent::AsrCompleted);
        assert!(result.is_err());
    }

    #[test]
    fn failure_always_enters_error_and_can_recover() {
        let errored = transition(RuntimeState::Recording, RuntimeEvent::RuntimeFailure).unwrap();
        assert_eq!(errored, RuntimeState::Error);
        let recovered = transition(errored, RuntimeEvent::ResetError).unwrap();
        assert_eq!(recovered, RuntimeState::Idle);
    }
}
