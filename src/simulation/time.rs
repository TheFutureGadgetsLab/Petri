use std::time::{Duration, Instant};

use fps_counter::FPSCounter;

pub struct Time {
    pub tick: u128,
    pub start_time: Instant,
    pub last_tick_time: Instant,

    pub tick_rate: usize,
    tick_counter: FPSCounter,
}

impl Default for Time {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            tick: 0,
            start_time: now,
            last_tick_time: now,
            tick_rate: 0,
            tick_counter: FPSCounter::default(),
        }
    }
}

#[allow(dead_code)]
impl Time {
    pub fn tick(&mut self) {
        self.tick += 1;
        self.tick_rate = self.tick_counter.tick();
        self.last_tick_time = Instant::now();
    }

    pub fn time_since_start(&self) -> Duration {
        return Instant::now() - self.start_time;
    }

    pub fn delta_time(&self) -> Duration {
        return Instant::now() - self.last_tick_time;
    }

    pub fn now() -> Instant {
        return Instant::now();
    }
}
