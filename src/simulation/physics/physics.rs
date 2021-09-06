use std::time::Instant;

use legion::*;

use super::{
    collision_structures::{Collision, CollisionSet},
    spatial_grid::DenseGrid,
};
use crate::{
    simulation::{Config, RigidCircle},
    timing::{registry::time_func, TIMING_DATABASE},
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
        let start = Instant::now();

        self.update_positions(world, resources);
        self.update_grid(world);

        let cols = self.detect_collisions(world);

        self.resolve_collisions(world, &cols);

        time_func!(physics, step, start);
    }

    fn update_positions(&self, world: &mut World, _resources: &Resources) {
        let start = Instant::now();

        let bounds = self.grid.safe_bounds();

        <&mut RigidCircle>::query().par_for_each_mut(world, |circ| {
            circ.pos += circ.vel;
            if (circ.pos.x - circ.radius) <= bounds.0.x || (circ.pos.x + circ.radius) >= bounds.1.x {
                circ.pos.x = circ.pos.x.clamp(bounds.0.x + circ.radius, bounds.1.x - circ.radius);
                circ.vel.x = -circ.vel.x;
            }
            if (circ.pos.y - circ.radius) <= bounds.0.y || (circ.pos.y + circ.radius) > bounds.1.y {
                circ.pos.y = circ.pos.y.clamp(bounds.0.y + circ.radius, bounds.1.y - circ.radius);
                circ.vel.y = -circ.vel.y;
            }

            circ.grid_ind = self.grid.flat_ind(circ.pos);
        });

        time_func!(physics, pos_update, start);
    }

    fn update_grid(&mut self, world: &World) {
        let start = Instant::now();

        self.grid.clear();
        <(Entity, &RigidCircle)>::query().for_each(world, |(entity, circ)| {
            self.grid.insert(circ.pos, *entity, circ.grid_ind);
        });

        time_func!(physics, grid_update, start);
    }

    fn detect_collisions(&self, world: &World) -> CollisionSet {
        let start = Instant::now();
        let cols: CollisionSet = CollisionSet::default();
        <(Entity, &RigidCircle)>::query().par_for_each(world, |(ent, circ)| {
            let around = self.grid.query(circ.pos, 2.0 * circ.radius, *ent);

            around.iter().for_each(|e| {
                cols.insert(Collision::new(*ent, *e));
            });
        });

        time_func!(physics, col_detect, start);
        cols
    }

    fn resolve_collisions(&self, world: &mut World, cols: &CollisionSet) {
        let start = Instant::now();

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

        time_func!(physics, col_resolve, start);
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
