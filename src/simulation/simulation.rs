use legion::*;

use crate::{
    config::Config,
    simulation::{components, time::Time, PhysicsPipeline},
};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
    pub physics: PhysicsPipeline,
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(Time::default());
        resources.insert(config);

        for _i in 0..config.n_cells {
            world.push((
                components::RigidCircle::new_rand(&config),
                components::Color::new_rand(),
            ));
        }

        let physics = PhysicsPipeline::new(&mut world, &config);

        Simulation {
            world,
            resources,
            physics,
        }
    }

    /// Returns false if the simulation should stop
    pub fn update(&mut self) -> bool {
        {
            let mut timer = self.resources.get_mut::<Time>().unwrap();
            let config = self.resources.get::<Config>().unwrap();

            timer.tick();
            if (config.max_ticks > 0) && (timer.tick > config.max_ticks) {
                return false;
            }
        }
        self.resources.get_mut::<Time>().unwrap().tick();
        self.physics.step(&mut self.world, &mut self.resources);

        true
    }
}
