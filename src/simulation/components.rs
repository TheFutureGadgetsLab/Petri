use glam::{vec2, Vec2};
use rand::prelude::*;

use super::Config;

#[derive(Clone, Copy)]
pub struct RigidCircle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub to_pos: Vec2,
    pub to_vel: Vec2,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub val: [f32; 4],
}

impl RigidCircle {
    pub fn new_rand(config: &Config) -> Self {
        let bounds = config.bounds;

        let pos = vec2(
            thread_rng().gen_range(bounds.0.x..bounds.1.x),
            thread_rng().gen_range(bounds.0.y..bounds.1.y),
        );

        let vel = vec2(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0));

        Self {
            pos,
            to_pos: pos,
            vel,
            to_vel: vel,
            radius: config.cell_radius,
        }
    }
}

impl Color {
    pub fn new_rand() -> Self {
        Self {
            val: [
                thread_rng().gen_range(0.0..1.0),
                thread_rng().gen_range(0.0..1.0),
                thread_rng().gen_range(0.0..1.0),
                1.0,
            ],
        }
    }
}
