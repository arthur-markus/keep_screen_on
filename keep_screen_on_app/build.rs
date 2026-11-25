use image::imageops::FilterType;
use std::{env, fs::File, io, path::PathBuf};

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let ico_path = out_dir.join("icon.ico");

        // Load your high-res source PNG (ideally 256x256 or 512x512)
        let src = image::open("assets/icon.png").expect("Failed to open assets/icon.png");

        // Create a multi-resolution ICO directory
        let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

        for size in [16, 32, 48, 64, 128, 256] {
            let resized = src
                .resize_exact(size, size, FilterType::Lanczos3)
                .to_rgba8();
            let icon_image = ico::IconImage::from_rgba_data(size, size, resized.into_raw());
            let entry = ico::IconDirEntry::encode(&icon_image).expect("Failed to encode ICO entry");
            icon_dir.add_entry(entry);
        }

        // Write the multi-size .ico to OUT_DIR
        let ico_file = File::create(&ico_path)?;
        icon_dir.write(ico_file)?;

        // Embed the .ico into the Windows executable
        let mut res = winresource::WindowsResource::new();
        res.set_icon(ico_path.to_str().unwrap());
        res.compile()?;
    }

    println!("cargo:rerun-if-changed=assets/icon.png");
    Ok(())
}
