use glam::{vec2, Vec2};

#[derive(Clone, Copy)]
pub struct Config {
    pub num_particles: u32,
    pub bounds: (Vec2, Vec2),
}

impl Config {
    pub fn default() -> Config {
        Config {
            num_particles: 1_000,
            bounds: (vec2(-500.0, -500.0), vec2(500.0, 500.0)),
        }
    }
}
