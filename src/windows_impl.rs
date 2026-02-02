use windows::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED};

pub(crate) struct KeepScreenOn;

impl KeepScreenOn {
    pub(crate) fn keep_screen_on(enable: bool) {
        unsafe {
            if enable {
                SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED);
            } else {
                SetThreadExecutionState(ES_CONTINUOUS);
            }
        }
    }
}
