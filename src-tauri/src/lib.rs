use std::error::Error;
use std::str::FromStr;
use std::sync::Mutex;
pub mod asr;
pub mod config;
pub mod record;
pub mod shared;

use crate::asr::sherpa_asr_processor::SherpaASRProcessor;
use crate::config::{DotfileConfigStore, IConfigStore};
use crate::record::cpal_record_service::CPALRecordService;
use crate::record::{RecordService, RecordSignal};
use anyhow::{Context, Result};
use asr::ASRService;
use enigo::{Enigo, Keyboard, Settings};
use tauri::window::Color;
use tauri::{App, AppHandle, Emitter, Manager, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut};
use tokio::sync::oneshot;

pub type AppConfigStore = DotfileConfigStore;

pub struct AppServices {
    asr_client: ASRService,
    record_service: CPALRecordService,
}
pub struct AppState {
    record_signal_tx: Mutex<Option<oneshot::Sender<RecordSignal>>>,
}

#[derive(Default)]
pub struct StartupErrorState {
    messages: Mutex<Vec<String>>,
}

impl StartupErrorState {
    pub fn push(&self, message: String) {
        self.messages
            .lock()
            .expect("StartupErrorState.messages poisoned")
            .push(message);
    }

    pub fn all(&self) -> Vec<String> {
        self.messages
            .lock()
            .expect("StartupErrorState.messages poisoned")
            .clone()
    }
}

pub fn on_record_started(app: &AppHandle) -> Result<()> {
    let window = app
        .get_webview_window("float")
        // recreate if fail
        .unwrap_or_else(|| build_float_window(app));
    window.show()?;
    window.center()?;

    let (signal_tx, signal_rx) = oneshot::channel();
    let handle_cpy = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(error) = workflow(&handle_cpy, signal_rx).await {
            emit_error_to_main_window(&handle_cpy, error.into_boxed_dyn_error());
        }
        // hide the window afterward
        if let Some(window) = handle_cpy.get_webview_window("float") {
            window.hide().unwrap();
        }
    });

    let state = app
        .try_state::<AppState>()
        .context("app state not initialized")?;
    // store the stop signal tx
    {
        let mut guard = state.record_signal_tx.lock().expect("poisoned");
        if let Some(sender) = guard.take() {
            sender
                .send(RecordSignal::Stop)
                .ok()
                .context("recording task dropped before stop signal")?;
        }
        *guard = Some(signal_tx);
    }
    Ok(())
}
async fn workflow(app: &AppHandle, signal: oneshot::Receiver<RecordSignal>) -> Result<()> {
    let services = app
        .try_state::<AppServices>()
        .context("app services not initialized")?;
    let recorded = services
        .inner()
        .record_service
        .record(signal)
        .await
        .context("recording failed")?;
    let result = services
        .inner()
        .asr_client
        .transcribe(recorded)
        .await
        .context("asr transcription failed")?;

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text(&result).expect("should inject text");
    Ok(())
}
pub fn on_record_ended(app: &AppHandle) -> Result<()> {
    let state = app
        .try_state::<AppState>()
        .context("app state not initialized")?;
    {
        let mut guard = state
            .record_signal_tx
            .lock()
            .expect("AppState.record_signal_rx poisoned");
        let sender = guard.take().context("no running workflow")?;
        sender
            .send(RecordSignal::Stop)
            .ok()
            .context("record signal tx dropped")?;
        *guard = None;
    }
    Ok(())
}

pub fn init_services(app: &AppHandle) -> Result<()> {
    let config_store = DotfileConfigStore::new()?;
    let config = config_store.get();
    app.manage(config_store);

    let asr_client = ASRService::new(SherpaASRProcessor::new)?;
    let recorder = CPALRecordService::new();

    let services = AppServices {
        asr_client,
        record_service: recorder,
    };
    app.manage(services);

    let state = AppState {
        record_signal_tx: Mutex::new(None),
    };
    app.manage(state);

    let code = Code::from_str(&config.hotkey_key)?;
    app.global_shortcut().register(Shortcut::new(None, code))?;
    Ok(())
}
pub fn initialize(app: &mut App) -> Result<(), Box<dyn Error>> {
    app.manage(StartupErrorState::default());
    build_main_window(app.handle());

    if let Err(e) = init_services(app.handle()).context("init services failed") {
        app.state::<StartupErrorState>().push(format!("{:#}", e));
        emit_error_to_main_window(app.handle(), e.into())
    }

    build_float_window(app.handle());
    Ok(())
}

pub fn build_main_window(app: &AppHandle) -> WebviewWindow {
    let main_window_url = tauri::WebviewUrl::App("index.html".into());
    let window = tauri::WebviewWindowBuilder::new(app, "main", main_window_url)
        .title("talkful")
        .build()
        .unwrap();
    window
}

pub fn build_float_window(app: &AppHandle) -> WebviewWindow {
    let float_window_url = tauri::WebviewUrl::App("index.html".into());
    let window = tauri::WebviewWindowBuilder::new(app, "float", float_window_url)
        .inner_size(256.0,128.0)
        .resizable(false)
        .closable(false)
        .focused(false)
        .focusable(false)
        .transparent(true)
        .background_color(Color(0, 0, 0, 128))
        .shadow(false)
        .decorations(false)
        .always_on_top(true)
        // .visible(false)
        .skip_taskbar(true)
        .build()
        .expect("failed to create window");
    window.center().unwrap();
    window.hide().unwrap();
    window
}

pub fn emit_error_to_main_window(app: &tauri::AppHandle, error: Box<dyn Error + Send + Sync>) {
    let window = app
        .get_webview_window("main")
        .unwrap_or_else(|| build_main_window(app));
    window.show().expect("show main window");
    window
        .emit("error", format!("{:#}", error))
        .expect("emit message error");
}
