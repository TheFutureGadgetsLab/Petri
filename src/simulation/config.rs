use std::fs::File;

use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use ultraviolet::Vec2;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Config {
    pub n_cells: u32,
    pub cell_radius: f32,
    pub bounds: (Vec2, Vec2),
}

pub fn read_config() -> Config {
    let input_path = format!("{}/configs/600k.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path).expect("Failed opening file");
    let config: Config = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    config
}
