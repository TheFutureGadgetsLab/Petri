use bevy_ecs::prelude::*;

use super::{
    collision::{collision_resolution, CollisionResolution},
    position_update::{update_positions, PositionUpdate},
};
use crate::{config::Config, simulation::physics::DenseGrid, timing::timer::time_func};

pub struct PhysicsPipeline {
    system: Schedule,
}

impl PhysicsPipeline {
    pub fn new(world: &mut World, config: &Config) -> Self {
        let grid = DenseGrid::new((config.cell_radius * 32.0) as u32, (config.bounds.1.x) as u32);

        let mut system = Schedule::from_world(world);

        system.add_stage(PositionUpdate, SystemStage::parallel());
        system.add_system_to_stage(PositionUpdate, update_positions);

        system.add_stage(CollisionResolution, SystemStage::parallel());
        system.add_system_to_stage(CollisionResolution, collision_resolution);

        world.insert_resource(grid);

        Self { system }
    }

    pub fn step(&mut self, world: &mut World) {
        time_func!("physics.step");

        self.update_positions(world);
        self.detect_collisions(world);
    }

    fn update_positions(&mut self, world: &mut World) {
        time_func!("physics.pos_update");
        self.system
            .get_stage_mut::<SystemStage>(PositionUpdate)
            .unwrap()
            .run(world);
    }

    fn detect_collisions(&mut self, world: &mut World) {
        time_func!("physics.col_detect");
        self.system
            .get_stage_mut::<SystemStage>(CollisionResolution)
            .unwrap()
            .run(world);
    }
}
