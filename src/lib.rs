#[cfg(target_os = "linux")]
use std::sync::Mutex;

#[cfg(target_os = "windows")]
mod windows_impl;

#[cfg(target_os = "linux")]
mod linux_impl;

pub struct KeepScreenOn {
    #[cfg(target_os = "linux")]
    keep_screen_on: Mutex<linux_impl::KeepScreenOn>,
}

impl KeepScreenOn {
    pub fn new() -> Self {
        #[cfg(target_os = "windows")]
        {
            Self
        }

        #[cfg(target_os = "linux")]
        {
            Self {
                keep_screen_on: Mutex::new(linux_impl::KeepScreenOn::default()),
            }
        }
    }

    pub fn enable(&self) -> Result<(), anyhow::Error> {
        #[cfg(target_os = "windows")]
        {
            windows_impl::ScreenKeepOn::keep_screen_on(true);

            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            let mut screen_keep_on = self.keep_screen_on.lock().unwrap();
            screen_keep_on.keep_screen_on(true)?;

            Ok(())
        }
    }

    pub fn disable(&self) -> Result<(), anyhow::Error> {
        #[cfg(target_os = "windows")]
        {
            windows_impl::ScreenKeepOn::keep_screen_on(false);

            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            let mut screen_keep_on = self.keep_screen_on.lock().unwrap();
            screen_keep_on.keep_screen_on(false)?;

            Ok(())
        }
    }
}
