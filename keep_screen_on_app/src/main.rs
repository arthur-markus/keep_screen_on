#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("This crate only supports Windows and Linux platforms.");

mod ui;

use eframe::egui;

#[cfg(all(target_os = "linux", not(feature = "csd")))]
fn is_gnome_desktop() -> bool {
    for env_var in &[
        "XDG_MENU_PREFIX",
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
        "GDMSESSION",
    ] {
        if let Ok(value) = std::env::var(env_var) {
            if value.to_lowercase().contains("gnome") {
                return true;
            }
        }
    }

    false
}

fn adjust_window_size(original_size: [f32; 2]) -> [f32; 2] {
    #[cfg(all(target_os = "linux", not(feature = "csd")))]
    {
        if is_gnome_desktop() {
            return [original_size[0] + 5.0, original_size[1] + 3.0];
        } else {
            return original_size;
        }
    }

    #[cfg(feature = "csd")]
    {
        return [original_size[0] + 4.0, original_size[1] + 27.0];
    }

    #[cfg(all(not(target_os = "linux"), not(feature = "csd")))]
    {
        return original_size;
    }
}

fn main() -> eframe::Result<()> {
    let base_size = [221.0, 110.0];

    let final_size = adjust_window_size(base_size);

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
        Box::new(|_cc| {
            #[cfg(feature = "csd")]
            {
                egui_material_icons::initialize(&_cc.egui_ctx);
                _cc.egui_ctx.add_font(egui_material_icons::font_insert());
            }

            Ok(Box::new(ui::AppUI::new()))
        }),
    )?;

    Ok(())
}
