use bevy_ecs::prelude::*;

use crate::{config::Config, simulation::RigidCircle, timing::timer::time_func};

pub fn update_positions(mut query: Query<&mut RigidCircle>, config: Res<Config>) {
    time_func!("physics.pos_update");

    let bounds = config.bounds;

    query.par_iter_mut().for_each_mut(|mut circ| {
        circ.pos = circ.pos + circ.vel;

        if ((circ.pos.x - circ.radius) <= bounds.0.x) || ((circ.pos.x + circ.radius) >= bounds.1.x) {
            circ.vel.x = -circ.vel.x;
        }
        if ((circ.pos.y - circ.radius) <= bounds.0.y) || ((circ.pos.y + circ.radius) >= bounds.1.y) {
            circ.vel.y = -circ.vel.y;
        }

        circ.pos = circ.pos.clamp(bounds.0 + circ.radius, bounds.1 - circ.radius);
    });
}
