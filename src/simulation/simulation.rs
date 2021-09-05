use legion::*;

use super::{components, config::Config, time::Time, PhysicsPipeline};

pub struct Simulation {
    pub world: World,
    pub resources: Resources,
    pub physics: PhysicsPipeline,
}

impl Simulation {
    pub fn new(config: Config) -> Simulation {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(Time::default());
        resources.insert(config);

        for _i in 0..config.n_cells {
            world.push((
                components::RigidCircle::new_rand(&config),
                components::Color::new_rand(),
            ));
        }

        let physics = PhysicsPipeline::new(&mut world, &config);

        Simulation {
            world,
            resources,
            physics,
        }
    }

    pub fn update(&mut self) {
        self.resources.get_mut::<Time>().unwrap().tick();
        self.physics.step(&mut self.world, &mut self.resources);
    }
}
