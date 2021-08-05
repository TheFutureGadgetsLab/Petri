use legion::*;
use super::{
    components,
    physics::PhysicsSystem, 
    time::Time, 
    config::Config
};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
    pub physics: PhysicsSystem,
}

impl Simulation {
    pub fn new(conf: Config) -> Simulation {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(Time::default());
        resources.insert(conf);

        world.push((components::Boundary::new(conf),));

        for _i in 1..conf.num_particles {
            world.push( (
                components::RigidCircle::new_rand(conf, 1.0),
            ));
        }

        Simulation {
            world,
            resources,
            physics: PhysicsSystem::default()
        }
    }

    pub fn update(&mut self) {
        self.resources.get_mut::<Time>().unwrap().tick();
        self.physics.step(&mut self.world, &mut self.resources);
    }
}
