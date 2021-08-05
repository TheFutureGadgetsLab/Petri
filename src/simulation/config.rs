#[derive(Clone, Copy)]
pub struct Config {
    pub num_particles: u32,
    pub boundary: BoundaryConfig,
}

impl Config {
    pub fn default() -> Config {
        Config {
            num_particles: 5_000,
            boundary: BoundaryConfig::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoundaryConfig {
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
}

impl BoundaryConfig {
    pub fn default() -> BoundaryConfig {
        BoundaryConfig {
            width: 500.,
            height: 500.,
            thickness: 1.,
        }
    }
}