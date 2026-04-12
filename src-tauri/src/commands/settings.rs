use tauri::State;

use talkful_lib::config::{AppConfig, DotfileConfigStore, IConfigStore};

#[tauri::command]
pub fn get_settings(config_store: State<'_, DotfileConfigStore>) -> AppConfig {
    config_store.get()
}

#[tauri::command]
pub fn set_settings(
    new_config: AppConfig,
    config_store: State<'_, DotfileConfigStore>,
) -> Result<AppConfig, String> {
    config_store
        .set(new_config)
        .map_err(|error| error.to_string())
}
