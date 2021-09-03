use flat_spatial::{grid::GridHandle, DenseGrid};
use legion::*;

type Grid = DenseGrid<Entity>;

use super::{config::Config, RigidCircle};

struct Col {
    a: Entity,
    b: Entity,
}

pub struct PhysicsPipeline {
    grid: Grid,
    schedule: Schedule,
}

impl PhysicsPipeline {
    pub fn new(world: &mut World, config: &Config) -> Self {
        let mut grid: Grid = Grid::new((config.cell_radius * 2.0) as _);

        for (entity, circ) in <(Entity, &mut RigidCircle)>::query().iter_mut(world) {
            let handle = grid.insert(circ.pos, *entity);
            circ.handle = handle;
        }

        let schedule = Schedule::builder().add_system(update_positions_system()).build();

        Self { grid, schedule }
    }

    pub fn step(&mut self, world: &mut World, resources: &mut Resources) {
        self.schedule.execute(world, resources);

        for circ in <&RigidCircle>::query().iter(world) {
            self.grid.set_position(circ.handle, circ.pos);
        }

        self.grid.maintain();

        let mut cols = vec![];
        <(Entity, &RigidCircle)>::query().for_each(world, |(ent, circ)| {
            let around: Vec<GridHandle> = self
                .grid
                .query_around(circ.pos, circ.radius * 2.0)
                .map(|(handle, ..)| handle)
                .collect();
            cols.extend(
                around
                    .iter()
                    .map(|handle| *self.grid.get(*handle).unwrap().1)
                    .filter_map(|e| match e != *ent {
                        true => Some(Col { a: e, b: *ent }),
                        false => None,
                    }),
            );
        });

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
