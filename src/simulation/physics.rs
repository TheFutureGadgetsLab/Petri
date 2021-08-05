use legion::*;
use crate::simulation::RigidCircle;

pub struct PhysicsSystem {
    schedule: Schedule
}


impl PhysicsSystem {
    pub fn default() -> PhysicsSystem {
        let schedule = Schedule::builder()
            .add_system(update_positions_system())
            .build();

        PhysicsSystem {
            schedule: schedule
        }
    }
    
    pub fn step(&mut self, world: &mut World, resources: &mut Resources) {
        self.schedule.execute(world, resources);
    }
}

#[system(par_for_each)]
pub fn update_positions(circ: &mut RigidCircle) {
    circ.pos += circ.vel;
}