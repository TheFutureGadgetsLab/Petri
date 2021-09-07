use legion::*;

use super::{
    collision_structures::{Collision, CollisionSet},
    spatial_grid::DenseGrid,
};
use crate::{
    simulation::{Config, RigidCircle},
    timing::timer::time_func,
};

pub struct PhysicsPipeline {
    grid: DenseGrid,
}

impl PhysicsPipeline {
    pub fn new(_world: &mut World, config: &Config) -> Self {
        let grid = DenseGrid::new((config.cell_radius * 32.0) as u32, (config.bounds.1.x) as u32);

        Self { grid }
    }

    pub fn step(&mut self, world: &mut World, resources: &mut Resources) {
        time_func!(physics, step);

        self.update_positions(world, resources);
        let cols = self.detect_collisions(world);
        self.resolve_collisions(world, &cols);
    }

    fn update_positions(&mut self, world: &mut World, _resources: &Resources) {
        time_func!(physics, pos_update);

        let bounds = self.grid.safe_bounds();

        self.grid.clear();
        <(Entity, &mut RigidCircle)>::query().par_for_each_mut(world, |(entity, circ)| {
            circ.pos += circ.vel;
            if (circ.pos.x - circ.radius) <= bounds.0.x || (circ.pos.x + circ.radius) >= bounds.1.x {
                circ.pos.x = circ.pos.x.clamp(bounds.0.x + circ.radius, bounds.1.x - circ.radius);
                circ.vel.x = -circ.vel.x;
            }
            if (circ.pos.y - circ.radius) <= bounds.0.y || (circ.pos.y + circ.radius) > bounds.1.y {
                circ.pos.y = circ.pos.y.clamp(bounds.0.y + circ.radius, bounds.1.y - circ.radius);
                circ.vel.y = -circ.vel.y;
            }

            self.grid.insert(circ.pos, *entity);
        });
    }

    fn detect_collisions(&self, world: &World) -> CollisionSet {
        time_func!(physics, col_detect);

        let cols: CollisionSet = CollisionSet::default();
        <(Entity, &RigidCircle)>::query().par_for_each(world, |(ent, circ)| {
            let around = self.grid.query(circ.pos, 2.0 * circ.radius, *ent);

            around.iter().for_each(|e| {
                cols.insert(Collision::new(*ent, *e));
            });
        });

        cols
    }

    fn resolve_collisions(&self, world: &mut World, cols: &CollisionSet) {
        time_func!(physics, col_resolve);

        cols.iter().for_each(|col| {
            let mut a = unsafe {
                world
                    .entry_ref(col.a)
                    .unwrap()
                    .into_component_unchecked::<RigidCircle>()
                    .unwrap()
            };
            let mut b = unsafe {
                world
                    .entry_ref(col.b)
                    .unwrap()
                    .into_component_unchecked::<RigidCircle>()
                    .unwrap()
            };

            elastic_collision(&mut a, &mut b);
        });
    }
}

/// Elastic collision between two circles.
/// Updates RigidCircles in place
fn elastic_collision(a: &mut RigidCircle, b: &mut RigidCircle) -> bool {
    let del = b.pos - a.pos;
    let dist = del.length();

    // No Collision
    if dist > (a.radius + b.radius) {
        return false;
    }

    let norm = del.length_squared();
    let vdel = b.vel - a.vel;

    a.vel -= ((-vdel).dot(-del) / norm) * (-del);
    b.vel -= ((vdel).dot(del) / norm) * (del);

    a.pos -= del / dist * (a.radius * 2.0 - dist) * 0.5;
    b.pos += del / dist * (b.radius * 2.0 - dist) * 0.5;

    true
}
