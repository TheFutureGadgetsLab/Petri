use bevy_ecs::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;
use spin::RwLock;
use ultraviolet::Vec2;

use crate::simulation::RigidCircle;

#[derive(Resource)]
pub struct DenseGrid {
    /// log2(ncells_side)
    log2_side: u32,
    /// log2(cell_size)
    log2_cell: u32,

    pub cells: Vec<Cell>,
}

impl DenseGrid {
    pub fn new(cell_size: u32, side_len: u32) -> Self {
        assert!(side_len.is_power_of_two());
        assert!(cell_size.is_power_of_two());
        let ncells_side = side_len / cell_size;
        Self {
            log2_side: ncells_side.ilog2(),
            log2_cell: cell_size.ilog2(),
            cells: (0..(ncells_side * ncells_side)).map(|_| Cell::default()).collect(),
        }
    }

    pub fn insert(&self, circ: Collider, entity: Entity) {
        let ind = self.flat_ind(&circ);
        let cell = self.cells.get(ind).unwrap();
        cell.insert(&circ, entity);
    }

    pub fn flat_ind(&self, circ: &Collider) -> usize {
        let x = (circ.pos.x as u32) >> self.log2_cell;
        let y = (circ.pos.y as u32) >> self.log2_cell;
        ((y << self.log2_side) | x) as usize
    }

    pub fn clear(&mut self) {
        self.cells.par_iter().for_each(|cell| cell.clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32, ignore: Entity) -> Vec<&Collider> {
        let radius2 = radius.powi(2);
        let mut hits = Vec::with_capacity(2);

        for ind in self.cell_range(pos, radius) {
            if let Some(cell) = self.cells.get(ind as usize) {
                // We know this is at a read only stage. Safe to disregard lock
                hits.extend(cell.unlock_unsafe().iter().filter_map(|(other, id)| {
                    if (*id != ignore) && ((pos - other.pos).mag_sq() < radius2) {
                        Some(other)
                    } else {
                        None
                    }
                }));
            }
        }

        hits
    }

    pub fn cell_range(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = u32> {
        let x1 = ((pos.x - radius) as u32) >> self.log2_cell;
        let y1 = ((pos.y - radius) as u32) >> self.log2_cell;
        let x2 = ((pos.x + radius) as u32) >> self.log2_cell;
        let y2 = ((pos.y + radius) as u32) >> self.log2_cell;

        let shift = self.log2_side;

        (x1..=x2).cartesian_product(y1..=y2).map(move |(x, y)| (y << shift) | x)
    }
}

#[derive(Default)]
pub struct Cell {
    ents: RwLock<Vec<(Collider, Entity)>>,
}

impl Cell {
    pub fn insert(&self, circ: &Collider, entity: Entity) {
        self.ents.write().push((*circ, entity));
    }

    pub fn clear(&self) {
        self.ents.write().clear();
    }

    pub fn unlock_unsafe(&self) -> &Vec<(Collider, Entity)> {
        unsafe { return &*self.ents.as_mut_ptr() }
    }
}

#[derive(Clone, Copy)]
pub struct Collider {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}

// From Mut<RigidCircle> to Collider, requires lifetime
impl From<&RigidCircle> for Collider {
    fn from(circ: &RigidCircle) -> Self {
        Self {
            pos: circ.pos,
            vel: circ.vel,
            radius: circ.radius,
        }
    }
}

impl From<Mut<'_, RigidCircle>> for Collider {
    fn from(circ: Mut<'_, RigidCircle>) -> Self {
        Self {
            pos: circ.pos,
            vel: circ.vel,
            radius: circ.radius,
        }
    }
}
