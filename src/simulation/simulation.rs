use bevy_ecs::prelude::*;

use crate::{
    config::Config,
    simulation::{components, time::Ticker, PhysicsPipeline},
};

pub struct Simulation {
    pub world: World,
    pub physics: PhysicsPipeline,
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();

        world.insert_resource(Ticker::default());
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
    pub fn step(&mut self) {
        self._tick_timer();
        self.physics.step(&mut self.world);
    }

    fn _tick_timer(&mut self) {
        let mut timer = self.world.get_resource_mut::<Ticker>().unwrap();
        timer.tick();
    }

    pub fn get_config(&self) -> &Config {
        return self.world.get_resource::<Config>().unwrap();
    }

    pub fn get_config_mut(&mut self) -> Mut<Config> {
        let config = self.world.get_resource_mut::<Config>().unwrap();
        config
    }

    pub fn should_step(&self) -> bool {
        let target_delta = 1.0 / (self.get_config().max_sim_tps as f32);
        let timer = self.world.get_resource::<Ticker>().unwrap();
        if timer.delta_time().as_secs_f32() > target_delta {
            return true;
        }
        false
    }
}
