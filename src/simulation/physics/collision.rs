use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::{
    simulation::{physics::DenseGrid, RigidCircle},
    timing::timer::time_func,
};

pub fn grid_build(query: Query<(&RigidCircle, Entity)>, mut grid: ResMut<DenseGrid>) {
    time_func!("physics.grid_build");

    grid.clear();
    query.par_iter().for_each(|(circ, entity)| {
        grid.insert(circ, entity);
    })
}

pub fn collision_resolution(mut query: Query<(&mut RigidCircle, Entity)>, grid: Res<DenseGrid>) {
    time_func!("physics.col_detect");
    query.par_iter_mut().for_each_mut(|(mut circ, entity)| {
        let around = grid.query(circ.pos, circ.radius, entity);
        let impulse: Vec2 = around.iter().map(|e| singular_resolution(&circ, e)).sum();
        circ.vel += impulse;
    });
}

fn singular_resolution(c1: &RigidCircle, c2: &RigidCircle) -> Vec2 {
    let normal = (c2.pos - c1.pos).normalize();

    let rel_vel = c2.vel - c1.vel;
    let vel_along_norm = rel_vel.dot(normal);

    // Check if the circles are moving towards each other
    let impulse = Vec2::ZERO;
    if vel_along_norm > 0.0 {
        return impulse;
    }
    vel_along_norm * normal
}
