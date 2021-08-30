use rand::prelude::*;
use glam::{Vec2, vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidCircle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: [f32; 4],
}

impl RigidCircle {
    pub fn new_rand(radius: f32, bounds: &(Vec2, Vec2)) -> RigidCircle {
        let width = bounds.1.x - bounds.0.x;
        let height = bounds.1.y - bounds.0.y;
        let shortest_side = f32::min(width, height);
        let pos = vec2(
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..1.0)
        ).normalize() * shortest_side * 0.25;

        let vel = pos * vec2(
            thread_rng().gen_range(-0.005..0.005), 
            thread_rng().gen_range(-0.005..0.005));

        RigidCircle {
            pos,
            vel,
            radius,
            color: [thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0), 1.0]
        }
    }
}