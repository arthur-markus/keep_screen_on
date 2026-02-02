#[cfg(target_os = "windows")]
mod windows_impl;

#[cfg(target_os = "linux")]
mod linux_impl;

pub fn keep_screen_on(enable: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        windows_impl::ScreenKeepOn::keep_screen_on(enable);

        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        use lazy_static::lazy_static;
        use std::sync::Mutex;

        lazy_static! {
            static ref SCREEN_KEEP_ON: Mutex<linux_impl::KeepScreenOn> =
                Mutex::new(linux_impl::KeepScreenOn::default());
        }

        let mut screen_keep_on = SCREEN_KEEP_ON.lock().unwrap();
        screen_keep_on
            .keep_screen_on(enable)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
