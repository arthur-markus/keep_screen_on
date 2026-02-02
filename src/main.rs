mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Keep Screen On",
        options,
        Box::new(|_cc| Ok(Box::new(ui::AppUI::default()))),
    )?;

    Ok(())
}
