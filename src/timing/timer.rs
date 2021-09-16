// I hate all of this

use std::{collections::HashMap, fmt, time::Duration};

use hdrhistogram::Histogram;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use quanta::Instant;

lazy_static! {
    pub static ref TIMING_DATABASE: RwLock<HashMap<String, Timer>> = RwLock::new(HashMap::default());
}

#[allow(dead_code)]
pub enum Resolution {
    Second,
    Milli,
    Micro,
    Nano,
}

pub struct Timer {
    pub res_str: String,
    res: Resolution,
    pub timer: Histogram<u64>,
}

impl Timer {
    pub fn new(res: Resolution) -> Self {
        let res_str = match res {
            Resolution::Second => "(s)",
            Resolution::Milli => "(ms)",
            Resolution::Micro => "(us)",
            Resolution::Nano => "(ns)",
        };
        Self {
            timer: Histogram::new(2).unwrap(),
            res,
            res_str: res_str.into(),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        let delta = match self.res {
            Resolution::Second => delta.as_secs(),
            Resolution::Milli => delta.as_millis() as u64,
            Resolution::Micro => delta.as_micros() as u64,
            Resolution::Nano => delta.as_nanos() as u64,
        };
        self.timer.record(delta).unwrap();
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\tmin: {} | mean: {:.2} | max: {}",
            self.timer.min(),
            self.timer.mean(),
            self.timer.max(),
        )
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::new(Resolution::Micro)
    }
}

pub struct DropTimer {
    start: Instant,
    target: String,
}

impl DropTimer {
    pub fn new(target: String) -> Self {
        Self {
            start: Instant::now(),
            target,
        }
    }
}

impl Drop for DropTimer {
    fn drop(&mut self) {
        TIMING_DATABASE
            .write()
            .entry(self.target.clone())
            .or_default()
            .update(Instant::now() - self.start);
    }
}

macro_rules! time_func {
    ($name:expr) => {
        use crate::timing::DropTimer;
        let __drop_timer = DropTimer::new($name.into());
    };
}
pub(crate) use time_func;
