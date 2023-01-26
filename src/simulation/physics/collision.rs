use bevy_ecs::prelude::*;
use ultraviolet::Vec2;

use crate::simulation::{physics::DenseGrid, RigidCircle};

#[derive(StageLabel)]
pub struct CollisionResolution;

pub fn collision_resolution(mut query: Query<(&mut RigidCircle, Entity)>, grid: Res<DenseGrid>) {
    query.par_for_each_mut(1024, |(mut circ, entity)| {
        let around = grid.query(circ.pos, 2.0 * circ.radius, entity);
        let impulse = around.iter().map(|e| singular_resolution(&mut circ, e)).sum();
        circ.vel += impulse;
    })
}

fn singular_resolution(c1: &Mut<RigidCircle>, c2: &RigidCircle) -> Vec2 {
    let mut normal = c2.pos - c1.pos;
    let impulse = Vec2::zero();

    normal.normalize();
    let relative_velocity = c2.vel - c1.vel;
    let velocity_along_normal = relative_velocity.dot(normal);

    // Check if the circles are moving towards each other
    if velocity_along_normal > 0.0 {
        return impulse;
    }

    // Calculate the impulse scalar
    let e = 1.0; // coefficient of restitution
    let j = -(1.0 + e) * velocity_along_normal;
    let j = j / 2.0;

    // Apply the impulse
    let impulse = j * normal;
    return -impulse;
}
