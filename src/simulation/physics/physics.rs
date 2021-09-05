use std::{hash::Hash, time::Instant};

use dashmap::DashSet;
use fxhash::FxBuildHasher;
use legion::*;

type ColSet = DashSet<Col, FxBuildHasher>;

use super::spatial_grid::DenseGrid;
use crate::{
    simulation::{Config, RigidCircle},
    timing::TIMING_DATABASE,
};

#[derive(Eq)]
struct Col {
    a: Entity,
    b: Entity,
}

impl PartialEq for Col {
    fn eq(&self, other: &Self) -> bool {
        ((self.a == other.a) & (self.b == other.b)) || ((self.a == other.b) & (self.b == other.a))
    }
}

impl Hash for Col {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}

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

        TIMING_DATABASE.write().physics.step.update(Instant::now() - start);
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
        TIMING_DATABASE
            .write()
            .physics
            .pos_update
            .update(Instant::now() - start);
    }

    fn update_grid(&mut self, world: &World) {
        let start = Instant::now();

        self.grid.clear();
        <(Entity, &RigidCircle)>::query().for_each(world, |(entity, circ)| {
            self.grid.insert(circ.pos, *entity, circ.grid_ind);
        });

        TIMING_DATABASE
            .write()
            .physics
            .grid_update
            .update(Instant::now() - start);
    }

    fn detect_collisions(&self, world: &World) -> ColSet {
        let start = Instant::now();
        let cols: DashSet<Col, FxBuildHasher> = DashSet::with_hasher(FxBuildHasher::default());
        <(Entity, &RigidCircle)>::query().par_for_each(world, |(ent, circ)| {
            let around = self.grid.query(circ.pos, 2.0 * circ.radius, *ent);

            around.iter().for_each(|e| {
                cols.insert(Col { a: *ent, b: *e });
            });
        });

        TIMING_DATABASE
            .write()
            .physics
            .col_detect
            .update(Instant::now() - start);

        cols
    }

    fn resolve_collisions(&self, world: &mut World, cols: &ColSet) {
        let start = Instant::now();

        cols.iter().for_each(|col| {
            let mut a_copy = *world.entry_ref(col.a).unwrap().get_component::<RigidCircle>().unwrap();
            let mut b_copy = *world.entry_ref(col.b).unwrap().get_component::<RigidCircle>().unwrap();

            // Updates position and velocity
            if elastic_collision(&mut a_copy, &mut b_copy) {
                let mut b_ent = world.entry_mut(col.b).unwrap();
                let b = b_ent.get_component_mut::<RigidCircle>().unwrap();
                b.pos = b_copy.pos;
                b.vel = b_copy.vel;

                let mut a_ent = world.entry_mut(col.a).unwrap();
                let a = a_ent.get_component_mut::<RigidCircle>().unwrap();
                a.pos = a_copy.pos;
                a.vel = a_copy.vel;
            }
        });

        TIMING_DATABASE
            .write()
            .physics
            .col_resolve
            .update(Instant::now() - start);
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
