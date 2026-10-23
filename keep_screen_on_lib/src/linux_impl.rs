use anyhow::Context;
use zbus::{Connection, Result, proxy};

#[proxy(
    interface = "org.freedesktop.ScreenSaver",
    default_service = "org.freedesktop.ScreenSaver",
    default_path = "/org/freedesktop/ScreenSaver"
)]
trait ScreenSaver {
    fn Inhibit(&self, application_name: String, reason_for_inhibit: String) -> Result<u32>;
    fn UnInhibit(&self, cookie: u32) -> Result<()>;
}

pub(crate) struct KeepScreenOn {
    cookie: u32,
    conn: Connection,
}

impl KeepScreenOn {
    pub(crate) fn new() -> Self {
        Self {
            cookie: 0,
            conn: zbus::block_on(async {
                Connection::session()
                    .await
                    .expect("Failed to connect to D-Bus")
            }),
        }
    }

    pub(crate) fn keep_screen_on(
        &mut self,
        enable: bool,
    ) -> std::result::Result<(), anyhow::Error> {
        zbus::block_on(async {
            let proxy = ScreenSaverProxy::new(&self.conn)
                .await
                .expect("Failed to create a D-Bus Proxy");

            if enable {
                self.cookie = proxy
                    .Inhibit(
                        "Keep Screen On".into(),
                        "Preventing screen from sleeping".into(),
                    )
                    .await
                    .context("Failed to inhibit screen saver")?;

                Ok(())
            } else {
                proxy
                    .UnInhibit(self.cookie)
                    .await
                    .context("Failed to uninhibit screen saver")?;

                self.cookie = 0;

                Ok(())
            }
        })
    }
}
