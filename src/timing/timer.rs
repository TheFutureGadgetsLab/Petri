use std::{collections::HashMap, time::Duration};

use hdrhistogram::Histogram;
use lazy_static::lazy_static;
use quanta::Instant;
use spin::RwLock;

lazy_static! {
    pub static ref TIMING_DATABASE: RwLock<HashMap<String, Timer>> = RwLock::new(HashMap::default());
}

pub struct Timer {
    hist: Histogram<u64>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            hist: Histogram::new(2).unwrap(),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        let delta = delta.as_nanos() as u64;
        self.hist.record(delta).unwrap();
    }

    pub fn min(&self) -> (u64, String) {
        self.time_to_readable(self.hist.min())
    }

    pub fn max(&self) -> (u64, String) {
        self.time_to_readable(self.hist.max())
    }

    pub fn mean(&self) -> (u64, String) {
        self.time_to_readable(self.hist.mean() as u64)
    }

    pub fn raw_imean(&self) -> u64 {
        self.hist.mean() as u64
    }

    pub fn reset(&mut self) {
        self.hist.reset();
    }

    fn time_to_readable(&self, nano_seconds: u64) -> (u64, String) {
        let (time, unit) = if nano_seconds >= 1_000_000_000 {
            (nano_seconds / 1_000_000_000, "s")
        } else if nano_seconds >= 1_000_000 {
            (nano_seconds / 1_000_000, "ms")
        } else if nano_seconds >= 1_000 {
            (nano_seconds / 1_000, "us")
        } else {
            (nano_seconds, "ns")
        };
        (time, String::from(unit))
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::new()
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
