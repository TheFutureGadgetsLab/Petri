use flat_spatial::{grid::GridHandle, DenseGrid};
use glam::Vec2;
use legion::*;

type Grid = DenseGrid<Entity>;

use super::{components, config::Config, time::Time, RigidCircle};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
    schedule: Schedule,
    grid: Grid,
}

struct Col {
    a: Entity,
    b: Entity,
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(Time::default());
        resources.insert(config);

        let mut grid = Grid::new((config.cell_radius * 2.0) as _);
        for _i in 0..config.n_cells {
            world.push((components::RigidCircle::new_rand(&config),));
        }

        for (entity, circ) in <(Entity, &mut RigidCircle)>::query().iter_mut(&mut world) {
            let handle = grid.insert(circ.pos, *entity);
            circ.handle = handle;
        }

        let schedule = Schedule::builder().add_system(update_positions_system()).build();

        Simulation {
            world,
            resources,
            schedule,
            grid,
        }
    }

    pub fn update(&mut self) {
        self.resources.get_mut::<Time>().unwrap().tick();
        self.schedule.execute(&mut self.world, &mut self.resources);
        let config = self.resources.get::<Config>().unwrap();

        for circ in <&RigidCircle>::query().iter(&self.world) {
            self.grid.set_position(circ.handle, circ.pos);
        }

        self.grid.maintain();

        let mut cols = vec![];
        <(Entity, &RigidCircle)>::query().for_each(&self.world, |(ent, circ)| {
            let around: Vec<GridHandle> = self
                .grid
                .query_around(circ.pos, circ.radius * 2.0)
                .map(|(handle, ..)| handle)
                .collect();

            around
                .iter()
                .map(|handle| *self.grid.get(*handle).unwrap().1)
                .filter(|e| e != ent)
                .for_each(|e| cols.push(Col { a: *ent, b: e }));
        });

        for col in cols {
            let RigidCircle {
                pos: c1,
                vel: v1,
                radius: r1,
                ..
            } = self
                .world
                .entry_ref(col.a)
                .unwrap()
                .get_component::<RigidCircle>()
                .unwrap()
                .clone();
            let RigidCircle {
                pos: c2,
                vel: v2,
                radius: r2,
                ..
            } = self
                .world
                .entry_ref(col.b)
                .unwrap()
                .get_component::<RigidCircle>()
                .unwrap()
                .clone();

            let norm = c1.distance_squared(c2);
            let dist = norm.sqrt();

            if dist > (r1 + r2) {
                continue;
            }

            let m1: f32 = 1.0;
            let m2: f32 = 1.0;

            let msum = m1 + m2;

            let nv1 = v1 - ((2. * m2) / msum * (v1 - v2).dot(c1 - c2) / norm) * (c1 - c2);
            let nv2 = v2 - ((2. * m1) / msum * (v2 - v1).dot(c2 - c1) / norm) * (c2 - c1);

            let del = c2 - c1;
            let nc1 = c1 - del / dist * (r1 * 2.0 - dist).max(0.0) * 0.5;
            let nc2 = c2 + del / dist * (r2 * 2.0 - dist).max(0.0) * 0.5;

            {
                let mut b_ent = self.world.entry_mut(col.b).unwrap();
                let b = b_ent.get_component_mut::<RigidCircle>().unwrap();
                b.vel = nv2;
                b.pos = nc2;

                let mut a_ent = self.world.entry_mut(col.a).unwrap();
                let a = a_ent.get_component_mut::<RigidCircle>().unwrap();
                a.vel = nv1;
                a.pos = nc1;
            }
        }
    }
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
