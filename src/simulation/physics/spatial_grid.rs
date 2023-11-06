use bevy_ecs::prelude::*;
use glam::Vec2;
use rayon::prelude::*;

use crate::simulation::RigidCircle;

#[derive(Resource)]
pub struct DenseGrid {
    pub cell_size: i32,
    pub sim_size: i32,
    pub ncells_side: i32,

    pub cells: Vec<Cell>,
}

impl DenseGrid {
    pub fn new(cell_size: i32, sim_size: i32) -> Self {
        let ncells_side = sim_size / cell_size;
        Self {
            cell_size,
            sim_size,
            cells: (0..(ncells_side * ncells_side)).map(|_| Cell::default()).collect(),
            ncells_side,
        }
    }

    pub fn insert(&self, circ: &RigidCircle, entity: Entity) {
        let ind = self.flat_ind(circ.pos);
        let cell = self.cells.get(ind as usize).unwrap();
        cell.insert(circ, entity);
    }

    pub fn flat_ind(&self, pos: Vec2) -> i32 {
        let loc = pos.as_ivec2() / self.cell_size;
        loc.y * self.ncells_side + loc.x
    }

    pub fn clear(&self) {
        self.cells.par_iter().for_each(|cell| cell.clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32, ignore: Entity) -> Vec<RigidCircle> {
        let mut hits = Vec::new();

        for ind in self.cell_range(pos, radius) {
            let cell = self.cells.get(ind).unwrap();
            // We know this is at a read only stage. Safe to disregard lock
            hits.extend(cell.unlock_unsafe().iter().filter_map(|(other, id)| {
                let hit = (pos - other.pos).length_squared() < (radius + other.radius).powi(2);
                if (*id != ignore) && hit {
                    Some(*other)
                } else {
                    None
                }
            }));
        }

        hits
    }

    pub fn cell_range(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = usize> {
        let cs = self.cell_size as f32;
        let nc = self.ncells_side as f32;

        let rc = pos / cs;
        let radius = radius / cs;

        let rmin = (rc.y - radius).max(0.0) as i32;
        let rmax = (rc.y + radius).min(nc - 1.0) as i32;
        let cmin = (rc.x - radius).max(0.0) as i32;
        let cmax = (rc.x + radius).min(nc - 1.0) as i32;

        let shift = self.ncells_side;
        (rmin..=rmax).flat_map(move |r| (cmin..=cmax).map(move |c| (r * shift + c) as usize))
    }
}

#[derive(Default)]
pub struct Cell {
    ents: std::sync::Mutex<Vec<(RigidCircle, Entity)>>,
}

impl Cell {
    pub fn insert(&self, circ: &RigidCircle, entity: Entity) {
        self.ents.lock().unwrap().push((*circ, entity));
    }

    pub fn clear(&self) {
        self.ents.lock().unwrap().clear();
    }

    pub fn unlock_unsafe(&self) -> Vec<(RigidCircle, Entity)> {
        self.ents.lock().unwrap().clone()
    }
}
