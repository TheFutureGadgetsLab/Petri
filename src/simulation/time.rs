use std::time;

pub struct Time {
    start: time::Instant,
    last_tick: time::Instant,
}

#[allow(dead_code)]
impl Time {
    pub fn default() -> Time {
        let now = time::Instant::now();
        Time {
            start: now,
            last_tick: now,
        }
    } 

    pub fn tick(&mut self) {
        self.last_tick = time::Instant::now();
    }

    pub fn time_since_start(&self) -> time::Duration {
        return time::Instant::now() - self.start;
    }
    
    pub fn delta_time(&self) -> time::Duration {
        return time::Instant::now() - self.last_tick;
    }

    pub fn now() -> time::Instant {
        return time::Instant::now();
    }
}