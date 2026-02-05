mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([240.0, 100.0])
            .with_max_inner_size([240.0, 100.0])
            .with_min_inner_size([240.0, 100.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "Keep Screen On",
        options,
        Box::new(|_cc| Ok(Box::new(ui::AppUI::new()))),
    )?;

    Ok(())
}
