use bevy_ecs::prelude::*;
use parking_lot::RwLock;
use rayon::prelude::*;
use ultraviolet::Vec2;

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
        // Calculate the cell index
        let r = (pos.y as i32) / self.cell_size;
        let c = (pos.x as i32) / self.cell_size;
        r * self.ncells_side + c
    }

    pub fn clear(&mut self) {
        self.cells.par_iter().for_each(|cell| cell.clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32, ignore: Entity) -> Vec<&RigidCircle> {
        let mut hits = Vec::new();

        for ind in self.cell_range(pos, radius) {
            if let Some(cell) = self.cells.get(ind) {
                // We know this is at a read only stage. Safe to disregard lock
                hits.extend(cell.unlock_unsafe().iter().filter_map(|(other, id)| {
                    let hit = (pos - other.pos).mag_sq() < (radius + other.radius).powi(2);
                    if (*id != ignore) && hit {
                        Some(other)
                    } else {
                        None
                    }
                }));
            }
        }

        hits
    }

    pub fn cell_range(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = usize> {
        let r = (pos.y as i32) / self.cell_size;
        let c = (pos.x as i32) / self.cell_size;

        let shift = self.ncells_side;
        let radius_cells = (radius / self.cell_size as f32) as i32;
        let rmin = (r - radius_cells).max(0);
        let rmax = (r + radius_cells).min(self.ncells_side - 1);
        let cmin = (c - radius_cells).max(0);
        let cmax = (c + radius_cells).min(self.ncells_side - 1);

        (rmin..=rmax).flat_map(move |r| (cmin..=cmax).map(move |c| (r * shift + c) as usize))
    }
}

#[derive(Default)]
pub struct Cell {
    ents: RwLock<Vec<(RigidCircle, Entity)>>,
}

impl Cell {
    pub fn insert(&self, circ: &RigidCircle, entity: Entity) {
        self.ents.write().push((*circ, entity));
    }

    pub fn clear(&self) {
        self.ents.write().clear();
    }

    pub fn unlock_unsafe(&self) -> &Vec<(RigidCircle, Entity)> {
        unsafe { &*self.ents.data_ptr() }
    }
}
