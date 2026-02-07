use std::time::{Duration, Instant};

use egui::Widget;

use keep_screen_on_lib::KeepScreenOn;

pub struct AppUI {
    current_mode: CurrentMode,
    current_state: CurrentState,
    duration_value: u32,
    duration_unit: DurationUnit,
    backend: KeepScreenOn,
    wrap_up_time: Instant,
    status_text: String,
    did_error_occur: bool,
}

#[derive(Default, PartialEq)]
enum CurrentState {
    Enabled,
    #[default]
    Disabled,
}

#[derive(Default, PartialEq)]
enum CurrentMode {
    #[default]
    Infinite,
    Timed,
}

#[derive(Default, PartialEq)]
enum DurationUnit {
    #[default]
    Minutes,
    Hours,
}

impl AppUI {
    pub fn new() -> Self {
        Self {
            current_mode: CurrentMode::default(),
            current_state: CurrentState::default(),
            duration_value: 1,
            duration_unit: DurationUnit::default(),
            backend: KeepScreenOn::new(),
            wrap_up_time: Instant::now(),
            status_text: "Ready".into(),
            did_error_occur: false,
        }
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.current_state == CurrentState::Enabled && self.current_mode == CurrentMode::Timed {
            if Instant::now() >= self.wrap_up_time {
                match self.backend.disable() {
                    Ok(_) => {
                        self.current_state = CurrentState::Disabled;
                        self.status_text = "Successfully Deactivated".into();
                        self.did_error_occur = false;
                    }
                    Err(_) => {
                        self.status_text = "Failed to toggle state".into();
                        self.did_error_occur = true;
                    }
                }
            }

            ctx.request_repaint_after(self.wrap_up_time.duration_since(Instant::now()));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.radio_value(
                    &mut self.current_mode,
                    CurrentMode::Infinite,
                    "Infinite Mode",
                );

                ui.radio_value(
                    &mut self.current_mode,
                    CurrentMode::Timed,
                    "Time-Limited Mode",
                );

                ui.horizontal(|ui| {
                    ui.label("Duration: ");

                    ui.add_enabled_ui(self.current_mode == CurrentMode::Timed, |ui| {
                        egui::DragValue::new(&mut self.duration_value)
                            .range(match self.duration_unit {
                                DurationUnit::Minutes => 1..=60,
                                DurationUnit::Hours => 1..=12,
                            })
                            .ui(ui)
                    });

                    ui.add_enabled_ui(self.current_mode == CurrentMode::Timed, |ui| {
                        egui::ComboBox::from_label("")
                            .selected_text(match self.duration_unit {
                                DurationUnit::Minutes => "Minutes",
                                DurationUnit::Hours => "Hours",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.duration_unit,
                                    DurationUnit::Minutes,
                                    "Minutes",
                                );
                                ui.selectable_value(
                                    &mut self.duration_unit,
                                    DurationUnit::Hours,
                                    "Hours",
                                );
                            });
                    });
                });

                ui.vertical_centered(|ui| {
                    if ui
                        .button(match self.current_state {
                            CurrentState::Enabled => "Deactivate",
                            CurrentState::Disabled => "Activate",
                        })
                        .clicked()
                    {
                        match self.current_state {
                            CurrentState::Enabled => match self.backend.disable() {
                                Ok(_) => {
                                    self.current_state = CurrentState::Disabled;
                                    self.status_text = "Successfully Deactivated".into();
                                    self.did_error_occur = false;
                                }
                                Err(_) => {
                                    self.status_text = "Failed to toggle state".into();
                                    self.did_error_occur = true;
                                }
                            },
                            CurrentState::Disabled => match self.backend.enable() {
                                Ok(_) => {
                                    self.wrap_up_time = match self.duration_unit {
                                        DurationUnit::Minutes => {
                                            Instant::now()
                                                + Duration::from_mins(self.duration_value.into())
                                        }
                                        DurationUnit::Hours => {
                                            Instant::now()
                                                + Duration::from_hours(self.duration_value.into())
                                        }
                                    };

                                    self.current_state = CurrentState::Enabled;
                                    self.status_text = "Successfully Activated".into();
                                    self.did_error_occur = false;
                                }
                                Err(_) => {
                                    self.status_text = "Failed to toggle state".into();
                                    self.did_error_occur = true;
                                }
                            },
                        }
                    }
                });

                ui.label(egui::RichText::new(self.status_text.clone()).color(
                    match self.did_error_occur {
                        true => egui::Color32::RED,
                        false => egui::Color32::GREEN,
                    },
                ));
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if self.current_state == CurrentState::Enabled {
            match self.backend.disable() {
                Ok(_) => {
                    self.current_state = CurrentState::Disabled;
                    self.status_text = "Successfully Deactivated".into();
                    self.did_error_occur = false;
                }
                Err(_) => {
                    self.status_text = "Failed to toggle state".into();
                    self.did_error_occur = true;
                }
            }
        }
    }
}
