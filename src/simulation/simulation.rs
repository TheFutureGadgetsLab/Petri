use bevy_ecs::prelude::*;

use crate::{
    config::Config,
    simulation::{components, time::Time, PhysicsPipeline},
};

pub struct Simulation {
    pub world: World,
    pub physics: PhysicsPipeline,
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();

        world.insert_resource(Time::default());
        world.insert_resource(config);

        for _i in 0..config.n_cells {
            world.spawn((
                components::RigidCircle::new_rand(&config),
                components::Color::new_rand(),
            ));
        }

        let physics = PhysicsPipeline::new(&mut world, &config);

        Simulation { world, physics }
    }

    /// Returns false if the simulation should stop
    pub fn update(&mut self) -> bool {
        {
            let mut timer = self.world.get_resource_mut::<Time>().unwrap();
            timer.tick();
        }
        let timer = self.world.get_resource::<Time>().unwrap();
        let config = self.world.get_resource::<Config>().unwrap();

        if (config.max_ticks > 0) && (timer.tick > config.max_ticks) {
            return false;
        }

        self.physics.step(&mut self.world);

        true
    }
}
