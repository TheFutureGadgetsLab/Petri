#[derive(Clone, Copy)]
pub struct Config {
    pub num_particles: u32,
}

impl Config {
    pub fn default() -> Config {
        Config {
            num_particles: 3_000_000,
        }
    }
}