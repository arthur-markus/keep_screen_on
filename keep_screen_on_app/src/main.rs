mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let window_size = [240.0, 115.0];

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_max_inner_size(window_size)
            .with_min_inner_size(window_size)
            .with_resizable(false)
            .with_maximize_button(false),
        run_and_return: false,
        ..Default::default()
    };

    eframe::run_native(
        "Keep Screen On",
        options,
        Box::new(|_| Ok(Box::new(ui::AppUI::new()))),
    )?;

    Ok(())
}
