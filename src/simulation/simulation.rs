use glam::Vec2;
use legion::*;

use super::{RigidCircle, components, config::Config, time::Time};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
    pub bounds: (Vec2, Vec2)
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(Time::default());
        resources.insert(config);

        for _i in 1..config.num_particles {
            world.push( (
                components::RigidCircle::new_rand(10.0, &config.bounds),
            ));
        }

        Simulation {
            world,
            resources,
            bounds: config.bounds
        }
    }

    pub fn update(&mut self) {
        self.resources.get_mut::<Time>().unwrap().tick();

        let mut query = <&mut RigidCircle>::query();

        let bounds = &self.bounds;

        query.par_for_each_mut(&mut self.world, |circ| {
            circ.pos += circ.vel;
            if circ.pos.x < bounds.0.x || circ.pos.x > bounds.1.x {
                circ.pos.x = circ.pos.x.clamp(bounds.0.x, bounds.1.x);
                circ.vel.x = -circ.vel.x;
            }
            if circ.pos.y < bounds.0.y || circ.pos.y > bounds.1.y {
                circ.pos.y = circ.pos.y.clamp(bounds.0.y, bounds.1.y);
                circ.vel.y = -circ.vel.y;
            }
        });
    }
}
