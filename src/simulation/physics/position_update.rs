use bevy_ecs::prelude::*;
use ultraviolet::Vec2;

use crate::{
    config::Config,
    simulation::{physics::DenseGrid, RigidCircle},
};

#[derive(StageLabel)]
pub struct PositionUpdate;

pub fn update_positions(
    mut query: Query<(&mut RigidCircle, Entity)>,
    mut grid: ResMut<DenseGrid>,
    config: Res<Config>,
) {
    let bounds = config.bounds;
    grid.clear();

    query.par_for_each_mut(1024, |(mut circ, entity)| {
        circ.vel = circ.to_vel;
        circ.pos = circ.to_pos + circ.vel;

        if ((circ.pos.x - circ.radius) <= bounds.0.x) || ((circ.pos.x + circ.radius) >= bounds.1.x) {
            circ.vel.x = -circ.vel.x;
        }
        if ((circ.pos.y - circ.radius) <= bounds.0.y) || ((circ.pos.y + circ.radius) > bounds.1.y) {
            circ.vel.y = -circ.vel.y;
        }

        let r = circ.radius;
        circ.pos
            .clamp(bounds.0 + Vec2::broadcast(r), bounds.1 - Vec2::broadcast(r));
        circ.to_vel = circ.vel;
        circ.to_pos = circ.pos;
        grid.insert(circ.into(), entity);
    });
}
