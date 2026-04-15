use talkful_lib::logging::{read_log_error_messages, LogState};
use tauri::State;

#[tauri::command]
pub fn get_log_errors(log_state: State<'_, LogState>) -> Result<Vec<String>, String> {
    read_log_error_messages(log_state.log_path()).map_err(|error| format!("{:#}", error))
}
