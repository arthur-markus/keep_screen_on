#[derive(Default)]
pub struct AppUI;

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //
        });
    }
}
