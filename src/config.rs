use std::{env, fs::File};

use bevy_ecs::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use ultraviolet::Vec2;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Resource)]
pub struct Config {
    pub n_cells: u32,
    pub cell_radius: f32,
    pub bounds: (Vec2, Vec2),
    pub max_ticks: u128, // 0 = infinite
}

pub fn build_config() -> Config {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a config file");
        println!("Usage: {} <config.ron>", args[0]);
        std::process::exit(1);
    }

    let f = File::open(&args[1]).expect("Failed opening file");
    let config: Config = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {e}");

            std::process::exit(1);
        }
    };

    config
}
