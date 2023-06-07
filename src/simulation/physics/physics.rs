use bevy_ecs::prelude::*;

use super::{
    collision::{collision_resolution, grid_build},
    position_update::{update_positions},
};
use crate::{config::Config, simulation::physics::DenseGrid, timing::timer::time_func};

pub struct PhysicsPipeline {
    pos_scheduler: Schedule,
    grid_build_scheduler: Schedule,
    collision_scheduler: Schedule
}

impl PhysicsPipeline {
    pub fn new(world: &mut World, config: &Config) -> Self {
        let grid = DenseGrid::new(config.spatial_hash_cell_size as i32, (config.bounds.1.x) as i32);

        let mut pos_scheduler = Schedule::from_world(world);
        pos_scheduler.add_system(update_positions);
        pos_scheduler.set_executor_kind(bevy_ecs::schedule::ExecutorKind::MultiThreaded);

        let mut grid_build_scheduler = Schedule::from_world(world);
        grid_build_scheduler.add_system(grid_build);
        grid_build_scheduler.set_executor_kind(bevy_ecs::schedule::ExecutorKind::MultiThreaded);

        let mut collision_scheduler = Schedule::from_world(world);
        collision_scheduler.add_system(collision_resolution);
        collision_scheduler.set_executor_kind(bevy_ecs::schedule::ExecutorKind::MultiThreaded);

        world.insert_resource(grid);
        Self {
            pos_scheduler,
            grid_build_scheduler,
            collision_scheduler
        }
    }

    pub fn step(&mut self, world: &mut World) {
        time_func!("physics.step");

        self.update_positions(world);
        self.build_grid(world);
        self.resolve_collisions(world);
    }

    fn update_positions(&mut self, world: &mut World) {
        time_func!("physics.pos_update");
        self.pos_scheduler.run(world);
    }

    fn build_grid(&mut self, world: &mut World) {
        time_func!("physics.grid_build");
        self.grid_build_scheduler.run(world);
    }

    fn resolve_collisions(&mut self, world: &mut World) {
        time_func!("physics.col_detect");
        self.collision_scheduler.run(world);
    }
}
