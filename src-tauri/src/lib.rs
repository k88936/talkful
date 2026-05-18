use std::error::Error;
use std::str::FromStr;
use std::sync::Mutex;
pub mod asr;
pub mod config;
pub mod logging;
pub mod record;
pub mod shared;

use crate::asr::sherpa_asr_processor::SherpaASRProcessor;
use crate::config::{DotfileConfigStore, IConfigStore};
use crate::record::cpal_record_service::CPALRecordService;
use crate::record::{RecordService, RecordSignal};
use anyhow::{Context, Result};
use clap::Parser;
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

pub fn on_record_started(app: &AppHandle) -> Result<()> {
    let window = app
        .get_webview_window("float")
        // recreate if fail
        .unwrap_or_else(|| build_float_window(app));
    center_window_on_current_screen(app, &window)?;

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
#[derive(Parser, Debug)]
#[command(name = "talkful")]
struct Args {
    #[arg(long)]
    silent_start: bool,
}
pub fn initialize(app: &mut App) -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let log_state = logging::init_logger().map_err(|error| {
        std::io::Error::other(format!("failed to initialize logger: {error:#}"))
    })?;
    app.manage(log_state);
    if (!args.silent_start){
        build_main_window(app.handle());
    }

    if let Err(e) = init_services(app.handle()).context("init services failed") {
        log::error!("{:#}", e);
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
    let float_window_url = tauri::WebviewUrl::App("float.html".into());
    let window = tauri::WebviewWindowBuilder::new(app, "float", float_window_url)
        .inner_size(256.0, 128.0)
        .resizable(false)
        .closable(false)
        .focused(false)
        .focusable(false)
        .transparent(true)
        .background_color(Color(0, 0, 0, 16))
        .shadow(false)
        .decorations(false)
        .always_on_top(true)
        // .visible(false)
        .skip_taskbar(true)
        .build()
        .expect("failed to create window");
    center_window_on_current_screen(app, &window).unwrap();
    window.hide().unwrap();
    window
}

fn center_window_on_current_screen(app: &AppHandle, window: &WebviewWindow) -> Result<()> {
    let cursor = app.cursor_position().context("failed to get cursor position")?;
    let monitor = app
        .monitor_from_point(cursor.x, cursor.y)
        .context("failed to get monitor from cursor")?
        .context("cursor is not on any monitor")?;
    let monitor_position = monitor.position();
    let monitor_size = monitor.size();
    let window_size = window.outer_size()?;

    let x = monitor_position.x + (monitor_size.width as i32 - window_size.width as i32)/2;
    let y = monitor_position.y + (monitor_size.height as i32 - window_size.height as i32)/2;
    window.show()?;
    window.set_position(tauri::PhysicalPosition::new(x, y))?;
    Ok(())
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
