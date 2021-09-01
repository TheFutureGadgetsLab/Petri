use legion::*;

use super::{components, config::Config, time::Time, RigidCircle};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
    schedule: Schedule,
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(Time::default());
        resources.insert(config);

        for _i in 0..config.n_cells {
            world.push((components::RigidCircle::new_rand(&config),));
        }

        let schedule = Schedule::builder().add_system(update_positions_system()).build();

        Simulation {
            world,
            resources,
            schedule,
        }
    }

    pub fn update(&mut self) {
        self.resources.get_mut::<Time>().unwrap().tick();
        self.schedule.execute(&mut self.world, &mut self.resources);
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
