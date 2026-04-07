use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RuntimeState {
    Idle,
    Recording,
    Processing,
    Error,
}

impl RuntimeState {
    pub(crate) fn label(self) -> &'static str {
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
pub(crate) enum RuntimeEvent {
    HotkeyToggle,
    HotkeyPress,
    HotkeyRelease,
    HotkeyTimeout,
    ProcessComplete,
    RuntimeFailure,
    ResetError,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TransitionError {
    pub(crate) from: RuntimeState,
    pub(crate) event: RuntimeEvent,
}

pub(crate) fn transition(
    from: RuntimeState,
    event: RuntimeEvent,
) -> Result<RuntimeState, TransitionError> {
    match (from, event) {
        (RuntimeState::Idle, RuntimeEvent::HotkeyPress) => Ok(RuntimeState::Recording),
        (RuntimeState::Recording, RuntimeEvent::HotkeyRelease)
        | (RuntimeState::Recording, RuntimeEvent::HotkeyTimeout) => Ok(RuntimeState::Processing),
        (RuntimeState::Processing, RuntimeEvent::ProcessComplete) => Ok(RuntimeState::Idle),
        (_, RuntimeEvent::RuntimeFailure) => Ok(RuntimeState::Error),
        (RuntimeState::Error, RuntimeEvent::ResetError)
        | (RuntimeState::Error, RuntimeEvent::HotkeyToggle) => Ok(RuntimeState::Idle),
        _ => Err(TransitionError { from, event }),
    }
}
