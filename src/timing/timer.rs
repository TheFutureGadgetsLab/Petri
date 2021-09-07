// I hate all of this

use std::{
    fmt,
    time::{Duration, Instant},
};

use hdrhistogram::Histogram;

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
    timer: Histogram<u64>,
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
        Self {
            timer: Histogram::new(3).unwrap(),
            res: Resolution::Milli,
            res_str: "(ms)".into(),
        }
    }
}

pub struct DropTimer<'a> {
    start: Instant,
    target: &'a mut Timer,
}

impl<'a> DropTimer<'a> {
    pub fn new(target: &'a mut Timer) -> Self {
        Self {
            start: Instant::now(),
            target: target,
        }
    }
}

impl<'a> Drop for DropTimer<'a> {
    fn drop(&mut self) {
        self.target.update(Instant::now() - self.start);
    }
}

macro_rules! time_func {
    ($module:ident,$stage:ident) => {
        use crate::timing::{DropTimer, TIMING_DATABASE};
        let mut __timer_data = unsafe { TIMING_DATABASE.data_ptr().as_mut().unwrap() };
        let __drop_timer = DropTimer::new(&mut __timer_data.$module.$stage);
    };
}
pub(crate) use time_func;
