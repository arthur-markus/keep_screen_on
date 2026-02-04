use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct Timer {
    channel: Option<mpsc::Sender<()>>,
}

impl Timer {
    pub fn new() -> Self {
        Self { channel: None }
    }

    pub fn start(&mut self, duration: Duration, mut callback: impl FnMut() + Send + 'static) {
        let start_time = Instant::now();
        let (tx, rx) = mpsc::channel::<()>();
        self.channel = Some(tx);

        std::thread::spawn(move || {
            loop {
                if rx.try_recv().is_ok() {
                    break;
                }

                if Instant::now().duration_since(start_time).as_secs() >= duration.as_secs() {
                    callback();
                    break;
                }

                std::thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn stop(&mut self) {
        if self.channel.is_some() {
            self.channel.take().unwrap().send(()).unwrap();
            self.channel = None;
        }
    }
}
