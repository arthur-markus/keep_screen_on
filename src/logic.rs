use std::sync::{Arc, Mutex, mpsc};

use crate::timer::Timer;

#[derive(Default, PartialEq, Copy, Clone)]
pub struct AppData {
    pub current_mode: CurrentMode,
    pub duration: u32,
    pub duration_unit: DurationUnit,
    pub current_state: CurrentState,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum CurrentState {
    Enabled,
    #[default]
    Disabled,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum CurrentMode {
    #[default]
    Infinite,
    Timed,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum DurationUnit {
    #[default]
    Minutes,
    Hours,
}

#[derive(Default, PartialEq, Clone)]
struct ProcessingResult {
    current_state: CurrentState,
    error: String,
}

pub struct Logic {
    backend: Arc<keep_screen_on_lib::KeepScreenOn>,
    timer: Arc<Mutex<Timer>>,
    processing_result: (
        mpsc::Sender<ProcessingResult>,
        mpsc::Receiver<ProcessingResult>,
    ),
    app_data_sender: Option<mpsc::Sender<AppData>>,
}

impl Logic {
    pub fn new() -> Self {
        Self {
            backend: Arc::new(keep_screen_on_lib::KeepScreenOn::new()),
            timer: Arc::new(Mutex::new(Timer::new())),
            processing_result: mpsc::channel::<ProcessingResult>(),
            app_data_sender: None,
        }
    }

    pub fn toggle_state(&mut self, app_data: AppData) -> Result<CurrentState, anyhow::Error> {
        let app_data = Arc::new(Mutex::new(app_data));
        let current_state = Arc::new(Mutex::new(app_data.lock().unwrap().current_state));
        let last_error = Arc::new(Mutex::new(anyhow::Error::msg("No error")));

        //

        if *last_error.lock().unwrap().to_string() == "No error".to_string() {
            Ok(*current_state.lock().unwrap())
        } else {
            Err(anyhow::Error::msg(last_error.lock().unwrap().to_string()))
        }
    }

    pub fn process_state_change(&mut self) {
        let backend = Arc::clone(&self.backend);
        let timer = Arc::clone(&self.timer);
        let processing_result = self.processing_result.0.clone();

        let (tx, rx) = mpsc::channel::<AppData>();

        std::thread::spawn(move || {
            loop {
                if let Ok(app_data) = rx.try_recv() {
                    let mut result = ProcessingResult {
                        current_state: app_data.current_state,
                        error: String::new(),
                    };

                    match app_data.current_state {
                        CurrentState::Enabled => match backend.disable() {
                            Ok(_) => {
                                result.current_state = CurrentState::Disabled;

                                timer.lock().unwrap().stop();
                            }
                            Err(e) => {
                                result.error = format!("Error resetting screen state: {}", e);
                            }
                        },
                        CurrentState::Disabled => match backend.enable() {
                            Ok(_) => {
                                result.current_state = CurrentState::Enabled;

                                match app_data.current_mode {
                                    CurrentMode::Timed => {
                                        let duration = match app_data.duration_unit {
                                            DurationUnit::Minutes => {
                                                std::time::Duration::from_secs(
                                                    (app_data.duration * 60) as u64,
                                                )
                                            }
                                            DurationUnit::Hours => std::time::Duration::from_secs(
                                                (app_data.duration * 3600) as u64,
                                            ),
                                        };

                                        let backend = Arc::clone(&backend);
                                        timer.lock().unwrap().start(duration, move || backend.disable().unwrap());
                                    }
                                    CurrentMode::Infinite => match backend.enable() {
                                        Ok(_) => {
                                            result.current_state = CurrentState::Enabled;
                                        }
                                        Err(e) => {
                                            result.error =
                                                format!("Error keeping screen on: {}", e);
                                        }
                                    },
                                }
                            }
                            Err(e) => {
                                result.error = format!("Error keeping screen on: {}", e);
                            }
                        },
                    }

                    processing_result
                        .send(result)
                        .expect("Failed to send processing result");
                }
            }
        });

        self.app_data_sender = Some(tx);
    }
}
