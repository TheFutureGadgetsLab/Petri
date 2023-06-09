use bevy_ecs::prelude::*;

use super::{
    collision::{collision_resolution, grid_build},
    position_update::update_positions,
};
use crate::{config::Config, simulation::physics::DenseGrid, timing::timer::time_func};
pub struct PhysicsPipeline {
    schedule: Schedule,
}

impl PhysicsPipeline {
    pub fn new(world: &mut World, config: &Config) -> Self {
        let grid = DenseGrid::new(config.spatial_hash_cell_size as i32, (config.bounds.1.x) as i32);

        let mut scheduler = Schedule::from_world(world);
        scheduler.add_systems((update_positions, grid_build, collision_resolution).chain());
        scheduler.set_executor_kind(bevy_ecs::schedule::ExecutorKind::MultiThreaded);

        world.insert_resource(grid);
        Self { schedule: scheduler }
    }

    pub fn step(&mut self, world: &mut World) {
        time_func!("physics.step");
        self.schedule.run(world);
    }
}
