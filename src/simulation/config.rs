use glam::{vec2, Vec2};

#[derive(Clone, Copy)]
pub struct Config {
    pub num_particles: u32,
    pub bounds: (Vec2, Vec2),
}

impl Config {
    pub fn default() -> Config {
        Config {
            num_particles: 100_000,
            bounds: (vec2(-1000.0, -1000.0), vec2(1000.0, 1000.0)),
        }
    }
}
