use bevy_ecs::prelude::*;
use ultraviolet::Vec2;

use crate::{config::Config, simulation::RigidCircle};

#[derive(StageLabel)]
pub struct PositionUpdate;

pub fn update_positions(mut query: Query<&mut RigidCircle>, config: Res<Config>) {
    let bounds = config.bounds;

    query.par_for_each_mut(1024, |mut circ| {
        circ.pos = circ.pos + circ.vel;

        if ((circ.pos.x - circ.radius) <= bounds.0.x) || ((circ.pos.x + circ.radius) >= bounds.1.x) {
            circ.vel.x = -circ.vel.x;
        }
        if ((circ.pos.y - circ.radius) <= bounds.0.y) || ((circ.pos.y + circ.radius) >= bounds.1.y) {
            circ.vel.y = -circ.vel.y;
        }

        let r = Vec2::broadcast(circ.radius);
        circ.pos.clamp(bounds.0 + r, bounds.1 - r);
    });
}
