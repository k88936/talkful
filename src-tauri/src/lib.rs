use std::error::Error;
use std::sync::Mutex;
pub mod asr;
pub mod config;
pub mod record;
use crate::asr::sherpa_asr_service::SherpaASRService;
use crate::asr::ASRService;
use crate::config::{DotfileConfigStore, IConfigStore};
use crate::record::cpal_record_service::CPALRecordService;
use crate::record::{RecordService, RecordSignal};
use anyhow::Result;
use enigo::{Enigo, Keyboard, Settings};
use tauri::window::Color;
use tauri::{App, AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut};
use tokio::sync::oneshot;

pub struct AppServices {
    asr_service: Mutex<SherpaASRService>,
    record_service: CPALRecordService,
}
pub struct AppState {
    record_signal_tx: Mutex<Option<oneshot::Sender<RecordSignal>>>,
}
pub fn on_record_started(app: &AppHandle) {
    // show the float window in the focused monitor
    let cursor_pos = app.cursor_position().unwrap();
    let target_monitor = app
        .monitor_from_point(cursor_pos.x, cursor_pos.y)
        .unwrap()
        .unwrap();
    let screen_size = target_monitor.size();
    let screen_pos = target_monitor.position();

    let window_width = 256;
    let window_height = 64;

    let x = screen_pos.x + (screen_size.width as i32 - window_width) / 2;
    let y = screen_pos.y + (screen_size.height as i32 - window_height) - 256;
    let window = app
        .get_webview_window("float")
        // recreate if fail
        .unwrap_or_else(|| build_float_window(app));
    // set pos and size
    window
        .set_size(PhysicalSize::new(window_width, window_height))
        .unwrap();
    window.set_position(PhysicalPosition::new(x, y)).unwrap();
    window.show().unwrap();

    let (signal_tx, signal_rx) = oneshot::channel();
    let handle_cpy = app.clone();
    tokio::spawn(async move {
        let services = handle_cpy.state::<AppServices>().inner();
        workflow(services, signal_rx).await;
        // hide the window afterward
        if let Some(window) = handle_cpy.get_webview_window("float") {
            window.hide().unwrap();
        }
    });

    let state = app.state::<AppState>();
    // store the stop signal tx
    {
        let mut guard = state.record_signal_tx.lock().expect("poisoned");
        if let Some(sender) = guard.take() {
            sender
                .send(RecordSignal::Stop)
                .expect("recording task dropped before stop signal");
        }
        *guard = Some(signal_tx);
    }
}
async fn workflow(services: &AppServices, signal: oneshot::Receiver<RecordSignal>) {
    if let Ok(recorded) = services.record_service.record(signal).await {
        let result = services.asr_service.lock().expect("poisoned").asr(recorded);

        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        enigo.text(&result).expect("should inject text");
    }
}
pub fn on_record_ended(app: &AppHandle) {
    let state = app.state::<AppState>();
    {
        let mut guard = state
            .record_signal_tx
            .lock()
            .expect("AppState.record_signal_rx poisoned");
        match guard.take() {
            Some(sender) => {
                sender
                    .send(RecordSignal::Stop)
                    .expect("record signal tx dropped");
                *guard = None;
            }
            None => {
                panic!("no running workflow")
            }
        }
    }
}

pub fn initialize(app: &mut App) -> Result<(), Box<dyn Error>> {
    // load config
    let config_store = DotfileConfigStore::new()?;
    let config = config_store.get();
    app.manage(config_store);

    // TODO accept the error, allowing user correct config in the web ui.
    let asr = SherpaASRService::new(&config.asr_model_filename, &config.asr_token_filename)
        .expect("should init asr service");
    let recorder = CPALRecordService::new();

    let services = AppServices {
        asr_service: Mutex::new(asr),
        record_service: recorder,
    };
    app.manage(services);
    let state = AppState {
        record_signal_tx: Mutex::new(None),
    };
    app.manage(state);

    // register trigger shortcut
    app.global_shortcut()
        .register(Shortcut::new(None, Code::F8))?;

    // window
    let main_window_url = tauri::WebviewUrl::App("index.html".into());
    tauri::WebviewWindowBuilder::new(app, "main", main_window_url)
        .title("talkful")
        .build()?;
    // float widget
    build_float_window(app.handle());

    Ok(())
}

pub fn build_float_window(app: &AppHandle) -> WebviewWindow {
    let float_window_url = tauri::WebviewUrl::App("index.html".into());
    let window = tauri::WebviewWindowBuilder::new(app, "float", float_window_url)
        .resizable(false)
        .closable(false)
        .focused(false)
        .transparent(true)
        .background_color(Color(0, 0, 0, 64))
        .shadow(false)
        .decorations(false)
        .always_on_top(true)
        // .visible(false)
        .skip_taskbar(true)
        .build()
        .expect("failed to create window");
    window
}
