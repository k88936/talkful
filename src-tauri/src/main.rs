// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use log::{error, info};
use talkful_lib::{build_main_window, initialize, on_record_ended, on_record_started,emit_error_to_main_window};
use tauri::{Emitter, Manager, WindowEvent};
use tauri_plugin_global_shortcut::ShortcutState;

use commands::settings::{get_settings, set_settings};
use crate::commands::asr::{download_model_files, get_model_directory_path};
use crate::commands::system_errors::get_startup_errors;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, _key, event| match event.state {
                    ShortcutState::Pressed => {
                        if let Err(error) = on_record_started(app) {
                            emit_error_to_main_window(app, format!("{:?}", error));
                        }
                    }
                    ShortcutState::Released => {
                        if let Err(error) = on_record_ended(app) {
                            emit_error_to_main_window(app, format!("{:?}", error));
                        }
                    }
                })
                .build(),
        )
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
            }
        }))
        .setup(initialize)
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                if let Err(err) = window.hide() {
                    error!("failed to hide window on close request: {err}");
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            set_settings,
            get_model_directory_path,
            download_model_files,
            get_startup_errors
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
