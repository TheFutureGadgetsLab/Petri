use std::time::Instant;

use legion::*;

use super::spatial_grid::DenseGrid;
use crate::simulation::{Config, RigidCircle};

struct Col {
    a: Entity,
    b: Entity,
}

pub struct PhysicsPipeline {
    grid: DenseGrid,
    schedule: Schedule,
}

impl PhysicsPipeline {
    pub fn new(_world: &mut World, config: &Config) -> Self {
        let (min, max) = config.bounds;

        let grid = DenseGrid::new(config.cell_radius * 2.0, min, max);

        let schedule = Schedule::builder().add_system(update_positions_system()).build();

        Self { grid, schedule }
    }

    pub fn step(&mut self, world: &mut World, resources: &mut Resources) {
        self.schedule.execute(world, resources);

        self.grid.clear();
        for (entity, circ) in <(Entity, &RigidCircle)>::query().iter(world) {
            self.grid.insert(circ.pos, *entity);
        }

        let start = Instant::now();
        let mut cols = vec![];
        <(Entity, &RigidCircle)>::query().for_each(world, |(ent, circ)| {
            let around = self.grid.query(circ.pos, 2.0 * circ.radius);
            cols.extend(around.iter().filter_map(|e| match e != ent {
                true => Some(Col { a: *e, b: *ent }),
                false => None,
            }));
        });

        println!(
            "Collision detection / s: {}",
            1.0 / (Instant::now() - start).as_secs_f32()
        );

        for col in cols.iter() {
            self.resolve_collision(world, col);
        }
    }

    fn resolve_collision(&mut self, world: &mut World, col: &Col) {
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
    }
}

/// Updates Rigid Circles in place
fn elastic_collision(a: &mut RigidCircle, b: &mut RigidCircle) -> bool {
    let norm = a.pos.distance_squared(b.pos);
    let dist = norm.sqrt();

    // No Collision
    if dist > (a.radius + b.radius) {
        return false;
    }

    let m1: f32 = 1.0;
    let m2: f32 = 1.0;

    let msum = m1 + m2;

    let avel = a.vel - ((2. * m2) / msum * (a.vel - b.vel).dot(a.pos - b.pos) / norm) * (a.pos - b.pos);
    let bvel = b.vel - ((2. * m1) / msum * (b.vel - a.vel).dot(b.pos - a.pos) / norm) * (b.pos - a.pos);

    a.vel = avel;
    b.vel = bvel;

    let del = b.pos - a.pos;
    a.pos -= del / dist * (a.radius * 2.0 - dist).max(0.0) * 0.5;
    b.pos += del / dist * (b.radius * 2.0 - dist).max(0.0) * 0.5;

    true
}

#[system(par_for_each)]
fn update_positions(circ: &mut RigidCircle, #[resource] config: &Config) {
    let bounds = config.bounds;

    circ.pos += circ.vel;
    if (circ.pos.x - circ.radius) <= bounds.0.x || (circ.pos.x + circ.radius) >= bounds.1.x {
        circ.pos.x = circ.pos.x.clamp(bounds.0.x + circ.radius, bounds.1.x - circ.radius);
        circ.vel.x = -circ.vel.x;
    }
    if (circ.pos.y - circ.radius) <= bounds.0.y || (circ.pos.y + circ.radius) > bounds.1.y {
        circ.pos.y = circ.pos.y.clamp(bounds.0.y + circ.radius, bounds.1.y - circ.radius);
        circ.vel.y = -circ.vel.y;
    }
}
