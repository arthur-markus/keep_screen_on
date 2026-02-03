use keep_screen_on_lib::KeepScreenOn;

pub struct Logic {
    backend: KeepScreenOn,
}

impl Logic {
    pub fn new() -> Self {
        Self {
            backend: KeepScreenOn::new(),
        }
    }

    pub fn keep_screen_on(&self, duration_secs: Option<u64>) -> Result<(), String> {
        todo!()
    }

    pub fn reset(&self) -> Result<(), String> {
        todo!()
    }
}
