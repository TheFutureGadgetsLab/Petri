use std::{fmt, time::Duration};

use hdrhistogram::Histogram;

#[allow(dead_code)]
pub enum Resolution {
    SECOND,
    MILLI,
    MICRO,
    NANO,
}

pub struct Timer {
    pub res_str: String,
    res: Resolution,
    timer: Histogram<u64>,
}

impl Timer {
    pub fn new(res: Resolution) -> Self {
        let res_str = match res {
            Resolution::SECOND => "(s)",
            Resolution::MILLI => "(ms)",
            Resolution::MICRO => "(us)",
            Resolution::NANO => "(ns)",
        };
        Self {
            timer: Histogram::new(3).unwrap(),
            res: res,
            res_str: res_str.into(),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        let delta = match self.res {
            Resolution::SECOND => delta.as_secs(),
            Resolution::MILLI => delta.as_millis() as u64,
            Resolution::MICRO => delta.as_micros() as u64,
            Resolution::NANO => delta.as_nanos() as u64,
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
        Self {
            timer: Histogram::new(3).unwrap(),
            res: Resolution::MILLI,
            res_str: "(ms)".into(),
        }
    }
}
