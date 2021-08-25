use std::time::Instant;

use fps_counter::FPSCounter;

pub struct Stats
{
    pub fps: usize,
    pub tick: usize,
    fps_counter: FPSCounter,
    
    start_time: Instant,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            fps: 0,
            tick: 0,
            fps_counter: FPSCounter::default(),
            start_time: Instant::now(),
        }
    }
}

impl Stats {
    pub fn tick(&mut self) {
        self.tick += 1;
        self.fps = self.fps_counter.tick();
    }

    pub fn secs_since_start(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
}