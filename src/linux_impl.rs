use dbus::blocking::Connection;
use std::time::Duration;

#[derive(Default)]
pub(crate) struct KeepScreenOn {
    cookie: u32,
}

impl KeepScreenOn {
    pub(crate) fn keep_screen_on(&mut self, enable: bool) {
        let conn = Connection::new_session().expect("Failed to connect to D-Bus session bus");
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
                .expect("Failed to Inhibit the ScreenSaver");
            self.cookie = cookie;
        } else {
            let () = proxy
                .method_call("org.freedesktop.ScreenSaver", "UnInhibit", ())
                .expect("Failed to UnInhibit the ScreenSaver");
        }
    }
}
