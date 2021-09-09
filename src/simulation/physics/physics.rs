use legion::{storage::Component, *};

use super::{
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
        self.detect_collisions(world);
    }

    fn update_positions(&mut self, world: &mut World, resources: &Resources) {
        time_func!(physics, pos_update);

        let bounds = resources.get::<Config>().unwrap().bounds;

        self.grid.clear();
        <(Entity, &mut RigidCircle)>::query().par_for_each_mut(world, |(entity, circ)| {
            circ.vel = circ.to_vel;
            circ.pos = circ.to_pos + circ.vel;

            if (circ.pos.x - circ.radius) <= bounds.0.x || (circ.pos.x + circ.radius) >= bounds.1.x {
                circ.pos.x = circ.pos.x.clamp(bounds.0.x + circ.radius, bounds.1.x - circ.radius);
                circ.vel.x = -circ.vel.x;
            }
            if (circ.pos.y - circ.radius) <= bounds.0.y || (circ.pos.y + circ.radius) > bounds.1.y {
                circ.pos.y = circ.pos.y.clamp(bounds.0.y + circ.radius, bounds.1.y - circ.radius);
                circ.vel.y = -circ.vel.y;
            }

            circ.to_vel = circ.vel;
            circ.to_pos = circ.pos;
            self.grid.insert(circ.pos, *entity);
        });
    }

    fn detect_collisions(&self, world: &mut World) {
        time_func!(physics, col_detect);

        let mut q = <(Entity, &mut RigidCircle)>::query().filter(component::<RigidCircle>());
        unsafe {
            q.par_for_each_unchecked(world, |(ent, c)| {
                let around = self.grid.query(c.pos, 2.0 * c.radius, *ent);
                around.iter().for_each(|e| {
                    elastic_collision(c, self.unsafe_component(world, *e));
                });
            });
        }
    }

    fn unsafe_component<'a, T: Component>(&self, world: &'a World, entity: Entity) -> &'a mut T {
        unsafe {
            world
                .entry_ref(entity)
                .unwrap()
                .into_component_unchecked::<T>()
                .unwrap()
        }
    }
}

/// Elastic collision between two circles.
/// Updates RigidCircles in place
fn elastic_collision(a: &mut RigidCircle, b: &RigidCircle) {
    let del = b.pos - a.pos;
    let dist = del.length();
    let norm = dist.powi(2);
    let vdel = b.vel - a.vel;

    a.to_vel += ((vdel).dot(del) / norm) * del;
    a.to_pos -= del / dist * (a.radius * 2.0 - dist) * 0.5;
}
