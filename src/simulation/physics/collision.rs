use bevy_ecs::prelude::*;

use super::Collider;
use crate::simulation::{physics::DenseGrid, RigidCircle};

#[derive(StageLabel)]
pub struct CollisionResolution;

pub fn collision_resolution(mut query: Query<(&mut RigidCircle, Entity)>, grid: Res<DenseGrid>) {
    query.par_for_each_mut(1024, |(mut circ, entity)| {
        let around = grid.query(circ.pos, 2.0 * circ.radius, entity);
        around.iter().for_each(|e| {
            elastic_collision(&mut circ, e);
        });
    })
}

fn elastic_collision(c1: &mut Mut<RigidCircle>, c2: &Collider) {
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
