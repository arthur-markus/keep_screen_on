use std::time::{Duration, Instant};

use egui::Widget;

#[cfg(feature = "csd")]
use egui_material_icons::icons::*;

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
            backend: KeepScreenOn::default(),
            wrap_up_time: Instant::now(),
            status_text: "Ready".into(),
            did_error_occur: false,
        }
    }

    fn clean_up(&mut self) {
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

    #[cfg(feature = "csd")]
    fn window_decorations(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(2)
                    .fill(egui::Color32::from_rgb(0, 0, 0)),
            )
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        let titlebar_interaction = ui.interact(
                            ui.max_rect(),
                            ui.id().with("drag_area"),
                            egui::Sense::click_and_drag(),
                        );

                        if titlebar_interaction.drag_started_by(egui::PointerButton::Primary) {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }

                        let title_interaction = ui
                            .add(
                                egui::Label::new(
                                    egui::RichText::new("Keep Screen On")
                                        .heading()
                                        .color(egui::Color32::from_rgb(210, 210, 210)),
                                )
                                .selectable(false),
                            )
                            .interact(egui::Sense::click_and_drag());

                        if title_interaction.drag_started_by(egui::PointerButton::Primary) {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            if ui
                                .add(egui::Button::new(ICON_CLOSE).fill(egui::Color32::TRANSPARENT))
                                .clicked()
                            {
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }

                            if ui
                                .add(
                                    egui::Button::new(ICON_MINIMIZE)
                                        .fill(egui::Color32::TRANSPARENT),
                                )
                                .clicked()
                            {
                                ui.ctx()
                                    .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                            }
                        });
                    });

                    ui.horizontal_centered(|ui| {
                        self.client_area(
                            ui,
                            egui::Frame::default()
                                .inner_margin(4)
                                .fill(egui::Color32::from_rgb(20, 20, 20)),
                        );
                    });
                });
            });
    }

    fn client_area(&mut self, ui: &mut egui::Ui, frame: egui::Frame) {
        let ctx = ui.ctx();

        if ctx.input(|i| i.viewport().maximized.unwrap_or(false)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }

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

        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            ui.vertical(|ui| {
                ui.add_enabled_ui(self.current_state == CurrentState::Disabled, |ui| {
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
                        ui.add_enabled_ui(self.current_mode == CurrentMode::Timed, |ui| {
                            ui.add(egui::Label::new("Duration: ").selectable(false));

                            if self.duration_unit == DurationUnit::Hours {
                                self.duration_value = std::cmp::min(12, self.duration_value);
                            }

                            egui::DragValue::new(&mut self.duration_value)
                                .range(match self.duration_unit {
                                    DurationUnit::Minutes => 1..=60,
                                    DurationUnit::Hours => 1..=12,
                                })
                                .ui(ui);

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

                ui.add(
                    egui::Label::new(egui::RichText::new(self.status_text.clone()).color(
                        match self.did_error_occur {
                            true => egui::Color32::RED,
                            false => egui::Color32::GREEN,
                        },
                    ))
                    .selectable(false),
                );
            });
        });
    }
}

impl eframe::App for AppUI {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        #[cfg(feature = "csd")]
        {
            self.window_decorations(ui);
        }

        #[cfg(not(feature = "csd"))]
        {
            self.client_area(ui, egui::Frame::default().inner_margin(6));
        }
    }

    fn on_exit(&mut self) {
        self.clean_up();
    }
}
