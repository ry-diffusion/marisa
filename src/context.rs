use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct Context {
    enabled: AtomicBool,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, val: bool) {
        self.enabled.store(val, Ordering::Relaxed)
    }
}
