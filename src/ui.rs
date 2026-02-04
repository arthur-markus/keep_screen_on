use egui::Widget;

use crate::logic::{AppData, CurrentMode, CurrentState, DurationUnit, Logic};

pub struct AppUI {
    logic: Logic,
    app_data: AppData,
    need_to_toggle_state: bool,
}

impl AppUI {
    pub fn new() -> Self {
        let mut logic = Logic::new();

        logic.process_state_change();

        Self {
            logic,
            app_data: AppData::default(),
            need_to_toggle_state: false,
        }
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.need_to_toggle_state {
            match self.logic.toggle_state(self.app_data) {
                Ok(result) => {
                    self.app_data.current_state = result;
                    self.need_to_toggle_state = false;
                }
                Err(e) => {
                    panic!("Error toggling state: {}", e);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.radio_value(
                    &mut self.app_data.current_mode,
                    CurrentMode::Infinite,
                    "Infinite Mode",
                );

                ui.radio_value(
                    &mut self.app_data.current_mode,
                    CurrentMode::Timed,
                    "Time-Limited Mode",
                );

                ui.horizontal(|ui| {
                    ui.label("Duration: ");

                    ui.add_enabled_ui(self.app_data.current_mode == CurrentMode::Timed, |ui| {
                        egui::DragValue::new(&mut self.app_data.duration)
                            .range(match self.app_data.duration_unit {
                                DurationUnit::Minutes => 1..=60,
                                DurationUnit::Hours => 1..=12,
                            })
                            .ui(ui)
                    });

                    ui.add_enabled_ui(self.app_data.current_mode == CurrentMode::Timed, |ui| {
                        egui::ComboBox::from_label("")
                            .selected_text(match self.app_data.duration_unit {
                                DurationUnit::Minutes => "Minutes",
                                DurationUnit::Hours => "Hours",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.app_data.duration_unit,
                                    DurationUnit::Minutes,
                                    "Minutes",
                                );
                                ui.selectable_value(
                                    &mut self.app_data.duration_unit,
                                    DurationUnit::Hours,
                                    "Hours",
                                );
                            });
                    });
                });

                ui.vertical_centered(|ui| {
                    if ui
                        .button(match self.app_data.current_state {
                            CurrentState::Enabled => "Deactivate",
                            CurrentState::Disabled => "Activate",
                        })
                        .clicked()
                    {
                        self.need_to_toggle_state = true;
                    }
                });
            });
        });
    }
}
