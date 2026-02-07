#[cfg(target_os = "windows")]
mod windows_impl;

#[cfg(target_os = "linux")]
mod linux_impl;

pub struct KeepScreenOn {
    #[cfg(target_os = "linux")]
    keep_screen_on: linux_impl::KeepScreenOn,
}

impl Default for KeepScreenOn {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        {
            Self
        }

        #[cfg(target_os = "linux")]
        {
            Self {
                keep_screen_on: linux_impl::KeepScreenOn::new(),
            }
        }
    }
}

impl KeepScreenOn {
    pub fn enable(&mut self) -> Result<(), anyhow::Error> {
        #[cfg(target_os = "windows")]
        {
            windows_impl::ScreenKeepOn::keep_screen_on(true);

            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            self.keep_screen_on.keep_screen_on(true)?;

            Ok(())
        }
    }

    pub fn disable(&mut self) -> Result<(), anyhow::Error> {
        #[cfg(target_os = "windows")]
        {
            windows_impl::ScreenKeepOn::keep_screen_on(false);

            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            self.keep_screen_on.keep_screen_on(false)?;

            Ok(())
        }
    }
}
