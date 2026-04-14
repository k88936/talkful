use tauri::{State, Window};
use talkful_lib::StartupErrorState;

#[tauri::command]
pub fn get_startup_errors(
    window: Window,
    startup_error_state: State<'_, StartupErrorState>,
) -> Vec<String> {
    if window.label() != "main" {
        return Vec::new();
    }

    startup_error_state.all()
}
