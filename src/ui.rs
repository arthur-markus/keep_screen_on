use egui::Widget;

use crate::logic;

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

pub struct AppUI {
    current_mode: CurrentMode,
    duration: u32,
    duration_unit: DurationUnit,
    current_state: CurrentState,
    logic: logic::Logic,
}

impl AppUI {
    pub fn new() -> Self {
        Self {
            current_mode: CurrentMode::default(),
            duration: 1,
            duration_unit: DurationUnit::default(),
            current_state: CurrentState::default(),
            logic: logic::Logic::new(),
        }
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                        egui::DragValue::new(&mut self.duration)
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
                            CurrentState::Enabled => match self.logic.reset() {
                                Ok(_) => self.current_state = CurrentState::Disabled,
                                Err(e) => {
                                    eprintln!("Error resetting screen state: {}", e);
                                }
                            },
                            CurrentState::Disabled => {
                                match self.logic.keep_screen_on(
                                    if self.current_mode == CurrentMode::Timed {
                                        let duration_secs = match self.duration_unit {
                                            DurationUnit::Minutes => self.duration as u64 * 60,
                                            DurationUnit::Hours => self.duration as u64 * 60 * 60,
                                        };
                                        Some(duration_secs)
                                    } else {
                                        None
                                    },
                                ) {
                                    Ok(_) => self.current_state = CurrentState::Enabled,
                                    Err(e) => {
                                        eprintln!("Error keeping screen on: {}", e);
                                    }
                                }
                            }
                        }
                    }
                });
            });
        });
    }
}
