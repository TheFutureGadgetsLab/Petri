use flat_spatial::{grid::GridHandle, DenseGrid};
use legion::*;

type Grid = DenseGrid<Entity>;

use super::{components, config::Config, time::Time, RigidCircle};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
    schedule: Schedule,
    grid: Grid,
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

        for circ in <&RigidCircle>::query().iter(&self.world) {
            self.grid.set_position(circ.handle, circ.pos);
        }

        self.grid.maintain();

        for circ in <&RigidCircle>::query().iter(&self.world) {
            let around: Vec<GridHandle> = self
                .grid
                .query_around(circ.pos, circ.radius * 2.0)
                .map(|(handle, ..)| handle)
                .collect();

            let ents: Vec<Entity> = around.iter().map(|handle| *self.grid.get(*handle).unwrap().1).collect();
        }
    }
}

#[system(par_for_each)]
fn update_positions(circ: &mut RigidCircle, #[resource] config: &Config) {
    let bounds = config.bounds;

    circ.pos += circ.vel;
    if (circ.pos.x - circ.radius) <= bounds.0.x || (circ.pos.x + circ.radius) >= bounds.1.x {
        circ.pos.x = circ.pos.x.clamp(bounds.0.x, bounds.1.x);
        circ.vel.x = -circ.vel.x;
    }
    if (circ.pos.y - circ.radius) <= bounds.0.y || (circ.pos.y + circ.radius) > bounds.1.y {
        circ.pos.y = circ.pos.y.clamp(bounds.0.y, bounds.1.y);
        circ.vel.y = -circ.vel.y;
    }
}
