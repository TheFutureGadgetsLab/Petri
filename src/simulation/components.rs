use rand::prelude::*;
use crate::simulation::Config;
use glam::{Vec2, Vec4};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidCircle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}


impl RigidCircle {
    pub fn new_rand(conf: Config, radius: f32) -> RigidCircle {
        let w = conf.boundary.width;
        let h = conf.boundary.height;
        let t = conf.boundary.thickness;

        RigidCircle {
            pos: [
                thread_rng().gen_range((2.*t+radius)..(w-2.*t-radius)),
                thread_rng().gen_range((2.*t+radius)..(h-2.*t-radius)),
            ].into(),
            vel: [
                thread_rng().gen_range(-2.0..2.0),
                thread_rng().gen_range(-2.0..2.0),
            ].into(),
            radius,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boundary {
    pub pos: Vec2,
    pub bounds: Vec4
}

impl Boundary {
    pub fn new(conf: Config) -> Boundary {
        Boundary {
            pos: Vec2::ZERO,
            bounds: Vec4::new(0.0, 0.0, conf.boundary.width, conf.boundary.height)
        }
    }
}