use anyhow::Context;
use dbus::blocking::Connection;
use std::time::Duration;

#[derive(Default)]
pub(crate) struct KeepScreenOn {
    cookie: u32,
}

impl KeepScreenOn {
    pub(crate) fn keep_screen_on(&mut self, enable: bool) -> Result<(), anyhow::Error> {
        let conn = Connection::new_session().context("Failed to connect to D-Bus")?;
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
                .context("Failed to Inhibit the ScreenSaver")?;
            self.cookie = cookie;

            Ok(())
        } else {
            let () = proxy
                .method_call("org.freedesktop.ScreenSaver", "UnInhibit", (self.cookie,))
                .context("Failed to UnInhibit the ScreenSaver")?;
            self.cookie = 0;

            Ok(())
        }
    }
}
