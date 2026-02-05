use egui::Widget;

use keep_screen_on_lib::KeepScreenOn;

pub struct AppUI {
    current_mode: CurrentMode,
    current_state: CurrentState,
    duration_value: u32,
    duration_unit: DurationUnit,
    backend: KeepScreenOn,
}

#[derive(Default, PartialEq, Copy, Clone)]
enum CurrentState {
    Enabled,
    #[default]
    Disabled,
}

#[derive(Default, PartialEq, Copy, Clone)]
enum CurrentMode {
    #[default]
    Infinite,
    Timed,
}

#[derive(Default, PartialEq, Copy, Clone)]
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
                                Ok(_) => self.current_state = CurrentState::Disabled,
                                Err(e) => {
                                    eprintln!("Failed to toggle state: {e}");
                                }
                            },
                            CurrentState::Disabled => match self.backend.enable() {
                                Ok(_) => self.current_state = CurrentState::Enabled,
                                Err(e) => {
                                    eprintln!("Failed to toggle state: {e}");
                                }
                            },
                        }
                    }
                });
            });
        });
    }
}
