#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("This crate only supports Windows and Linux platforms.");

mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    #[cfg(feature = "csd")]
    let window_size = [225.0, 135.0];

    #[cfg(not(feature = "csd"))]
    let window_size = [237.0, 110.0];

    let viewport = egui::ViewportBuilder::default()
        .with_inner_size(window_size)
        .with_max_inner_size(window_size)
        .with_min_inner_size(window_size)
        .with_resizable(false)
        .with_maximize_button(false);

    let options = eframe::NativeOptions {
        viewport: match cfg!(feature = "csd") {
            true => viewport.with_decorations(false),
            false => viewport,
        },
        run_and_return: false,
        ..Default::default()
    };

    #[cfg_attr(not(feature = "csd"), allow(unused_variables))]
    eframe::run_native(
        "Keep Screen On",
        options,
        Box::new(|cc| {
            #[cfg(feature = "csd")]
            {
                egui_material_icons::initialize(&cc.egui_ctx);
                cc.egui_ctx.add_font(egui_material_icons::font_insert());
            }

            Ok(Box::new(ui::AppUI::new()))
        }),
    )?;

    Ok(())
}
