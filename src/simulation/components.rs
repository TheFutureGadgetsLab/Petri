use rand::prelude::*;
use crate::simulation::Config;
use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidCircle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: [f32; 4],
}

impl RigidCircle {
    pub fn new_rand(_conf: Config, radius: f32) -> RigidCircle {
        RigidCircle {
            pos: Vec2::ZERO,
            vel: Vec2::new(
                thread_rng().gen_range(-0.001..0.001),
                thread_rng().gen_range(-0.001..0.001)
            ),
            radius,
            color: [thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0), 1.0]
        }
    }
}