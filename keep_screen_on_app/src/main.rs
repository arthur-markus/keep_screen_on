#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("This crate only supports Windows and Linux platforms.");

mod ui;

use eframe::egui;

fn adjust_window_size(original_size: [f32; 2], _scale_factor: f32) -> [f32; 2] {
    #[cfg(all(target_os = "linux", not(feature = "csd")))]
    {
        if ui::is_gnome_desktop() {
            return [
                original_size[0] + (5.0 * _scale_factor),
                original_size[1] + (3.0 * _scale_factor),
            ];
        } else {
            return original_size;
        }
    }

    #[cfg(feature = "csd")]
    {
        [
            original_size[0] + (4.0 * _scale_factor),
            original_size[1] + (27.0 * _scale_factor),
        ]
    }

    #[cfg(all(not(target_os = "linux"), not(feature = "csd")))]
    {
        original_size
    }
}

fn main() -> eframe::Result<()> {
    let scale_factor = display_info::DisplayInfo::all()
        .ok()
        .and_then(|display| display.into_iter().find(|d| d.is_primary))
        .map(|primary| primary.scale_factor)
        .unwrap_or(1.0);

    let base_size = [221.0 * scale_factor, 117.0 * scale_factor];

    let final_size = adjust_window_size(base_size, scale_factor);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(final_size)
            .with_max_inner_size(final_size)
            .with_min_inner_size(final_size)
            .with_resizable(false)
            .with_maximize_button(false)
            .with_decorations(cfg!(not(feature = "csd"))),
        run_and_return: false,
        ..Default::default()
    };

    eframe::run_native(
        "Keep Screen On",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            cc.egui_ctx.set_theme(egui::Theme::Dark);

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
