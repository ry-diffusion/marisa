use std::{
    sync::{
        atomic::{AtomicBool, AtomicI64, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};

pub struct Context {
    enabled: AtomicBool,
    history: Mutex<Vec<Duration>>,
    current: Mutex<Option<Instant>>,
    timeout: Mutex<Option<Instant>>,
    last_executed_num: AtomicI64,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            last_executed_num: AtomicI64::new(0),
            history: Mutex::new(vec![]),
            current: Mutex::new(None),
            timeout: Mutex::new(None),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, val: bool) {
        self.enabled.store(val, Ordering::Relaxed)
    }

    pub fn replace_click(&self) {
        *self.current.try_lock().unwrap() = Some(Instant::now());
    }

    pub fn update_average(&self) {
        let mut avg = if let Ok(l) = self.history.try_lock() {
            l
        } else {
            return;
        };

        // max samples
        if avg.len() > 500 {
            avg.clear();
        }

        if let Some(c) = self.current.try_lock().unwrap().take() {
            if c.elapsed() > Duration::from_millis(2) {
                avg.push(c.elapsed());
            }
        }
    }

    pub fn get_min(&self) -> Duration {
        let mut avg = if let Ok(l) = self.history.lock() {
            l
        } else {
            return Duration::from_millis(50);
        };

        let get = |a: &[Duration]| *a.iter().min().unwrap_or(&Duration::from_micros(50));
        let t = get(&avg);
        if t > Duration::from_millis(2) {
            if let Some(pos) = avg.iter().position(|x| x == &t) {
                avg.remove(pos);
            }

            t
        } else {
            if let Some(pos) = avg.iter().position(|x| x == &t) {
                avg.remove(pos);
            }

            get(&avg)
        }
    }

    pub fn execute(&self) {
        self.last_executed_num.store(
            self.last_executed_num.load(Ordering::Relaxed) + 1,
            Ordering::Relaxed,
        );
    }

    pub fn can_execute(&self) -> bool {
        if self.last_executed_num.load(Ordering::Relaxed) >= 30 {
            let mut timeout = self.timeout.try_lock().unwrap();

            if let Some(t) = timeout.as_ref() {
                if t.elapsed() > Duration::from_millis(100) {
                    self.last_executed_num.store(0, Ordering::Relaxed);
                }
            } else {
                *timeout = Some(Instant::now())
            }

            false
        } else {
            true
        }
    }
}
