use legion::*;

use super::{RigidCircle, components, config::Config, time::Time};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(Time::default());
        resources.insert(config);

        for _i in 1..config.num_particles {
            world.push( (
                components::RigidCircle::new_rand(10.0),
            ));
        }

        Simulation {
            world,
            resources
        }
    }

    pub fn update(&mut self) {
        self.resources.get_mut::<Time>().unwrap().tick();

        let mut query = <&mut RigidCircle>::query();

        query.par_for_each_mut(&mut self.world, |circ| {
            circ.pos += circ.vel;
        });
    }
}
