use dbus::blocking::Connection;
use std::time::Duration;
use thiserror::Error;

#[derive(Default)]
pub(crate) struct KeepScreenOn {
    cookie: u32,
}

#[derive(Error, Debug)]
pub(crate) enum LinuxError {
    #[error("Failed to connect to D-Bus session bus")]
    ConnectionError,
    #[error("Failed to Inhibit the ScreenSaver")]
    InhibitError,
    #[error("Failed to UnInhibit the ScreenSaver")]
    UnInhibitError,
}

impl KeepScreenOn {
    pub(crate) fn keep_screen_on(&mut self, enable: bool) -> Result<(), LinuxError> {
        let conn = Connection::new_session().map_err(|_| LinuxError::ConnectionError)?;
        let proxy = conn.with_proxy(
            "org.freedesktop.ScreenSaver",
            "/org/freedesktop/ScreenSaver",
            Duration::from_millis(5000),
        );

        if enable {
            let (cookie,) = proxy
                .method_call(
                    "org.freedesktop.ScreenSaver",
                    "Inhibit",
                    ("keep_screen_on", "Preventing screen from sleeping"),
                )
                .map_err(|_| LinuxError::InhibitError)?;
            self.cookie = cookie;

            Ok(())
        } else {
            let () = proxy
                .method_call("org.freedesktop.ScreenSaver", "UnInhibit", (self.cookie,))
                .map_err(|_| LinuxError::UnInhibitError)?;
            self.cookie = 0;

            Ok(())
        }
    }
}
