use bevy_ecs::prelude::*;

use super::position_update::{update_positions, PositionUpdate};
use crate::{
    config::Config,
    simulation::{physics::DenseGrid, RigidCircle},
    timing::timer::time_func,
};

pub struct PhysicsPipeline {
    system: Schedule,
}

impl PhysicsPipeline {
    pub fn new(world: &mut World, config: &Config) -> Self {
        let grid = DenseGrid::new((config.cell_radius * 32.0) as u32, (config.bounds.1.x) as u32);

        let mut system = Schedule::from_world(world);
        system.add_stage(PositionUpdate, SystemStage::parallel());
        system.add_system_to_stage(PositionUpdate, update_positions);

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

    fn detect_collisions(&self, world: &mut World) {
        time_func!("physics.col_detect");
    }
}

fn collision_resolution(mut query: Query<Entity, With<RigidCircle>>, grid: Res<DenseGrid>) {
    query.par_for_each_mut(1024, |entity| {
        let circ = query.get_component_mut::<RigidCircle>(entity).unwrap();
        let around = grid.query(circ.pos, 2.0 * circ.radius, entity);
        around.iter().for_each(|e| {
            let res = .get_component::<RigidCircle>(*e).unwrap();
            elastic_collision(&mut circ, &res);
        });
    })
}

fn elastic_collision(c1: &mut RigidCircle, c2: &RigidCircle) {
    let mut normal = c2.pos - c1.pos;
    let dist_sq = normal.mag_sq();
    let radius_sum = c1.radius + c2.radius;
    if dist_sq > radius_sum * radius_sum {
        return;
    }

    normal.normalize();
    let relative_velocity = c2.vel - c1.vel;
    let velocity_along_normal = relative_velocity.dot(normal);

    // Check if the circles are moving towards each other
    if velocity_along_normal > 0.0 {
        return;
    }

    // Calculate the impulse scalar
    let e = 1.0; // coefficient of restitution
    let j = -(1.0 + e) * velocity_along_normal;
    // let j = j / (1.0 / c1.mass + 1.0 / c2.mass);
    let j = j / 2.0;

    // Apply the impulse
    let impulse = j * normal;
    // c1.to_vel += impulse / c1.mass;
    c1.to_vel -= impulse;

    // Update the positions
    c1.to_pos = c1.pos + c1.to_vel;
}
