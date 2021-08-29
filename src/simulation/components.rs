use rand::prelude::*;
use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidCircle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: [f32; 4],
}

impl RigidCircle {
    pub fn new_rand(radius: f32) -> RigidCircle {
        let dir = Vec2::new(
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..1.0)
        ).normalize() * 300.0;

        RigidCircle {
            pos: Vec2::new(0.0, 0.0) + dir,
            vel: Vec2::new(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0)) * 0.05,
            radius,
            color: [thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0), 1.0]
        }
    }
}