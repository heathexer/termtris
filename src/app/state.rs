use std::time::Duration;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized { duration: Duration },
}

impl AppState {
    pub fn initialized() -> Self {
        Self::Initialized {
            duration: Duration::from_secs(1),
        }
    }
}
