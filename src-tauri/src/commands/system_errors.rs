use tauri::{State};
use talkful_lib::StartupErrorState;

#[tauri::command]
pub fn get_startup_errors(
    startup_error_state: State<'_, StartupErrorState>,
) -> Vec<String> {
    startup_error_state.all()
}
