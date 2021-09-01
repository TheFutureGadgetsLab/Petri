use glam::{vec2, Vec2};

#[derive(Clone, Copy)]
pub struct Config {
    pub n_cells: u32,
    pub cell_radius: f32,
    pub bounds: (Vec2, Vec2),
}

impl Config {
    pub fn default() -> Config {
        Config {
            n_cells: 1_000_000,
            cell_radius: 10.0,
            bounds: (vec2(-500.0, -500.0), vec2(500.0, 500.0)),
        }
    }
}
